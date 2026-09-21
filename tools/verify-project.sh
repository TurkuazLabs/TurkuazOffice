#!/usr/bin/env bash
# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/tools/verify-project.sh
# 📌 Amac: Linux ve CI ortaminda Writer v0.2.0 Core + Desktop + storage + recovery + file protection + font/layout + clipboard fragment kontratlarini dogrular
# 📌 Modul - FileType: Tool - Shell
# Version: 0.2.0
# Aciklama: Header, version, zorunlu dosya, katman, editor, storage, recovery, file protection, font/layout ve clipboard fragment contract kontrollerini uygular
# Bagimli Oldugu Katman: Tool

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="0.2.0"

required_files=(
  "README.md"
  "CHANGELOG.md"
  "Cargo.toml"
  "rust-toolchain.toml"
  ".gitignore"
  "config/project.yml"
  "docs/README.md"
  "docs/08-implementation/m1-writer-domain-v0.2.0.md"
  "docs/08-implementation/m1-desktop-shell-v0.2.0.md"
  "docs/08-implementation/m1-rich-text-ime-v0.2.0.md"
  "docs/08-implementation/m1-rich-text-ime-validation.md"
  "docs/08-implementation/m1-typography-ribbon-v0.2.0.md"
  "docs/08-implementation/m1-typography-ribbon-validation.md"
  "docs/08-implementation/m1-local-open-save-v0.2.0.md"
  "docs/08-implementation/m1-local-open-save-validation.md"
  "docs/07-quality/local-open-save-test-matrix.md"
  "docs/06-adr/0016-tko-v1-local-safe-save.md"
  "docs/08-implementation/m1-autosave-recovery-v0.2.0.md"
  "docs/08-implementation/m1-autosave-recovery-validation.md"
  "docs/07-quality/autosave-recovery-test-matrix.md"
  "docs/06-adr/0017-autosave-recovery-snapshot.md"
  "docs/07-quality/writer-domain-test-matrix.md"
  "docs/07-quality/desktop-shell-test-matrix.md"
  "docs/07-quality/rich-text-ime-test-matrix.md"
  "docs/07-quality/typography-ribbon-test-matrix.md"
  "docs/06-adr/0010-writer-logical-offset.md"
  "docs/06-adr/0011-writer-undo-redo-baseline.md"
  "docs/06-adr/0012-solidjs-desktop-ui.md"
  "docs/06-adr/0013-desktop-ipc-thin-bridge.md"
  "docs/06-adr/0014-ime-rich-text-contenteditable.md"
  "docs/06-adr/0015-caret-typing-style-ribbon-typography.md"
  "crates/turkuaz-office-core/Cargo.toml"
  "crates/turkuaz-office-writer/Cargo.toml"
  "crates/turkuaz-office-writer/src/lib.rs"
  "crates/turkuaz-office-writer/src/services/writer_command.rs"
  "crates/turkuaz-office-writer/src/services/writer_asset_service.rs"
  "crates/turkuaz-office-writer/src/services/writer_command_service.rs"
  "crates/turkuaz-office-writer/src/services/writer_editor_service.rs"
  "crates/turkuaz-office-writer/src/services/tko_package_service.rs"
  "crates/turkuaz-office-writer/src/services/tko_package_types.rs"
  "crates/turkuaz-office-writer/src/tools/tko_archive_tool.rs"
  "crates/turkuaz-office-writer/src/tools/tko_yaml_tool.rs"
  "crates/turkuaz-office-writer/tests/tko_package_tests.rs"
  "crates/turkuaz-office-writer/src/services/writer_types.rs"
  "crates/turkuaz-office-writer/src/views/writer_view.rs"
  "apps/desktop/package.json"
  "apps/desktop/tsconfig.json"
  "apps/desktop/vite.config.ts"
  "apps/desktop/src/main.tsx"
  "apps/desktop/src/config/dom-contract.ts"
  "apps/desktop/src/config/typography.ts"
  "apps/desktop/src/config/ribbon.ts"
  "apps/desktop/src/controllers/writer.controller.ts"
  "apps/desktop/src/services/writer-session.service.ts"
  "apps/desktop/src/repositories/writer-session.repository.ts"
  "apps/desktop/src/tools/tauri-writer.tool.ts"
  "apps/desktop/src/tools/text-offset.tool.ts"
  "apps/desktop/src/tools/dom-selection.tool.ts"
  "apps/desktop/src/tools/native-file-dialog.tool.ts"
  "apps/desktop/src/config/file-format.ts"
  "apps/desktop/src/views/writer-ribbon.tsx"
  "apps/desktop/src/views/writer-shell.tsx"
  "apps/desktop/src-tauri/Cargo.toml"
  "apps/desktop/src-tauri/tauri.conf.json5"
  "apps/desktop/src-tauri/capabilities/main-capability.toml"
  "apps/desktop/src-tauri/src/controllers/writer_desktop_controller.rs"
  "apps/desktop/src-tauri/src/services/writer_desktop_service.rs"
  "apps/desktop/src-tauri/src/services/writer_storage_service.rs"
  "apps/desktop/src/views/writer-recovery-panel.tsx"
  "apps/desktop/src-tauri/src/services/writer_recovery_service.rs"
  "apps/desktop/src-tauri/src/tools/recovery_path_tool.rs"
  "apps/desktop/src-tauri/src/tools/recovery_metadata_tool.rs"
  "apps/desktop/src-tauri/src/views/recovery_dto.rs"
  "apps/desktop/src-tauri/src/tools/local_file_tool.rs"
  "apps/desktop/src-tauri/tests/writer_desktop_service_tests.rs"
  "docs/06-adr/0018-external-change-cooperative-lock.md"
  "docs/07-quality/external-change-test-matrix.md"
  "docs/08-implementation/m1-external-change-v0.2.0.md"
  "docs/08-implementation/m1-external-change-validation.md"
  "apps/desktop/src-tauri/src/services/writer_file_session_service.rs"
  "apps/desktop/src-tauri/src/tools/file_fingerprint_tool.rs"
  "apps/desktop/src-tauri/src/tools/file_lock_tool.rs"
  "apps/desktop/src-tauri/src/views/file_session_dto.rs"
  "apps/desktop/src/views/writer-file-protection-banner.tsx"
  "docs/06-adr/0019-twip-page-layout-font-fallback.md"
  "docs/07-quality/font-layout-test-matrix.md"
  "docs/08-implementation/m1-font-layout-v0.2.0.md"
  "docs/08-implementation/m1-font-layout-validation.md"
  "apps/desktop/src/config/layout.ts"
  "apps/desktop/src/services/writer-layout.service.ts"
  "apps/desktop/src/tools/font-capability.tool.ts"
  "apps/desktop/src/config/clipboard.ts"
  "apps/desktop/src/models/clipboard-model.ts"
  "apps/desktop/src/services/clipboard.service.ts"
  "apps/desktop/src/tools/clipboard.tool.ts"
  "apps/desktop/src/tools/clipboard-dom.tool.ts"
  "apps/desktop/src/tools/image-asset.tool.ts"
  "apps/desktop/src/views/writer-image.tsx"
  "apps/desktop/src/services/clipboard.service.test.ts"
  "docs/07-quality/clipboard-styled-fragment-test-matrix.md"
  "docs/08-implementation/m1-clipboard-styled-fragment-v0.2.0.md"
  "docs/08-implementation/m1-clipboard-styled-fragment-validation.md"
)

