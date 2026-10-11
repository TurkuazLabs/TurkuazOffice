# 📄 Dosya Yolu: /tools/installer/Test-OfficeSignedManifestPreflight.ps1
# 📌 Amac: Gecici lab sertifikasi ile Office manifest CMS sign/check ve sahte imza/pin reddini Windows'ta sinamak
# 📌 Modul - Tool PowerShell
# Version: 0.1.0
# Aciklama: Yalniz lab signed-metadata testi; Office installer build, yayin, trust-store kurulumu veya kalici sertifika uretmez
# Bagimli Oldugu Katman: Tool | CI | Test

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if (-not $IsWindows -or [string]::IsNullOrWhiteSpace($env:RUNNER_TEMP)) {
    throw 'LAB manifest smoke requires disposable Windows hosted runner.'
}
$root = Join-Path $env:RUNNER_TEMP ('office-manifest-fixture-' + [guid]::NewGuid().ToString('N'))
[System.IO.Directory]::CreateDirectory($root) | Out-Null
$fixture = Join-Path $root 'Fixture-Setup.exe'
$signer = $null
try {
    $notepad = Join-Path $env:WINDIR 'System32\notepad.exe'
    if (-not (Test-Path -LiteralPath $notepad -PathType Leaf)) { throw 'Signed Windows fixture unavailable.' }
    Copy-Item -LiteralPath $notepad -Destination $fixture
    $auth = Get-AuthenticodeSignature -LiteralPath $fixture
    if ($auth.Status -ne [System.Management.Automation.SignatureStatus]::Valid -or
        $null -eq $auth.SignerCertificate) {
        throw 'Windows fixture is not trust-verified Authenticode.'
    }
    $artPin = $auth.SignerCertificate.GetCertHashString(
        [System.Security.Cryptography.HashAlgorithmName]::SHA256)

    $signer = New-SelfSignedCertificate -Subject ('CN=TurkuazOffice LAB Manifest ' + [guid]::NewGuid().ToString('N')) -CertStoreLocation 'Cert:\CurrentUser\My' -Type CodeSigningCert -KeyExportPolicy NonExportable -HashAlgorithm SHA256 -NotAfter (Get-Date).AddDays(1)
    if ($null -eq $signer -or -not $signer.HasPrivateKey) {
        throw 'Ephemeral lab signer has no private key.'
    }
    $manifestPin = $signer.GetCertHashString(
        [System.Security.Cryptography.HashAlgorithmName]::SHA256)
    $signedOutput = Join-Path $root 'signed'
    $common = @{
        SetupPath = $fixture
        ArtifactUri = 'https://downloads.example.invalid/fixtures/Fixture-Setup.exe'
        ArtifactPublisherSubject = $auth.SignerCertificate.Subject
        ExpectedArtifactCertificateSha256 = $artPin
        ManifestSignerThumbprint = $signer.Thumbprint
        ManifestSignerSubject = $signer.Subject
        ExpectedManifestCertificateSha256 = $manifestPin
        PackageId = 'turkuazlabs.turkuazoffice.lab'
        Channel = 'beta'
    }

    & "$PSScriptRoot/New-OfficeSignedManifest.ps1" @common -OutputDirectory $signedOutput
    $manifestPath = Join-Path $signedOutput 'installer-manifest.yml'
    $sigPath = $manifestPath + '.p7s'
    if (-not (Test-Path -LiteralPath $manifestPath -PathType Leaf) -or
        -not (Test-Path -LiteralPath $sigPath -PathType Leaf)) {
        throw 'Signed lab manifest files were not produced.'
    }

    Add-Type -AssemblyName System.Security.Cryptography.Pkcs
    $raw = [System.IO.File]::ReadAllBytes($manifestPath)
    $sig = [System.IO.File]::ReadAllBytes($sigPath)
    $cms = [System.Security.Cryptography.Pkcs.SignedCms]::new(
        [System.Security.Cryptography.Pkcs.ContentInfo]::new($raw), $true)
    $cms.Decode($sig)
    $cms.CheckSignature($true)
    if ($cms.SignerInfos.Count -ne 1) { throw 'LAB signer count invalid.' }
    Write-Host 'OFFICE_SIGNED_MANIFEST_REAL_CMS_OK'

    $tampered = [byte[]]$raw.Clone()
    $tampered[$tampered.Length - 1] = [byte](($tampered[$tampered.Length - 1] + 1) % 256)
    $tamperedCms = [System.Security.Cryptography.Pkcs.SignedCms]::new(
        [System.Security.Cryptography.Pkcs.ContentInfo]::new($tampered), $true)
    $tamperedCms.Decode($sig)
    $rejectedTamper = $false
    try { $tamperedCms.CheckSignature($true) }
    catch [System.Security.Cryptography.CryptographicException] { $rejectedTamper = $true }
    if (-not $rejectedTamper) { throw 'Tampered manifest was incorrectly accepted.' }
    Write-Host 'OFFICE_SIGNED_MANIFEST_TAMPER_REJECTED'

    $wrongPin = '0' * 64
    if ($wrongPin -eq $manifestPin.ToLowerInvariant()) { $wrongPin = 'f' * 64 }
    $badOutput = Join-Path $root 'wrong-pin'
    $rejectedPin = $false
    $badArgs = @{}
    foreach ($key in $common.Keys) { $badArgs[$key] = $common[$key] }
    $badArgs.ExpectedManifestCertificateSha256 = $wrongPin
    try {
        & "$PSScriptRoot/New-OfficeSignedManifest.ps1" @badArgs -OutputDirectory $badOutput
    }
    catch { $rejectedPin = $true }
    if (-not $rejectedPin -or (Test-Path -LiteralPath (Join-Path $badOutput 'installer-manifest.yml'))) {
        throw 'Wrong external signer pin was not rejected before manifest output.'
    }
    Write-Host 'OFFICE_SIGNED_MANIFEST_PIN_REJECTED'
    Write-Host 'OFFICE_SIGNED_MANIFEST_LAB_ONLY_SMOKE_OK'
}
finally {
    if ($null -ne $signer) {
        Remove-Item -LiteralPath ('Cert:\CurrentUser\My\' + $signer.Thumbprint) -Force -ErrorAction SilentlyContinue
    }
    Remove-Item -LiteralPath $root -Recurse -Force -ErrorAction SilentlyContinue
}
