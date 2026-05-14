#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
read -r -a CARGO_CMD <<<"${CARGO:-cargo}"

SCENARIOS=(
  "menu-button:placement-four-directions:all-four-directions-visible"
  "menu-button:close-trigger:closed-by-trigger-reclick"
  "menu-button:close-outside:closed-by-outside-click"
  "menu-button:close-esc:closed-by-escape"
  "menu-button:close-selection:closed-by-menu-item"
)

cd "$ROOT_DIR"
RUSTFLAGS="-D warnings" "${CARGO_CMD[@]}" test \
  -p katana-ui-widget \
  --all-features \
  composite::menu_button \
  -- --nocapture

cd "$ROOT_DIR/storybook"
RUSTFLAGS="-D warnings" "${CARGO_CMD[@]}" build

binary="$ROOT_DIR/storybook/target/debug/storybook"
for scenario in "${SCENARIOS[@]}"; do
  page="${scenario%%:*}"
  rest="${scenario#*:}"
  interaction="${rest%%:*}"
  expected_detail="${rest#*:}"
  echo "menu-button contract: $interaction -> $expected_detail"
  KATANA_UI_WIDGET_STORYBOOK_PAGE="$page" \
    KATANA_UI_WIDGET_STORYBOOK_INTERACTION="$interaction" \
    KATANA_UI_WIDGET_STORYBOOK_EXPECTED_DETAIL="$expected_detail" \
    "$binary" --headless-scenario
done
