#!/usr/bin/env python3
from pathlib import Path
import re
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CHANGE = ROOT / "openspec/changes/establish-kuc-atoms-molecules-catalog"
TASKS = CHANGE / "tasks.md"
DESIGN = CHANGE / "design.md"
STORYBOOK_SPEC = CHANGE / "specs/kuc-storybook-catalog/spec.md"
QUALITY_SPEC = CHANGE / "specs/kuc-quality-gates/spec.md"
QUALITY_CONTRACT = CHANGE / "quality-gates-contract.md"
LEGACY_DOD = CHANGE / "legacy-01-24-dod.md"
LEGACY_CATALOG_CONTRACT = ROOT / "crates/katana-ui-core-storybook/tests/legacy_01_24_catalog_contract.rs"
STORYBOOK_REQUIREMENT_GATE = ROOT / "scripts/storybook-requirement-gate.sh"
CANONICAL_FILES = (
    CHANGE / "proposal.md",
    DESIGN,
    CHANGE / "quality-gates-contract.md",
    CHANGE / "storybook-catalog-contract.md",
    CHANGE / "core-foundation-contract.md",
    LEGACY_DOD,
    CHANGE / "specs/kuc-core-foundation/spec.md",
    STORYBOOK_SPEC,
    QUALITY_SPEC,
    CHANGE / "specs/kuc-widget-layer/spec.md",
    TASKS,
)
REPOSITORY_POLICY_FILES = CANONICAL_FILES + (
    ROOT / "AGENTS.md",
    ROOT / "README.md",
    ROOT / "docs/directory-structure.md",
    ROOT / "docs/widget-extraction-policy.md",
    ROOT / "docs/ui-separation-plan.md",
    ROOT / "docs/architecture/ui-separation/owned-ui-task-map.md",
    ROOT / "docs/architecture/ui-separation/ui-core-parity-gap.md",
    ROOT / "openspec/changes/README.md",
)
GATE_FILES = (
    ROOT / "Justfile",
    STORYBOOK_REQUIREMENT_GATE,
    ROOT / "scripts/assert-storybook-page-layout.py",
)

