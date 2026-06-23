#!/usr/bin/env bash
set -euo pipefail

version="$(bash "$(dirname "$0")/verify-version.sh" "${1:-}" | awk -F= '$1 == "version_bare" { print $2 }')"
packages=("katana-ui-core" "katana-ui-core-storybook")
pending=()

for package in "${packages[@]}"; do
  if cargo info "${package}@${version}" --registry crates-io >/dev/null 2>&1; then
    echo "${package} ${version} is already published on crates.io; release publish will skip it."
    continue
  fi

  pending+=("${package}")
done

if [[ "${#pending[@]}" -eq 0 ]]; then
  echo "all crates.io publish targets are already published; release check is idempotent."
  exit 0
fi

printf 'crates.io publish targets pending: %s\n' "${pending[*]}"
