#!/usr/bin/env bash
# 📄 Dosya Yolu: E:/Projects/TurkuazOffice/tools/audit-unused-typescript.sh
# 📌 Amac: Desktop ve Web icin silme yapmadan advisory TypeScript unused sembol raporu uretir
# 📌 Modul - FileType: Tool - Bash
# Version: 0.1.0
# Aciklama: Mevcut TypeScript derleyicisiyle isimsiz kod adaylarini raporlar; mevcut CI build gate'ini degistirmez
# Bagimli Oldugu Katman: Tool -> Config

set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: audit-unused-typescript.sh <module-directory>" >&2
  exit 2
fi

module_directory="$1"

if [[ ! -d "$module_directory" ]]; then
  echo "audit_error: module directory missing: $module_directory" >&2
  exit 2
fi

cd "$module_directory"

if [[ ! -x "./node_modules/.bin/tsc" ]]; then
  echo "audit_error: local TypeScript compiler missing; install module dependencies" >&2
  exit 2
fi

report_file="$(mktemp)"
trap 'rm -f "$report_file"' EXIT

set +e
./node_modules/.bin/tsc --noEmit --noUnusedLocals --noUnusedParameters --pretty false > "$report_file" 2>&1
compiler_status=$?
set -e

unused_count="$(grep -Ec 'error TS(6133|6192|6196):' "$report_file" || true)"

printf 'typescript_unused_audit module=%s unused_diagnostics=%s compiler_exit=%s mode=advisory\n' \
  "$PWD" "$unused_count" "$compiler_status"

if [[ -s "$report_file" ]]; then
  cat "$report_file"
else
  echo "No TypeScript unused diagnostics."
fi

if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
  {
    printf '### TypeScript unused advisory: %s\n\n' "$PWD"
    printf 'Diagnostic count (TS6133/TS6192/TS6196): %s  \n' "$unused_count"
    printf 'Compiler exit: %s (non-blocking; ordinary build and tests remain blocking)\n\n' "$compiler_status"
    printf 'The findings are review candidates, not permission to delete runtime exports.\n\n'
  } >> "$GITHUB_STEP_SUMMARY"
fi

# Audit diagnostics are intentionally advisory. Missing tools and bad arguments
# remain hard failures above; normal build and test checks stay authoritative.
exit 0
