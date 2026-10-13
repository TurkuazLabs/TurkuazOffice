# 📄 Dosya Yolu: /tools/installer/Test-OfficeSignedTurkuazInstallerCli.ps1
# 📌 Amac: Gercek Tauri Office paketini signed manifest ile TurkuazInstaller CLI'ye baglar
# 📌 Modul - Tool PowerShell
# Version: 0.6.0
# Aciklama: Hosted Windows LAB; ephemeral CMS, independently pinned trust, real lifecycle; production degildir
# Bagimli Oldugu Katman: Tool | CI | Test

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$CliPath,
    [Parameter(Mandatory = $true)][string]$InventoryProbePath,
    [string]$CompiledBinaryRoot = ".pilot-vpk/binaries",
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
$probe = (Resolve-Path -LiteralPath $InventoryProbePath).Path
$assets = (Resolve-Path -LiteralPath $AssetsPath).Path
$compiled = (Resolve-Path -LiteralPath $CompiledBinaryRoot).Path
$compiledV1 = (Resolve-Path -LiteralPath (Join-Path $compiled 'office-v1.exe')).Path
$compiledV2 = (Resolve-Path -LiteralPath (Join-Path $compiled 'office-v2.exe')).Path
$compiledHashV1 = (Get-FileHash -LiteralPath $compiledV1 -Algorithm SHA256).Hash
$compiledHashV2 = (Get-FileHash -LiteralPath $compiledV2 -Algorithm SHA256).Hash
if ($compiledHashV1 -ceq $compiledHashV2) {
    throw 'Real signed CLI E2E requires two DIFFERENT compiled Tauri EXEs.'
}
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
$testJournalCreated = $false

