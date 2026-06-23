#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

bash "$ROOT_DIR/scripts/storybook-requirement-gate.sh"
binary="$ROOT_DIR/target/release/katana-ui-core-storybook"
audit_output="$("$binary" --headless-interaction-audit)"
case "$audit_output" in
  *"checkbox_changed=true"*"radio_changed=true"*) ;;
  *)
    echo "storybook live interaction audit failed"
    echo "$audit_output"
    exit 1
    ;;
esac

audit_report="$ROOT_DIR/target/storybook-live-interaction-audit.json"
if [[ ! -s "$audit_report" ]]; then
  echo "storybook live interaction audit report failed"
  exit 1
fi
python3 - "$audit_report" <<'PY'
import json
import sys

report = json.load(open(sys.argv[1], encoding="utf-8"))
scenarios = report["scenarios"]
required = {
    "checkbox": ("checkbox_toggle", "checked_changed"),
    "radio": ("radio_select", "radio_selected"),
}
for page, (action, event) in required.items():
    scenario = next(
        (
            scenario
            for scenario in scenarios
            if scenario["page"] == page and scenario["operation"] == "row_click"
        ),
        None,
    )
    if scenario is None:
        raise SystemExit(f"missing {page} live interaction scenario")
    checks = [
        scenario["operation"] == "row_click",
        scenario["clicked"] is True,
        scenario["passed"] is True,
        scenario["action"] == action,
        scenario["event"] == event,
        scenario["body_pixel_diff"] > 0,
    ]
    if not all(checks):
        raise SystemExit(f"{page} live interaction scenario failed: {scenario}")
PY
python3 "$ROOT_DIR/scripts/storybook_manifest_interaction_smoke.py" \
  --manifest docs/storybook-77ui-interaction-manifest.json \
  --audit target/storybook-live-interaction-audit.json
