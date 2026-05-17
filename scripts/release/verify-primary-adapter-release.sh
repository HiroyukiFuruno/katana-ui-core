#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
version="$(bash "$ROOT_DIR/scripts/release/verify-version.sh" "${1:-}" | awk -F= '$1 == "version_bare" { print $2 }')"
core_package="katana-ui-core"
adapter_package="katana-ui-core-floem"

cd "$ROOT_DIR"

if cargo info "${core_package}@${version}" --registry crates-io >/dev/null 2>&1; then
  cargo package -p "$adapter_package" --locked --allow-dirty
  cargo publish -p "$adapter_package" --dry-run --locked --allow-dirty
  exit 0
fi

echo "${core_package} ${version} is not published yet; running first-publish adapter gate."
cargo package -p "$adapter_package" --list --allow-dirty >/dev/null
cargo check -p "$adapter_package" --all-targets --locked
cargo test -p "$adapter_package" --locked
