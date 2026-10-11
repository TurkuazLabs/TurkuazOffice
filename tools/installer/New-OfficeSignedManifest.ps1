# 📄 Dosya Yolu: /tools/installer/New-OfficeSignedManifest.ps1
# 📌 Amac: Yalniz onceden Authenticode imzali Office Setup.exe icin SHA256-pinned detached CMS installer manifesti uretmek
# 📌 Modul - Tool PowerShell
# Version: 0.1.0
# Aciklama: Kullanicinin sagladigi gercek imza sertifikalarini dogrular; private key export etmez ve trust store degistirmez
# Bagimli Oldugu Katman: Tool | Config | Signing

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$SetupPath,
    [Parameter(Mandatory = $true)][string]$ArtifactUri,
    [Parameter(Mandatory = $true)][string]$ArtifactPublisherSubject,
    [Parameter(Mandatory = $true)][ValidatePattern('^[0-9a-fA-F]{64}$')][string]$ExpectedArtifactCertificateSha256,
    [Parameter(Mandatory = $true)][ValidatePattern('^[0-9a-fA-F]{40}$')][string]$ManifestSignerThumbprint,
    [Parameter(Mandatory = $true)][string]$ManifestSignerSubject,
    [Parameter(Mandatory = $true)][ValidatePattern('^[0-9a-fA-F]{64}$')][string]$ExpectedManifestCertificateSha256,
    [Parameter(Mandatory = $true)][string]$OutputDirectory,
    [string]$PackageId = 'turkuazlabs.turkuazoffice',
    [string]$Version = '',
    [ValidateSet('stable', 'beta')][string]$Channel = 'stable'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if (-not $IsWindows) { throw 'Signing preflight requires Windows Authenticode.' }
if ($PackageId -notin @('turkuazlabs.turkuazoffice', 'turkuazlabs.turkuazoffice.lab')) {
    throw 'Only the Office product ID or explicit lab ID is accepted.'
}
if ($PackageId.EndsWith('.lab', [StringComparison]::Ordinal) -and $Channel -eq 'stable') {
    throw 'The lab package ID cannot be emitted on the stable channel.'
}

$setup = Get-Item -LiteralPath $SetupPath -ErrorAction Stop
if ($setup.PSIsContainer -or -not $setup.Name.EndsWith('Setup.exe', [StringComparison]::OrdinalIgnoreCase)) {
    throw 'Artifact must be a real Velopack Setup.exe file.'
}
if ($setup.Length -le 0) { throw 'Artifact file is empty.' }
$uri = $null
if (-not [Uri]::TryCreate($ArtifactUri, [UriKind]::Absolute, [ref]$uri) -or
    $uri.Scheme -cne 'https' -or
    -not [string]::IsNullOrEmpty($uri.UserInfo) -or
    -not [string]::IsNullOrEmpty($uri.Fragment)) {
    throw 'Artifact URI must be absolute HTTPS with no userinfo or fragment.'
}
function Assert-YamlScalar {
    param([Parameter(Mandatory = $true)][string]$Text)
    if ([string]::IsNullOrWhiteSpace($Text) -or $Text -match '[\r\n]' -or $Text.Contains([char]0)) {
        throw 'YAML field contains invalid or unsafe characters.'
    }
    return ("'" + $Text.Replace("'", "''") + "'")
}

$workspace = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
$package = Get-Content -LiteralPath (Join-Path $workspace 'apps/desktop/package.json') -Raw | ConvertFrom-Json
$actualVersion = [string]$package.version
if ($actualVersion -notmatch '^\d+\.\d+\.\d+$') { throw 'Office package version is not strict SemVer.' }
if ($Version -eq '') { $Version = $actualVersion }
if ($Version -notmatch '^\d+\.\d+\.\d+$' -or $Version -cne $actualVersion) {
    throw 'Manifest version must match the checked-out Office desktop version.'
}

# The artifact MUST already be signed and trusted by Windows.
$authenticode = Get-AuthenticodeSignature -LiteralPath $setup.FullName
if ($authenticode.Status -ne [System.Management.Automation.SignatureStatus]::Valid -or
    $null -eq $authenticode.SignerCertificate) {
    throw 'Unsigned, invalid, or untrusted Setup.exe is not accepted.'
}
$artifactCert = $authenticode.SignerCertificate
$artifactHash = $artifactCert.GetCertHashString([System.Security.Cryptography.HashAlgorithmName]::SHA256)
if (-not [string]::Equals($artifactCert.Subject, $ArtifactPublisherSubject, [StringComparison]::OrdinalIgnoreCase) -or
    -not [string]::Equals($artifactHash, $ExpectedArtifactCertificateSha256, [StringComparison]::OrdinalIgnoreCase)) {
    throw 'Artifact publisher subject or certificate SHA-256 does not match externally approved values.'
}

