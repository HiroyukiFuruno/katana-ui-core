#!/usr/bin/env bash
set -euo pipefail

case "${KUC_COVERAGE_EPHEMERAL_CLEANUP:-0}" in
  0) exit 0 ;;
  1) ;;
  *) echo "KUC_COVERAGE_EPHEMERAL_CLEANUP must be 0 or 1" >&2; exit 1 ;;
esac
if [[ "${GITHUB_ACTIONS:-}" != "true" || "${RUNNER_ENVIRONMENT:-}" != "github-hosted" || "${RUNNER_OS:-}" != "Linux" ]]; then
  echo "coverage storage cleanup requires a disposable GitHub-hosted Linux runner" >&2
  exit 1
fi
if [[ "$#" -ne 1 || -n "${CARGO_TARGET_DIR:-}" ]]; then
  echo "coverage storage cleanup requires the repository default target directory" >&2
  exit 1
fi
coverage_repo="$(git rev-parse --show-toplevel)"
if [[ "$1" != "${coverage_repo}" || "${GITHUB_WORKSPACE:-}" != "${coverage_repo}" || "${coverage_repo}" == "/" ]]; then
  echo "coverage storage cleanup requires the current GitHub workspace" >&2
  exit 1
fi

# 検証済みhost buildとcoverage buildの二重占有を避け、検証範囲は変えない。
df -h "${coverage_repo}"
cargo clean --manifest-path "${coverage_repo}/Cargo.toml" --target-dir "${coverage_repo}/target"
df -h "${coverage_repo}"