INCOMPLETE_TASK = re.compile(r"^- \[(?: |/)\] .+", re.MULTILINE)
RELEASE_TRACK_CHANGE = re.compile(r"^\d{2}-add-.+")
NO_IMAGE_POLICY_TERMS = (
    "画像回帰",
    "画像検証",
    "画像証跡",
    "画像差し替え",
    "visual regression",
    "screenshot",
    "screenshots",
    "スクリーンショット",
    "固定SS",
    "目視補助",
    "ユーザー検証",
    "user verification",
    "manual-only",
    "manual operation",
    "snapshot PNG",
)
COMPLETION_EVIDENCE_TERMS = (
    "完了根拠",
    "完了判定",
    "品質ゲート",
    "quality gate",
    "readiness",
    "DoD",
    "evidence",
    "proof",
    "verified",
    "検証",
    "回帰",
    "gate",
)
ALLOWED_NO_IMAGE_CONTEXTS = (
    "旧基準",
    "旧 ",
    "legacy",
    "old ",
    "archived",
    "履歴",
    "禁止",
    "拒否",
    "reject",
    "fail",
    "failure",
    "MUST NOT",
    "NOT be",
    "NOT use",
    "not accepted",
    "not by",
    "代替にしない",
    "にしない",
    "にはしない",
    "完了根拠にしない",
    "完了根拠にせず",
    "主根拠や",
    "Non-Goals",
    "受け入れない",
    "扱わない",
    "してはならない",
    "扱ってはならない",
    "不要",
)
FORBIDDEN_STORYBOOK_ROLE_TERMS = (
    "確認環境",
    "操作確認",
    "目視確認",
    "ユーザー検証",
    "部品カタログ",
)
FORBIDDEN_IMAGE_GATE_TERMS = (
    "--visual-snapshot",
    "storybook-visual-snapshot",
    "storybook-panel.png",
    "storybook-panel-bottom.png",
    "storybook-panel-modal-window.png",
    "SnapshotOutput::evidence",
    "assert_fresh_snapshot",
    "sentinel_text",
)
TRACEABILITY_REQUIREMENTS = (
    (
        "14",
        ROOT / "crates/katana-ui-core-storybook/tests/page_contract_materialization.rs",
        ("preview must show only the selected story", "GENERIC_PRESET_LABELS", "Settings"),
    ),
    (
        "15",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_button_tests.rs",
        ("clicked_button_updates_visible_button_body", "settings_update_selected_preview_body", "button_action_hit_rect"),
    ),
    (
        "16",
        ROOT / "crates/katana-ui-core-storybook/tests/context_menu_story_contract.rs",
        ("context_menu_open", "context_menu_opened", "navigation panel is missing"),
    ),
    (
        "17",
        ROOT / "crates/katana-ui-core-storybook/tests/storybook_panel_scroll_contract.rs",
        ("assert_independent_scroll_states", "vertical_scrollbar_visible", "content_height > scroll.viewport_height"),
    ),
    (
        "18",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_button_tests.rs",
        ("clicked_button_updates_visible_button_body", "button_layout_presets_change_button_body_size", "MIN_BUTTON_WIDTH"),
    ),
    (
        "19",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tests.rs",
        ("clicked_toggle_updates_visible_switch_body", "INPUT_PAGE", "SEARCH_PAGE"),
    ),
    (
        "20",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tests.rs",
        ("SELECT_BOX_PAGE", "SEGMENTED_PAGE", "clicked_operable_pages_update_preview_body"),
    ),
    (
        "21",
        ROOT / "crates/katana-ui-core/tests/atom_button_variant_contract.rs",
        ("UiButtonLayoutPreset", "UiButtonLayoutPatchDto", "custom_layout"),
    ),
    (
        "21",
        ROOT / "crates/katana-ui-core/tests/molecule_models/structured_contract.rs",
        ("line_style", "directory_icon", "toggle_trigger_area"),
    ),
    (
        "22",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tests.rs",
        ("COLOR_SWATCH_PAGE", "clicked_operable_pages_update_preview_body"),
    ),
    (
        "23",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tests.rs",
        ("TOOLTIP_PAGE", "POPOVER_PAGE", "clicked_operable_pages_update_preview_body"),
    ),
    (
        "24",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tests.rs",
        ("ACCORDION_PAGE", "SPLIT_PANE_PAGE", "MODAL_PAGE"),
    ),
    (
        "25",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tests.rs",
        ("COLOR_PICKER_PAGE", "CODE_DIFF_PAGE", "clicked_operable_pages_update_preview_body"),
    ),
    (
        "26",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tests.rs",
        ("BADGE_PAGE", "CARD_PAGE", "clicked_operable_pages_update_preview_body"),
    ),
    (
        "27",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tests.rs",
        ("settings_change_updates_passive_atom_preview_bodies", "THEME_PAGE", "PROGRESS_PAGE"),
    ),
    (
        "28",
        ROOT / "crates/katana-ui-core-storybook/src/catalog/types.rs",
        ("pub fn is_complete", "has_action_contract", "has_preset_contract"),
    ),
    (
        "29",
        ROOT / "openspec/changes/establish-kuc-atoms-molecules-catalog/specs/kuc-storybook-catalog/spec.md",
        ("interactive feedback surface", "all-components card grid", "panel-local scroll"),
    ),
    (
        "31.12",
        ROOT / "crates/katana-ui-core-storybook/tests/legacy_01_24_catalog_contract.rs",
        ("LEGACY_PAGE_COVERAGE", "legacy_01_to_24_each_have_option_action_event_state_preset", "preset_visual_changes"),
    ),
)
LEGACY_DOD_TRACE = {
    "01": ("01-theme-tokens", "legacy-01-theme-panel-theme"),
    "02": ("02-text", "legacy-02-text"),
    "03": ("03-icon", "legacy-03-icon"),
    "04": ("04-loading", "legacy-04-loading-dots"),
    "05": ("05-svg-button", "legacy-05-svg-button"),
    "06": ("06-text-button", "legacy-06-text-button"),
    "07": ("07-icon-text-button", "legacy-07-icon-text-button"),
    "08": ("08-toggle", "legacy-08-toggle"),
    "09": ("09-segmented-toggle", "legacy-09-segmented-toggle"),
    "10": ("10-select-box", "legacy-10-select-box"),
    "11": ("11-color-swatch", "legacy-11-color-swatch"),
    "12": ("12-text-input", "legacy-12-text-input"),
    "13": ("13-search-box", "legacy-13-search-box"),
    "14": ("14-tooltip", "legacy-14-tooltip"),
    "15": ("15-badge", "legacy-15-badge"),
    "16": ("16-key-cap", "legacy-16-key-cap"),
    "17": ("17-card", "legacy-17-card"),
    "18": ("18-accordion", "legacy-18-accordion"),
    "19": ("19-split-pane", "legacy-19-split-pane"),
    "20": ("20-modal-overlay", "legacy-20-modal"),
    "21": ("21-popover", "legacy-21-popover"),
    "22": ("22-rgba-color-picker", "legacy-22-color-picker"),
    "23": ("23-color-picker-parity", "legacy-23-color-picker-parity"),
    "24": ("24-code-diff", "legacy-24-code-diff"),
}