$signerList = @(Get-ChildItem -Path 'Cert:\CurrentUser\My' | Where-Object {
    $_.Thumbprint -eq $ManifestSignerThumbprint
})
if ($signerList.Count -ne 1) { throw 'Manifest signer certificate was not found uniquely in CurrentUser/Personal.' }
$signer = $signerList[0]
$signerHash = $signer.GetCertHashString([System.Security.Cryptography.HashAlgorithmName]::SHA256)
$now = [DateTime]::UtcNow
if (-not $signer.HasPrivateKey -or $now -lt $signer.NotBefore.ToUniversalTime() -or
    $now -gt $signer.NotAfter.ToUniversalTime() -or
    -not [string]::Equals($signer.Subject, $ManifestSignerSubject, [StringComparison]::OrdinalIgnoreCase) -or
    -not [string]::Equals($signerHash, $ExpectedManifestCertificateSha256, [StringComparison]::OrdinalIgnoreCase)) {
    throw 'Manifest signer key, validity, subject or external certificate SHA-256 approval is invalid.'
}

$artifactDigest = (Get-FileHash -LiteralPath $setup.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
$artifactCertPin = $artifactHash.ToLowerInvariant()
$rows = @(
    '# Turkuaz Office verified manifest. Detached raw-byte CMS signature required.'
    'schema_version: 1'
    'package:'
    ('  id: ' + (Assert-YamlScalar $PackageId))
    ('  version: ' + (Assert-YamlScalar $Version))
    ('  channel: ' + $Channel)
    'artifact:'
    ('  uri: ' + (Assert-YamlScalar $uri.AbsoluteUri))
    ('  sha256: ' + $artifactDigest)
    ('  size_bytes: ' + $setup.Length)
    '  signature:'
    '    algorithm: authenticode'
    ('    publisher_subject: ' + (Assert-YamlScalar $ArtifactPublisherSubject))
    ('    certificate_sha256: ' + $artifactCertPin)
    'install:'
    '  mode: full'
    "  target: '%LOCALAPPDATA%/Programs/TurkuazOffice'"
    '  prerequisites:'
    '    - id: windows-build'
    '      version: ">=17763"'
    '    - id: architecture'
    '      version: "x64"'
    '  preserve_paths:'
    '    - "UserData"'
    'rollback:'
    '  supported: true'
    '  previous_version_required: true'
)
$manifest = ([string]::Join([char]10, $rows) + [char]10)
$bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($manifest)
if ($bytes.Length -gt 1048576) { throw 'Manifest exceeds Installer 1 MiB trust limit.' }

Add-Type -AssemblyName System.Security.Cryptography.Pkcs
$cms = [System.Security.Cryptography.Pkcs.SignedCms]::new(
    [System.Security.Cryptography.Pkcs.ContentInfo]::new($bytes), $true)
$cmsSigner = [System.Security.Cryptography.Pkcs.CmsSigner]::new($signer)
$cmsSigner.DigestAlgorithm = [System.Security.Cryptography.Oid]::new('2.16.840.1.101.3.4.2.1')
$cmsSigner.IncludeOption = [System.Security.Cryptography.X509Certificates.X509IncludeOption]::EndCertOnly
$cms.ComputeSignature($cmsSigner)
$signature = $cms.Encode()
if ($signature.Length -gt 262144) { throw 'Detached CMS exceeds Installer 256 KiB trust limit.' }

$verify = [System.Security.Cryptography.Pkcs.SignedCms]::new(
    [System.Security.Cryptography.Pkcs.ContentInfo]::new($bytes), $true)
$verify.Decode($signature)
if ($verify.SignerInfos.Count -ne 1) { throw 'Detached CMS must have exactly one signer.' }
$verify.CheckSignature($true)
$verifiedCertificate = $verify.SignerInfos[0].Certificate
if ($null -eq $verifiedCertificate -or
    -not [string]::Equals(
        $verifiedCertificate.GetCertHashString([System.Security.Cryptography.HashAlgorithmName]::SHA256),
        $ExpectedManifestCertificateSha256,
        [StringComparison]::OrdinalIgnoreCase)) {
    throw 'Detached CMS signer does not match external certificate pin.'
}

$dest = [System.IO.Path]::GetFullPath($OutputDirectory)
$manifestPath = Join-Path $dest 'installer-manifest.yml'
$signaturePath = $manifestPath + '.p7s'
if ((Test-Path -LiteralPath $manifestPath) -or (Test-Path -LiteralPath $signaturePath)) {
    throw 'Refusing to overwrite an existing signed manifest pair.'
}
[System.IO.Directory]::CreateDirectory($dest) | Out-Null
try {
    [System.IO.File]::WriteAllBytes($manifestPath, $bytes)
    [System.IO.File]::WriteAllBytes($signaturePath, $signature)
}
catch {
    Remove-Item -LiteralPath $manifestPath, $signaturePath -Force -ErrorAction SilentlyContinue
    throw
}
Write-Host ('OFFICE_SIGNED_MANIFEST_PAIR_OK package={0} version={1} bytes={2}' -f $PackageId, $Version, $bytes.Length)
Write-Host 'Publisher trust-store entry must still be independently provisioned. No release was published.'
