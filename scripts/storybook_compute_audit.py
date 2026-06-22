#!/usr/bin/env python3
"""Compute-driven final audit for the 77 KUC Storybook pages."""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs/storybook-77ui-interaction-manifest.json"
AUDIT_JSON = ROOT / "target/storybook-live-interaction-audit.json"
DEFAULT_BINARY = ROOT / "target/release/katana-ui-core-storybook"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", default=str(DEFAULT_BINARY))
    parser.add_argument("--manifest", default=str(MANIFEST))
    parser.add_argument("--audit", default=str(AUDIT_JSON))
    parser.add_argument("--output", default=str(ROOT / "target/storybook-compute-audit"))
    parser.add_argument("--themes", default="dark,light")
    parser.add_argument("--presets", default="0,1,2,3")
    parser.add_argument("--skip-snapshots", action="store_true")
    parser.add_argument("--limit", type=int, default=0)
    return parser.parse_args()


def load_json(path: Path) -> Any:
    return json.loads(path.read_text())


def resolve_under_root(path: str) -> Path:
    candidate = Path(path)
    if candidate.is_absolute():
        return candidate
    return ROOT / candidate


def manifest_pages(manifest: dict[str, Any]) -> list[dict[str, Any]]:
    pages = manifest.get("ui")
    if not isinstance(pages, list):
        raise SystemExit("manifest ui must be a list")
    return pages


def required_operations(manifest: dict[str, Any], item: dict[str, Any]) -> list[str]:
    explicit = item.get("required_operations")
    if isinstance(explicit, list) and explicit:
        return [str(it) for it in explicit]
    engine = item.get("engine")
    defaults = manifest.get("defaults_by_engine", {}).get(engine, {})
    ops = defaults.get("required_operations", [])
    return [str(it) for it in ops]


def run_snapshot(
    binary: Path,
    output: Path,
    page: str,
    theme: str,
    preset: int,
    clicked: bool = False,
) -> None:
    output.parent.mkdir(parents=True, exist_ok=True)
    command = [
        str(binary),
        "--visual-snapshot",
        str(output),
        page,
        theme,
        f"preset-{preset}",
    ]
    if clicked:
        command.append("clicked")
    subprocess.run(command, cwd=ROOT, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)


def read_png_header(path: Path) -> tuple[int, int]:
    data = path.read_bytes()
    if not data.startswith(b"\x89PNG\r\n\x1a\n"):
        raise ValueError(f"{path} is not a PNG")
    if data[12:16] != b"IHDR":
        raise ValueError(f"{path} missing IHDR")
    return int.from_bytes(data[16:20], "big"), int.from_bytes(data[20:24], "big")


def image_stats(path: Path) -> dict[str, Any]:
    width, height = read_png_header(path)
    data = path.read_bytes()
    return {
        "path": str(path.relative_to(ROOT)),
        "width": width,
        "height": height,
        "file_size": path.stat().st_size,
        "sha256": hashlib.sha256(data).hexdigest(),
    }


def snapshot_changed(path_a: Path, path_b: Path) -> bool:
    return image_stats(path_a)["sha256"] != image_stats(path_b)["sha256"]


def scenario_index(audit: dict[str, Any]) -> dict[str, list[dict[str, Any]]]:
    index: dict[str, list[dict[str, Any]]] = {}
    for scenario in audit.get("scenarios", []):
        page = str(scenario.get("page"))
        index.setdefault(page, []).append(scenario)
    return index


def page_has_passed_kind(scenarios: list[dict[str, Any]], kind: str) -> bool:
    return any(it.get("operation_kind") == kind and it.get("passed") for it in scenarios)


KNOWN_EVIDENCE = {
    "checkbox": [
        "checkbox_pointer_checks_both_rows",
        "checkbox_control_toggle_reset",
        "checkbox_disabled_pointer_block",
    ],
    "toggle": [
        "preview_click",
        "toggle_keyboard_toggle",
        "preview_hover",
    ],
    "progress-bar": [
        "progress_timed_tick",
        "progress_timed_cycle",
        "progress_indeterminate_segment_motion",
    ],
    "tooltip": [
        "tooltip_hover_idempotent",
        "tooltip_anchor_hover_open",
        "tooltip_hover_leave_close",
    ],
    "modal": [
        "modal_keyboard_escape",
        "modal_focus_trap",
    ],
    "tree-view": [
        "tree_scroll_retained",
        "tree_keyboard_select",
    ],
    "text": [
        "text_drag_selection",
        "text_keyboard_copy",
        "text_keyboard_paste",
        "text_zero_distance_drag_no_selection",
    ],
}


def known_evidence_status(page: str, scenarios: list[dict[str, Any]]) -> dict[str, bool]:
    required = KNOWN_EVIDENCE.get(page, [])
    status = {}
    for needle in required:
        status[needle] = any(
            needle in str(it.get("operation", "")) and it.get("passed") for it in scenarios
        )
    return status


def failed_scenario_label(scenario: dict[str, Any]) -> str:
    page = scenario.get("page", "<unknown>")
    operation = scenario.get("operation", "<unknown>")
    kind = scenario.get("operation_kind", "<unknown>")
    action = scenario.get("action", "<unknown>")
    event = scenario.get("event", "<unknown>")
    diff = scenario.get("body_pixel_diff", "<unknown>")
    return (
        f"{page}: failed audit scenario {operation} "
        f"kind={kind} action={action} event={event} body_pixel_diff={diff}"
    )


