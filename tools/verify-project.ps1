# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/tools/verify-project.ps1
# 📌 Amac: Windows ortaminda Writer v0.2.0 Core + Desktop + storage + recovery + file protection + font/layout kontratlarini dogrular
# 📌 Modul - FileType: Tool - PowerShell
# Version: 0.2.0
# Aciklama: Header, version, zorunlu dosya, typography, ribbon, TKO package, Desktop storage ve recovery kontratlarini uygular
# Bagimli Oldugu Katman: Tool

$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $PSScriptRoot
$Version = "0.2.0"
$RequiredFiles = @(
    "README.md",
    "CHANGELOG.md",
    "Cargo.toml",
    "config/project.yml",
    "docs/08-implementation/m1-writer-domain-v0.2.0.md",
    "docs/08-implementation/m1-desktop-shell-v0.2.0.md",
    "docs/08-implementation/m1-rich-text-ime-v0.2.0.md",
    "docs/08-implementation/m1-rich-text-ime-validation.md",
    "docs/08-implementation/m1-typography-ribbon-v0.2.0.md",
    "docs/08-implementation/m1-typography-ribbon-validation.md",
    "docs/08-implementation/m1-local-open-save-v0.2.0.md",
    "docs/08-implementation/m1-local-open-save-validation.md",
    "docs/07-quality/local-open-save-test-matrix.md",
    "docs/06-adr/0016-tko-v1-local-safe-save.md",
    "docs/06-adr/0017-autosave-recovery-snapshot.md",
    "docs/07-quality/autosave-recovery-test-matrix.md",
    "docs/08-implementation/m1-autosave-recovery-v0.2.0.md",
    "docs/08-implementation/m1-autosave-recovery-validation.md",
    "docs/07-quality/desktop-shell-test-matrix.md",
    "docs/07-quality/rich-text-ime-test-matrix.md",
    "docs/07-quality/typography-ribbon-test-matrix.md",
    "docs/06-adr/0012-solidjs-desktop-ui.md",
    "docs/06-adr/0013-desktop-ipc-thin-bridge.md",
    "docs/06-adr/0014-ime-rich-text-contenteditable.md",
    "docs/06-adr/0015-caret-typing-style-ribbon-typography.md",
    "crates/turkuaz-office-writer/src/services/writer_command.rs",
    "crates/turkuaz-office-writer/src/services/writer_command_service.rs",
    "crates/turkuaz-office-writer/src/services/writer_editor_service.rs",
    "crates/turkuaz-office-writer/src/services/tko_package_service.rs",
    "crates/turkuaz-office-writer/src/services/tko_package_types.rs",
    "crates/turkuaz-office-writer/src/tools/tko_archive_tool.rs",
    "crates/turkuaz-office-writer/src/tools/tko_yaml_tool.rs",
    "crates/turkuaz-office-writer/tests/tko_package_tests.rs",
    "crates/turkuaz-office-writer/src/services/writer_types.rs",
    "crates/turkuaz-office-writer/src/views/writer_view.rs",
    "apps/desktop/package.json",
    "apps/desktop/tsconfig.json",
    "apps/desktop/vite.config.ts",
    "apps/desktop/src/main.tsx",
    "apps/desktop/src/config/dom-contract.ts",
    "apps/desktop/src/config/typography.ts",
    "apps/desktop/src/config/ribbon.ts",
    "apps/desktop/src/controllers/writer.controller.ts",
    "apps/desktop/src/services/writer-session.service.ts",
    "apps/desktop/src/repositories/writer-session.repository.ts",
    "apps/desktop/src/tools/tauri-writer.tool.ts",
    "apps/desktop/src/tools/text-offset.tool.ts",
    "apps/desktop/src/tools/dom-selection.tool.ts",
    "apps/desktop/src/tools/native-file-dialog.tool.ts",
    "apps/desktop/src/config/file-format.ts",
    "apps/desktop/src/views/writer-ribbon.tsx",
    "apps/desktop/src/views/writer-shell.tsx",
    "apps/desktop/src-tauri/Cargo.toml",
    "apps/desktop/src-tauri/tauri.conf.json5",
    "apps/desktop/src-tauri/capabilities/main-capability.toml",
    "apps/desktop/src-tauri/src/controllers/writer_desktop_controller.rs",
    "apps/desktop/src-tauri/src/services/writer_desktop_service.rs",
    "apps/desktop/src-tauri/src/services/writer_storage_service.rs",
    "apps/desktop/src-tauri/src/services/writer_recovery_service.rs",
    "apps/desktop/src-tauri/src/tools/recovery_path_tool.rs",
    "apps/desktop/src-tauri/src/tools/recovery_metadata_tool.rs",
    "apps/desktop/src-tauri/src/views/recovery_dto.rs",
    "apps/desktop/src/views/writer-recovery-panel.tsx",
    "apps/desktop/src-tauri/src/tools/local_file_tool.rs",
    "apps/desktop/src-tauri/tests/writer_desktop_service_tests.rs",
    "docs/06-adr/0018-external-change-cooperative-lock.md",
    "docs/07-quality/external-change-test-matrix.md",
    "docs/08-implementation/m1-external-change-v0.2.0.md",
    "docs/08-implementation/m1-external-change-validation.md",
    "apps/desktop/src-tauri/src/services/writer_file_session_service.rs",
    "apps/desktop/src-tauri/src/tools/file_fingerprint_tool.rs",
    "apps/desktop/src-tauri/src/tools/file_lock_tool.rs",
    "apps/desktop/src-tauri/src/views/file_session_dto.rs",
    "apps/desktop/src/views/writer-file-protection-banner.tsx",
    "docs/06-adr/0019-twip-page-layout-font-fallback.md",
    "docs/07-quality/font-layout-test-matrix.md",
    "docs/08-implementation/m1-font-layout-v0.2.0.md",
    "docs/08-implementation/m1-font-layout-validation.md",
    "apps/desktop/src/config/layout.ts",
    "apps/desktop/src/services/writer-layout.service.ts",
    "apps/desktop/src/tools/font-capability.tool.ts"
)