def missing_tokens(path: Path, tokens: tuple[str, ...]) -> list[str]:
    source = path.read_text(encoding="utf-8")
    return [f"{path.relative_to(ROOT)}: missing `{token}`" for token in tokens if token not in source]


def path_label(path: Path, root: Path = ROOT) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.as_posix()


def incomplete_task_line_failures(path: Path, source: str, root: Path = ROOT) -> list[str]:
    return [
        f"{path_label(path, root)}:{source[:match.start()].count(chr(10)) + 1}: {match.group(0)}"
        for match in INCOMPLETE_TASK.finditer(source)
    ]


def incomplete_task_failures(path: Path = TASKS, root: Path = ROOT) -> list[str]:
    source = path.read_text(encoding="utf-8")
    return incomplete_task_line_failures(path, source, root)


def is_release_track_change(name: str) -> bool:
    return name == CHANGE.name or bool(RELEASE_TRACK_CHANGE.match(name))


def release_track_task_files(root: Path = ROOT) -> list[Path]:
    changes = root / "openspec/changes"
    if not changes.exists():
        return []
    return [
        change / "tasks.md"
        for change in sorted(changes.iterdir())
        if change.is_dir() and is_release_track_change(change.name)
    ]


def release_track_task_failures(root: Path = ROOT) -> list[str]:
    failures: list[str] = []
    for path in release_track_task_files(root):
        if not path.exists():
            failures.append(f"{path_label(path, root)}: release-track tasks.md is missing")
            continue
        failures.extend(incomplete_task_failures(path, root))
    return failures


def dod_failures() -> list[str]:
    required = ("katana", "katana-chat-ui", "v0.1.0", "Storybook")
    failures: list[str] = []
    for path in (DESIGN, STORYBOOK_SPEC, QUALITY_SPEC, QUALITY_CONTRACT):
        failures.extend(missing_tokens(path, required))
    return failures


def line_has_policy_term(line: str) -> bool:
    lowered = line.lower()
    return any(term.lower() in lowered for term in NO_IMAGE_POLICY_TERMS)


def line_claims_completion_evidence(line: str) -> bool:
    lowered = line.lower()
    return any(term.lower() in lowered for term in COMPLETION_EVIDENCE_TERMS)


def line_is_allowed_history_or_rejection(line: str) -> bool:
    lowered = line.lower()
    if "ではなく" in line and line_has_policy_term(line.split("ではなく", maxsplit=1)[0]):
        return True
    return any(context.lower() in lowered for context in ALLOWED_NO_IMAGE_CONTEXTS)


def no_image_policy_failures(paths: tuple[Path, ...] = CANONICAL_FILES) -> list[str]:
    failures: list[str] = []
    for path in paths:
        source = path.read_text(encoding="utf-8")
        for line_number, line in enumerate(source.splitlines(), start=1):
            if (
                line_has_policy_term(line)
                and line_claims_completion_evidence(line)
                and not line_is_allowed_history_or_rejection(line)
            ):
                failures.append(
                    f"{path.relative_to(ROOT)}:{line_number}: no-image policy rejects completion evidence: {line.strip()}"
                )
    return failures


def line_rejects_forbidden_role(line: str) -> bool:
    if "ではなく" in line:
        before_rejection = line.split("ではなく", maxsplit=1)[0]
        if any(term in before_rejection for term in FORBIDDEN_STORYBOOK_ROLE_TERMS):
            return True
    return line_is_allowed_history_or_rejection(line)


def storybook_role_failures(paths: tuple[Path, ...] = REPOSITORY_POLICY_FILES) -> list[str]:
    failures: list[str] = []
    for path in paths:
        source = path.read_text(encoding="utf-8")
        for line_number, line in enumerate(source.splitlines(), start=1):
            if any(term in line for term in FORBIDDEN_STORYBOOK_ROLE_TERMS) and not line_rejects_forbidden_role(line):
                failures.append(
                    f"{path.relative_to(ROOT)}:{line_number}: Storybook role policy rejects this wording: {line.strip()}"
                )
    return failures