for relative_path in "${required_files[@]}"; do
  if [[ ! -f "$ROOT/$relative_path" ]]; then
    echo "Eksik zorunlu dosya: $relative_path" >&2
    exit 1
  fi
done

while IFS= read -r -d '' file; do
  head_content="$(head -n 14 "$file")"
  if ! grep -q "Dosya Yolu" <<<"$head_content"; then
    echo "Header eksik: $file" >&2
    exit 1
  fi
  if ! grep -q "Version: $VERSION" <<<"$head_content"; then
    echo "Version header uyumsuz: $file" >&2
    exit 1
  fi
done < <(
  find "$ROOT" \
    -path "$ROOT/node_modules" -prune -o \
    -path "$ROOT/apps/desktop/node_modules" -prune -o \
    -path "$ROOT/apps/desktop/dist" -prune -o \
    -path "$ROOT/target" -prune -o \
    -type f \( \
      -name '*.md' -o -name '*.rs' -o -name '*.toml' -o -name '*.yml' -o -name '*.yaml' \
      -o -name '*.ps1' -o -name '*.sh' -o -name '*.ts' -o -name '*.tsx' -o -name '*.css' \
      -o -name '*.html' -o -name '*.json' -o -name '*.json5' \
    \) -print0
)

grep -q "Version: $VERSION" "$ROOT/.gitignore"
grep -q '"apps/desktop/src-tauri"' "$ROOT/Cargo.toml"
grep -q 'framework: solidjs' "$ROOT/config/project.yml"
grep -q 'canonical_state_in_frontend: false' "$ROOT/config/project.yml"
grep -q 'surface: contenteditable' "$ROOT/config/project.yml"
grep -q 'materialize_on_text_insert: true' "$ROOT/config/project.yml"
grep -q 'active_tab: home' "$ROOT/config/project.yml"
grep -q 'WRITER_PARAGRAPH_MARKER_VALUE' "$ROOT/apps/desktop/src/config/dom-contract.ts"
grep -q 'WRITER_PARAGRAPH_SELECTOR' "$ROOT/apps/desktop/src/config/dom-contract.ts"
grep -q 'WRITER_RIBBON_TABS' "$ROOT/apps/desktop/src/config/ribbon.ts"
grep -q 'focusAndRestoreParagraphSelection' "$ROOT/apps/desktop/src/tools/dom-selection.tool.ts"
grep -q 'WRITER_FONT_FAMILIES' "$ROOT/apps/desktop/src/config/typography.ts"
grep -q 'writerApplyCharacterStyle' "$ROOT/apps/desktop/src/config/ipc-commands.ts"
grep -q 'writerApplyParagraphAlignment' "$ROOT/apps/desktop/src/config/ipc-commands.ts"
grep -q 'InsertStyledText' "$ROOT/crates/turkuaz-office-writer/src/services/writer_command.rs"
grep -q 'ApplyParagraphStyle' "$ROOT/crates/turkuaz-office-writer/src/services/writer_command.rs"
grep -q 'typingStyle' "$ROOT/apps/desktop/src/repositories/writer-session.repository.ts"
grep -q 'WriterRibbon' "$ROOT/apps/desktop/src/views/writer-shell.tsx"
grep -q 'pub fn execute_batch' "$ROOT/crates/turkuaz-office-writer/src/services/writer_editor_service.rs"
grep -q 'TextOffsetTool' "$ROOT/apps/desktop/src/services/writer-session.service.ts"
grep -q 'IPC_COMMANDS' "$ROOT/apps/desktop/src/tools/tauri-writer.tool.ts"
grep -q 'capabilities: \["main-capability"\]' "$ROOT/apps/desktop/src-tauri/tauri.conf.json5"

