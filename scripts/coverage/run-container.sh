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

coverage_image_id="$(
  docker image inspect "${coverage_image}" \
    | python3 scripts/coverage/image-runtime-id.py
)"
if [[ ! "${coverage_image_id}" =~ ^runtime-v1:sha256:[0-9a-f]{64}$ ]]; then
  echo "coverage image runtime identity has an invalid format" >&2
  exit 1
fi

docker run --rm \
  --volume "${repo_root}:/source:ro" \
  --volume kuc-coverage-cargo-registry:/usr/local/cargo/registry \
  --volume kuc-coverage-target:/tmp/kuc-target \
  --workdir /source \
  --env CARGO_BUILD_JOBS="${coverage_build_jobs}" \
  --env CARGO_INCREMENTAL=0 \
  --env CARGO_TARGET_DIR=/tmp/kuc-target \
  --env COVERAGE_TEST_THREADS="${coverage_test_threads}" \
  --env KUC_COVERAGE_REUSE="${coverage_reuse}" \
  --env KUC_COVERAGE_RUNTIME=container \
  --env KUC_COVERAGE_IMAGE_ID="${coverage_image_id}" \
  --env KUC_COVERAGE_SUPPLEMENT_TARGET \
  --env KUC_COVERAGE_SUPPLEMENT_FILTER \
  "${coverage_image}" \
  bash scripts/coverage/run-in-container.sh