def manifest_manual_acceptance_pending(item: dict[str, Any]) -> bool:
    if bool(item.get("manual_acceptance_pending")):
        return True
    gaps = item.get("gaps") or item.get("missing") or []
    return any("manual_acceptance_pending" in str(gap) for gap in gaps)


def main() -> int:
    args = parse_args()
    binary = resolve_under_root(args.binary)
    output = resolve_under_root(args.output)
    manifest = load_json(resolve_under_root(args.manifest))
    audit = load_json(resolve_under_root(args.audit))
    pages = manifest_pages(manifest)
    if args.limit:
        pages = pages[: args.limit]
    themes = [it.strip() for it in args.themes.split(",") if it.strip()]
    presets = [int(it.strip()) for it in args.presets.split(",") if it.strip()]
    snapshot_dir = output / "snapshots"
    if not args.skip_snapshots:
        for item in pages:
            page = item["page"]
            for theme in themes:
                for preset in presets:
                    run_snapshot(
                        binary,
                        snapshot_dir / theme / f"preset-{preset}" / f"{page}.png",
                        page,
                        theme,
                        preset,
                    )
            run_snapshot(
                binary,
                snapshot_dir / "dark" / "clicked" / f"{page}.png",
                page,
                "dark",
                0,
                clicked=True,
            )

    by_page = scenario_index(audit)
    results = []
    blocking_failures = []
    warnings = []
    for scenario in audit.get("scenarios", []):
        if not scenario.get("passed"):
            blocking_failures.append(failed_scenario_label(scenario))
    for item in pages:
        page = item["page"]
        base = snapshot_dir / "dark" / "preset-0" / f"{page}.png"
        light = snapshot_dir / "light" / "preset-0" / f"{page}.png"
        clicked = snapshot_dir / "dark" / "clicked" / f"{page}.png"
        page_result: dict[str, Any] = {
            "page": page,
            "engine": item.get("engine"),
            "audit_status": item.get("audit_status"),
            "manual_acceptance_pending": manifest_manual_acceptance_pending(item),
            "acceptance_checks": item.get("acceptance_checks", []),
            "required_operations": required_operations(manifest, item),
        }
        try:
            stats = image_stats(base)
            page_result["snapshot"] = stats
            if stats["width"] < 1000 or stats["height"] < 700 or stats["file_size"] < 50_000:
                blocking_failures.append(f"{page}: default snapshot looks blank/too sparse")
        except Exception as error:
            blocking_failures.append(f"{page}: default snapshot unreadable: {error}")
            page_result["snapshot_error"] = str(error)
        if light.exists() and base.exists():
            changed = snapshot_changed(base, light)
            page_result["light_dark_changed"] = changed
            if not changed:
                warnings.append(f"{page}: light/dark snapshot hash did not change")
        preset_changes = []
        for preset in presets[1:]:
            candidate = snapshot_dir / "dark" / f"preset-{preset}" / f"{page}.png"
            if candidate.exists() and base.exists():
                preset_changes.append(snapshot_changed(base, candidate))
        page_result["preset_snapshot_changes"] = preset_changes
        if preset_changes and not any(preset_changes):
            warnings.append(f"{page}: first four preset snapshots are identical")
        if clicked.exists() and base.exists():
            page_result["clicked_snapshot_changed"] = snapshot_changed(base, clicked)

        scenarios = by_page.get(page, [])
        page_result["scenario_count"] = len(scenarios)
        page_result["passed_scenario_count"] = sum(1 for it in scenarios if it.get("passed"))
        missing_ops = [
            op for op in page_result["required_operations"] if not page_has_passed_kind(scenarios, op)
        ]
        page_result["missing_required_operation_evidence"] = missing_ops
        if missing_ops:
            blocking_failures.append(f"{page}: missing passed scenario for {', '.join(missing_ops)}")
        known = known_evidence_status(page, scenarios)
        page_result["known_issue_evidence"] = known
        for name, ok in known.items():
            if not ok:
                blocking_failures.append(f"{page}: missing known-issue evidence {name}")
        results.append(page_result)

    report = {
        "schema_version": 1,
        "page_count": len(pages),
        "snapshot_count": len(list(snapshot_dir.rglob("*.png"))),
        "audit_scenario_count": len(audit.get("scenarios", [])),
        "audit_passed_count": sum(1 for it in audit.get("scenarios", []) if it.get("passed")),
        "blocking_failures": blocking_failures,
        "warnings": warnings,
        "pages": results,
    }
    output.mkdir(parents=True, exist_ok=True)
    (output / "storybook-compute-audit-report.json").write_text(
        json.dumps(report, indent=2, ensure_ascii=False)
    )
    print(
        "storybook-compute-audit: "
        f"pages={report['page_count']} snapshots={report['snapshot_count']} "
        f"scenarios={report['audit_scenario_count']} passed={report['audit_passed_count']} "
        f"blocking_failures={len(blocking_failures)} warnings={len(warnings)}"
    )
    if blocking_failures:
        for failure in blocking_failures[:50]:
            print(f"BLOCKING {failure}")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
