#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
read -r -a CARGO_CMD <<<"${CARGO:-cargo}"

cd "$ROOT_DIR"
RUSTFLAGS="-D warnings" "${CARGO_CMD[@]}" build -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked

binary="$ROOT_DIR/target/debug/katana-ui-core-storybook"
echo "core storybook contract: katana-ui-core model only"
"$binary" --headless-scenario