def image_gate_failures(paths: tuple[Path, ...] = GATE_FILES) -> list[str]:
    failures: list[str] = []
    for path in paths:
        source = path.read_text(encoding="utf-8")
        for line_number, line in enumerate(source.splitlines(), start=1):
            if path.name == "Justfile" and not line.strip().startswith("storybook-regression:"):
                continue
            for term in FORBIDDEN_IMAGE_GATE_TERMS:
                if term in line:
                    failures.append(
                        f"{path.relative_to(ROOT)}:{line_number}: no-image gate rejects `{term}` in release/storybook gate"
                    )
    return failures


def traceability_failures() -> list[str]:
    failures: list[str] = []
    for requirement_id, path, tokens in TRACEABILITY_REQUIREMENTS:
        source = path.read_text(encoding="utf-8")
        missing = [token for token in tokens if token not in source]
        for token in missing:
            failures.append(
                f"{path.relative_to(ROOT)}: requirement {requirement_id} traceability missing `{token}`"
            )
    return failures


def legacy_dod_ids(source: str) -> set[str]:
    return set(re.findall(r"^\|\s*(\d{2})\s*\|", source, re.MULTILINE))


def legacy_contract_tokens(source: str) -> set[str]:
    return set(re.findall(r'\("(\d{2}-[^"]+)",\s*&\[', source))


def legacy_gate_markers(source: str) -> set[str]:
    return set(re.findall(r"\blegacy-\d{2}-[a-z0-9-]+\b", source))


def legacy_dod_trace_failures(
    dod_source: str | None = None,
    contract_source: str | None = None,
    gate_source: str | None = None,
) -> list[str]:
    dod_source = LEGACY_DOD.read_text(encoding="utf-8") if dod_source is None else dod_source
    contract_source = (
        LEGACY_CATALOG_CONTRACT.read_text(encoding="utf-8")
        if contract_source is None
        else contract_source
    )
    gate_source = (
        STORYBOOK_REQUIREMENT_GATE.read_text(encoding="utf-8")
        if gate_source is None
        else gate_source
    )

    dod_ids = legacy_dod_ids(dod_source)
    contract_tokens = legacy_contract_tokens(contract_source)
    gate_markers = legacy_gate_markers(gate_source)
    failures: list[str] = []
    for legacy_id, (contract_token, gate_marker) in LEGACY_DOD_TRACE.items():
        if legacy_id not in dod_ids:
            failures.append(f"{LEGACY_DOD.relative_to(ROOT)}: legacy DoD row {legacy_id} is missing")
        if contract_token not in contract_tokens:
            failures.append(
                f"{LEGACY_CATALOG_CONTRACT.relative_to(ROOT)}: legacy DoD {legacy_id} missing `{contract_token}`"
            )
        if gate_marker not in gate_markers:
            failures.append(
                f"{STORYBOOK_REQUIREMENT_GATE.relative_to(ROOT)}: legacy DoD {legacy_id} missing `{gate_marker}`"
            )
    return failures


