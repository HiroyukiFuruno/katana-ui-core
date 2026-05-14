#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

STORYBOOK_REQUIREMENT_SCENARIOS="${STORYBOOK_INTERACTION_SCENARIOS:-popover:replay-open:render-open popover:open:render-open popover:placement-four-directions:all-four-directions-visible popover:close-trigger:closed-by-trigger-reclick popover:close-outside:closed-by-outside-click popover:close-esc:closed-by-escape menu-button:open:initial-open menu-button:placement-four-directions:all-four-directions-visible menu-button:close-trigger:closed-by-trigger-reclick menu-button:close-outside:closed-by-outside-click menu-button:close-esc:closed-by-escape menu-button:close-selection:closed-by-menu-item tooltip:open:initial-visible tooltip:placement-four-directions:all-four-directions-visible tooltip:close-pointer-leave:closed-by-pointer-leave tooltip:close-focus-loss:closed-by-focus-loss tooltip:close-esc:closed-by-escape combo-box:open:initial-open select-box:open:initial-open color-picker-rgba:open:initial-open}" \
  bash "$ROOT_DIR/scripts/storybook-requirement-gate.sh"
