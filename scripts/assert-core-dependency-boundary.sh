#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
read -r -a CARGO_CMD <<<"${CARGO:-cargo}"

cd "$ROOT_DIR"
tree="$("${CARGO_CMD[@]}" tree -p katana-ui-core --locked --prefix none)"
failures=()

while IFS= read -r line; do
  case "$line" in
    katana-*)
      case "$line" in
        katana-ui-core\ v*) ;;
        *) failures+=("Katana domain dependency leaked into core: $line") ;;
      esac
      ;;
  esac
done <<<"$tree"

if [ "${#failures[@]}" -gt 0 ]; then
  printf '%s\n' "core dependency boundary failed"
  printf -- '- %s\n' "${failures[@]}"
  exit 1
fi
