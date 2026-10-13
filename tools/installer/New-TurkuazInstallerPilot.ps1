# 📄 Dosya Yolu: /tools/installer/New-TurkuazInstallerPilot.ps1
# 📌 Amac: Mevcut Tauri Windows EXE'den ek NSIS degisikligi olmadan Velopack pilot Setup ve full nupkg uretmek
# 📌 Modul - Tool PowerShell
# Version: 0.5.0
# Aciklama: Ayrica derlenmis V1/V2 LAB exe'lerini paketler; signing/NSIS/file association/production yapmaz
# Bagimli Oldugu Katman: Tool | Config | CI

[CmdletBinding()]
param(
    [string]$ToolRoot = ".pilot-vpk",
    [string]$OutputRoot = "artifacts/turkuazinstaller-pilot",
    [string]$CompiledBinaryRoot = ".pilot-vpk/binaries"
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
# LAB V2 uses a separate compiled Tauri binary with opt-in build
# identity. Both compiled binaries share the same application source;
# this is not an authentic historical customer release upgrade.
$parsedVersion = [version]$version
if ($parsedVersion.Build -ge 2147483647) { throw 'Pilot patch version overflow.' }
$nextVersion = '{0}.{1}.{2}' -f $parsedVersion.Major, $parsedVersion.Minor, ($parsedVersion.Build + 1)
if ($workspaceCargo -notmatch "(?m)^version\s*=\s*\u0022$escapedVersion\u0022\s*$" -or
    $tauriConfig -notmatch "(?m)^\s*version:\s*\u0022$escapedVersion\u0022,?\s*$") {
    throw "package.json, Cargo workspace and Tauri config versions do not match."
}

$vpkPath = [System.IO.Path]::GetFullPath((Join-Path $repoRoot "$ToolRoot/vpk.exe"))
$compiledRoot = [IO.Path]::GetFullPath((Join-Path $repoRoot $CompiledBinaryRoot))
$nativeV1 = Join-Path $compiledRoot "office-v1.exe"
$nativeV2 = Join-Path $compiledRoot "office-v2.exe"
if (-not (Test-Path -LiteralPath $vpkPath -PathType Leaf)) {
    throw "Pinned vpk.exe was not found: $vpkPath"
}
foreach ($nativeExe in @($nativeV1, $nativeV2)) {
    if (-not (Test-Path -LiteralPath $nativeExe -PathType Leaf)) {
        throw "Separately compiled Office LAB executable was not found: $nativeExe"
    }
}
$compiledHashV1 = (Get-FileHash -LiteralPath $nativeV1 -Algorithm SHA256).Hash
$compiledHashV2 = (Get-FileHash -LiteralPath $nativeV2 -Algorithm SHA256).Hash
if ($compiledHashV1 -ceq $compiledHashV2) {
    throw "Refusing marker-only V2 packaging: Office LAB compiled binaries are identical."
}
Write-Host "OFFICE_PILOT_COMPILED_BINARY_HASHES_DISTINCT_OK"

# Use the same Windows icon already declared by Tauri instead of duplicating
# a second branding path in this packaging tool.
$iconDeclaration = [regex]::Match(
    $tauriConfig,
    '(?m)^\s*icon:\s*\[[^\]\r\n]*"(?<path>icons/[A-Za-z0-9_./-]+\.ico)"'
)
if (-not $iconDeclaration.Success) {
    throw 'Tauri bundle configuration must declare a Windows .ico file.'
}
$iconRelativePath = $iconDeclaration.Groups['path'].Value
if ($iconRelativePath -match '(^|/)\.\.(/|$)') {
    throw 'Tauri Windows icon path cannot traverse parent directories.'
}
$iconFile = Join-Path $repoRoot (Join-Path 'apps/desktop/src-tauri' $iconRelativePath)
if (-not (Test-Path -LiteralPath $iconFile -PathType Leaf)) {
    throw "Tauri Windows icon missing: $iconFile"
}
$iconBytes = [System.IO.File]::ReadAllBytes($iconFile)
if ($iconBytes.Length -lt 6 -or $iconBytes[0] -ne 0 -or $iconBytes[1] -ne 0 -or
    $iconBytes[2] -ne 1 -or $iconBytes[3] -ne 0 -or
    ($iconBytes[4] -eq 0 -and $iconBytes[5] -eq 0)) {
    throw 'Tauri Windows icon has an invalid ICO header.'
}
Write-Host 'OFFICE_VELOPACK_ICON_SOURCE_OK'

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

Copy-Item -LiteralPath $nativeV1 -Destination (Join-Path $payload "turkuaz-office-desktop.exe")
$marker = Join-Path $payload "installer-pilot-version.txt"
Set-Content -LiteralPath $marker -Value $version -NoNewline -Encoding utf8
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
    --icon $iconFile `
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

# Create the second intentionally LAB-only patch-version package using a
# SEPARATELY COMPILED EXE (different identity code), NOT a marker-only
# repackaging. The actual Writer/Sheet business logic remains the same.
$releasesTwo = Join-Path $root "releases-v2"
New-Item -ItemType Directory -Path $releasesTwo -Force | Out-Null
Copy-Item -LiteralPath $nativeV2 -Destination (Join-Path $payload "turkuaz-office-desktop.exe") -Force
Set-Content -LiteralPath $marker -Value $nextVersion -NoNewline -Encoding utf8
$outputTwo = & $vpkPath --legacyConsole true --yes true --skip-updates true --verbose true pack `
    --runtime win-x64 `
    --packId "turkuazlabs.turkuazoffice.pilot" `
    --packTitle "Turkuaz Office (LAB ONLY)" `
    --packVersion $nextVersion `
    --packDir $payload `
    --mainExe "turkuaz-office-desktop.exe" `
    --icon $iconFile `
    --outputDir $releasesTwo `
    --delta none 2>&1
$outputTwo | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -ne 0) { throw "Second Velopack pilot package failed." }
$fullTwo = @(Get-ChildItem -LiteralPath $releasesTwo -File -Filter "*-$nextVersion-full.nupkg")
if ($fullTwo.Count -ne 1 -or $fullTwo[0].Length -le 0) {
    throw "Expected exactly one non-empty second-version full nupkg."
}
Copy-Item -LiteralPath $fullTwo[0].FullName -Destination (Join-Path $assets $fullTwo[0].Name)

$instructions = @'
TURKUAZ OFFICE + TURKUAZINSTALLER = EXPERIMENTAL LAB ARTIFACT
================================================================
This package is unsigned and NOT a TurkuazInstaller production release.
It proves that two separately compiled Tauri Windows LAB binaries with
different opt-in compile identities can be packaged into Velopack
Setup.exe and two full nupkgs with the pinned vpk used by Core E2E.
They use the SAME Office application source and Writer/Sheet logic.
Do NOT interpret v2 as an independently developed product release.
Do not run on a customer's machine, publish as stable, or disable AV.
Use an isolated disposable Windows x64 test VM.

PRODUCTION BLOCKERS (NOT IMPLEMENTED HERE):
- No signed installer-manifest.yml + .p7s and external cert-pinned trust.
- CI verifies lifecycle on disposable hosted Windows only; clean end-user
  Windows 10/11 GUI/UAC lifecycle is still NOT manually accepted.
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

Write-Host "OFFICE_VELOPACK_PACKAGING_PROBE_OK versions=$version,$nextVersion"