grep -q 'package_v1:' "$ROOT/config/project.yml"
grep -q 'direct_target_truncate_allowed: false' "$ROOT/config/project.yml"
grep -q 'TKO_MANIFEST_ENTRY' "$ROOT/crates/turkuaz-office-writer/src/services/tko_package_service.rs"
grep -q 'CompressionMethod::Stored' "$ROOT/crates/turkuaz-office-writer/src/tools/tko_archive_tool.rs"
grep -q 'DirectoryEntryUnsupported' "$ROOT/crates/turkuaz-office-writer/src/tools/tko_archive_tool.rs"
grep -q 'write_safe_replace' "$ROOT/apps/desktop/src-tauri/src/tools/local_file_tool.rs"
grep -q 'writerOpenDocument' "$ROOT/apps/desktop/src/config/ipc-commands.ts"
grep -q 'writerSaveDocument' "$ROOT/apps/desktop/src/config/ipc-commands.ts"
grep -q 'confirmDiscardIfNeeded' "$ROOT/apps/desktop/src/services/writer-session.service.ts"
grep -q 'dialog:allow-message' "$ROOT/apps/desktop/src-tauri/capabilities/main-capability.toml"

grep -q 'desktop_recovery:' "$ROOT/config/project.yml"
grep -q 'overwrite_source_on_autosave: false' "$ROOT/config/project.yml"
grep -q 'writerCreateRecoverySnapshot' "$ROOT/apps/desktop/src/config/ipc-commands.ts"
grep -q 'writerRestoreRecoverySnapshot' "$ROOT/apps/desktop/src/config/ipc-commands.ts"
grep -q 'autosaveRecovery' "$ROOT/apps/desktop/src/services/writer-session.service.ts"
grep -q 'WriterRecoveryPanel' "$ROOT/apps/desktop/src/views/writer-shell.tsx"
grep -q 'TkoPackageService::serialize' "$ROOT/apps/desktop/src-tauri/src/services/writer_recovery_service.rs"
grep -q 'RECOVERY_MAX_SNAPSHOTS_PER_DOCUMENT' "$ROOT/apps/desktop/src-tauri/src/config/constants.rs"


