#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_SRC="$ROOT_DIR/crates/katana-ui-core/src"

violations="$(
  rg -n "\\b(floem|gpui|egui)::|\\b(View|ViewId|Element)\\b|egui::Ui" \
    "$CORE_SRC/runtime" \
    "$CORE_SRC/window" \
    "$CORE_SRC/surface" \
    "$CORE_SRC/render_model" \
    "$CORE_SRC/adapter_contract" \
    --glob '*.rs'
)" || true

if [ -n "$violations" ]; then
  echo "core public API exposes framework-native symbols"
  echo "$violations"
  exit 1
fi
