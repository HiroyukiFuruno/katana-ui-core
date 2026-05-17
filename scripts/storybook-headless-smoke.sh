#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT_DIR"
RUSTFLAGS="-D warnings" cargo build -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked

binary="$ROOT_DIR/target/debug/katana-ui-core-storybook"
output="$("$binary" --headless-page)"
case "$output" in
  katana-ui-core-storybook:*) ;;
  *)
    echo "storybook headless smoke failed"
    echo "$output"
    exit 1
    ;;
esac
