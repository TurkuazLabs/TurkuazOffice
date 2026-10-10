# 📄 Dosya Yolu: /tools/installer/New-TurkuazInstallerPilot.ps1
# 📌 Amac: Mevcut Tauri Windows EXE'den ek NSIS degisikligi olmadan Velopack pilot Setup ve full nupkg uretmek
# 📌 Modul - Tool PowerShell
# Version: 0.2.0
# Aciklama: Yalniz paketleme probe'u; imzalama, registry/file association veya urun guncelleme yetkisi uretmez
# Bagimli Oldugu Katman: Tool | Config | CI

[CmdletBinding()]
param(
    [string]$ToolRoot = ".pilot-vpk",
    [string]$OutputRoot = "artifacts/turkuazinstaller-pilot"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "../..")).Path
$packageJson = Get-Content -LiteralPath (Join-Path $repoRoot "apps/desktop/package.json") -Raw |
    ConvertFrom-Json
$version = [string]$packageJson.version

if ($version -notmatch '^\d+\.\d+\.\d+$') {
    throw "Desktop version must be a strict three-part SemVer for this pilot."
}

$workspaceCargo = Get-Content -LiteralPath (Join-Path $repoRoot "Cargo.toml") -Raw
$tauriConfig = Get-Content -LiteralPath (Join-Path $repoRoot "apps/desktop/src-tauri/tauri.conf.json5") -Raw
$escapedVersion = [regex]::Escape($version)
if ($workspaceCargo -notmatch "(?m)^version\s*=\s*\u0022$escapedVersion\u0022\s*$" -or
    $tauriConfig -notmatch "(?m)^\s*version:\s*\u0022$escapedVersion\u0022,?\s*$") {
    throw "package.json, Cargo workspace and Tauri config versions do not match."
}

$vpkPath = [System.IO.Path]::GetFullPath((Join-Path $repoRoot "$ToolRoot/vpk.exe"))
$nativeExe = Join-Path $repoRoot "target/release/turkuaz-office-desktop.exe"
if (-not (Test-Path -LiteralPath $vpkPath -PathType Leaf)) {
    throw "Pinned vpk.exe was not found: $vpkPath"
}
if (-not (Test-Path -LiteralPath $nativeExe -PathType Leaf)) {
    throw "Tauri native Windows executable was not found: $nativeExe"
}

$root = [System.IO.Path]::GetFullPath((Join-Path $repoRoot $OutputRoot))
if (Test-Path -LiteralPath $root) {
    Remove-Item -LiteralPath $root -Recurse -Force
}
$payload = Join-Path $root "payload"
$releases = Join-Path $root "releases"
$assets = Join-Path $root "assets"
$temp = Join-Path $root "tmp"
foreach ($folder in @($payload, $releases, $assets, $temp)) {
    New-Item -ItemType Directory -Path $folder -Force | Out-Null
}

Copy-Item -LiteralPath $nativeExe -Destination (Join-Path $payload "turkuaz-office-desktop.exe")
$env:VELOPACK_TEMP = $temp

# The optional Windows pilot feature initializes the official Velopack
# startup handler before Tauri. Do NOT skip vpk app verification. Still
# lab-only until real Windows installation lifecycle acceptance passes.
$output = & $vpkPath --legacyConsole true --yes true --skip-updates true --verbose true pack `
    --runtime win-x64 `
    --packId "turkuazlabs.turkuazoffice.pilot" `
    --packTitle "Turkuaz Office (LAB ONLY)" `
    --packVersion $version `
    --packDir $payload `
    --mainExe "turkuaz-office-desktop.exe" `
    --outputDir $releases `
    --delta none 2>&1
$output | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -ne 0) {
    throw "Real Velopack packaging failed."
}

$setupFiles = @(Get-ChildItem -LiteralPath $releases -File -Filter "*Setup.exe")
$fullFiles = @(Get-ChildItem -LiteralPath $releases -File -Filter "*-full.nupkg")
if ($setupFiles.Count -ne 1 -or $fullFiles.Count -ne 1) {
    throw "Expected exactly one Velopack Setup and one full nupkg (setup=$($setupFiles.Count), full=$($fullFiles.Count))."
}
foreach ($file in @($setupFiles[0], $fullFiles[0])) {
    if ($file.Length -le 0) { throw "Empty Velopack artifact: $($file.Name)" }
    Copy-Item -LiteralPath $file.FullName -Destination (Join-Path $assets $file.Name)
}

$instructions = @'
TURKUAZ OFFICE + TURKUAZINSTALLER = EXPERIMENTAL LAB ARTIFACT
================================================================
This package is unsigned and NOT a TurkuazInstaller production release.
It proves only that a Tauri Windows executable can be packaged into
Velopack Setup.exe and full nupkg with the same vpk used in Core E2E.
Do not run on a customer's machine, publish as stable, or disable AV.
Use an isolated disposable Windows x64 test VM.

PRODUCTION BLOCKERS (NOT IMPLEMENTED HERE):
- No signed installer-manifest.yml + .p7s and external cert-pinned trust.
- Optional Velopack startup hook is checked, but real Windows VM lifecycle is NOT yet proven.
- NSIS Writer/Sheet Start Menu shortcuts are NOT migrated.
- NSIS .tko file association is NOT migrated.
- Need WebView2/prerequisite, clean install/update/repair/rollback/uninstall
  with real TurkuazInstaller, and preserve-data acceptance on Windows.
- Existing NSIS preview/release pipeline remains authoritative.
- No Pro licensing or staged rollout is enabled by this pilot.
'@
Set-Content -LiteralPath (Join-Path $assets "READ-ME-LAB-ONLY.txt") -Value $instructions -Encoding utf8

$lines = Get-ChildItem -LiteralPath $assets -File |
    Sort-Object Name |
    ForEach-Object {
        $hash = (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
        "$hash *$($_.Name)"
    }
[System.IO.File]::WriteAllLines(
    (Join-Path $assets "SHA256SUMS.txt"),
    $lines,
    [System.Text.UTF8Encoding]::new($false)
)

Write-Host "OFFICE_VELOPACK_PACKAGING_PROBE_OK version=$version"
