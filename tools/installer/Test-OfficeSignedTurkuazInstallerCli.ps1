# 📄 Dosya Yolu: /tools/installer/Test-OfficeSignedTurkuazInstallerCli.ps1
# 📌 Amac: Gercek Tauri Office paketini signed manifest ile TurkuazInstaller CLI'ye baglar
# 📌 Modul - Tool PowerShell
# Version: 0.1.0
# Aciklama: Hosted Windows LAB; ephemeral CMS, independently pinned trust, real lifecycle; production degildir
# Bagimli Oldugu Katman: Tool | CI | Test

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$CliPath,
    [string]$AssetsPath = "artifacts/turkuazinstaller-pilot/assets",
    [ValidateRange(60, 600)][int]$ProcessTimeoutSeconds = 240
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT -or
    $env:GITHUB_ACTIONS -cne 'true' -or
    $env:RUNNER_ENVIRONMENT -cne 'github-hosted') {
    throw 'Real Office signed CLI LAB requires disposable hosted Windows.'
}
$packageId = 'turkuazlabs.turkuazoffice.pilot'
$cli = (Resolve-Path -LiteralPath $CliPath).Path
$assets = (Resolve-Path -LiteralPath $AssetsPath).Path
$versionOne = [string](Get-Content 'apps/desktop/package.json' -Raw | ConvertFrom-Json).version
if ($versionOne -notmatch '^\d+\.\d+\.\d+$') { throw 'Invalid Office SemVer.' }
$v = [version]$versionOne
$versionTwo = '{0}.{1}.{2}' -f $v.Major, $v.Minor, ($v.Build + 1)
$setupFiles = @(Get-ChildItem -LiteralPath $assets -File -Filter '*Setup.exe')
$fullOnes = @(Get-ChildItem -LiteralPath $assets -File -Filter "*-$versionOne-full.nupkg")
$fullTwos = @(Get-ChildItem -LiteralPath $assets -File -Filter "*-$versionTwo-full.nupkg")
if ($setupFiles.Count -ne 1 -or $fullOnes.Count -ne 1 -or $fullTwos.Count -ne 1) {
    throw 'Expected exactly one real Office Setup and each versioned full nupkg.'
}
$setup = $setupFiles[0]
$fullOne = $fullOnes[0]
$fullTwo = $fullTwos[0]
$known = @{}
foreach ($line in [IO.File]::ReadAllLines((Join-Path $assets 'SHA256SUMS.txt'))) {
    if ($line -notmatch '^([0-9a-fA-F]{64}) \*([a-zA-Z0-9_.-]+)$') {
        throw 'Unsafe or invalid producer checksum.'
    }
    if ($known.ContainsKey($Matches[2])) { throw 'Duplicate producer checksum.' }
    $known[$Matches[2]] = $Matches[1]
}
foreach ($file in @($setup, $fullOne, $fullTwo)) {
    $hash = (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash
    if (-not $known.ContainsKey($file.Name) -or
        -not [string]::Equals($known[$file.Name], $hash, [StringComparison]::OrdinalIgnoreCase)) {
        throw "Office pilot package checksum mismatch: $($file.Name)"
    }
}

$local = [Environment]::GetFolderPath([Environment+SpecialFolder]::LocalApplicationData)
$id = [guid]::NewGuid().ToString('N')
$root = Join-Path (Resolve-Path $env:RUNNER_TEMP).Path "office-cli-signed-$id"
$installRoot = Join-Path $local "Programs/OfficeInstallerCi-$id"
$state = Join-Path $local "TurkuazInstaller/state/$packageId.json"
$journal = Join-Path $local "TurkuazInstaller/journal/$packageId.json"
$trust = Join-Path $local 'TurkuazInstaller/config/manifest-trust.yml'
$marker = Join-Path $installRoot 'current/installer-pilot-version.txt'
$userData = Join-Path $installRoot 'UserData/owned.txt'
if ((Test-Path $state) -or (Test-Path $journal) -or
    (Test-Path $trust) -or (Test-Path $installRoot)) {
    throw 'Existing install state, journal, trust or path; LAB refuses overwrite.'
}
[IO.Directory]::CreateDirectory($root) | Out-Null
[IO.Directory]::CreateDirectory((Split-Path $trust -Parent)) | Out-Null

Add-Type -AssemblyName System.Security.Cryptography.Pkcs
$rsa = [Security.Cryptography.RSA]::Create(2048)
$request = [Security.Cryptography.X509Certificates.CertificateRequest]::new(
    'CN=Office Signed CLI Ephemeral CI Only', $rsa,
    [Security.Cryptography.HashAlgorithmName]::SHA256,
    [Security.Cryptography.RSASignaturePadding]::Pkcs1)
$request.CertificateExtensions.Add(
    [Security.Cryptography.X509Certificates.X509KeyUsageExtension]::new(
        [Security.Cryptography.X509Certificates.X509KeyUsageFlags]::DigitalSignature, $true))
$cert = $request.CreateSelfSigned(
    [DateTimeOffset]::UtcNow.AddMinutes(-5),
    [DateTimeOffset]::UtcNow.AddHours(2))
$trustCreated = $false

function New-LabManifest {
    param([string]$Name, [IO.FileInfo]$Artifact, [string]$Version, [switch]$WrongHash)
    $sha = (Get-FileHash $Artifact.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($WrongHash) { $sha = '0' * 64 }
    $url = ([Uri]::new($Artifact.FullName)).AbsoluteUri
    $rows = @(
        'schema_version: 1'
        'package:'
        ('  id: ' + $packageId)
        ('  version: ' + $Version)
        '  channel: beta'
        'artifact:'
        ('  uri: "' + $url + '"')
        ('  sha256: ' + $sha)
        ('  size_bytes: ' + $Artifact.Length)
        'install:'
        '  mode: full'
        ('  target: "' + $installRoot.Replace('\', '/') + '"')
        '  preserve_paths:'
        '    - UserData'
        'rollback:'
        '  supported: true'
        '  previous_version_required: true'
        ''
    )
    $bytes = [Text.UTF8Encoding]::new($false).GetBytes([string]::Join([char]10, $rows))
    $cms = [Security.Cryptography.Pkcs.SignedCms]::new(
        [Security.Cryptography.Pkcs.ContentInfo]::new($bytes), $true)
    $signer = [Security.Cryptography.Pkcs.CmsSigner]::new($cert)
    $signer.IncludeOption = [Security.Cryptography.X509Certificates.X509IncludeOption]::EndCertOnly
    $signer.DigestAlgorithm = [Security.Cryptography.Oid]::new('2.16.840.1.101.3.4.2.1')
    $cms.ComputeSignature($signer)
    $path = Join-Path $root "$Name.yml"
    [IO.File]::WriteAllBytes($path, $bytes)
    [IO.File]::WriteAllBytes(($path + '.p7s'), $cms.Encode())
    return $path
}

function Invoke-Core {
    param([string]$Label, [string[]]$Arguments, [switch]$Reject)
    $start = [Diagnostics.ProcessStartInfo]::new()
    $start.FileName = $cli
    $start.UseShellExecute = $false
    $start.CreateNoWindow = $true
    $start.WorkingDirectory = (Split-Path $cli -Parent)
    foreach ($arg in $Arguments) { $start.ArgumentList.Add($arg) }
    $process = [Diagnostics.Process]::Start($start)
    if ($null -eq $process) { throw "Cannot launch Installer CLI: $Label" }
    try {
        if (-not $process.WaitForExit($ProcessTimeoutSeconds * 1000)) {
            $process.Kill($true)
            $process.WaitForExit(10000) | Out-Null
            throw "Installer CLI timed out: $Label"
        }
        $code = $process.ExitCode
        if (($Reject -and $code -eq 0) -or (-not $Reject -and $code -ne 0)) {
            throw "Unexpected Installer CLI $Label exit=$code"
        }
        Write-Host ('OFFICE_REAL_INSTALLER_SIGNED_CLI_{0}_OK code={1}' -f $Label.ToUpperInvariant(), $code)
    }
    finally { $process.Dispose() }
}
function Assert-State {
    param([string]$Version)
    if (-not (Test-Path $marker -PathType Leaf) -or
        [IO.File]::ReadAllText($marker).Trim() -cne $Version -or
        -not (Test-Path (Join-Path $installRoot 'current/turkuaz-office-desktop.exe') -PathType Leaf) -or
        -not (Test-Path $state -PathType Leaf)) {
        throw "Installed Office binary/marker/committed state invalid at version $Version"
    }
}
function Assert-UserData {
    if (-not (Test-Path $userData -PathType Leaf) -or
        [IO.File]::ReadAllText($userData) -cne 'office-user-data-ci') {
        throw 'Office UserData changed during signed CLI operation.'
    }
}

try {
    $pin = $cert.GetCertHashString(
        [Security.Cryptography.HashAlgorithmName]::SHA256).ToLowerInvariant()
    $trustRows = @(
        'schema_version: 1'
        'packages:'
        ('  - id: ' + $packageId)
        ('    publisher_subject: "' + $cert.Subject + '"')
        ('    certificate_sha256: ' + $pin)
        ''
    )
    [IO.File]::WriteAllText($trust, [string]::Join([char]10, $trustRows),
        [Text.UTF8Encoding]::new($false))
    $trustCreated = $true
    $m1 = New-LabManifest 'setup-v1' $setup $versionOne
    $m2 = New-LabManifest 'full-v2' $fullTwo $versionTwo
    $mRollback = New-LabManifest 'full-v1' $fullOne $versionOne
    $mBad = New-LabManifest 'wrong-v2-digest' $fullTwo $versionTwo -WrongHash
    $mTamper = New-LabManifest 'tampered-v1' $setup $versionOne
    [IO.File]::AppendAllText($mTamper, [Environment]::NewLine + '# changed after signing')

    Invoke-Core 'CMS_TAMPER_REJECTED' @(
        'install', '--package', $packageId, '--manifest', $mTamper,
        '--channel', 'beta', '--target', $installRoot, '--silent') -Reject
    if ((Test-Path $state) -or (Test-Path $marker)) {
        throw 'Tampered manifest unexpectedly created installed Office.'
    }
    Invoke-Core 'INSTALL' @(
        'install', '--package', $packageId, '--manifest', $m1,
        '--channel', 'beta', '--target', $installRoot, '--silent')
    Assert-State $versionOne
    [IO.Directory]::CreateDirectory((Split-Path $userData -Parent)) | Out-Null
    [IO.File]::WriteAllText($userData, 'office-user-data-ci')

    Invoke-Core 'BAD_ARTIFACT_REJECTED' @(
        'update', '--package', $packageId, '--manifest', $mBad, '--channel', 'beta', '--silent') -Reject
    Assert-State $versionOne
    Assert-UserData
    Invoke-Core 'UPDATE' @(
        'update', '--package', $packageId, '--manifest', $m2, '--channel', 'beta', '--silent')
    Assert-State $versionTwo
    Assert-UserData

    [IO.File]::WriteAllText($marker, 'deliberate-ci-corruption')
    Invoke-Core 'REPAIR' @(
        'repair', '--package', $packageId, '--manifest', $m2, '--channel', 'beta', '--silent')
    Assert-State $versionTwo
    Assert-UserData
    Invoke-Core 'ROLLBACK' @(
        'rollback', '--package', $packageId, '--manifest', $m2,
        '--rollback-manifest', $mRollback, '--channel', 'beta', '--silent')
    Assert-State $versionOne
    Assert-UserData
    Invoke-Core 'UNINSTALL' @(
        'uninstall', '--package', $packageId, '--silent')

    $deadline = [DateTime]::UtcNow.AddSeconds(35)
    do {
        $active = (Test-Path (Join-Path $installRoot 'Update.exe')) -or
            (Test-Path (Join-Path $installRoot 'current')) -or
            (Test-Path $state)
        if (-not $active) { break }
        Start-Sleep -Milliseconds 500
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($active) { throw 'Signed CLI uninstall retained binaries/state.' }
    Write-Host 'OFFICE_REAL_PRODUCT_TURKUAZINSTALLER_SIGNED_CLI_LIFECYCLE_OK'
}
finally {
    $cert.Dispose()
    $rsa.Dispose()
    if ($trustCreated -and (Test-Path $trust)) {
        Remove-Item -LiteralPath $trust -Force
    }
    # Only GUID-isolated hosted CI paths may be removed; no user NSIS migration.
    if (Test-Path $root) {
        Remove-Item -LiteralPath $root -Recurse -Force -ErrorAction SilentlyContinue
    }
    if (Test-Path $installRoot) {
        Remove-Item -LiteralPath $installRoot -Recurse -Force -ErrorAction SilentlyContinue
    }
}
