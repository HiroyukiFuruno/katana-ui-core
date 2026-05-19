#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"
RUSTFLAGS="-D warnings" cargo build --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked
binary="$ROOT_DIR/target/release/katana-ui-core-storybook"
legacy_preview_signatures=25
output="$("$binary" --headless-scenario)"
case "$output" in
  *"stories="*"validated=73"*"state_conflicts=0"*"structure_failures=0"*"missing_required_pages=0"*"page_contract_failures=0"*"nodes="*"panel_nodes=4"*"panel_theme_configured=true"*"panel_theme_variants=2"*"themed_story_roots=1"*"styled_story_roots=1"*"details_panel_configured=true"*"detail_sections=6"*"panel_scroll_configured=true"*"independent_panel_scrolls=4"*"story_selection=button"*"theme_switch=light->dark"*"theme_control=true"*"operation_sequence=1"*"selector_operations=4"*"overlay_dismissals=5"*"color_picker_updates=1"*"settings_mutations=74"*"legacy_ui_markers=27"*"legacy_settings_mutations=27"*"legacy_preset_differences=27"*"tree_view_option_mutations=12"*"callback_log=1"*"required_ui=73"*"dedicated_ui=73"*"required_ui_fallbacks=0"*"initial_visible_fallbacks=0"*"modal_required=true"*"non_empty_pixels="*"theme_difference_pixels="*"operation_difference_pixels="*"selected_preview_visible=true"*"selected_preview_interaction_visible=true"*"detail_tables_hidden=true"*"scrollbar_thumb_bottom=true"*"contract_rows_fit=true"*"inspector_rows_fit=true"*"tree_view_selected=true"*"tree_view_settings_visible=true"*"tree_view_line_option_visible=true"*"tree_view_icon_option_visible=true"*"tree_view_trigger_option_visible=true"*"tree_view_action_logged=true"*"panel_scrollbars_visible=true"*"navigation_collapsed_pixels_changed="*"legacy_preview_signatures=${legacy_preview_signatures}"*"legacy_preview_signature_collisions=0"*) ;;
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
if ! grep -q '"required_ui": 73' "$coverage_report" \
  || ! grep -q '"dedicated_ui": 73' "$coverage_report" \
  || ! grep -q '"modal_required": true' "$coverage_report"; then
  echo "storybook modal coverage failed"
  cat "$coverage_report"
  exit 1
fi
if ! grep -q '"selector_operations":' "$panel_report" \
  || ! grep -q '"overlay_dismissals":' "$panel_report" \
  || ! grep -q '"color_picker_updates":' "$panel_report" \
  || ! grep -q '"settings_mutations":' "$panel_report" \
  || ! grep -q '"legacy_ui_markers":' "$panel_report" \
  || ! grep -q '"preset_differences":' "$panel_report" \
  || ! grep -q '"tree_view_option_mutations":' "$panel_report"; then
  echo "storybook panel interaction markers failed"
  cat "$panel_report"
  exit 1
fi
if ! grep -q '"action": "tree_click_toggle"' "$panel_report" \
  || ! grep -q '"action": "tree_toggle_trigger"' "$panel_report"; then
  echo "storybook settings and tree option mutation markers failed"
  cat "$panel_report"
  exit 1
fi
python3 - "$panel_report" <<'PY'
import json, sys
expected_markers = set(
    "legacy-01-theme-panel-theme legacy-02-text legacy-03-icon "
    "legacy-04-loading-dots legacy-04-spinner legacy-04-progress-bar legacy-05-svg-button "
    "legacy-06-text-button legacy-07-icon-text-button legacy-08-toggle "
    "legacy-09-segmented-toggle legacy-10-select-box legacy-11-color-swatch "
    "legacy-12-text-input legacy-13-search-box legacy-14-tooltip legacy-15-badge "
    "legacy-16-key-cap legacy-17-card legacy-18-accordion legacy-19-split-pane "
    "legacy-20-modal legacy-20-modal-overlay legacy-21-popover legacy-22-color-picker "
    "legacy-23-color-picker-parity legacy-24-code-diff".split()
)
report = json.load(open(sys.argv[1], encoding="utf-8"))
markers = {entry["ui_marker"]: entry for entry in report["legacy_ui_markers"]}
settings = {it["ui_marker"]: it for it in report["settings_mutations"] if it["ui_marker"] in expected_markers}
presets = {entry["ui_marker"]: entry for entry in report["preset_differences"]}
for label, actual in (("ui marker", markers), ("settings", settings), ("preset", presets)):
    missing = expected_markers - set(actual)
    if missing:
        raise SystemExit(f"legacy {label} pages mismatch: {sorted(missing)}")
for page, entry in settings.items():
    option, state, preview = entry["option"], entry["state"], entry["preview"]
    valid = [
        entry["ui_marker"].startswith("legacy-"),
        option["name"] and option["value_type"],
        option["before_value"] != option["after_value"],
        state["before"] != state["after"],
        preview["before"] != preview["after"],
    ]
    if not all(valid):
        raise SystemExit(f"{page} settings mutation is not typed before/after evidence")
for page, entry in presets.items():
    values = [entry[f"{name}_marker"] for name in ("default", "interactive", "edge", "theme")]
    if len(set(values)) != len(values) or not all(entry["ui_marker"] in it for it in values):
        raise SystemExit(f"{page} preset marker is not ui-specific")
PY
if ! grep -Eq '"non_empty_pixels": [1-9][0-9]*' "$coverage_report" \
  || ! grep -Eq '"theme_difference_pixels": [1-9][0-9]*' "$coverage_report" \
  || ! grep -Eq '"operation_difference_pixels": [1-9][0-9]*' "$coverage_report"; then
  echo "storybook visual pixel markers failed"
  cat "$coverage_report"
  exit 1
fi
if ! grep -q "\"legacy_preview_signatures\": ${legacy_preview_signatures}" "$coverage_report" \
  || ! grep -q '"legacy_preview_signature_collisions": 0' "$coverage_report"; then
  echo "storybook legacy preview signature markers failed"
  cat "$coverage_report"
  exit 1
fi
if ! grep -q '"selected_preview_visible": true' "$coverage_report" \
  || ! grep -q '"selected_preview_interaction_visible": true' "$coverage_report" \
  || ! grep -q '"detail_tables_hidden": true' "$coverage_report" \
  || ! grep -q '"scrollbar_thumb_bottom": true' "$coverage_report" \
  || ! grep -q '"contract_rows_fit": true' "$coverage_report" \
  || ! grep -q '"inspector_rows_fit": true' "$coverage_report"; then
  echo "storybook selected component detail and table fit markers failed"
  cat "$coverage_report"
  exit 1
fi
if ! grep -q '"tree_view_selected": true' "$coverage_report" \
  || ! grep -q '"tree_view_settings_visible": true' "$coverage_report" \
  || ! grep -q '"tree_view_line_option_visible": true' "$coverage_report" \
  || ! grep -q '"tree_view_icon_option_visible": true' "$coverage_report" \
  || ! grep -q '"tree_view_trigger_option_visible": true' "$coverage_report" \
  || ! grep -q '"tree_view_action_logged": true' "$coverage_report" \
  || ! grep -q '"panel_scrollbars_visible": true' "$coverage_report" \
  || ! grep -Eq '"navigation_collapsed_pixels_changed": [1-9][0-9]*' "$coverage_report"; then
  echo "storybook tree view coverage markers failed"
  cat "$coverage_report"
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
