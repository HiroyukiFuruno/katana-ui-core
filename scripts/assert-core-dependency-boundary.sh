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

raster_tree="$("${CARGO_CMD[@]}" tree -p katana-ui-core --no-default-features --features raster-host --locked --prefix none)"
while IFS= read -r line; do
  case "$line" in
    eframe\ v*|egui\ v*|winit\ v*)
      failures+=("raster-host must not link a GUI runtime: $line")
      ;;
  esac
done <<<"$raster_tree"

if [ "${#failures[@]}" -gt 0 ]; then
  printf '%s\n' "core dependency boundary failed"
  printf -- '- %s\n' "${failures[@]}"
  exit 1
fi

storybook_manifest="$ROOT_DIR/crates/katana-ui-core-storybook/Cargo.toml"
if grep -q "katana-document-viewer" "$storybook_manifest"; then
  printf '%s\n' "core dependency boundary failed"
  printf -- '- %s\n' "Storybook release gate must not depend on katana-document-viewer; keep KDV integration in the consumer repo"
  exit 1
fi