function New-LabManifest {
    param([string]$Name, [IO.FileInfo]$Artifact, [string]$Version,
        [switch]$WrongHash, [string]$PackageIdentity = '')
    if ([string]::IsNullOrWhiteSpace($PackageIdentity)) {
        $PackageIdentity = $packageId
    }
    if ($PackageIdentity.Length -gt 63 -or $PackageIdentity -match '[^a-z0-9._-]') {
        throw 'LAB package identity contains unsafe characters.'
    }
    $sha = (Get-FileHash $Artifact.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($WrongHash) { $sha = '0' * 64 }
    $url = ([Uri]::new($Artifact.FullName)).AbsoluteUri
    $rows = @(
        'schema_version: 1'
        'package:'
        ('  id: ' + $PackageIdentity)
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

function New-SignedLabInventory {
    param([string]$Name, [string]$Manifest, [IO.FileInfo]$FullPackage)

    $output = Join-Path $root "$Name.json"
    # Keep CLI stdout (the unsigned inventory probe's success marker)
    # OUT of the function's return value. PowerShell otherwise yields an
    # object[] containing the marker and path, so audit sees a false path.
    & $probe --package $packageId --channel beta --manifest $Manifest --trust $trust --full-package $FullPackage.FullName --output $output |
        ForEach-Object { Write-Host $_ }
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path -LiteralPath $output -PathType Leaf)) {
        throw "Trusted full-package inventory derivation failed: $Name"
    }
    $bytes = [IO.File]::ReadAllBytes($output)
    if ($bytes.Length -le 0 -or $bytes.Length -gt 1048576) {
        throw 'Trusted Office full-package inventory exceeds signed limits.'
    }
    $cms = [Security.Cryptography.Pkcs.SignedCms]::new(
        [Security.Cryptography.Pkcs.ContentInfo]::new($bytes), $true)
    $signer = [Security.Cryptography.Pkcs.CmsSigner]::new($cert)
    $signer.IncludeOption = [Security.Cryptography.X509Certificates.X509IncludeOption]::EndCertOnly
    $signer.DigestAlgorithm = [Security.Cryptography.Oid]::new('2.16.840.1.101.3.4.2.1')
    $cms.ComputeSignature($signer)
    [IO.File]::WriteAllBytes(($output + '.p7s'), $cms.Encode())
    return $output
}

function Invoke-Audit {
    param([string]$Label, [string]$Manifest, [string]$Inventory,
        [switch]$ExpectMismatch, [string]$ExpectUnattributedPath = '')

    # Audit stdout is one typed JSON report, not a repair/resume command.
    $auditArgs = @('audit', '--package', $packageId,
        '--manifest', $Manifest, '--inventory', $Inventory, '--channel', 'beta')
    $reportText = & $cli @auditArgs
    $exitCode = $LASTEXITCODE
    if ($null -eq $reportText) {
        throw "Signed Office audit did not produce a report: $Label code=$exitCode"
    }
    $report = ($reportText | Out-String | ConvertFrom-Json)
    $hasMarkerMismatch = @($report.mismatches | Where-Object {
        $_.path -ceq 'current/installer-pilot-version.txt'
    }).Count -eq 1
    if ($report.diagnostic_only -ne $true -or
        $report.permits_repair -ne $false -or
        $report.snapshot_atomic -ne $false -or
        $report.recovery_hold_cleared -ne $false) {
        throw "Signed Office audit report must never claim repair authorization: $Label"
    }

    if ($ExpectMismatch -and -not [string]::IsNullOrWhiteSpace($ExpectUnattributedPath)) {
        throw 'An audit probe cannot expect two unrelated negative classes.'
    }
    if ($ExpectMismatch) {
        if ($exitCode -ne 4 -or $report.observed_clean -ne $false -or
            -not $hasMarkerMismatch) {
            throw "Signed Office audit failed to identify corrupted marker: $Label"
        }
    } elseif (-not [string]::IsNullOrWhiteSpace($ExpectUnattributedPath)) {
        $hasForeignFile = @($report.unattributed_current_files | Where-Object {
            $_ -ceq $ExpectUnattributedPath
        }).Count -eq 1
        if ($exitCode -ne 4 -or $report.observed_clean -ne $false -or
            -not $hasForeignFile -or @($report.mismatches).Count -ne 0) {
            throw "Signed Office audit failed to report unexpected current/ file: $Label"
        }
    } else {
        if ($exitCode -ne 0 -or $report.observed_clean -ne $true -or
            @($report.mismatches).Count -ne 0 -or
            @($report.unattributed_current_files).Count -ne 0 -or
            $report.inspected_signed_files -lt 2) {
            throw "Signed Office complete payload audit not clean: $Label code=$exitCode"
        }
    }
    Write-Host ('OFFICE_REAL_SIGNED_NUPKG_AUDIT_{0}_OK code={1} files={2}' -f $Label.ToUpperInvariant(), $exitCode, $report.inspected_signed_files)
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
        # A negative security gate must fail specifically as an operation
        # failure, not accidentally pass due to invalid arguments (2).
        if (($Reject -and $code -ne 4) -or (-not $Reject -and $code -ne 0)) {
            throw "Unexpected Installer CLI $Label exit=$code"
        }
        Write-Host ('OFFICE_REAL_INSTALLER_SIGNED_CLI_{0}_OK code={1}' -f $Label.ToUpperInvariant(), $code)
    }
    finally { $process.Dispose() }
}
function Assert-State {
    param([string]$Version)
    $exe = Join-Path $installRoot 'current/turkuaz-office-desktop.exe'
    if (-not (Test-Path $marker -PathType Leaf) -or
        [IO.File]::ReadAllText($marker).Trim() -cne $Version -or
        -not (Test-Path $exe -PathType Leaf) -or
        -not (Test-Path $state -PathType Leaf)) {
        throw "Installed Office binary/marker/committed state invalid at version $Version"
    }

    $expectedHash = if ($Version -ceq $versionOne) { $compiledHashV1 } elseif (
        $Version -ceq $versionTwo) { $compiledHashV2 } else {
        throw "No expected compiled Office LAB EXE for version $Version"
    }
    $actualHash = (Get-FileHash -LiteralPath $exe -Algorithm SHA256).Hash
    if ($actualHash -cne $expectedHash) {
        throw "Installed Office compiled EXE SHA256 mismatch at version $Version"
    }
    $variant = if ($Version -ceq $versionOne) { 'v1' } else { 'v2' }
    $probeProcess = Start-Process -FilePath $exe -ArgumentList "--office-pilot-verify-compiled-$variant" -Wait -PassThru
    if ($probeProcess.ExitCode -ne 0) {
        throw "Installed Office compiled binary identity mismatch at $Version code=$($probeProcess.ExitCode)"
    }
}
function Assert-UserData {
    if (-not (Test-Path $userData -PathType Leaf) -or
        [IO.File]::ReadAllText($userData) -cne 'office-user-data-ci') {
        throw 'Office UserData changed during signed CLI operation.'
    }
}

