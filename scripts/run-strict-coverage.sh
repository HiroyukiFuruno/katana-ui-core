#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != "Linux" ]]; then
  echo "strict coverage requires Linux with Xvfb" >&2
  exit 1
fi

if ! command -v Xvfb >/dev/null; then
  echo "Xvfb is required for strict native-window coverage" >&2
  exit 1
fi

if [[ -n "${CARGO:-}" ]]; then
  read -r -a cargo_command <<<"${CARGO}"
elif command -v rtk >/dev/null; then
  cargo_command=(rtk cargo)
else
  cargo_command=(cargo)
fi

run_cargo() {
  "${cargo_command[@]}" "$@"
}

display_number=99
while [[ -e "/tmp/.X${display_number}-lock" || -S "/tmp/.X11-unix/X${display_number}" ]]; do
  display_number=$((display_number + 1))
done
xvfb_log="${CARGO_TARGET_DIR:-target}/kuc-xvfb.log"
mkdir -p "$(dirname "${xvfb_log}")"
Xvfb ":${display_number}" -screen 0 1600x1200x24 -nolisten tcp -ac >"${xvfb_log}" 2>&1 &
xvfb_pid=$!

cleanup_xvfb() {
  kill "${xvfb_pid}" 2>/dev/null || true
  wait "${xvfb_pid}" 2>/dev/null || true
}
trap cleanup_xvfb EXIT

for _ in {1..100}; do
  if [[ -S "/tmp/.X11-unix/X${display_number}" ]]; then
    break
  fi
  if ! kill -0 "${xvfb_pid}" 2>/dev/null; then
    cat "${xvfb_log}" >&2
    exit 1
  fi
  sleep 0.1
done
if [[ ! -S "/tmp/.X11-unix/X${display_number}" ]]; then
  echo "Xvfb did not create its X11 socket" >&2
  cat "${xvfb_log}" >&2
  exit 1
fi
export DISPLAY=":${display_number}"
export XDG_RUNTIME_DIR="${CARGO_TARGET_DIR:-target}/kuc-xdg-runtime"
mkdir -p "${XDG_RUNTIME_DIR}"
chmod 700 "${XDG_RUNTIME_DIR}"
export KUC_STORYBOOK_MOUSE_TRACE="${CARGO_TARGET_DIR:-target}/kuc-storybook-mouse-trace.jsonl"
# LLVM更新で実行済みgeneric関数が未到達の最適化instanceとして集計されないようにする。
export CARGO_PROFILE_TEST_OPT_LEVEL=0

coverage_target_dir="${CARGO_TARGET_DIR:-target}/llvm-cov-target"
coverage_reuse="${KUC_COVERAGE_REUSE:-0}"
coverage_test_threads="${COVERAGE_TEST_THREADS:-4}"
coverage_started_at="${SECONDS}"
case "${coverage_reuse}" in
  0)
    coverage_mode="clean"
    run_cargo clean --target-dir "$coverage_target_dir"
    run_cargo llvm-cov clean --workspace
    ;;
  1)
    coverage_mode="reuse"
    # WHY: 反復時も実行profileは再利用せず、compiler outputだけを保持する。
    run_cargo llvm-cov clean --profraw-only
    ;;
  *)
    echo "KUC_COVERAGE_REUSE must be 0 or 1" >&2
    exit 1
    ;;
esac
if [[ ! "${coverage_test_threads}" =~ ^[1-9][0-9]*$ ]]; then
  echo "COVERAGE_TEST_THREADS must be a positive integer" >&2
  exit 1
fi
coverage_min_free_gib="${KUC_COVERAGE_MIN_FREE_GIB:-2}"
if [[ ! "${coverage_min_free_gib}" =~ ^[1-9][0-9]*$ ]]; then
  echo "KUC_COVERAGE_MIN_FREE_GIB must be a positive integer" >&2
  exit 1
fi
coverage_available_kib="$(df -Pk "${CARGO_TARGET_DIR:-target}" | awk 'NR == 2 { print $4 }')"
coverage_required_kib="$((coverage_min_free_gib * 1024 * 1024))"
if ((coverage_available_kib < coverage_required_kib)); then
  echo "strict coverage requires at least ${coverage_min_free_gib} GiB free after cleanup" >&2
  exit 1
fi
echo "coverage mode: ${coverage_mode}; test threads: ${coverage_test_threads}"
run_cargo llvm-cov \
  -p katana-ui-core \
  -p katana-ui-core-egui-adapter \
  -p katana-ui-core-storybook \
  -p katana-ui-core-svg-raster \
  -p katana-ui-core-text-raster \
  -p kuc-consumer-app \
  --all-targets \
  --all-features \
  --locked \
  --no-report \
  -- \
  --include-ignored \
  --test-threads="${coverage_test_threads}"
run_cargo llvm-cov report \
  --summary-only \
  --ignore-filename-regex '(^|/)(tests/|tests\.rs$|[^/]+_tests\.rs$)' \
  --fail-under-functions 100 \
  --fail-under-lines 100 \
  --fail-uncovered-functions 0 \
  --fail-uncovered-lines 0
echo "strict coverage elapsed seconds: $((SECONDS - coverage_started_at))"
