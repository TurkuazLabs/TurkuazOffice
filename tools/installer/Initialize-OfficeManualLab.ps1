# 📄 Dosya Yolu: /tools/installer/Initialize-OfficeManualLab.ps1
# 📌 Amac: Portable Office LAB paketlerini yerel Windows yollarinda CMS imzali test manifestine hazirlar
# 📌 Modul - Tool PowerShell
# Version: 1.0.1
# Aciklama: Test kitini hash ile kontrol eder; sadece acik Prepare isteginde yeni yerel test sertifikasi ve dis trust olusturur
# Bagimli Oldugu Katman: Tool | Config | CI

[CmdletBinding()]
param(
    [ValidateSet('VerifyOnly', 'Prepare')][string]$Mode = 'VerifyOnly',
    [string]$AssetsPath = $PSScriptRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if ($PSVersionTable.PSVersion.Major -lt 7) {
    throw 'This LAB signing helper requires PowerShell 7 or later (pwsh).'
}
if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT -or
    -not [Environment]::Is64BitOperatingSystem) {
    throw 'Office manual LAB preparation requires Windows x64.'
}

$assets = (Resolve-Path -LiteralPath $AssetsPath).Path
$checksums = Join-Path $assets 'SHA256SUMS.txt'
if (-not (Test-Path -LiteralPath $checksums -PathType Leaf)) {
    throw 'GitHub LAB asset checksums are missing.'
}
$known = @{}
foreach ($line in [IO.File]::ReadAllLines($checksums)) {
    if ($line -notmatch '^([0-9a-fA-F]{64}) \*([a-zA-Z0-9_.-]+)$') {
        throw 'Unrecognized SHA256SUMS entry.'
    }
    $name = $Matches[2]
    if ($name -ceq 'SHA256SUMS.txt' -or $known.ContainsKey($name)) {
        throw 'Duplicate or self-referential SHA256SUMS entry.'
    }
    $known[$name] = $Matches[1]
}
foreach ($name in $known.Keys) {
    $filePath = Join-Path $assets $name
    if (-not (Test-Path -LiteralPath $filePath -PathType Leaf)) {
        throw "Expected LAB asset missing: $name"
    }
    $actual = (Get-FileHash -LiteralPath $filePath -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actual -cne $known[$name]) {
        throw "LAB asset checksum mismatch: $name"
    }
}
$allAssets = @(Get-ChildItem -LiteralPath $assets -File)
foreach ($file in $allAssets) {
    if ($file.Name -cne 'SHA256SUMS.txt' -and -not $known.ContainsKey($file.Name)) {
        throw "Unlisted extra LAB asset refused: $($file.Name)"
    }
}
$provenancePath = Join-Path $assets 'LAB_BUILD_PROVENANCE.json'
$provenance = Get-Content -LiteralPath $provenancePath -Raw | ConvertFrom-Json
if ($provenance.schema_version -ne 1 -or
    $provenance.lab_only -ne $true -or
    $provenance.production_release_approved -ne $false -or
    $provenance.independently_developed_office_versions -ne $false -or
    $provenance.signing_policy -cne 'UNSIGNED_LAB_ONLY_EPHEMERAL_CMS_IN_E2E') {
    throw 'LAB provenance metadata does not attest a disposable unsigned preview.'
}
$versionOne = [string]$provenance.release_version_v1
$versionTwo = [string]$provenance.release_version_v2
if ($versionOne -notmatch '^\d+\.\d+\.\d+$' -or
    $versionTwo -notmatch '^\d+\.\d+\.\d+$' -or
    $versionOne -ceq $versionTwo) {
    throw 'LAB version metadata is invalid.'
}
$setup = @(Get-ChildItem -LiteralPath $assets -File -Filter '*Setup.exe')
$fullV1 = @(Get-ChildItem -LiteralPath $assets -File -Filter "*-$versionOne-full.nupkg")
$fullV2 = @(Get-ChildItem -LiteralPath $assets -File -Filter "*-$versionTwo-full.nupkg")
if ($setup.Count -ne 1 -or $fullV1.Count -ne 1 -or $fullV2.Count -ne 1 -or
    @($provenance.packages).Count -ne 3) {
    throw 'Exactly one Setup, one V1 nupkg and one V2 nupkg are required.'
}
foreach ($file in @($setup[0], $fullV1[0], $fullV2[0])) {
    $entry = @($provenance.packages | Where-Object { $_.name -ceq $file.Name })
    if ($entry.Count -ne 1 -or
        $entry[0].sha256 -cne (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash.ToLowerInvariant() -or
        $entry[0].size_bytes -ne $file.Length) {
        throw "Producer provenance mismatch for $($file.Name)"
    }
}
Write-Host 'OFFICE_MANUAL_LAB_PORTABLE_PACKAGE_HASHES_OK'
if ($Mode -ceq 'VerifyOnly') { return }

# No existing managed state, journal, installation, trust policy or
# previous LAB preparation may be overwritten by this manual helper.
$local = [Environment]::GetFolderPath([Environment+SpecialFolder]::LocalApplicationData)
$packageId = 'turkuazlabs.turkuazoffice.pilot'
$trust = Join-Path $local 'TurkuazInstaller/config/manifest-trust.yml'
$state = Join-Path $local "TurkuazInstaller/state/$packageId.json"
$journal = Join-Path $local "TurkuazInstaller/journal/$packageId.json"
$target = Join-Path $local 'Programs/TurkuazOfficeInstallerLab'
$labRoot = Join-Path $local 'TurkuazOfficeInstallerManualLab'
foreach ($path in @($trust, $state, $journal, $target, $labRoot)) {
    if (Test-Path -LiteralPath $path) {
        throw "Existing user data or trust policy found: $path. No overwrite permitted."
    }
}

Add-Type -AssemblyName System.Security.Cryptography.Pkcs
$rsa = [Security.Cryptography.RSA]::Create(2048)
$cert = $null
try {
    $request = [Security.Cryptography.X509Certificates.CertificateRequest]::new(
        'CN=Office Manual Installer LAB Only', $rsa,
        [Security.Cryptography.HashAlgorithmName]::SHA256,
        [Security.Cryptography.RSASignaturePadding]::Pkcs1)
    $request.CertificateExtensions.Add(
        [Security.Cryptography.X509Certificates.X509KeyUsageExtension]::new(
            [Security.Cryptography.X509Certificates.X509KeyUsageFlags]::DigitalSignature, $true))
    $cert = $request.CreateSelfSigned(
        [DateTimeOffset]::UtcNow.AddMinutes(-5),
        [DateTimeOffset]::UtcNow.AddHours(72))
    [IO.Directory]::CreateDirectory($labRoot) | Out-Null

    function Write-SignedLabManifest {
        param(
            [Parameter(Mandatory)][string]$Name,
            [Parameter(Mandatory)][IO.FileInfo]$Artifact,
            [Parameter(Mandatory)][string]$Version
        )
        $uri = ([Uri]::new($Artifact.FullName)).AbsoluteUri
        $sha = (Get-FileHash -LiteralPath $Artifact.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
        $rows = @(
            'schema_version: 1'
            'package:'
            ('  id: ' + $packageId)
            ('  version: ' + $Version)
            '  channel: beta'
            'artifact:'
            ('  uri: "' + $uri + '"')
            ('  sha256: ' + $sha)
            ('  size_bytes: ' + $Artifact.Length)
            'install:'
            '  mode: full'
            ('  target: "' + $target.Replace('\', '/') + '"')
            '  preserve_paths:'
            '    - UserData'
            'rollback:'
            '  supported: true'
            '  previous_version_required: true'
            ''
        )
        $bytes = [Text.UTF8Encoding]::new($false).GetBytes([string]::Join([char]10, $rows))
        $manifest = Join-Path $labRoot "$Name.yml"
        $cms = [Security.Cryptography.Pkcs.SignedCms]::new(
            [Security.Cryptography.Pkcs.ContentInfo]::new($bytes), $true)
        $signer = [Security.Cryptography.Pkcs.CmsSigner]::new($cert)
        $signer.IncludeOption = [Security.Cryptography.X509Certificates.X509IncludeOption]::EndCertOnly
        $signer.DigestAlgorithm = [Security.Cryptography.Oid]::new('2.16.840.1.101.3.4.2.1')
        $cms.ComputeSignature($signer)
        $signature = $cms.Encode()

        # Immediately recheck the detached signature with the PUBLIC
        # certificate; never persist or export the private signing key.
        $check = [Security.Cryptography.Pkcs.SignedCms]::new(
            [Security.Cryptography.Pkcs.ContentInfo]::new($bytes), $true)
        $check.Decode($signature)
        $check.CheckSignature($true)
        if ($check.SignerInfos.Count -ne 1) {
            throw 'Signed manifest has an unexpected signer count.'
        }
        [IO.File]::WriteAllBytes($manifest, $bytes)
        [IO.File]::WriteAllBytes(($manifest + '.p7s'), $signature)
        return $manifest
    }

    $manifestV1 = Write-SignedLabManifest 'office-install-v1' $setup[0] $versionOne
    $manifestV2 = Write-SignedLabManifest 'office-update-v2' $fullV2[0] $versionTwo
    $manifestRollback = Write-SignedLabManifest 'office-rollback-v1' $fullV1[0] $versionOne

    $pin = $cert.GetCertHashString([Security.Cryptography.HashAlgorithmName]::SHA256)
    $trustRows = @(
        'schema_version: 1'
        'packages:'
        ('  - id: ' + $packageId)
        ('    publisher_subject: "' + $cert.Subject + '"')
        ('    certificate_sha256: ' + $pin)
        ''
    )
    $trustBytes = [Text.UTF8Encoding]::new($false).GetBytes(
        [string]::Join([char]10, $trustRows))
    [IO.Directory]::CreateDirectory((Split-Path -Parent $trust)) | Out-Null
    $stream = [IO.File]::Open($trust, [IO.FileMode]::CreateNew,
        [IO.FileAccess]::Write, [IO.FileShare]::None)
    try {
        $stream.Write($trustBytes, 0, $trustBytes.Length)
        $stream.Flush()
    }
    finally { $stream.Dispose() }

    Write-Host 'OFFICE_MANUAL_LAB_DETACHED_CMS_AND_EXTERNAL_TRUST_READY'
    Write-Host "PACKAGE_ID=$packageId"
    Write-Host 'CHANNEL=Beta'
    Write-Host "INSTALL_MANIFEST=$manifestV1"
    Write-Host "UPDATE_MANIFEST=$manifestV2"
    Write-Host "ROLLBACK_MANIFEST=$manifestRollback"
    Write-Host "TRUST_FILE=$trust"
    Write-Host "TARGET=$target"
    Write-Host 'PRIVATE_KEY=NOT_EXPORTED_OR_WRITTEN'
    Write-Host 'Use only the unsigned test Bootstrapper in an isolated Windows x64 VM.'
}
finally {
    if ($null -ne $cert) { $cert.Dispose() }
    $rsa.Dispose()
}
