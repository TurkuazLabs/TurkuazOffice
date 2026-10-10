# 📄 Dosya Yolu: /tools/installer/Test-TurkuazOfficeVelopackInstall.ps1
# 📌 Amac: Turkuaz Office pilot paketini gecici Windows runner'a gercekten kurup kaldirarak temel lifecycle kabulunu sinamak
# 📌 Modul - Tool PowerShell
# Version: 0.3.0
# Aciklama: Setup.exe/Update.exe gercek Windows processleri; private temp hedef, SHA256 ve timeout ile fail-closed smoke test
# Bagimli Oldugu Katman: Tool | CI | Test

[CmdletBinding()]
param(
    [string]$AssetsPath = "artifacts/turkuazinstaller-pilot/assets",
    [ValidateRange(30, 360)]
    [int]$ProcessTimeoutSeconds = 180
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if (-not $IsWindows) {
    throw 'This smoke test must run on Windows.'
}
$assets = (Resolve-Path -LiteralPath $AssetsPath).Path
$setups = @(Get-ChildItem -LiteralPath $assets -File -Filter '*Setup.exe')
$fulls = @(Get-ChildItem -LiteralPath $assets -File -Filter '*-full.nupkg')
if ($setups.Count -ne 1 -or $fulls.Count -ne 1) {
    throw 'Expected one pilot Setup.exe and one full.nupkg.'
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
foreach ($file in @($setups[0], $fulls[0])) {
    if (-not $verified.ContainsKey($file.Name)) {
        throw "Checksum entry missing for package: $($file.Name)"
    }
}

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

    Invoke-CheckedInstaller -Executable $updater -Operation 'uninstall' -Arguments @(
        '--silent', '--rootDir', $installRoot, 'uninstall'
    )

    if ((Test-Path -LiteralPath $updater -PathType Leaf) -or
        (Test-Path -LiteralPath (Join-Path $installRoot 'current'))) {
        throw 'Velopack uninstall left active installation files.'
    }
    Write-Host 'OFFICE_REAL_VELOPACK_UNINSTALL_OK'
    Write-Host 'OFFICE_REAL_VELOPACK_INSTALL_SMOKE_OK'
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
