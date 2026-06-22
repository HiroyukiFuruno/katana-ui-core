#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
read -r -a CARGO_CMD <<<"${CARGO:-cargo}"

cd "$ROOT_DIR"
"${CARGO_CMD[@]}" rustc -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings

binary="$ROOT_DIR/target/debug/katana-ui-core-storybook"
echo "core storybook contract: katana-ui-core model only"
"$binary" --headless-scenario
