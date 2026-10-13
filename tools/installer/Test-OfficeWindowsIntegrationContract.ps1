# 📄 Dosya Yolu: /tools/installer/Test-OfficeWindowsIntegrationContract.ps1
# 📌 Amac: NSIS Writer/Sheet shortcut and .tko contract regression without mutating Windows
# 📌 Modul - Tool PowerShell
# Version: 0.1.0
# Aciklama: Lab-only read-only source parity and compiled native module parser verification
# Bagimli Oldugu Katman: Tool | CI | Test

[CmdletBinding()]
param(
    [string]$CompiledBinaryRoot = ".pilot-vpk/binaries",
    [ValidateRange(2000, 30000)][int]$ProcessTimeoutMilliseconds = 12000
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if (-not $IsWindows) { throw 'Office Windows integration contract requires Windows.' }
$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
$tauri = [IO.File]::ReadAllText(
    (Join-Path $repoRoot 'apps/desktop/src-tauri/tauri.conf.json5'))
$hooks = [IO.File]::ReadAllText(
    (Join-Path $repoRoot 'apps/desktop/src-tauri/windows/installer-hooks.nsh'))
$moduleSource = [IO.File]::ReadAllText(
    (Join-Path $repoRoot 'apps/desktop/src-tauri/src/tools/desktop_launch_tool.rs'))

if ($tauri -notmatch 'installerHooks:\s*"\./windows/installer-hooks\.nsh"' -or
    $tauri -notmatch 'startMenuFolder:\s*"Turkuaz Office"' -or
    $tauri -notmatch 'ext:\s*\[\s*"tko"\s*\]' -or
    $tauri -notmatch 'mimeType:\s*"application/x-turkuaz-office"') {
    throw 'Tauri NSIS shortcut or .tko association baseline has changed.'
}
foreach ($expected in @(
    '!macro NSIS_HOOK_POSTINSTALL',
    '!macro NSIS_HOOK_PREUNINSTALL',
    'CreateShortcut "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}\${TURKUAZ_WRITER_SHORTCUT}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe" "--module writer"',
    'CreateShortcut "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}\${TURKUAZ_SHEET_SHORTCUT}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe" "--module sheet"',
    'Delete "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}\${TURKUAZ_WRITER_SHORTCUT}.lnk"',
    'Delete "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}\${TURKUAZ_SHEET_SHORTCUT}.lnk"'
)) {
    if (-not $hooks.Contains($expected, [StringComparison]::Ordinal)) {
        throw "NSIS Writer/Sheet install/uninstall shortcut contract changed: $expected"
    }
}
foreach ($expected in @('value == "--module"', 'strip_prefix("--module=")', '"writer"', '"sheet"')) {
    if (-not $moduleSource.Contains($expected, [StringComparison]::Ordinal)) {
        throw "Native Writer/Sheet module parser source contract changed: $expected"
    }
}
Write-Host 'OFFICE_NSIS_SHORTCUT_AND_TKO_SOURCE_CONTRACT_OK'

$compiledRoot = (Resolve-Path -LiteralPath $CompiledBinaryRoot).Path
foreach ($version in @('v1', 'v2')) {
    $binary = (Resolve-Path -LiteralPath (Join-Path $compiledRoot "office-$version.exe")).Path
    $start = [Diagnostics.ProcessStartInfo]::new()
    $start.FileName = $binary
    $start.ArgumentList.Add('--office-pilot-verify-module-launch')
    $start.UseShellExecute = $false
    $start.CreateNoWindow = $true
    $process = [Diagnostics.Process]::Start($start)
    if ($null -eq $process) { throw "Cannot start Office $version headless module probe." }
    try {
        if (-not $process.WaitForExit($ProcessTimeoutMilliseconds)) {
            $process.Kill($true)
            $process.WaitForExit(5000) | Out-Null
            throw "Office $version module parser probe timed out."
        }
        if ($process.ExitCode -ne 0) {
            throw "Office $version module parser probe failed, exit=$($process.ExitCode)."
        }
    }
    finally { $process.Dispose() }
    Write-Host "OFFICE_COMPILED_MODULE_LAUNCH_$($version.ToUpperInvariant())_OK"
}
Write-Host 'OFFICE_NSIS_TKO_AND_COMPILED_MODULE_LAUNCH_READ_ONLY_CONTRACT_OK'
