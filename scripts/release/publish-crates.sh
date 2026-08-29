#!/usr/bin/env bash
set -euo pipefail

version="$(bash "$(dirname "$0")/verify-version.sh" "${1:-}" | awk -F= '$1 == "version_bare" { print $2 }')"
packages=(
  katana-ui-core
  katana-ui-core-text-raster
  katana-ui-core-svg-raster
  katana-ui-core-egui-adapter
)

if [[ -z "${CARGO_REGISTRY_TOKEN:-}" ]]; then
  echo "CARGO_REGISTRY_TOKEN is required." >&2
  exit 1
fi

wait_until_available() {
  local package="$1"
  for _ in {1..30}; do
    if cargo info "${package}@${version}" --registry crates-io >/dev/null 2>&1; then
      return 0
    fi
    sleep 10
  done
  echo "${package} ${version} was not available from crates.io after publish." >&2
  return 1
}

for package in "${packages[@]}"; do
  if cargo info "${package}@${version}" --registry crates-io >/dev/null 2>&1; then
    echo "${package} ${version} already published; skipping."
    continue
  fi
  cargo publish -p "${package}" --locked --token "${CARGO_REGISTRY_TOKEN}"
  wait_until_available "${package}"
done
