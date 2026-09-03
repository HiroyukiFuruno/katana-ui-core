#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_SRC="$ROOT_DIR/crates/katana-ui-core/src"
CORE_API_PATHS=(
  "$CORE_SRC/facade.rs"
  "$CORE_SRC/runtime"
  "$CORE_SRC/window"
  "$CORE_SRC/surface"
  "$CORE_SRC/panel"
  "$CORE_SRC/style"
  "$CORE_SRC/theme"
  "$CORE_SRC/render_model"
  "$CORE_SRC/widget"
  "$CORE_SRC/adapter_contract"
  "$CORE_SRC/raster_host"
)

violations="$(
  rg -n "\\b(View|ViewId|Element)\\b" \
    "${CORE_API_PATHS[@]}" \
    --glob '*.rs'
)" || true

if [ -n "$violations" ]; then
  echo "core public API exposes framework-native symbols"
  echo "$violations"
  exit 1
fi

platform_violations="$(
  rg -n "(/System/Library/Fonts|/Library/Fonts|/usr/share/fonts|C:\\\\Windows\\\\Fonts|\\b(AppKit|UIKit|CoreText|DirectWrite|fontconfig|eframe|egui|winit|tao|x11rb|wayland_client|cocoa)::)" \
    "${CORE_API_PATHS[@]}" \
    --glob '*.rs'
)" || true

if [ -n "$platform_violations" ]; then
  echo "core public API exposes OS-specific font paths or backend symbols"
  echo "$platform_violations"
  exit 1
fi

app_specific_violations="$(
  rg -n "\"viewer\\.|\\bVIEWER_" \
    "${CORE_API_PATHS[@]}" \
    --glob '*.rs'
)" || true

if [ -n "$app_specific_violations" ]; then
  echo "core public API exposes app-specific viewer host action symbols"
  echo "$app_specific_violations"
  exit 1
fi
