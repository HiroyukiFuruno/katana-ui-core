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
export KUC_STORYBOOK_MOUSE_TRACE="${CARGO_TARGET_DIR:-target}/kuc-storybook-mouse-trace.jsonl"
# LLVM更新で実行済みgeneric関数が未到達の最適化instanceとして集計されないようにする。
export CARGO_PROFILE_TEST_OPT_LEVEL=0

coverage_target_dir="${CARGO_TARGET_DIR:-target}/llvm-cov-target"
run_cargo clean --target-dir "$coverage_target_dir"
run_cargo llvm-cov clean --workspace
run_cargo llvm-cov \
  -p katana-ui-core \
  -p katana-ui-core-storybook \
  -p kuc-consumer-app \
  --all-targets \
  --all-features \
  --locked \
  --no-report \
  -- \
  --include-ignored
run_cargo llvm-cov report \
  --summary-only \
  --fail-under-functions 100 \
  --fail-under-lines 100 \
  --fail-uncovered-functions 0 \
  --fail-uncovered-lines 0
