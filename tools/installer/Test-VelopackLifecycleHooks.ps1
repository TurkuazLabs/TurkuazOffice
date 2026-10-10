# 📄 Dosya Yolu: /tools/installer/Test-VelopackLifecycleHooks.ps1
# 📌 Amac: Velopack hook argv olaylarinin normal Tauri arayuzunu acmadan hizla cikmasini dogrulamak
# 📌 Modul - Tool PowerShell
# Version: 0.1.0
# Aciklama: Sadece Windows pilot feature build'i icin child-process hook exit ve timeout regression testi
# Bagimli Oldugu Katman: Tool | CI | Test

[CmdletBinding()]
param(
    [string]$BinaryPath = "target/release/turkuaz-office-desktop.exe",
    [string]$Version = "0.3.1",
    [ValidateRange(2000, 30000)]
    [int]$TimeoutMilliseconds = 12000
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$executable = (Resolve-Path -LiteralPath $BinaryPath).Path
if (-not (Test-Path -LiteralPath $executable -PathType Leaf)) {
    throw "Pilot binary does not exist: $executable"
}
if ($Version -notmatch '^\d+\.\d+\.\d+$') {
    throw "Invalid test version"
}

$hookNames = @(
    "--veloapp-install",
    "--veloapp-obsolete",
    "--veloapp-updated",
    "--veloapp-uninstall"
)

foreach ($hook in $hookNames) {
    $start = [System.Diagnostics.ProcessStartInfo]::new()
    $start.FileName = $executable
    $start.Arguments = "$hook $Version"
    $start.UseShellExecute = $false
    $start.CreateNoWindow = $true

    $process = [System.Diagnostics.Process]::Start($start)
    if ($null -eq $process) {
        throw "Could not start hook process: $hook"
    }
    try {
        if (-not $process.WaitForExit($TimeoutMilliseconds)) {
            $process.Kill()
            $process.WaitForExit(5000) | Out-Null
            throw "Velopack hook did not exit promptly: $hook"
        }
        if ($process.ExitCode -ne 0) {
            throw "Velopack hook failed: $hook exit=$($process.ExitCode)"
        }
        Write-Host "VELOPACK_HOOK_OK $hook"
    }
    finally {
        $process.Dispose()
    }
}
Write-Host "ALL_VELOPACK_PILOT_HOOKS_OK"
