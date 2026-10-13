# 📄 Dosya Yolu: /tools/installer/Test-TurkuazOfficeVelopackInstall.ps1
# 📌 Amac: Turkuaz Office pilot paketini gecici Windows runner'a gercekten kurup kaldirarak temel lifecycle kabulunu sinamak
# 📌 Modul - Tool PowerShell
# Version: 0.7.2
# Aciklama: Setup.exe/Update.exe gercek Windows processleri; private temp hedef, SHA256 ve timeout ile fail-closed smoke test
# Bagimli Oldugu Katman: Tool | CI | Test

[CmdletBinding()]
param(
    [string]$AssetsPath = "artifacts/turkuazinstaller-pilot/assets",
    [string]$CompiledBinaryRoot = ".pilot-vpk/binaries",
    [ValidateRange(30, 360)]
    [int]$ProcessTimeoutSeconds = 180
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if (-not $IsWindows) {
    throw 'This smoke test must run on Windows.'
}
$assets = (Resolve-Path -LiteralPath $AssetsPath).Path
$compiled = (Resolve-Path -LiteralPath $CompiledBinaryRoot).Path
$compiledV1 = (Resolve-Path -LiteralPath (Join-Path $compiled 'office-v1.exe')).Path
$compiledV2 = (Resolve-Path -LiteralPath (Join-Path $compiled 'office-v2.exe')).Path
$compiledDigestV1 = (Get-FileHash -LiteralPath $compiledV1 -Algorithm SHA256).Hash
$compiledDigestV2 = (Get-FileHash -LiteralPath $compiledV2 -Algorithm SHA256).Hash
if ($compiledDigestV1 -ceq $compiledDigestV2) { throw 'Office pilot V1/V2 compiled EXEs are identical.' }
$setups = @(Get-ChildItem -LiteralPath $assets -File -Filter '*Setup.exe')
$fulls = @(Get-ChildItem -LiteralPath $assets -File -Filter '*-full.nupkg')
$packageJson = Get-Content -LiteralPath 'apps/desktop/package.json' -Raw | ConvertFrom-Json
$version = [string]$packageJson.version
if ($version -notmatch '^\d+\.\d+\.\d+$') {
    throw 'Invalid Office pilot version.'
}
$parsedVersion = [version]$version
$nextVersion = '{0}.{1}.{2}' -f $parsedVersion.Major, $parsedVersion.Minor, ($parsedVersion.Build + 1)
$fullV1 = @($fulls | Where-Object { $_.Name.EndsWith("-$version-full.nupkg", [StringComparison]::OrdinalIgnoreCase) })
$fullV2 = @($fulls | Where-Object { $_.Name.EndsWith("-$nextVersion-full.nupkg", [StringComparison]::OrdinalIgnoreCase) })
if ($setups.Count -ne 1 -or $fulls.Count -ne 2 -or $fullV1.Count -ne 1 -or $fullV2.Count -ne 1) {
    throw 'Expected one pilot Setup.exe and two distinct versioned full.nupkg files.'
}

# Prove that the downloaded/copied artifact bytes match the producer
# checksum manifest before any Windows installer process is started.
$checksums = Join-Path $assets 'SHA256SUMS.txt'
if (-not (Test-Path -LiteralPath $checksums -PathType Leaf)) {
    throw 'SHA256SUMS.txt missing.'
}
$verified = @{}
foreach ($line in [System.IO.File]::ReadAllLines($checksums)) {
    if ($line -notmatch '^([0-9a-fA-F]{64}) \*([a-zA-Z0-9_.-]+)$') {
        throw 'Unexpected SHA256SUMS format or unsafe filename.'
    }
    $name = $Matches[2]
    if ($verified.ContainsKey($name)) {
        throw "Duplicate checksum entry: $name"
    }
    $path = Join-Path $assets $name
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing checksummed file: $name"
    }
    $actual = (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash
    if (-not [string]::Equals($actual, $Matches[1], [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "SHA256 mismatch: $name"
    }
    $verified[$name] = $true
}
$requiredPackages = @($setups[0], $fullV1[0], $fullV2[0])
foreach ($file in $requiredPackages) {
    if (-not $verified.ContainsKey($file.Name)) {
        throw "Checksum entry missing for package: $($file.Name)"
    }
}

# A diagnostic LAB provenance file must match every exact output and must
# NEVER label a transient generated dependency lock production reproducible.
$provenancePath = Join-Path $assets 'LAB_BUILD_PROVENANCE.json'
if (-not $verified.ContainsKey('LAB_BUILD_PROVENANCE.json') -or
    -not (Test-Path -LiteralPath $provenancePath -PathType Leaf)) {
    throw 'Checked LAB build provenance is absent.'
}
$provenance = Get-Content -LiteralPath $provenancePath -Raw | ConvertFrom-Json
$checkoutSha = (& git rev-parse HEAD)
if ($LASTEXITCODE -ne 0 -or $checkoutSha -notmatch '^[0-9a-fA-F]{40}$') {
    throw 'Current source SHA could not be verified.'
}
$rustLock = (@(& git ls-files -- Cargo.lock) -contains 'Cargo.lock') -and
    (Test-Path -LiteralPath Cargo.lock -PathType Leaf)
$nodeLock = (@(& git ls-files -- apps/desktop/package-lock.json) -contains 'apps/desktop/package-lock.json') -and
    (Test-Path -LiteralPath apps/desktop/package-lock.json -PathType Leaf)
if ($provenance.schema_version -ne 1 -or
    $provenance.lab_only -ne $true -or
    $provenance.production_release_approved -ne $false -or
    $provenance.independently_developed_office_versions -ne $false -or
    $provenance.source_repository -cne 'TurkuazLabs/TurkuazOffice' -or
    $provenance.source_commit -cne $checkoutSha.ToLowerInvariant() -or
    $provenance.compile_variant_v1 -cne 'installer-velopack-pilot' -or
    $provenance.compile_variant_v2 -cne 'installer-velopack-pilot-v2' -or
    $provenance.release_version_v1 -cne $version -or
    $provenance.release_version_v2 -cne $nextVersion -or
    $provenance.compiled_executables_sha256_distinct -ne $true -or
    $provenance.compiled_executable_v1_sha256 -cne $compiledDigestV1.ToLowerInvariant() -or
    $provenance.compiled_executable_v2_sha256 -cne $compiledDigestV2.ToLowerInvariant() -or
    $provenance.rust_lockfile_tracked_in_checkout -ne $rustLock -or
    $provenance.node_lockfile_tracked_in_checkout -ne $nodeLock -or
    $provenance.dependencies_locked_for_reproduction -ne ($rustLock -and $nodeLock) -or
    $provenance.signing_policy -cne 'UNSIGNED_LAB_ONLY_EPHEMERAL_CMS_IN_E2E' -or
    @($provenance.packages).Count -ne 3) {
    throw 'Office LAB build provenance does not match actual inputs or release safety policy.'
}
foreach ($entry in @($provenance.packages)) {
    $found = @($requiredPackages | Where-Object { $_.Name -ceq $entry.name })
    if ($found.Count -ne 1 -or
        $entry.sha256 -cne (Get-FileHash -LiteralPath $found[0].FullName -Algorithm SHA256).Hash.ToLowerInvariant() -or
        $entry.size_bytes -ne $found[0].Length) {
        throw "Office LAB provenance package digest/size mismatch: $($entry.name)"
    }
}
Write-Host 'OFFICE_PILOT_LAB_PROVENANCE_VERIFIED_OK'

# Confirm the packaged Setup.exe carries a real Windows icon resource.
# A successful installer process alone cannot detect missing Explorer branding.
$iconInterop = @'
using System;
using System.Runtime.InteropServices;
public static class TurkuazOfficeIconResource {
    [DllImport("shell32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    public static extern uint ExtractIconEx(
        string fileName, int iconIndex, IntPtr[] largeIcons, IntPtr[] smallIcons, uint iconCount);
}
'@
Add-Type -TypeDefinition $iconInterop -ErrorAction Stop
$setupIconCount = [TurkuazOfficeIconResource]::ExtractIconEx(
    $setups[0].FullName, -1, $null, $null, 0)
if ($setupIconCount -lt 1) {
    throw 'Velopack Setup.exe has no embedded Windows icon resources.'
}
Write-Host "OFFICE_VELOPACK_SETUP_ICON_OK count=$setupIconCount"

$runnerTemp = $env:RUNNER_TEMP
if ([string]::IsNullOrWhiteSpace($runnerTemp) -or -not (Test-Path -LiteralPath $runnerTemp)) {
    throw 'Disposable GitHub Windows runner temporary folder is required.'
}
$testId = [guid]::NewGuid().ToString('N')
$testRoot = Join-Path $runnerTemp "office-velopack-smoke-$testId"
$installRoot = Join-Path $testRoot 'install'
New-Item -ItemType Directory -Path $testRoot -Force | Out-Null

function Invoke-CheckedInstaller {
    param(
        [Parameter(Mandatory = $true)][string]$Executable,
        [Parameter(Mandatory = $true)][string[]]$Arguments,
        [Parameter(Mandatory = $true)][string]$Operation
    )
    $info = [System.Diagnostics.ProcessStartInfo]::new()
    $info.FileName = $Executable
    $info.UseShellExecute = $false
    $info.CreateNoWindow = $true
    $info.WorkingDirectory = (Split-Path -Path $Executable -Parent)
    foreach ($arg in $Arguments) {
        $info.ArgumentList.Add($arg)
    }
    $proc = [System.Diagnostics.Process]::Start($info)
    if ($null -eq $proc) {
        throw "Could not start process: $Operation"
    }
    try {
        if (-not $proc.WaitForExit($ProcessTimeoutSeconds * 1000)) {
            $proc.Kill($true)
            $proc.WaitForExit(5000) | Out-Null
            throw "Installer process timed out: $Operation"
        }
        if ($proc.ExitCode -ne 0) {
            throw "Installer process failed: $Operation exit=$($proc.ExitCode)"
        }
        Write-Host "OFFICE_REAL_VELOPACK_PROCESS_OK $Operation"
    }
    finally {
        $proc.Dispose()
    }
}

try {
    Invoke-CheckedInstaller -Executable $setups[0].FullName -Operation 'install' -Arguments @(
        '--silent', '--installto', $installRoot
    )

    $updater = Join-Path $installRoot 'Update.exe'
    $installedExe = Join-Path $installRoot 'current/turkuaz-office-desktop.exe'
    if (-not (Test-Path -LiteralPath $updater -PathType Leaf)) {
        throw 'Real Velopack Update.exe was not installed.'
    }
    if (-not (Test-Path -LiteralPath $installedExe -PathType Leaf)) {
        throw 'Real Turkuaz Office Tauri executable was not installed.'
    }
    Write-Host 'OFFICE_REAL_VELOPACK_INSTALL_OK'
    $installedHashV1 = (Get-FileHash -LiteralPath $installedExe -Algorithm SHA256).Hash
    if ($installedHashV1 -cne $compiledDigestV1) {
        throw 'Installed Office V1 EXE differs from the separately compiled V1 input.'
    }
    Invoke-CheckedInstaller -Executable $installedExe -Operation 'compiled-v1-identity' -Arguments @(
        '--office-pilot-verify-compiled-v1'
    )
    Write-Host 'OFFICE_INSTALLED_COMPILED_V1_BINARY_OK'

    # Real installed binary must serve embedded assets without a Vite server.
    # A window/process-only smoke previously missed this user-visible defect.
    Invoke-CheckedInstaller -Executable $installedExe -Operation 'embedded-frontend' -Arguments @(
        '--office-pilot-verify-embedded-frontend'
    )
    Write-Host 'OFFICE_INSTALLED_FRONTEND_EMBEDDED_OK'

    $marker = Join-Path $installRoot 'current/installer-pilot-version.txt'
    if (-not (Test-Path -LiteralPath $marker -PathType Leaf) -or
        [System.IO.File]::ReadAllText($marker).Trim() -cne $version) {
        throw "Installed v1 payload marker mismatch: $version"
    }
    $userDataFolder = Join-Path $installRoot 'UserData'
    New-Item -ItemType Directory -Path $userDataFolder -Force | Out-Null
    $userFile = Join-Path $userDataFolder 'pilot-preserved-data.txt'
    Set-Content -LiteralPath $userFile -Value 'user-data-preserve-probe' -NoNewline -Encoding utf8

    $packagesDirectory = Join-Path $installRoot 'packages'
    New-Item -ItemType Directory -Path $packagesDirectory -Force | Out-Null
    function Apply-FullPilotPackage {
        param(
            [Parameter(Mandatory = $true)][System.IO.FileInfo]$Package,
            [Parameter(Mandatory = $true)][string]$ExpectedVersion,
            [Parameter(Mandatory = $true)][string]$Operation
        )
        $stagedPath = Join-Path $packagesDirectory $Package.Name
        Copy-Item -LiteralPath $Package.FullName -Destination $stagedPath -Force
        $sourceHash = (Get-FileHash -LiteralPath $Package.FullName -Algorithm SHA256).Hash
        $stagedHash = (Get-FileHash -LiteralPath $stagedPath -Algorithm SHA256).Hash
        if ($sourceHash -cne $stagedHash) { throw "Staged package SHA256 mismatch: $Operation" }
        Invoke-CheckedInstaller -Executable $updater -Operation $Operation -Arguments @(
            '--silent', '--rootDir', $installRoot,
            '--packageDir', $packagesDirectory, 'apply', '--norestart',
            '--package', $stagedPath
        )
        # The updater may stage a follow-on file swap before returning. Wait
        # for actual expected payload state, never claim success on exit 0.
        $markerDeadline = [DateTime]::UtcNow.AddSeconds(30)
        do {
            $actualMarker = if (Test-Path -LiteralPath $marker -PathType Leaf) {
                [System.IO.File]::ReadAllText($marker).Trim()
            } else { '<missing>' }
            if ($actualMarker -ceq $ExpectedVersion) { break }
            Start-Sleep -Milliseconds 500
        } while ([DateTime]::UtcNow -lt $markerDeadline)
        if ($actualMarker -cne $ExpectedVersion) {
            throw "Real Velopack $Operation failed: expected version $ExpectedVersion, found $actualMarker"
        }
        if (-not (Test-Path -LiteralPath $installedExe -PathType Leaf)) {
            throw "Tauri executable absent after $Operation"
        }
        $expectedHash = if ($ExpectedVersion -ceq $nextVersion) {
            $compiledDigestV2
        } else {
            $compiledDigestV1
        }
        $actualHash = (Get-FileHash -LiteralPath $installedExe -Algorithm SHA256).Hash
        if ($actualHash -cne $expectedHash) {
            throw "Velopack $Operation kept wrong Office compiled executable at $ExpectedVersion"
        }
        $expectedVariant = if ($ExpectedVersion -ceq $nextVersion) { 'v2' } else { 'v1' }
        Invoke-CheckedInstaller -Executable $installedExe -Operation "compiled-$expectedVariant-$Operation" -Arguments @(
            "--office-pilot-verify-compiled-$expectedVariant"
        )
        if (-not (Test-Path -LiteralPath $userFile -PathType Leaf) -or
            [System.IO.File]::ReadAllText($userFile) -cne 'user-data-preserve-probe') {
            throw "UserData not preserved across $Operation"
        }
        Write-Host "OFFICE_REAL_VELOPACK_LIFECYCLE_OK $Operation $ExpectedVersion"
    }

    Apply-FullPilotPackage -Package $fullV2[0] -ExpectedVersion $nextVersion -Operation 'update'
    Write-Host 'OFFICE_REAL_VELOPACK_DISTINCT_EXECUTABLE_UPDATE_OK'

    # Deliberately corrupt non-executable fixture to prove same-version
    # repair restores bytes rather than reporting success for a no-op.
    Set-Content -LiteralPath $marker -Value 'damaged-by-repair-smoke' -NoNewline -Encoding utf8
    Apply-FullPilotPackage -Package $fullV2[0] -ExpectedVersion $nextVersion -Operation 'repair'

    Apply-FullPilotPackage -Package $fullV1[0] -ExpectedVersion $version -Operation 'rollback'

    # Windows Update.exe may hand off its own final deletion. Exit code 0
    # confirms the launcher finished, not necessarily that all files have
    # disappeared at the same millisecond. Do NOT accept leftover binaries:
    # poll briefly, then dump the *actual* filesystem state and fail closed.
    $uninstallLog = Join-Path $testRoot 'velopack-uninstall.log'
    Invoke-CheckedInstaller -Executable $updater -Operation 'uninstall' -Arguments @(
        '--silent', '--rootDir', $installRoot, '--log', $uninstallLog, 'uninstall'
    )

    $uninstallDeadline = [DateTime]::UtcNow.AddSeconds(30)
    $currentFolder = Join-Path $installRoot 'current'
    do {
        $updaterPresent = Test-Path -LiteralPath $updater -PathType Leaf
        $currentPresent = Test-Path -LiteralPath $currentFolder
        if (-not $updaterPresent -and -not $currentPresent) {
            break
        }
        Start-Sleep -Milliseconds 500
    } while ([DateTime]::UtcNow -lt $uninstallDeadline)

    if ($updaterPresent -or $currentPresent) {
        Write-Warning ('Uninstall retained active paths after bounded wait: Update.exe={0}; current={1}' -f $updaterPresent, $currentPresent)
        if (Test-Path -LiteralPath $installRoot) {
            Get-ChildItem -LiteralPath $installRoot -Force -Recurse -ErrorAction Continue |
                Select-Object -First 50 -ExpandProperty FullName |
                ForEach-Object { Write-Warning "REMAINING_PATH: $_" }
        }
        if (Test-Path -LiteralPath $uninstallLog -PathType Leaf) {
            Get-Content -LiteralPath $uninstallLog -Tail 100 |
                ForEach-Object { Write-Warning "VELOPACK_UNINSTALL_LOG: $_" }
        }
        throw 'Velopack uninstall did not remove active binaries within 30 seconds.'
    }
    Write-Host 'OFFICE_REAL_VELOPACK_UNINSTALL_OK'
    Write-Host 'OFFICE_REAL_VELOPACK_FULL_LIFECYCLE_OK'
}
finally {
    # If a step failed, attempt the same official uninstall command before
    # deleting any files. Only disposable runner-temp paths are touched.
    $updater = Join-Path $installRoot 'Update.exe'
    if (Test-Path -LiteralPath $updater -PathType Leaf) {
        try {
            Invoke-CheckedInstaller -Executable $updater -Operation 'cleanup-uninstall' -Arguments @(
                '--silent', '--rootDir', $installRoot, 'uninstall'
            )
        }
        catch {
            Write-Warning "Cleanup uninstall could not finish: $_"
        }
    }
    if (Test-Path -LiteralPath $testRoot) {
        Remove-Item -LiteralPath $testRoot -Recurse -Force -ErrorAction SilentlyContinue
    }
}