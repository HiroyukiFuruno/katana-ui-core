#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

bash "$ROOT_DIR/scripts/release/verify-version.sh" "${1:-}" >/dev/null

workspace_packages="$(
  cargo metadata --no-deps --locked --format-version 1 \
    | python3 -c 'import json, sys; data = json.load(sys.stdin); print("\n".join(sorted(p["name"] for p in data["packages"])))'
)"

expected_workspace=$'katana-ui-core\nkatana-ui-core-egui-adapter\nkatana-ui-core-storybook\nkatana-ui-core-svg-raster\nkatana-ui-core-text-raster\nkuc-consumer-app'
if [[ "$workspace_packages" != "$expected_workspace" ]]; then
  echo "workspace release scope failed"
  echo "$workspace_packages"
  exit 1
fi

publishable_packages="$(
  cargo metadata --no-deps --locked --format-version 1 \
    | python3 -c 'import json, sys; data = json.load(sys.stdin); print("\n".join(sorted(p["name"] for p in data["packages"] if p["publish"] != [])))'
)"
expected_publishable=$'katana-ui-core\nkatana-ui-core-egui-adapter\nkatana-ui-core-svg-raster\nkatana-ui-core-text-raster'
if [[ "$publishable_packages" != "$expected_publishable" ]]; then
  echo "publishable crate scope failed"
  echo "$publishable_packages"
  exit 1
fi

echo "workspace release scope verified: four public crates and two private consumers."
