#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

MANIFEST_PATH = Path("docs/storybook-77ui-interaction-manifest.json")
OPEN_WINDOW_PREFIX = (
    "rtk cargo run --release -p katana-ui-core-storybook --bin "
    "katana-ui-core-storybook --locked -- --open-window"
)
MANUAL_GATE = "do not proceed to the next UI until this page is approved"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--json", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    queue = manual_acceptance_queue(args.manifest)
    if args.json:
        print(json.dumps(queue, ensure_ascii=False, indent=2))
        return 0
    for entry in queue:
        print(format_queue_entry(entry))
    return 0


def manual_acceptance_queue(manifest_path: Path) -> list[dict[str, Any]]:
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    entries = manifest.get("ui", [])
    if not isinstance(entries, list):
        return []
    queue: list[dict[str, Any]] = []
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        page = entry.get("page")
        gaps = entry.get("gaps", [])
        if not isinstance(page, str) or not isinstance(gaps, list):
            continue
        if not any(
            isinstance(gap, str) and "manual_acceptance_pending" in gap for gap in gaps
        ):
            continue
        require_pending_audit_status(entry, page)
        order = manual_acceptance_order(entry, page)
        dependency_layer = required_string(entry, "dependency_layer", page)
        depends_on = required_unique_string_list(
            entry,
            "depends_on",
            page,
            allow_empty=True,
        )
        operations = required_unique_string_list(entry, "required_operations", page)
        acceptance_checks = required_unique_string_list(
            entry,
            "acceptance_checks",
            page,
        )
        acceptance_observations = required_unique_string_list(
            entry,
            "acceptance_observations",
            page,
        )
        evidence_contract = optional_evidence_contract(entry, page)
        frames = minimum_observation_frames(entry, page)
        queue.append(
            {
                "page": page,
                "audit_status": entry.get("audit_status"),
                "manual_acceptance_order": order,
                "dependency_layer": dependency_layer,
                "depends_on": depends_on,
                "required_operations": operations,
                "command": f"{OPEN_WINDOW_PREFIX} {page}",
                "smoke_command": smoke_command(page, frames),
                "minimum_observation_frames": frames,
                "acceptance_checks": acceptance_checks,
                "acceptance_observations": acceptance_observations,
                "acceptance_evidence_contract": evidence_contract,
                "manual_gate": MANUAL_GATE,
                "gaps": [gap for gap in gaps if isinstance(gap, str)],
            }
        )
    return sorted(queue, key=lambda item: item["manual_acceptance_order"])


def format_queue_entry(entry: dict[str, Any]) -> str:
    operations = ",".join(string_values(entry.get("required_operations", [])))
    depends_on = ",".join(string_values(entry.get("depends_on", [])))
    checks = ",".join(string_values(entry.get("acceptance_checks", [])))
    observations = ";".join(string_values(entry.get("acceptance_observations", [])))
    manual_gate = entry.get("manual_gate", "")
    return "\t".join(
        [
            str(entry.get("page", "")),
            f"operations={operations}",
            f"order={entry.get('manual_acceptance_order', '')}",
            f"layer={entry.get('dependency_layer', '')}",
            f"depends_on={depends_on}",
            f"checks={checks}",
            f"observe={observations}",
            f"manual_gate={manual_gate}",
            f"command={entry.get('command', '')}",
            f"smoke={entry.get('smoke_command', '')}",
        ]
    )


def string_values(values: Any) -> list[str]:
    if not isinstance(values, list):
        return []
    return [value for value in values if isinstance(value, str)]


def manual_acceptance_order(entry: dict[str, Any], page: str) -> int:
    value = entry.get("manual_acceptance_order")
    if not isinstance(value, int) or value <= 0:
        raise ValueError(f"{page}: manual_acceptance_order must be a positive integer")
    return value


def require_pending_audit_status(entry: dict[str, Any], page: str) -> None:
    if entry.get("audit_status") != "partial":
        raise ValueError(
            f"{page}: audit_status must be partial while manual_acceptance_pending remains"
        )


def required_string(entry: dict[str, Any], key: str, page: str) -> str:
    value = entry.get(key)
    if not isinstance(value, str) or not value:
        raise ValueError(f"{page}: {key} must be a non-empty string")
    return value


def required_string_list(entry: dict[str, Any], key: str, page: str) -> list[str]:
    value = entry.get(key)
    if not isinstance(value, list) or any(not isinstance(item, str) for item in value):
        raise ValueError(f"{page}: {key} must be a string array")
    return value


def required_unique_string_list(
    entry: dict[str, Any],
    key: str,
    page: str,
    allow_empty: bool = False,
) -> list[str]:
    values = required_string_list(entry, key, page)
    if not values and not allow_empty:
        raise ValueError(f"{page}: {key} must be a non-empty string array")
    if any(not value.strip() for value in values):
        raise ValueError(f"{page}: {key} must contain only non-empty strings")
    seen: set[str] = set()
    duplicates: list[str] = []
    for value in values:
        if value in seen and value not in duplicates:
            duplicates.append(value)
        seen.add(value)
    if duplicates:
        raise ValueError(
            f"{page}: {key} must not contain duplicate values: {', '.join(duplicates)}"
        )
    return values


def optional_evidence_contract(entry: dict[str, Any], page: str) -> list[dict[str, str]]:
    value = entry.get("acceptance_evidence_contract", [])
    if not isinstance(value, list):
        raise ValueError(f"{page}: acceptance_evidence_contract must be an array")
    contracts: list[dict[str, str]] = []
    for index, item in enumerate(value):
        if not isinstance(item, dict):
            raise ValueError(
                f"{page}: acceptance_evidence_contract[{index}] must be an object"
            )
        contract: dict[str, str] = {}
        for key, item_value in item.items():
            if not isinstance(key, str) or not isinstance(item_value, str) or not item_value:
                raise ValueError(
                    f"{page}: acceptance_evidence_contract[{index}] must contain string values"
                )
            contract[key] = item_value
        if "check" not in contract:
            raise ValueError(
                f"{page}: acceptance_evidence_contract[{index}] must include check"
            )
        contracts.append(contract)
    return contracts


def smoke_command(page: str, frames: int) -> str:
    return f"{OPEN_WINDOW_PREFIX} {frames} {page}"


def minimum_observation_frames(entry: dict[str, Any], page: str) -> int:
    value = entry.get("minimum_observation_frames")
    if not isinstance(value, int) or value <= 0:
        raise ValueError(f"{page}: minimum_observation_frames must be a positive integer")
    return value


def require_no_pending_dependencies(
    entry: dict[str, Any],
    queue: list[dict[str, Any]],
) -> None:
    pending_pages = {str(item.get("page", "")) for item in queue}
    pending_dependencies = [
        dependency
        for dependency in string_values(entry.get("depends_on", []))
        if dependency in pending_pages
    ]
    if pending_dependencies:
        page = str(entry.get("page", "<unknown>"))
        raise ValueError(
            f"{page} depends on pending pages: {', '.join(pending_dependencies)}"
        )


if __name__ == "__main__":
    raise SystemExit(main())
