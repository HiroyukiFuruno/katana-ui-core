#!/usr/bin/env bash
set -euo pipefail

version="$(bash "$(dirname "$0")/verify-version.sh" "${1:-}" | awk -F= '$1 == "version_bare" { print $2 }')"
package="katana-ui-widget"

if [[ -z "${CARGO_REGISTRY_TOKEN:-}" ]]; then
  echo "CARGO_REGISTRY_TOKEN is required." >&2
  exit 1
fi

if cargo info "${package}@${version}" --registry crates-io >/dev/null 2>&1; then
  echo "${package} ${version} already published; skipping."
  exit 0
fi

cargo publish -p "${package}" --locked --token "${CARGO_REGISTRY_TOKEN}"
