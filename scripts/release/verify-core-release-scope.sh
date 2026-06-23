#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

bash "$ROOT_DIR/scripts/release/verify-version.sh" "${1:-}" >/dev/null

packages="$(
  cargo metadata --no-deps --locked --format-version 1 \
    | python3 -c 'import json, sys; data = json.load(sys.stdin); print("\n".join(sorted(p["name"] for p in data["packages"])))'
)"

expected=$'katana-ui-core\nkatana-ui-core-storybook\nkuc-consumer-app'
if [[ "$packages" != "$expected" ]]; then
  echo "core release scope failed"
  echo "$packages"
  exit 1
fi

echo "core release scope verified: active workspace contains core, storybook, and consumer app."