def self_test() -> int:
    allowed = (
        "旧基準では visual regression を使っていたが、現行基準では禁止する。",
        "screenshot storage MUST NOT be used as completion evidence.",
    )
    rejected = (
        "The quality gate MUST include visual regression as proof.",
        "画像回帰を完了根拠にする。",
    )
    role_allowed = (
        "Storybook は確認環境ではなく、フィードバック用の実画面である。",
        "ユーザー検証を完了根拠にする表現を拒否する。",
    )
    role_rejected = (
        "Storybook は操作確認と目視確認の場である。",
        "Storybook は部品カタログです。",
    )
    image_gate_rejected = "storybook-regression: storybook-visual-snapshot"
    legacy_trace_dod_source = "".join(f"| {legacy_id} | UI |\n" for legacy_id in LEGACY_DOD_TRACE)
    legacy_trace_contract_source = "".join(
        f'("{contract_token}", &["page"]),\n'
        for contract_token, _ in LEGACY_DOD_TRACE.values()
    )
    legacy_trace_gate_source = " ".join(gate_marker for _, gate_marker in LEGACY_DOD_TRACE.values())
    legacy_trace_good = legacy_dod_trace_failures(
        dod_source=legacy_trace_dod_source,
        contract_source=legacy_trace_contract_source,
        gate_source=legacy_trace_gate_source,
    )
    legacy_trace_bad = legacy_dod_trace_failures(
        dod_source=legacy_trace_dod_source.replace("| 02 | UI |\n", ""),
        contract_source=legacy_trace_contract_source.replace('("02-text", &["page"]),\n', ""),
        gate_source=legacy_trace_gate_source.replace(" legacy-02-text", ""),
    )
    with tempfile.TemporaryDirectory() as tmp:
        active_root = Path(tmp)
        active_task = active_root / "openspec/changes/01-add-context-menu/tasks.md"
        active_task.parent.mkdir(parents=True)
        active_task.write_text("- [ ] 1. 未完了 task\n", encoding="utf-8")
        archived_task = active_root / "openspec/changes/archive/01-old/tasks.md"
        archived_task.parent.mkdir(parents=True)
        archived_task.write_text("- [ ] 1. archive task\n", encoding="utf-8")
        active_task_bad = release_track_task_failures(active_root)
    with tempfile.TemporaryDirectory() as tmp:
        missing_root = Path(tmp)
        (missing_root / "openspec/changes/02-add-drag-drop-primitive").mkdir(parents=True)
        active_task_missing = release_track_task_failures(missing_root)
    allowed_failed = [
        line
        for line in allowed
        if line_has_policy_term(line)
        and line_claims_completion_evidence(line)
        and not line_is_allowed_history_or_rejection(line)
    ]
    rejected_passed = [
        line
        for line in rejected
        if not (
            line_has_policy_term(line)
            and line_claims_completion_evidence(line)
            and not line_is_allowed_history_or_rejection(line)
        )
    ]
    role_allowed_failed = [
        line
        for line in role_allowed
        if any(term in line for term in FORBIDDEN_STORYBOOK_ROLE_TERMS) and not line_rejects_forbidden_role(line)
    ]
    role_rejected_passed = [
        line
        for line in role_rejected
        if not (any(term in line for term in FORBIDDEN_STORYBOOK_ROLE_TERMS) and not line_rejects_forbidden_role(line))
    ]
    image_gate_rejected_passed = not any(term in image_gate_rejected for term in FORBIDDEN_IMAGE_GATE_TERMS)
    legacy_trace_good_failed = bool(legacy_trace_good)
    legacy_trace_bad_passed = not any("02" in line for line in legacy_trace_bad)
    active_task_bad_passed = len(active_task_bad) != 1 or "archive" in active_task_bad[0]
    active_task_missing_passed = not any("tasks.md is missing" in line for line in active_task_missing)
    if (
        allowed_failed
        or rejected_passed
        or role_allowed_failed
        or role_rejected_passed
        or image_gate_rejected_passed
        or legacy_trace_good_failed
        or legacy_trace_bad_passed
        or active_task_bad_passed
        or active_task_missing_passed
    ):
        print("KUC release readiness self-test failed", file=sys.stderr)
        for line in allowed_failed:
            print(f"- allowed line rejected: {line}", file=sys.stderr)
        for line in rejected_passed:
            print(f"- rejected line allowed: {line}", file=sys.stderr)
        for line in role_allowed_failed:
            print(f"- allowed Storybook role line rejected: {line}", file=sys.stderr)
        for line in role_rejected_passed:
            print(f"- rejected Storybook role line allowed: {line}", file=sys.stderr)
        if image_gate_rejected_passed:
            print("- rejected image gate line allowed", file=sys.stderr)
        if legacy_trace_good_failed:
            print("- valid legacy DoD trace rejected", file=sys.stderr)
        if legacy_trace_bad_passed:
            print("- invalid legacy DoD trace allowed", file=sys.stderr)
        if active_task_bad_passed:
            print("- active release-track incomplete task allowed", file=sys.stderr)
        if active_task_missing_passed:
            print("- active release-track missing tasks.md allowed", file=sys.stderr)
        return 1
    return 0


def main() -> int:
    if "--self-test" in sys.argv:
        return self_test()
    failures = release_track_task_failures()
    failures.extend(dod_failures())
    failures.extend(no_image_policy_failures(REPOSITORY_POLICY_FILES))
    failures.extend(storybook_role_failures())
    failures.extend(image_gate_failures())
    failures.extend(traceability_failures())
    failures.extend(legacy_dod_trace_failures())
    if failures:
        print("KUC v0.1.0 release readiness failed", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
