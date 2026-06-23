#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SEARCH_ROOTS=("$ROOT_DIR/crates/katana-ui-core" "$ROOT_DIR/crates/katana-ui-core-storybook")
violations="$(rg -n "\\b(add_overlay|remove_overlay)\\b" "${SEARCH_ROOTS[@]}" --glob '*.rs')" || true

if [ -n "$violations" ]; then
  echo "overlay lifecycle violation: direct overlay mutation must not live in active core/storybook code"
  echo "$violations"
  exit 1
fi