try {
    $otherPackageId = 'turkuazlabs.turkuazoffice.foreign-ci'
    $pin = $cert.GetCertHashString(
        [Security.Cryptography.HashAlgorithmName]::SHA256).ToLowerInvariant()
    $trustRows = @(
        'schema_version: 1'
        'packages:'
        ('  - id: ' + $packageId)
        ('    publisher_subject: "' + $cert.Subject + '"')
        ('    certificate_sha256: ' + $pin)
        ('  - id: ' + $otherPackageId)
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

    # Without the detached .p7s, even an unchanged, otherwise valid
    # Office manifest cannot authorize a real installation or update.
    $unsignedManifestV1 = Join-Path $root 'office-v1-no-signature.yml'
    [IO.File]::Copy($m1, $unsignedManifestV1)
    $unsignedManifestV2 = Join-Path $root 'office-v2-no-signature.yml'
    [IO.File]::Copy($m2, $unsignedManifestV2)

    # Both package identities have a valid external trust entry and
    # the SAME CMS signer. Package ID must still scope rollback authority.
    $foreignRollback = New-LabManifest 'office-foreign-rollback' $fullOne $versionOne -PackageIdentity $otherPackageId

    Invoke-Core 'CMS_TAMPER_REJECTED' @(
        'install', '--package', $packageId, '--manifest', $mTamper,
        '--channel', 'beta', '--target', $installRoot, '--silent') -Reject
    Invoke-Core 'INSTALL_MISSING_DETACHED_CMS_REJECTED' @(
        'install', '--package', $packageId, '--manifest', $unsignedManifestV1,
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
    Invoke-Core 'UPDATE_MISSING_DETACHED_CMS_REJECTED' @(
        'update', '--package', $packageId, '--manifest', $unsignedManifestV2,
        '--channel', 'beta', '--silent') -Reject
    Assert-State $versionOne
    Assert-UserData
    Invoke-Core 'UPDATE' @(
        'update', '--package', $packageId, '--manifest', $m2, '--channel', 'beta', '--silent')
    Assert-State $versionTwo
    Write-Host 'OFFICE_SIGNED_CLI_DISTINCT_COMPILED_EXECUTABLE_UPDATE_OK'
    Assert-UserData

    # This uses the SAME archive inventory builder as Installer's own
    # Windows E2E: checks the entire real Office nupkg SHA-256 and lib/app
    # file list before ephemeral CMS signing and on-disk comparison.
    $inventoryV2 = New-SignedLabInventory 'office-full-v2' $m2 $fullTwo
    Invoke-Audit 'V2_CLEAN' $m2 $inventoryV2

    # A correct inventory blob without the required detached sidecar
    # must fail closed, without altering the real installed Office.
    $unsignedInventoryV2 = Join-Path $root 'office-v2-no-signature.json'
    [IO.File]::Copy($inventoryV2, $unsignedInventoryV2)
    Invoke-Core 'AUDIT_MISSING_DETACHED_CMS_REJECTED' @(
        'audit', '--package', $packageId, '--manifest', $m2,
        '--inventory', $unsignedInventoryV2, '--channel', 'beta') -Reject
    Assert-State $versionTwo
    Assert-UserData
    Invoke-Audit 'V2_MISSING_CMS_RESTORED_CLEAN' $m2 $inventoryV2

    # Inventory signatures are not interchangeable between package releases.
    # The same trusted LAB signer signs V1 and V2 inventories, but the
    # actual version and artifact SHA-256 are independently bound.
    $inventoryV1 = New-SignedLabInventory 'office-full-v1' $mRollback $fullOne
    Invoke-Core 'AUDIT_WRONG_RELEASE_INVENTORY_REJECTED' @(
        'audit', '--package', $packageId, '--manifest', $m2,
        '--inventory', $inventoryV1, '--channel', 'beta') -Reject
    Assert-State $versionTwo
    Assert-UserData

    # A persisted operation checkpoint must block a "clean" result even
    # when every expected Office file hashes correctly. 5=Applying and
    # 1=Update are the stable JSON enum values of this pinned Installer.
    # Never ask audit to remove a journal or resume/repair the installation.
    $now = [DateTimeOffset]::UtcNow.ToString('O')
    $journalDocument = [ordered]@{
        OperationId = [Guid]::NewGuid().ToString('D')
        PackageId = $packageId
        Operation = 1
        Version = $versionTwo
        TargetPath = $installRoot
        Phase = 5
        StartedAtUtc = $now
        UpdatedAtUtc = $now
        Failure = $null
        PendingPrerequisiteId = $null
        InterruptedPhase = $null
    }
    $journalBytes = [Text.UTF8Encoding]::new($false).GetBytes(
        ($journalDocument | ConvertTo-Json -Depth 4))
    $journalStream = [IO.File]::Open(
        $journal, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
    try {
        $journalStream.Write($journalBytes, 0, $journalBytes.Length)
        $journalStream.Flush()
    }
    finally { $journalStream.Dispose() }
    $testJournalCreated = $true
    $expectedJournalHash = [Convert]::ToHexString(
        [Security.Cryptography.SHA256]::HashData($journalBytes))
    try {
        Invoke-Core 'AUDIT_PERSISTED_APPLY_JOURNAL_REJECTED' @(
            'audit', '--package', $packageId, '--manifest', $m2,
            '--inventory', $inventoryV2, '--channel', 'beta') -Reject
        if (-not (Test-Path -LiteralPath $journal -PathType Leaf) -or
            (Get-FileHash -LiteralPath $journal -Algorithm SHA256).Hash -cne $expectedJournalHash) {
            throw 'Read-only Office audit modified or removed the operation journal.'
        }
        Assert-State $versionTwo
        Assert-UserData
    }
    finally {
        # Explicitly delete only this GUID-isolated test fixture after
        # the audit proves that it did NOT mutate the checkpoint.
        if ($testJournalCreated -and (Test-Path -LiteralPath $journal -PathType Leaf)) {
            Remove-Item -LiteralPath $journal -Force
            $testJournalCreated = $false
        }
    }
    Invoke-Audit 'V2_TEST_JOURNAL_REMOVED_CLEAN' $m2 $inventoryV2

    # Even a syntactically valid JSON inventory must fail if changed
    # AFTER signing. This does not overwrite the trusted baseline.
    $tamperedInventory = Join-Path $root 'office-v2-tampered-inventory.json'
    [IO.File]::WriteAllBytes($tamperedInventory, [IO.File]::ReadAllBytes($inventoryV2))
    [IO.File]::WriteAllBytes(
        ($tamperedInventory + '.p7s'), [IO.File]::ReadAllBytes($inventoryV2 + '.p7s'))
    [IO.File]::AppendAllText($tamperedInventory, [Environment]::NewLine + ' ')
    Invoke-Core 'AUDIT_TAMPERED_INVENTORY_REJECTED' @(
        'audit', '--package', $packageId, '--manifest', $m2,
        '--inventory', $tamperedInventory, '--channel', 'beta') -Reject
    Assert-State $versionTwo
    Assert-UserData

    # External trust must NOT be learnable from the signed input.
    # The wrong pin is transient and restored in finally, even if
    # verification fails unexpectedly. Never modify a non-LAB store.
    $originalTrust = [IO.File]::ReadAllBytes($trust)
    try {
        $badTrust = @(
            'schema_version: 1'
            'packages:'
            ('  - id: ' + $packageId)
            ('    publisher_subject: "' + $cert.Subject + '"')
            ('    certificate_sha256: ' + ('0' * 64))
            ''
        )
        [IO.File]::WriteAllText(
            $trust, [string]::Join([char]10, $badTrust),
            [Text.UTF8Encoding]::new($false))
        Invoke-Core 'AUDIT_WRONG_EXTERNAL_PIN_REJECTED' @(
            'audit', '--package', $packageId, '--manifest', $m2,
            '--inventory', $inventoryV2, '--channel', 'beta') -Reject
    }
    finally {
        [IO.File]::WriteAllBytes($trust, $originalTrust)
    }
    Assert-State $versionTwo
    Assert-UserData
    Invoke-Audit 'V2_TRUST_RESTORED_CLEAN' $m2 $inventoryV2

    # A foreign file in current/ must be reported but never removed
    # by the diagnostic. Removing the disposable marker here is an
    # explicit test-fixture cleanup, not an audit repair operation.
    $foreignRelative = 'current/office-foreign-file.lab'
    $foreign = Join-Path $installRoot 'current/office-foreign-file.lab'
    [IO.File]::WriteAllText($foreign, 'foreign-payload-must-survive-audit')
    Invoke-Audit 'V2_FOREIGN_FILE_REPORTED' $m2 $inventoryV2 -ExpectUnattributedPath $foreignRelative
    if ([IO.File]::ReadAllText($foreign) -cne 'foreign-payload-must-survive-audit') {
        throw 'Read-only signed audit modified unattributed Office file.'
    }
    Remove-Item -LiteralPath $foreign
    Assert-UserData
    Invoke-Audit 'V2_NEGATIVE_GATES_RESTORED_CLEAN' $m2 $inventoryV2

    [IO.File]::WriteAllText($marker, 'deliberate-ci-corruption')
    Invoke-Audit 'V2_CORRUPT_REJECTED' $m2 $inventoryV2 -ExpectMismatch
    Invoke-Core 'REPAIR' @(
        'repair', '--package', $packageId, '--manifest', $m2, '--channel', 'beta', '--silent')
    Assert-State $versionTwo
    Assert-UserData
    Invoke-Audit 'V2_REPAIRED_CLEAN' $m2 $inventoryV2

    # The foreign rollback manifest is itself CMS-valid and signer-pinned:
    # matching signer trust does NOT override the installed package ID.
    Invoke-Core 'ROLLBACK_FOREIGN_PACKAGE_REJECTED' @(
        'rollback', '--package', $packageId, '--manifest', $m2,
        '--rollback-manifest', $foreignRollback, '--channel', 'beta', '--silent') -Reject
    Assert-State $versionTwo
    Assert-UserData
    Invoke-Audit 'V2_FOREIGN_ROLLBACK_REJECTED_CLEAN' $m2 $inventoryV2
    Invoke-Core 'ROLLBACK' @(
        'rollback', '--package', $packageId, '--manifest', $m2,
        '--rollback-manifest', $mRollback, '--channel', 'beta', '--silent')
    Assert-State $versionOne
    Assert-UserData
    Invoke-Audit 'V1_ROLLBACK_CLEAN' $mRollback $inventoryV1
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
    Write-Host 'OFFICE_REAL_PRODUCT_TURKUAZINSTALLER_SIGNED_CLI_LIFECYCLE_AND_INVENTORY_OK'
}
finally {
    # LAB cleanup only: an initially existing journal makes the test abort
    # before creation. Never delete any journal outside this test ownership.
    if ($testJournalCreated -and (Test-Path -LiteralPath $journal -PathType Leaf)) {
        Remove-Item -LiteralPath $journal -Force
    }
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
