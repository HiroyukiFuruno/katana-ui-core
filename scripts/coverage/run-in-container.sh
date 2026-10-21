#!/usr/bin/env bash
set -euo pipefail

if [[ "${KUC_COVERAGE_RUNTIME:-}" != "container" ]]; then
  echo "container coverage requires KUC_COVERAGE_RUNTIME=container" >&2
  exit 1
fi
if [[ ! "${KUC_COVERAGE_IMAGE_ID:-}" =~ ^runtime-v1:sha256:[0-9a-f]{64}$ ]]; then
  echo "container coverage requires a validated runtime image identity" >&2
  exit 1
fi

workspace=/tmp/kuc-workspace
coverage_log_dir="${KUC_COVERAGE_LOG_DIR:-}"
readonly linux_emoji_font=/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf
readonly pinned_linux_emoji_sha256=e5899ed38b8ed83e08bd3ac5de09791e9d19d288333a796de1d35ad17396f1ec
actual_linux_emoji_sha256="$(sha256sum "${linux_emoji_font}" | cut -d ' ' -f 1)"
if [[ "${actual_linux_emoji_sha256}" != "${pinned_linux_emoji_sha256}" ]]; then
  echo "coverage image emoji font hash does not match the pinned release input" >&2
  exit 1
fi

copy_failed_coverage_logs() {
  local exit_status="$?"
  local coverage_file
  local run_dir
  if [[ "${exit_status}" -eq 0 || -z "${coverage_log_dir}" ]]; then
    return "${exit_status}"
  fi
  mkdir -p "${coverage_log_dir}"
  shopt -s nullglob
  for run_dir in /tmp/kuc-target/llvm-cov-target/kuc-test-run-*; do
    if [[ -d "${run_dir}/logs" ]]; then
      cp -a "${run_dir}/logs" "${coverage_log_dir}/$(basename "${run_dir}")"
    fi
  done
  for coverage_file in \
    /tmp/kuc-target/kuc-workspace-coverage-summary.json \
    /tmp/kuc-target/kuc-workspace-coverage.lcov \
    /tmp/kuc-target/kuc-workspace-coverage-profile-v3.sha256 \
    /tmp/kuc-target/kuc-workspace-coverage-profile-v3.sha256.strict-state; do
    if [[ -f "${coverage_file}" ]]; then
      cp -a "${coverage_file}" "${coverage_log_dir}/"
    fi
  done
  {
    printf 'coverage_exit_status=%s\n' "${exit_status}"
    df -Pk /tmp/kuc-target || true
  } >"${coverage_log_dir}/failure-metadata.txt"
  return "${exit_status}"
}
trap copy_failed_coverage_logs EXIT

mkdir -p "${workspace}"
tar \
  --exclude-vcs \
  --exclude=target \
  --exclude='*/target' \
  --create \
  --file=- \
  --directory=/source \
  . \
  | tar --extract --file=- --directory="${workspace}"

cd "${workspace}"
bash scripts/run-strict-coverage.sh

# 成功済みcoverageのbuildだけを解放し、後続package検証とsummaryを保持する。
if [[ "${KUC_COVERAGE_EPHEMERAL_CLEANUP:-0}" == "1" ]]; then
  if [[ "${KUC_COVERAGE_REUSE:-0}" != "0" || "${CARGO_TARGET_DIR:-}" != "/tmp/kuc-target" ]]; then
    echo "ephemeral coverage cleanup requires a clean run with the dedicated target" >&2
    exit 1
  fi
  cargo clean --target-dir /tmp/kuc-target/llvm-cov-target
fi
