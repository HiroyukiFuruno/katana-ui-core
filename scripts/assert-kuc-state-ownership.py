#!/usr/bin/env python3
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def main() -> int:
    failures: list[str] = []
    render_model = "\n".join(
        path.read_text(encoding="utf-8")
        for path in sorted((ROOT / "crates/katana-ui-core/src/render_model").glob("*.rs"))
    )
    atom_model = read("crates/katana-ui-core/src/atom/mod.rs")
    contract_test = read("crates/katana-ui-core/tests/core_contract.rs")

    required_render_tokens = (
        "pub struct UiStateId",
        "pub struct UiInteractionState",
        "state_id: UiStateId",
        "AtomicU64",
        "fetch_add",
    )
    for token in required_render_tokens:
        if token not in render_model:
            failures.append(f"render_model missing state ownership token: {token}")

    forbidden_atom_tokens = (
        "pub struct AtomState",
        "pub disabled:",
        "pub focusable:",
        "pub accessibility_label:",
    )
    for token in forbidden_atom_tokens:
        if token in atom_model:
            failures.append(f"atom state is externally mutable: {token}")

    if "duplicate_ui_instances_have_unique_state_identity" not in contract_test:
        failures.append("core_contract lacks duplicate UI state identity test")
    if "complex_ui_state_is_owned_by_the_component_model" not in contract_test:
        failures.append("core_contract lacks complex UI internal state test")

    if failures:
        print("KUC state ownership guard failed", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
