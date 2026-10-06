#!/usr/bin/env bash
# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/tools/build-web-wasm.sh
# 📌 Amac: Rust Web aggregate bridge wasm32 binarysini pinned wasm-bindgen ile browser-ready JS/WASM artifactina donusturur
# 📌 Modul - FileType: Tool - Shell
# Version: 0.1.0
# Aciklama: Core capability + Writer TKO codec bridge release build, generated binding ve export kontrat kontrolunu tek komutta toplar
# Bagimli Oldugu Katman: Tool | Config

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WASM_BINDGEN_VERSION="0.2.129"
WASM_TARGET="wasm32-unknown-unknown"
WASM_CRATE="turkuaz-office-web-bridge"
WASM_BINARY="$ROOT/target/$WASM_TARGET/release/turkuaz_office_web_bridge.wasm"
WASM_OUT_DIR="$ROOT/apps/web/public/wasm"
WASM_OUT_NAME="turkuaz_office_web_bridge"

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "wasm-bindgen CLI bulunamadi. Beklenen surum: $WASM_BINDGEN_VERSION" >&2
  exit 1
fi

ACTUAL_VERSION="$(wasm-bindgen --version | awk '{print $2}')"
if [[ "$ACTUAL_VERSION" != "$WASM_BINDGEN_VERSION" ]]; then
  echo "wasm-bindgen CLI surumu uyusmuyor. Beklenen=$WASM_BINDGEN_VERSION Gercek=$ACTUAL_VERSION" >&2
  exit 1
fi

cargo build \
  -p "$WASM_CRATE" \
  --release \
  --target "$WASM_TARGET"

rm -rf "$WASM_OUT_DIR"
mkdir -p "$WASM_OUT_DIR"

wasm-bindgen \
  "$WASM_BINARY" \
  --target web \
  --out-dir "$WASM_OUT_DIR" \
  --out-name "$WASM_OUT_NAME"

JS_FILE="$WASM_OUT_DIR/$WASM_OUT_NAME.js"
WASM_FILE="$WASM_OUT_DIR/${WASM_OUT_NAME}_bg.wasm"

test -s "$JS_FILE"
test -s "$WASM_FILE"

grep -q 'web_core_abi_version' "$JS_FILE"
grep -q 'web_core_document_schema_version' "$JS_FILE"
grep -q 'web_core_bridge_kind' "$JS_FILE"
grep -q 'web_core_native_file_system_access' "$JS_FILE"
grep -q 'web_writer_tko_inspect' "$JS_FILE"
grep -q 'web_writer_tko_reencode' "$JS_FILE"

echo "Turkuaz Office Web aggregate WASM artifact generation BASARILI."
