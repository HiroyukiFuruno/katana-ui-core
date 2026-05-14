#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ALLOWED_FILE="$ROOT_DIR/crates/katana-ui-widget/src/overlay_lifecycle.rs"

violations="$(
  rg -n "\\b(add_overlay|remove_overlay)\\b" \
    "$ROOT_DIR/crates/katana-ui-widget/src" \
    "$ROOT_DIR/storybook/src" \
    --glob '*.rs' \
    | awk -F: -v allowed="$ALLOWED_FILE" '$1 != allowed { print }'
)" || true

if [ -n "$violations" ]; then
  echo "overlay lifecycle violation: use OverlayLifecycle instead of direct add_overlay/remove_overlay"
  echo "$violations"
  exit 1
fi
