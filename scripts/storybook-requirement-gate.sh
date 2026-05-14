#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCENARIOS="${STORYBOOK_REQUIREMENT_SCENARIOS:-overview:theme-toggle:dark-true overview:select-page:svg-button-selected overview:select-page-cycle:all-pages-stable toggle:toggle-value:value-true segmented-toggle:select-grid:value-grid spinner:toggle-visible:visible-false color-swatch:select-color:selected-green text-input:input-change:changed-replay search-box:toggle-search-options:options-all-true tabs:select-tab:selected-settings tabs:close-tab:close-count-1 dynamic-array-editor:add-item:added-index-3 tree-view:select-leaf:leaf-tree-view breadcrumb:click-crumb:clicked-font command-palette:query-command:query-main toolbar:toolbar-action:action-search modal:open:native-window-created modal:setting-size-sm:size-sm-window-created modal:setting-size-lg:size-lg-window-created modal:setting-size-custom:size-custom-window-created modal:setting-esc-enabled:esc-enabled-window-created modal:setting-esc-disabled:esc-disabled-window-created modal:setting-parent-block:parent-block-window-created modal:setting-parent-allow:parent-allow-window-created modal:setting-footer-confirm:footer-confirm-window-created modal:setting-footer-form:footer-form-window-created modal:setting-footer-detail:footer-detail-window-created popover:replay-open:render-open popover:placement-four-directions:all-four-directions-visible popover:close-trigger:closed-by-trigger-reclick popover:close-outside:closed-by-outside-click popover:close-esc:closed-by-escape menu-button:open:initial-open menu-button:placement-four-directions:all-four-directions-visible menu-button:close-trigger:closed-by-trigger-reclick menu-button:close-outside:closed-by-outside-click menu-button:close-esc:closed-by-escape menu-button:close-selection:closed-by-menu-item tooltip:open:initial-visible tooltip:placement-four-directions:all-four-directions-visible tooltip:close-pointer-leave:closed-by-pointer-leave tooltip:close-focus-loss:closed-by-focus-loss tooltip:close-esc:closed-by-escape combo-box:open:initial-open select-box:open:initial-open color-picker-rgba:open:initial-open side-menu:open:initial-popover-open}"
LOG_PREFIX="katana-storybook-interaction:supported"
EXERCISED_PREFIX="katana-storybook-interaction:exercised"
cd "$ROOT_DIR/storybook"
RUSTFLAGS="-D warnings" cargo build

binary="$ROOT_DIR/storybook/target/debug/storybook"
tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/katana-ui-widget-storybook-requirement.XXXXXX")"

pids=()
logs=()
labels=()

for scenario in $SCENARIOS; do
  page="${scenario%%:*}"
  rest="${scenario#*:}"
  interaction="${rest%%:*}"
  expected_detail="${rest#*:}"
  log="$tmp_dir/${page}-${interaction}.log"
  echo "storybook requirement gate: $page ($interaction -> $expected_detail)"

  (
    if ! KATANA_UI_WIDGET_STORYBOOK_PAGE="$page" \
      KATANA_UI_WIDGET_STORYBOOK_INTERACTION="$interaction" \
      KATANA_UI_WIDGET_STORYBOOK_EXPECTED_DETAIL="$expected_detail" \
      "$binary" --headless-scenario >"$log" 2>&1; then
      echo "storybook requirement gate crashed: $scenario"
      cat "$log"
      exit 1
    fi

    supported_marker="$LOG_PREFIX page=$page interaction=$interaction"
    exercised_marker="$EXERCISED_PREFIX page=$page interaction=$interaction detail=$expected_detail"
    grep -F "$supported_marker" "$log" >/dev/null
    grep -F "$exercised_marker" "$log" >/dev/null
  ) &
  pids+=("$!")
  logs+=("$log")
  labels+=("$scenario")
done

for index in "${!pids[@]}"; do
  if ! wait "${pids[$index]}"; then
    echo "storybook requirement gate failed: ${labels[$index]}"
    cat "${logs[$index]}"
    exit 1
  fi
done