foreach ($RelativePath in $RequiredFiles) {
    $FullPath = Join-Path $Root $RelativePath
    if (-not (Test-Path $FullPath)) {
        throw "Eksik zorunlu dosya: $RelativePath"
    }
}

$TextExtensions = @(".md", ".rs", ".toml", ".yml", ".yaml", ".ps1", ".sh", ".ts", ".tsx", ".css", ".html", ".json", ".json5")
$Files = Get-ChildItem -Path $Root -Recurse -File | Where-Object {
    ($TextExtensions -contains $_.Extension) -and
    ($_.FullName -notmatch "[\/]node_modules[\/]") -and
    ($_.FullName -notmatch "[\/]dist[\/]") -and
    ($_.FullName -notmatch "[\/]target[\/]")
}

foreach ($File in $Files) {
    $Head = Get-Content -Path $File.FullName -TotalCount 14 -Raw
    if ($Head -notmatch "Dosya Yolu") {
        throw "Header eksik: $($File.FullName)"
    }
    if ($Head -notmatch "Version: $Version") {
        throw "Version header uyumsuz: $($File.FullName)"
    }
}

$Cargo = Get-Content -Path (Join-Path $Root "Cargo.toml") -Raw
if ($Cargo -notmatch 'apps/desktop/src-tauri') {
    throw "Desktop Tauri workspace member eksik"
}

$ProjectConfig = Get-Content -Path (Join-Path $Root "config/project.yml") -Raw
if ($ProjectConfig -notmatch 'framework: solidjs') {
    throw "SolidJS desktop UI baseline config eksik"
}
if ($ProjectConfig -notmatch 'canonical_state_in_frontend: false') {
    throw "Frontend canonical state yasa kontrati eksik"
}
if ($ProjectConfig -notmatch 'materialize_on_text_insert: true') {
    throw "Typing-style materialization kontrati eksik"
}

$WriterCommand = Get-Content -Path (Join-Path $Root "crates/turkuaz-office-writer/src/services/writer_command.rs") -Raw
if ($WriterCommand -notmatch 'InsertStyledText') {
    throw "InsertStyledText command eksik"
}
if ($WriterCommand -notmatch 'ApplyParagraphStyle') {
    throw "ApplyParagraphStyle command eksik"
}

$DomSelectionTool = Get-Content -Path (Join-Path $Root "apps/desktop/src/tools/dom-selection.tool.ts") -Raw
if ($DomSelectionTool -notmatch 'focusAndRestoreParagraphSelection') {
    throw "Font select sonrasi selection restore kontrati eksik"
}

$SessionRepo = Get-Content -Path (Join-Path $Root "apps/desktop/src/repositories/writer-session.repository.ts") -Raw
if ($SessionRepo -notmatch 'typingStyle') {
    throw "Typing-style Repository state eksik"
}

$WriterShell = Get-Content -Path (Join-Path $Root "apps/desktop/src/views/writer-shell.tsx") -Raw
if ($WriterShell -notmatch 'WriterRibbon') {
    throw "WriterRibbon shell baglantisi eksik"
}


$TkoService = Get-Content -Path (Join-Path $Root "crates/turkuaz-office-writer/src/services/tko_package_service.rs") -Raw
if ($TkoService -notmatch 'TKO_MANIFEST_ENTRY') { throw "TKO manifest package contract eksik" }

