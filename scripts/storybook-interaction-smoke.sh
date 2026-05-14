#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

STORYBOOK_REQUIREMENT_SCENARIOS="${STORYBOOK_INTERACTION_SCENARIOS:-popover:replay-open:render-open popover:open:render-open menu-button:open:initial-open tooltip:open:initial-visible combo-box:open:initial-open select-box:open:initial-open color-picker-rgba:open:initial-open}" \
  bash "$ROOT_DIR/scripts/storybook-requirement-gate.sh"
