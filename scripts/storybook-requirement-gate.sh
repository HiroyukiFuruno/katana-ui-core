#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT_DIR"
RUSTFLAGS="-D warnings" cargo build -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked

binary="$ROOT_DIR/target/debug/katana-ui-core-storybook"
output="$("$binary" --headless-scenario)"
case "$output" in
  *"stories="*"validated="*"state_conflicts=0"*"structure_failures=0"*"missing_required_pages=0"*"nodes="*"panel_nodes=3"*"panel_theme_configured=true"*"panel_theme_variants=2"*"themed_story_roots=53"*"styled_story_roots=53"*) ;;
  *)
    echo "storybook requirement gate failed"
    echo "$output"
    exit 1
    ;;
esac

snapshot="$ROOT_DIR/target/storybook-panel.png"
"$binary" --visual-snapshot "$snapshot" >/dev/null
if [[ ! -s "$snapshot" ]]; then
  echo "storybook visual snapshot failed"
  exit 1
fi

runtime_output="$("$binary" --runtime-regression)"
case "$runtime_output" in
  *"state_reflected=true"*"overlay_rendered=true"*"modal_plan_same_display=true"*"modal_plan_frontmost=true"*) ;;
  *)
    echo "storybook runtime regression failed"
    echo "$runtime_output"
    exit 1
    ;;
esac

modal_output="$("$binary" --open-modal-window 2)"
case "$modal_output" in
  *"modal_window_opened=true"*"same_display=true"*"frontmost=true"*"state_reflected=true"*"overlay_rendered=true"*) ;;
  *)
    echo "storybook modal window regression failed"
    echo "$modal_output"
    exit 1
    ;;
esac
