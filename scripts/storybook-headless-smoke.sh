#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PAGES="${STORYBOOK_SMOKE_PAGES:-overview theme-tokens text icon spinner svg-button text-button icon-text-button toggle segmented-toggle select-box combo-box color-swatch color-picker-rgba text-input search-box tooltip badge key-cap card accordion menu-button side-menu command-palette split-pane modal popover align-center loading-dots toolbar progress-bar status-bar selection-list notification-toast slide-control breadcrumb dynamic-array-editor tabs tree-view code-diff}"
LOG_PREFIX="katana-storybook-headless:page"

cd "$ROOT_DIR/storybook"
RUSTFLAGS="-D warnings" cargo build

binary="$ROOT_DIR/storybook/target/debug/storybook"
tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/katana-ui-core-storybook-headless-smoke.XXXXXX")"

pids=()
logs=()
labels=()

for page in $PAGES; do
  log="$tmp_dir/${page}.log"
  echo "storybook headless smoke: $page"
  (
    KATANA_UI_WIDGET_STORYBOOK_PAGE="$page" "$binary" --headless-page >"$log" 2>&1
    marker="$LOG_PREFIX page=$page"
    grep -F "$marker" "$log" >/dev/null
  ) &
  pids+=("$!")
  logs+=("$log")
  labels+=("$page")
done

for index in "${!pids[@]}"; do
  if ! wait "${pids[$index]}"; then
    echo "storybook headless smoke failed: ${labels[$index]}"
    cat "${logs[$index]}"
    exit 1
  fi
done