$ArchiveTool = Get-Content -Path (Join-Path $Root "crates/turkuaz-office-writer/src/tools/tko_archive_tool.rs") -Raw
if ($ArchiveTool -notmatch 'CompressionMethod::Stored') { throw "TKO Stored compression contract eksik" }
if ($ArchiveTool -notmatch 'DirectoryEntryUnsupported') { throw "TKO directory entry guard eksik" }

$StorageTool = Get-Content -Path (Join-Path $Root "apps/desktop/src-tauri/src/tools/local_file_tool.rs") -Raw
if ($StorageTool -notmatch 'write_safe_replace') { throw "Desktop safe replace Tool eksik" }

$Capability = Get-Content -Path (Join-Path $Root "apps/desktop/src-tauri/capabilities/main-capability.toml") -Raw
if ($Capability -notmatch 'dialog:allow-message') { throw "Unsaved confirm dialog permission eksik" }

if ($ProjectConfig -notmatch 'desktop_recovery:') { throw "Desktop recovery config eksik" }
if ($ProjectConfig -notmatch 'overwrite_source_on_autosave: false') { throw "Autosave source overwrite yasa kontrati eksik" }

$RecoveryService = Get-Content -Path (Join-Path $Root "apps/desktop/src-tauri/src/services/writer_recovery_service.rs") -Raw
if ($RecoveryService -notmatch 'TkoPackageService::serialize') { throw "Recovery TKO serializer reuse kontrati eksik" }

$RecoveryPanel = Get-Content -Path (Join-Path $Root "apps/desktop/src/views/writer-recovery-panel.tsx") -Raw
if ($RecoveryPanel -notmatch 'recoverSnapshot') { throw "Recovery UI Recover aksiyonu eksik" }


if ($ProjectConfig -notmatch 'desktop_file_protection:') { throw "Desktop file protection config eksik" }
if ($ProjectConfig -notmatch 'timestamp_only_detection: false') { throw "Content fingerprint kontrati eksik" }
if ($ProjectConfig -notmatch 'backend_mutation_guard: true') { throw "Backend mutation guard kontrati eksik" }
$FileSessionService = Get-Content -Path (Join-Path $Root "apps/desktop/src-tauri/src/services/writer_file_session_service.rs") -Raw
if ($FileSessionService -notmatch 'ExternalChangeConflict') { throw "External change conflict guard eksik" }
$FileLockTool = Get-Content -Path (Join-Path $Root "apps/desktop/src-tauri/src/tools/file_lock_tool.rs") -Raw
if ($FileLockTool -notmatch 'create_new\(true\)') { throw "Cooperative lock create_new kontrati eksik" }
$ProtectionBanner = Get-Content -Path (Join-Path $Root "apps/desktop/src/views/writer-file-protection-banner.tsx") -Raw
if ($ProtectionBanner -notmatch 'keepLocalVersion') { throw "External change protection banner eksik" }



if ($ProjectConfig -notmatch 'writer_layout:') { throw "Writer layout config eksik" }
if ($ProjectConfig -notmatch 'canonical_unit: twip') { throw "Canonical twip layout kontrati eksik" }
if ($ProjectConfig -notmatch 'browser_dom_measurement_is_canonical: false') { throw "DOM canonical layout yasa kontrati eksik" }
$WriterView = Get-Content -Path (Join-Path $Root "crates/turkuaz-office-writer/src/views/writer_view.rs") -Raw
if ($WriterView -notmatch 'WriterPageSettingsView') { throw "Writer PageSettings View eksik" }
$LayoutConfig = Get-Content -Path (Join-Path $Root "apps/desktop/src/config/layout.ts") -Raw
if ($LayoutConfig -notmatch 'TWIPS_PER_INCH') { throw "Twip render config eksik" }
if ($LayoutConfig -notmatch 'WRITER_FONT_FALLBACK_PROFILES') { throw "Font fallback profile config eksik" }
$LayoutService = Get-Content -Path (Join-Path $Root "apps/desktop/src/services/writer-layout.service.ts") -Raw
if ($LayoutService -notmatch 'class WriterLayoutService') { throw "WriterLayoutService eksik" }
$FontTool = Get-Content -Path (Join-Path $Root "apps/desktop/src/tools/font-capability.tool.ts") -Raw
if ($FontTool -notmatch 'class FontCapabilityTool') { throw "FontCapabilityTool eksik" }
$Statusbar = Get-Content -Path (Join-Path $Root "apps/desktop/src/views/writer-statusbar.tsx") -Raw
if ($Statusbar -notmatch 'controller.zoomIn') { throw "Statusbar zoom kontrolu eksik" }

Write-Host "Turkuaz Office Writer v0.2.0 Font Layout verification BASARILI."
