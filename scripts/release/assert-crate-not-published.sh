#!/usr/bin/env bash
set -euo pipefail

version="$(bash "$(dirname "$0")/verify-version.sh" "${1:-}" | awk -F= '$1 == "version_bare" { print $2 }')"
package="katana-ui-core"

if cargo info "${package}@${version}" --registry crates-io >/dev/null 2>&1; then
  echo "${package} ${version} is already published on crates.io." >&2
  exit 1
fi

echo "crates.io target version is unpublished"
