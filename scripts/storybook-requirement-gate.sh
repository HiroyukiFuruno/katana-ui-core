#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT_DIR"
RUSTFLAGS="-D warnings" cargo build --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked

binary="$ROOT_DIR/target/release/katana-ui-core-storybook"
output="$("$binary" --headless-scenario)"
case "$output" in
  *"stories="*"validated="*"state_conflicts=0"*"structure_failures=0"*"missing_required_pages=0"*"page_contract_failures=0"*"nodes="*"panel_nodes=4"*"panel_theme_configured=true"*"panel_theme_variants=2"*"themed_story_roots=53"*"styled_story_roots=53"*"details_panel_configured=true"*"detail_sections=6"*"story_selection=button"*"theme_switch=light->dark"*"theme_control=true"*"operation_sequence=1"*"selector_operations=7"*"overlay_dismissals=3"*"color_picker_updates=3"*"callback_log=1"*"required_ui=53"*"dedicated_ui=53"*"required_ui_fallbacks=0"*"initial_visible_fallbacks=0"*"modal_required=true"*"non_empty_pixels="*"theme_difference_pixels="*"operation_difference_pixels="*) ;;
  *)
    echo "storybook requirement gate failed"
    echo "$output"
    exit 1
    ;;
esac

panel_report="$ROOT_DIR/target/storybook-panel-interaction-report.json"
coverage_report="$ROOT_DIR/target/storybook-visual-coverage.json"
if [[ ! -s "$panel_report" || ! -s "$coverage_report" ]]; then
  echo "storybook headless report failed"
  exit 1
fi
if ! grep -q '"required_ui_fallbacks": 0' "$coverage_report" \
  || ! grep -q '"initial_visible_fallbacks": 0' "$coverage_report"; then
  echo "storybook visual coverage failed"
  cat "$coverage_report"
  exit 1
fi
if ! grep -q '"theme_control": true' "$panel_report"; then
  echo "storybook theme control marker failed"
  cat "$panel_report"
  exit 1
fi
if ! grep -q '"required_ui": 53' "$coverage_report" \
  || ! grep -q '"dedicated_ui": 53' "$coverage_report" \
  || ! grep -q '"modal_required": true' "$coverage_report"; then
  echo "storybook modal coverage failed"
  cat "$coverage_report"
  exit 1
fi
if ! grep -q '"selector_operations":' "$panel_report" \
  || ! grep -q '"overlay_dismissals":' "$panel_report" \
  || ! grep -q '"color_picker_updates":' "$panel_report"; then
  echo "storybook panel interaction markers failed"
  cat "$panel_report"
  exit 1
fi
if ! grep -Eq '"non_empty_pixels": [1-9][0-9]*' "$coverage_report" \
  || ! grep -Eq '"theme_difference_pixels": [1-9][0-9]*' "$coverage_report" \
  || ! grep -Eq '"operation_difference_pixels": [1-9][0-9]*' "$coverage_report"; then
  echo "storybook visual pixel markers failed"
  cat "$coverage_report"
  exit 1
fi

snapshot="$ROOT_DIR/target/storybook-panel.png"
modal_snapshot="$ROOT_DIR/target/storybook-panel-modal-window.png"
printf 'stale screenshot sentinel' > "$snapshot"
snapshot_output="$("$binary" --visual-snapshot "$snapshot")"
case "$snapshot_output" in
  *"katana-ui-core-storybook-snapshot:"*"bytes="*"modified_unix="*) ;;
  *)
    echo "storybook visual snapshot evidence failed"
    echo "$snapshot_output"
    exit 1
    ;;
esac
if [[ ! -s "$snapshot" ]]; then
  echo "storybook visual snapshot failed"
  exit 1
fi
if [[ ! -s "$modal_snapshot" ]]; then
  echo "storybook modal visual snapshot failed"
  exit 1
fi

runtime_output="$("$binary" --runtime-regression)"
case "$runtime_output" in
  *"state_reflected=true"*"overlay_rendered=true"*"modal_plan_same_display=true"*"modal_plan_frontmost=true"*) ;;
  *)
    echo "storybook runtime regression failed"
    echo "$runtime_output"
    exit 1
    ;;
esac

modal_output="$("$binary" --open-modal-window 2)"
case "$modal_output" in
  *"modal_window_opened=true"*"same_display=true"*"frontmost=true"*"state_reflected=true"*"overlay_rendered=true"*) ;;
  *)
    echo "storybook modal window regression failed"
    echo "$modal_output"
    exit 1
    ;;
esac
