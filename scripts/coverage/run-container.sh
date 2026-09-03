#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 5 ]]; then
  echo "usage: run-container.sh IMAGE REPO_ROOT BUILD_JOBS TEST_THREADS REUSE" >&2
  exit 1
fi

coverage_image="$1"
repo_root="$2"
coverage_build_jobs="$3"
coverage_test_threads="$4"
coverage_reuse="$5"
coverage_target_mount="kuc-coverage-target:/tmp/kuc-target"
coverage_bind_target=0
coverage_source_snapshot=""

cleanup_source_snapshot() {
  if [[ -n "${coverage_source_snapshot}" && -d "${coverage_source_snapshot}" ]]; then
    rm -rf -- "${coverage_source_snapshot}"
  fi
}
trap cleanup_source_snapshot EXIT

if [[ ! "${coverage_build_jobs}" =~ ^[1-9][0-9]*$ ]]; then
  echo "coverage build jobs must be a positive integer" >&2
  exit 1
fi
if [[ "${coverage_test_threads}" != "auto" && ! "${coverage_test_threads}" =~ ^[1-9][0-9]*$ ]]; then
  echo "coverage test threads must be auto or a positive integer" >&2
  exit 1
fi
if [[ "${coverage_reuse}" != "0" && "${coverage_reuse}" != "1" ]]; then
  echo "coverage reuse must be 0 or 1" >&2
  exit 1
fi
if [[ -n "${KUC_COVERAGE_HOST_TARGET_DIR:-}" ]]; then
  if [[ "${KUC_COVERAGE_HOST_TARGET_DIR}" != /* || "${KUC_COVERAGE_HOST_TARGET_DIR}" == "/" ]]; then
    echo "KUC_COVERAGE_HOST_TARGET_DIR must be a non-root absolute path" >&2
    exit 1
  fi
  mkdir -p "${KUC_COVERAGE_HOST_TARGET_DIR}"
  coverage_target_mount="${KUC_COVERAGE_HOST_TARGET_DIR}:/tmp/kuc-target"
  coverage_bind_target=1
fi

# Colima は bind mount 対象を VM へ転送するため、生成済み cache を除外して
# coverage の転送量が過去の実行回数ではなく source 量に比例するようにする。
mkdir -p "${repo_root}/.codex"
coverage_source_snapshot="$(mktemp -d "${repo_root}/.codex/coverage-source.XXXXXX")"
tar \
  --exclude='./.git' \
  --exclude='./target' \
  --exclude='./.codex' \
  --exclude='./tmp' \
  -C "${repo_root}" \
  -cf - . \
  | tar -C "${coverage_source_snapshot}" -xf -

coverage_image_id="$(
  docker image inspect "${coverage_image}" \
    | python3 scripts/coverage/image-runtime-id.py
)"
if [[ ! "${coverage_image_id}" =~ ^runtime-v1:sha256:[0-9a-f]{64}$ ]]; then
  echo "coverage image runtime identity has an invalid format" >&2
  exit 1
fi

docker run --rm \
  --volume "${coverage_source_snapshot}:/source:ro" \
  --volume kuc-coverage-cargo-registry:/usr/local/cargo/registry \
  --volume "${coverage_target_mount}" \
  --workdir /source \
  --env CARGO_BUILD_JOBS="${coverage_build_jobs}" \
  --env CARGO_INCREMENTAL=0 \
  --env CARGO_TARGET_DIR=/tmp/kuc-target \
  --env COVERAGE_TEST_THREADS="${coverage_test_threads}" \
  --env KUC_COVERAGE_REUSE="${coverage_reuse}" \
  --env KUC_COVERAGE_BIND_TARGET="${coverage_bind_target}" \
  --env KUC_COVERAGE_RUNTIME=container \
  --env KUC_COVERAGE_IMAGE_ID="${coverage_image_id}" \
  --env KUC_COVERAGE_SUPPLEMENT_TARGET \
  --env KUC_COVERAGE_SUPPLEMENT_FILTER \
  "${coverage_image}" \
  bash scripts/coverage/run-in-container.sh