grep -q 'desktop_file_protection:' "$ROOT/config/project.yml"
grep -q 'timestamp_only_detection: false' "$ROOT/config/project.yml"
grep -q 'backend_mutation_guard: true' "$ROOT/config/project.yml"
grep -q 'writerGetFileSession' "$ROOT/apps/desktop/src/config/ipc-commands.ts"
grep -q 'writerAcknowledgeExternalChange' "$ROOT/apps/desktop/src/config/ipc-commands.ts"
grep -q 'writerReloadFromDisk' "$ROOT/apps/desktop/src/config/ipc-commands.ts"
grep -q 'ensure_writable' "$ROOT/apps/desktop/src-tauri/src/services/writer_desktop_service.rs"
grep -q 'ExternalChangeConflict' "$ROOT/apps/desktop/src-tauri/src/services/writer_file_session_service.rs"
grep -q 'create_new(true)' "$ROOT/apps/desktop/src-tauri/src/tools/file_lock_tool.rs"
grep -q 'content_hash' "$ROOT/apps/desktop/src-tauri/src/tools/file_fingerprint_tool.rs"
grep -q 'WriterFileProtectionBanner' "$ROOT/apps/desktop/src/views/writer-shell.tsx"
grep -q 'WRITER_EXTERNAL_CHANGE_POLL_MS' "$ROOT/apps/desktop/src/config/runtime-config.ts"



grep -q 'writer_layout:' "$ROOT/config/project.yml"
grep -q 'canonical_unit: twip' "$ROOT/config/project.yml"
grep -q 'browser_dom_measurement_is_canonical: false' "$ROOT/config/project.yml"
grep -q 'WriterPageSettingsView' "$ROOT/crates/turkuaz-office-writer/src/views/writer_view.rs"
grep -q 'page_settings: WriterPageSettingsDto' "$ROOT/apps/desktop/src-tauri/src/views/writer_dto.rs"
grep -q 'TWIPS_PER_INCH' "$ROOT/apps/desktop/src/config/layout.ts"
grep -q 'WRITER_FONT_FALLBACK_PROFILES' "$ROOT/apps/desktop/src/config/layout.ts"
grep -q 'class WriterLayoutService' "$ROOT/apps/desktop/src/services/writer-layout.service.ts"
grep -q 'class FontCapabilityTool' "$ROOT/apps/desktop/src/tools/font-capability.tool.ts"
grep -q 'zoomPercent' "$ROOT/apps/desktop/src/repositories/writer-session.repository.ts"
grep -q 'controller.zoomIn' "$ROOT/apps/desktop/src/views/writer-statusbar.tsx"
grep -q 'document.pageSettings' "$ROOT/apps/desktop/src/services/writer-session.service.ts"
grep -q 'writer_view_exposes_primary_page_settings_without_pixel_conversion' "$ROOT/crates/turkuaz-office-writer/tests/writer_domain_tests.rs"


