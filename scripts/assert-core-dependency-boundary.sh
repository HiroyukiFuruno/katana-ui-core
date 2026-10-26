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
storybook_katana_deps="$(
  rg -n '^\s*katana-[A-Za-z0-9_-]+\s*=' "$storybook_manifest" \
    | rg -v '^.*katana-ui-core(-storybook)?\s*=' || true
)"
if [ -n "$storybook_katana_deps" ]; then
  printf '%s\n' "core dependency boundary failed"
  printf -- '- %s\n' "Storybook must not depend on an external katana-* crate; keep sibling integration in the consumer repo"
  printf '%s\n' "$storybook_katana_deps"
  exit 1
fi