grep -q 'ReplaceRangeWithStyledRuns' "$ROOT/crates/turkuaz-office-writer/src/services/writer_command.rs"
grep -q 'InsertImageData' "$ROOT/crates/turkuaz-office-writer/src/services/writer_command.rs"
grep -q 'struct WriterAsset' "$ROOT/crates/turkuaz-office-writer/src/services/writer_types.rs"
grep -q 'TKO_ASSET_INDEX_ENTRY' "$ROOT/crates/turkuaz-office-writer/src/config/constants.rs"
grep -q 'validate_image_payload' "$ROOT/crates/turkuaz-office-writer/src/services/writer_asset_service.rs"
grep -q 'decode_assets' "$ROOT/crates/turkuaz-office-writer/src/services/tko_package_service.rs"
grep -q 'CrossParagraphFragmentReplaceNotSupported' "$ROOT/crates/turkuaz-office-writer/src/services/writer_command.rs"
grep -q 'StyledTextRun' "$ROOT/crates/turkuaz-office-writer/src/services/writer_types.rs"
grep -q 'writerReplaceRangeWithStyledRuns' "$ROOT/apps/desktop/src/config/ipc-commands.ts"
grep -q 'replaceRangeWithStyledRuns' "$ROOT/apps/desktop/src/tools/tauri-writer.tool.ts"
grep -q 'replaceSelectionWithStyledRuns' "$ROOT/apps/desktop/src/services/writer-session.service.ts"
grep -q 'replaceSelectionWithStyledRuns' "$ROOT/apps/desktop/src/controllers/writer.controller.ts"
grep -q 'copySelection' "$ROOT/apps/desktop/src/controllers/writer.controller.ts"
grep -q 'pasteSelection' "$ROOT/apps/desktop/src/controllers/writer.controller.ts"
grep -q 'WRITER_CLIPBOARD_MIME' "$ROOT/apps/desktop/src/config/clipboard.ts"
grep -q 'class ClipboardService' "$ROOT/apps/desktop/src/services/clipboard.service.ts"
grep -q 'parseInternalFragment' "$ROOT/apps/desktop/src/services/clipboard.service.ts"
grep -q 'parseHtmlFragment' "$ROOT/apps/desktop/src/services/clipboard.service.ts"
grep -q 'class ClipboardTool' "$ROOT/apps/desktop/src/tools/clipboard.tool.ts"
grep -q 'class ClipboardDomTool' "$ROOT/apps/desktop/src/tools/clipboard-dom.tool.ts"
grep -q 'WRITER_CLIPBOARD_MAX_DOM_NODES' "$ROOT/apps/desktop/src/config/clipboard.ts"
grep -q 'WRITER_CLIPBOARD_MAX_DOM_DEPTH' "$ROOT/apps/desktop/src/config/clipboard.ts"
grep -q 'limitExceeded' "$ROOT/apps/desktop/src/tools/clipboard-dom.tool.ts"
grep -q 'internal MIME has paste priority' "$ROOT/apps/desktop/src/services/clipboard.service.test.ts"
grep -q 'invalid rich payload falls back to plain text' "$ROOT/apps/desktop/src/services/clipboard.service.test.ts"
grep -q 'cut writes clipboard before atomic empty fragment replace' "$ROOT/apps/desktop/src/services/clipboard.service.test.ts"
grep -q 'image-only paste consumes default DOM paste and inserts canonical asset' "$ROOT/apps/desktop/src/services/clipboard.service.test.ts"
grep -q 'image_asset_insert_and_fetch_are_exposed_by_desktop_service' "$ROOT/apps/desktop/src-tauri/tests/writer_desktop_service_tests.rs"
grep -q 'onPaste' "$ROOT/apps/desktop/src/views/writer-paragraph.tsx"
grep -q 'onCut' "$ROOT/apps/desktop/src/views/writer-paragraph.tsx"
grep -q 'onCopy' "$ROOT/apps/desktop/src/views/writer-paragraph.tsx"
grep -q 'writer_replace_range_with_styled_runs' "$ROOT/apps/desktop/src-tauri/src/controllers/writer_desktop_controller.rs"
grep -q 'writer_get_asset' "$ROOT/apps/desktop/src-tauri/src/controllers/writer_desktop_controller.rs"
grep -q 'writer_insert_image_data' "$ROOT/apps/desktop/src-tauri/src/controllers/writer_desktop_controller.rs"
grep -q 'WriterImageView' "$ROOT/crates/turkuaz-office-writer/src/views/writer_view.rs"
grep -q 'writerGetAsset' "$ROOT/apps/desktop/src/config/ipc-commands.ts"
grep -q 'class ImageAssetTool' "$ROOT/apps/desktop/src/tools/image-asset.tool.ts"
grep -q 'WriterImageBlock' "$ROOT/apps/desktop/src/views/writer-image.tsx"
grep -q 'replace_range_with_styled_runs_preserves_neighbor_styles_and_undoes_once' "$ROOT/crates/turkuaz-office-writer/tests/writer_domain_tests.rs"
grep -q 'replace_range_with_empty_fragment_keeps_editable_empty_run' "$ROOT/crates/turkuaz-office-writer/tests/writer_domain_tests.rs"

echo "Turkuaz Office Writer v0.2.0 Font Layout + Clipboard Runtime verification BASARILI."
