#!/usr/bin/env python3
from pathlib import Path
import json
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
STORYBOOK_INTERACTION_MANIFEST = ROOT / "docs/storybook-77ui-interaction-manifest.json"
STORYBOOK_DEEP_AUDIT_LEDGER = ROOT / "docs/storybook-77ui-deep-audit-ledger.md"
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
CONSUMER_APP_CARGO = ROOT / "examples/kuc-consumer-app/Cargo.toml"
CONSUMER_APP_LIB = ROOT / "examples/kuc-consumer-app/src/lib.rs"
CONSUMER_APP_FIXTURES = ROOT / "examples/kuc-consumer-app/src/fixtures.rs"
CONSUMER_APP_TESTS = ROOT / "examples/kuc-consumer-app/src/tests.rs"
CONSUMER_APP_MAIN = ROOT / "examples/kuc-consumer-app/src/main.rs"

INCOMPLETE_TASK = re.compile(r"^- \[ \] .+", re.MULTILINE)
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
        "18",
        ROOT / "crates/katana-ui-core/tests/atom_button_variant_contract.rs",
        ("button_atom_variants_default_to_pointer_cursor", "button_layout_label_align_center_is_part_of_core_dto_contract", "UiCursor::Pointer"),
    ),
    (
        "18",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_button_hover_tests.rs",
        ("hover_draws_visible_border_for_all_button_surfaces", "hover_border", "must not use text color"),
    ),
    (
        "18",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_button_center_tests.rs",
        ("button_label_center_uses_measured_text_width", "measure_button_label_width", "centered_label_x_for_test"),
    ),
    (
        "18",
        ROOT / "crates/katana-ui-core-storybook/src/visual/window_interaction/tests/button_operation_tests.rs",
        ("BUTTON_FAMILY_CURSOR_PAGES", "StorybookCursorStyle::PointingHand", "\"menu-button\""),
    ),
    (
        "18",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_menu_button_tests.rs",
        ("menu_button_hover_draws_shared_button_family_border_token", "hover_border", "ThemeSnapshot::dark"),
    ),
    (
        "19",
        ROOT / "crates/katana-ui-core-storybook/src/visual/visual_interaction_tests.rs",
        ("clicked_toggle_updates_visible_row_and_switch_body", "INPUT_PAGE", "SEARCH_PAGE"),
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
STORYBOOK_MANIFEST_REQUIRED_ARRAYS = (
    "public_props_options",
    "state",
    "action",
    "event",
    "callback",
    "required_operations",
    "evidence",
)
STORYBOOK_MANIFEST_REQUIRED_TEST_KEYS = (
    "window_interaction",
    "visual_interaction",
    "guard",
)
STORYBOOK_MANIFEST_OPERATION_KINDS = (
    "pointer",
    "keyboard",
    "scroll",
    "drag",
    "context_menu",
    "focus",
    "hover",
    "resize",
    "timed_tick",
)


def missing_tokens(path: Path, tokens: tuple[str, ...]) -> list[str]:
    source = path.read_text(encoding="utf-8")
    return [f"{path.relative_to(ROOT)}: missing `{token}`" for token in tokens if token not in source]


def path_label(path: Path, root: Path = ROOT) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.as_posix()


def storybook_effective_value(
    manifest: dict,
    entry: dict,
    key: str,
) -> object:
    value = entry.get(key)
    if value:
        return value
    engine = entry.get("engine")
    defaults = manifest.get("defaults_by_engine", {})
    if not isinstance(defaults, dict) or not isinstance(engine, str):
        return value
    engine_defaults = defaults.get(engine, {})
    if not isinstance(engine_defaults, dict):
        return value
    return engine_defaults.get(key, value)


def storybook_effective_list_failures(
    manifest_path: Path,
    root: Path,
    manifest: dict,
    entry: dict,
    page: str,
) -> list[str]:
    failures: list[str] = []
    for key in STORYBOOK_MANIFEST_REQUIRED_ARRAYS:
        value = storybook_effective_value(manifest, entry, key)
        if not isinstance(value, list) or not value:
            failures.append(
                f"{path_label(manifest_path, root)}: Storybook page `{page}` "
                f"must have effective `{key}`"
            )
    operations = storybook_effective_value(manifest, entry, "required_operations")
    if isinstance(operations, list):
        declared = set(manifest.get("operation_kinds", STORYBOOK_MANIFEST_OPERATION_KINDS))
        unknown = sorted(str(operation) for operation in operations if operation not in declared)
        if unknown:
            failures.append(
                f"{path_label(manifest_path, root)}: Storybook page `{page}` "
                f"has unknown required operation(s): {', '.join(unknown)}"
            )
    return failures


def storybook_effective_test_failures(
    manifest_path: Path,
    root: Path,
    manifest: dict,
    entry: dict,
    page: str,
) -> list[str]:
    tests = storybook_effective_value(manifest, entry, "tests")
    if not isinstance(tests, dict):
        return [
            f"{path_label(manifest_path, root)}: Storybook page `{page}` "
            "must have effective `tests`"
        ]
    failures: list[str] = []
    for key in STORYBOOK_MANIFEST_REQUIRED_TEST_KEYS:
        value = tests.get(key)
        if not isinstance(value, list) or not value:
            failures.append(
                f"{path_label(manifest_path, root)}: Storybook page `{page}` "
                f"must have effective `tests.{key}`"
            )
    return failures


def storybook_release_gate_failures(
    root: Path = ROOT,
    manifest_source: str | None = None,
    ledger_source: str | None = None,
) -> list[str]:
    manifest_path = root / "docs/storybook-77ui-interaction-manifest.json"
    ledger_path = root / "docs/storybook-77ui-deep-audit-ledger.md"
    failures: list[str] = []

    if manifest_source is None:
        if not manifest_path.exists():
            failures.append(f"{path_label(manifest_path, root)}: Storybook interaction manifest is missing")
            return failures
        manifest_source = manifest_path.read_text(encoding="utf-8")

    try:
        manifest = json.loads(manifest_source)
    except json.JSONDecodeError as error:
        failures.append(
            f"{path_label(manifest_path, root)}:{error.lineno}: Storybook interaction manifest is invalid JSON"
        )
        return failures

    pages = manifest.get("ui") if isinstance(manifest, dict) else None
    if not isinstance(pages, list):
        failures.append(f"{path_label(manifest_path, root)}: Storybook interaction manifest missing `ui` list")
        pages = []
    if len(pages) != 77:
        failures.append(
            f"{path_label(manifest_path, root)}: Storybook interaction manifest must cover 77 UI pages, found {len(pages)}"
        )

    seen_pages: set[str] = set()
    for index, entry in enumerate(pages, start=1):
        if not isinstance(entry, dict):
            failures.append(
                f"{path_label(manifest_path, root)}: Storybook manifest row {index} is not an object"
            )
            continue
        page = str(entry.get("page") or f"<missing:{index}>")
        if page in seen_pages:
            failures.append(f"{path_label(manifest_path, root)}: duplicate Storybook page `{page}`")
        seen_pages.add(page)
        audit_status = entry.get("audit_status")
        if audit_status != "verified":
            failures.append(
                f"{path_label(manifest_path, root)}: Storybook page `{page}` has audit_status `{audit_status}`, not `verified`"
            )
        failures.extend(
            storybook_effective_list_failures(manifest_path, root, manifest, entry, page)
        )
        failures.extend(
            storybook_effective_test_failures(manifest_path, root, manifest, entry, page)
        )
        gaps = entry.get("gaps") or []
        if not isinstance(gaps, list):
            failures.append(f"{path_label(manifest_path, root)}: Storybook page `{page}` gaps must be a list")
            continue
        for gap in gaps:
            if "manual_acceptance_pending" in str(gap):
                failures.append(
                    f"{path_label(manifest_path, root)}: Storybook page `{page}` still has `manual_acceptance_pending` gap"
                )

    if ledger_source is None:
        if not ledger_path.exists():
            failures.append(f"{path_label(ledger_path, root)}: Storybook deep audit ledger is missing")
            return failures
        ledger_source = ledger_path.read_text(encoding="utf-8")

    ledger_rows = 0
    for line_number, line in enumerate(ledger_source.splitlines(), start=1):
        if not line.startswith("|"):
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if len(cells) < 6 or cells[0] in {"No", "---:"}:
            continue
        if not re.match(r"^\d{2}[a-z]?$", cells[0]):
            continue
        ledger_rows += 1
        status = cells[-1]
        ui_name = cells[1]
        if status != "実証済み":
            failures.append(
                f"{path_label(ledger_path, root)}:{line_number}: Storybook ledger row `{ui_name}` is `{status}`, not `実証済み`"
            )
    if ledger_rows < 77:
        failures.append(
            f"{path_label(ledger_path, root)}: Storybook ledger must contain at least 77 audited UI rows, found {ledger_rows}"
        )

    return failures


def incomplete_task_line_failures(path: Path, source: str, root: Path = ROOT) -> list[str]:
    return [
        f"{path_label(path, root)}:{source[:match.start()].count(chr(10)) + 1}: {match.group(0)}"
        for match in INCOMPLETE_TASK.finditer(source)
    ]


def incomplete_task_failures(path: Path = TASKS, root: Path = ROOT) -> list[str]:
    source = path.read_text(encoding="utf-8")
    return incomplete_task_line_failures(path, source, root)


def is_release_track_change(name: str) -> bool:
    return (
        name == CHANGE.name
        or name.startswith("storybook-page-")
        or bool(RELEASE_TRACK_CHANGE.match(name))
    )


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


def consumer_app_failures(root: Path = ROOT) -> list[str]:
    root_cargo = root / "Cargo.toml"
    justfile = root / "Justfile"
    cargo = root / "examples/kuc-consumer-app/Cargo.toml"
    lib = root / "examples/kuc-consumer-app/src/lib.rs"
    fixtures = root / "examples/kuc-consumer-app/src/fixtures.rs"
    tests = root / "examples/kuc-consumer-app/src/tests.rs"
    main = root / "examples/kuc-consumer-app/src/main.rs"
    src_dir = root / "examples/kuc-consumer-app/src"
    failures: list[str] = []
    for path in (justfile, cargo, lib, fixtures, tests, main):
        if not path.exists():
            failures.append(f"{path_label(path, root)}: consumer app file is missing")
    if failures:
        return failures

    root_source = root_cargo.read_text(encoding="utf-8")
    justfile_source = justfile.read_text(encoding="utf-8")
    cargo_source = cargo.read_text(encoding="utf-8")
    combined = "\n".join(
        path.read_text(encoding="utf-8") for path in sorted(src_dir.glob("*.rs"))
    )
    required_root_tokens = ('"examples/kuc-consumer-app"',)
    required_justfile_tokens = (
        "consumer-app-contract:",
        "test -p kuc-consumer-app --locked",
        "test -p katana-ui-core --test generic_rust_app_contract --locked",
        "test -p katana-ui-core --test generic_rust_app_layout_contract --locked",
        "test -p katana-ui-core --test generic_rust_app_action_contract --locked",
        "integration-test: consumer-app-contract",
        "e2e-test:",
        "bash scripts/storybook-requirement-gate.sh",
        "smoke-test: storybook-smoke storybook-interaction-smoke",
        "kuc-guardrails: consumer-app-contract",
        "release-readiness-check: integration-test e2e-test smoke-test",
    )
    required_cargo_tokens = ("katana-ui-core.workspace = true",)
    required_source_tokens = (
        "ComponentTree::new",
        "Panel::new",
        "SplitPane::new",
        "ScrollArea::new",
        "CloseableTabStrip::new",
        "Input::new",
        "TextArea::new",
        "SelectionList::new",
        "Toolbar::new",
        "apply_action",
        "UiAction::invoke_callback",
        "invoke_search_callback",
        "add_fixed_tab",
        "CloseableTabStripAction::AddTab",
        "CloseableTabStripAction::CloseOthers",
        "CloseableTabStripAction::CloseToRight",
        "CloseableTabStripAction::CloseToLeft",
        "CloseableTabStripAction::CloseAll",
        "CloseableTabStripAction::PinTab",
        "CloseableTabStripAction::MoveToGroup",
        "CloseableTabStripEvent::TabPinChanged",
        "CloseableTabStripEvent::TabGroupChanged",
        "CloseableTabStripEvent::GroupCreated",
        "CloseableTabContextCommand::CloseOthers",
        "consumer_app_handles_input_textarea_scroll_split_and_tabs",
        "consumer_app_handles_workspace_tab_bulk_actions",
        "consumer_app_keeps_endpoint_closes_noop_and_non_closeable_tabs",
        "consumer_app_handles_workspace_tab_context_commands",
        "consumer_app_observes_tab_pin_and_group_events",
    )
    failures.extend(
        f"Cargo.toml: consumer app workspace member missing `{token}`"
        for token in required_root_tokens
        if token not in root_source
    )
    failures.extend(
        f"Justfile: consumer app contract recipe missing `{token}`"
        for token in required_justfile_tokens
        if token not in justfile_source
    )
    failures.extend(
        f"{path_label(cargo, root)}: consumer app dependency missing `{token}`"
        for token in required_cargo_tokens
        if token not in cargo_source
    )
    for forbidden in ("katana-ui-core-storybook",):
        if forbidden in cargo_source:
            failures.append(
                f"{path_label(cargo, root)}: consumer app must depend only on katana-ui-core, not `{forbidden}`"
            )
    failures.extend(
        f"examples/kuc-consumer-app: consumer app contract missing `{token}`"
        for token in required_source_tokens
        if token not in combined
    )
    return failures


def adapter_consumer_app_failures(root: Path = ROOT) -> list[str]:
    return []


def storybook_requirement_gate_command_failures(root: Path = ROOT) -> list[str]:
    failures: list[str] = []
    required_commands = (
        (
            root / "scripts/storybook-requirement-gate.sh",
            "cargo rustc --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings",
        ),
        (
            root / "scripts/storybook-headless-smoke.sh",
            "cargo rustc -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings",
        ),
        (
            root / "scripts/assert-menu-button-contract.sh",
            '"${CARGO_CMD[@]}" rustc -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings',
        ),
    )
    forbidden = (
        'RUSTFLAGS="-D warnings" cargo build',
        "RUSTFLAGS='-D warnings' cargo build",
        'RUSTFLAGS="-D warnings" "${CARGO_CMD[@]}" build',
        "RUSTFLAGS='-D warnings' \"${CARGO_CMD[@]}\" build",
    )
    for gate, command in required_commands:
        if not gate.exists():
            failures.append(f"{path_label(gate, root)}: Storybook gate script is missing")
            continue
        source = gate.read_text(encoding="utf-8")
        if command not in source:
            failures.append(
                f"{path_label(gate, root)}: Storybook gate must deny warnings on the Storybook target with `{command}`"
            )
        failures.extend(
            f"{path_label(gate, root)}: Storybook gate must not apply KUC warning policy to path dependencies with `{token}`"
            for token in forbidden
            if token in source
        )
    return failures


def justfile_fmt_scope_failures(root: Path = ROOT) -> list[str]:
    justfile = root / "Justfile"
    if not justfile.exists():
        return [f"{path_label(justfile, root)}: Justfile is missing"]
    source = justfile.read_text(encoding="utf-8")
    required = (
        'KUC_FORMAT_PACKAGES := "-p katana-ui-core -p katana-ui-core-text-raster -p katana-ui-core-svg-raster -p katana-ui-core-egui-adapter -p katana-ui-core-storybook -p kuc-consumer-app"',
        "{{CARGO}} fmt {{KUC_FORMAT_PACKAGES}}",
        "{{CARGO}} fmt {{KUC_FORMAT_PACKAGES}} -- --check",
    )
    forbidden = (
        "{{CARGO}} fmt --all",
        "{{CARGO}} fmt --workspace",
        "cargo fmt --all",
        "cargo fmt --workspace",
    )
    failures = [
        f"{path_label(justfile, root)}: fmt gate must scope formatting to KUC workspace packages with `{token}`"
        for token in required
        if token not in source
    ]
    failures.extend(
        f"{path_label(justfile, root)}: fmt gate must not format path dependencies with `{token}`"
        for token in forbidden
        if token in source
    )
    return failures


def justfile_lint_scope_failures(root: Path = ROOT) -> list[str]:
    justfile = root / "Justfile"
    if not justfile.exists():
        return [f"{path_label(justfile, root)}: Justfile is missing"]
    source = justfile.read_text(encoding="utf-8")
    required = (
        'KUC_WORKSPACE_PACKAGES := "-p katana-ui-core -p katana-ui-core-storybook -p kuc-consumer-app"',
        "{{CARGO}} clippy -j {{JOBS}} {{KUC_WORKSPACE_PACKAGES}} --all-targets --all-features --locked -- -D warnings",
    )
    forbidden = (
        'RUSTFLAGS="-D warnings" {{CARGO}} clippy',
        "RUSTFLAGS='-D warnings' {{CARGO}} clippy",
        "{{CARGO}} clippy -j {{JOBS}} --workspace",
        "cargo clippy --workspace",
    )
    failures = [
        f"{path_label(justfile, root)}: lint gate must scope Clippy to KUC workspace packages with `{token}`"
        for token in required
        if token not in source
    ]
    failures.extend(
        f"{path_label(justfile, root)}: lint gate must not deny warnings for path dependencies with `{token}`"
        for token in forbidden
        if token in source
    )
    return failures


def justfile_test_scope_failures(root: Path = ROOT) -> list[str]:
    justfile = root / "Justfile"
    if not justfile.exists():
        return [f"{path_label(justfile, root)}: Justfile is missing"]
    source = justfile.read_text(encoding="utf-8")
    justfile_required = (
        "{{CARGO}} test {{KUC_WORKSPACE_PACKAGES}} --all-targets --all-features --locked",
        "bash scripts/run-strict-coverage.sh",
        "{{CARGO}} test {{KUC_WORKSPACE_PACKAGES}} --all-targets --locked",
        "coverage: fmt-check ast-lint\n    just coverage-container",
        "coverage-iterate: fmt-check ast-lint\n    just coverage-container-iterate",
        "KUC_COVERAGE_REUSE=1 CARGO=\"{{CARGO}}\" bash scripts/run-strict-coverage.sh",
        "check: fmt-check ast-lint check-types lint unit-test",
        "release-check: release-target-check fmt-check ast-lint release-readiness-check release-verify",
    )
    justfile_forbidden = (
        "{{CARGO}} test --workspace",
        "{{CARGO}} llvm-cov --workspace",
        'RUSTFLAGS="-D warnings" cargo test --workspace',
        "cargo test --workspace",
    )
    failures = [
        f"{path_label(justfile, root)}: Rust test gate must scope execution to KUC workspace packages with `{token}`"
        for token in justfile_required
        if token not in source
    ]
    failures.extend(
        f"{path_label(justfile, root)}: Rust test gate must not execute path dependency tests with `{token}`"
        for token in justfile_forbidden
        if token in source
    )
    coverage_script = root / "scripts" / "run-strict-coverage.sh"
    if not coverage_script.exists():
        failures.append(
            f"{path_label(coverage_script, root)}: strict coverage script is missing"
        )
        return failures
    coverage_source = coverage_script.read_text(encoding="utf-8")
    coverage_required = (
        "-p katana-ui-core",
        "-p katana-ui-core-storybook",
        "-p kuc-consumer-app",
        "--all-targets",
        "--include-ignored",
        "export CARGO_PROFILE_TEST_OPT_LEVEL=0",
        'run_cargo clean --target-dir "$coverage_target_dir"',
        'coverage_reuse="${KUC_COVERAGE_REUSE:-0}"',
        "run_cargo llvm-cov clean --profraw-only",
        'coverage_min_free_gib="${KUC_COVERAGE_MIN_FREE_GIB:-2}"',
        'df -Pk "${CARGO_TARGET_DIR:-target}"',
        "run_cargo llvm-cov report",
        "--fail-under-functions 100",
        "--fail-under-lines 100",
        "--fail-uncovered-functions 0",
        "--fail-uncovered-lines 0",
    )
    failures.extend(
        f"{path_label(coverage_script, root)}: strict coverage gate must include `{token}`"
        for token in coverage_required
        if token not in coverage_source
    )
    return failures


def justfile_storybook_command_scope_failures(root: Path = ROOT) -> list[str]:
    justfile = root / "Justfile"
    if not justfile.exists():
        return [f"{path_label(justfile, root)}: Justfile is missing"]
    source = justfile.read_text(encoding="utf-8")
    required = (
        "{{CARGO}} rustc --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings",
        "{{CARGO}} rustc -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings",
        "{{CARGO}} rustc -p katana-ui-core-storybook --lib --locked -- -D warnings",
        "{{CARGO}} run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 0",
        "{{CARGO}} run -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked",
        "{{CARGO}} run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-modal-window 0",
        "{{CARGO}} run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --visual-snapshot target/storybook-panel.png",
    )
    forbidden = (
        'RUSTFLAGS="-D warnings" {{CARGO}} run',
        'RUSTFLAGS="-D warnings" {{CARGO}} check',
        'RUSTFLAGS="-D warnings" cargo run',
        'RUSTFLAGS="-D warnings" cargo check',
    )
    failures = [
        f"{path_label(justfile, root)}: Storybook command must deny warnings on the Storybook target with `{token}`"
        for token in required
        if token not in source
    ]
    failures.extend(
        f"{path_label(justfile, root)}: Storybook command must not deny warnings for path dependencies with `{token}`"
        for token in forbidden
        if token in source
    )
    return failures


def viewer_consumer_event_contract_failures(root: Path = ROOT) -> list[str]:
    required_sources = (
        (
            root / "crates/katana-ui-core/src/render_model/host_action_types.rs",
            (
                "pub struct UiHostActionPlan",
                "pub action_id: String",
                "pub enabled: bool",
                "ui.link.open",
                "ui.disclosure.",
                "ui.image.highlight",
            ),
        ),
        (
            root / "crates/katana-ui-core/src/render_model/host_action_plan.rs",
            (
                "pub fn collect_from_tree",
                "push_context_menu_item_plans",
            ),
        ),
        (
            root / "crates/katana-ui-core/src/render_model/common.rs",
            ("pub host_actions: Vec<UiHostActionSpec>",),
        ),
        (
            root / "crates/katana-ui-core/src/render_model/mod.rs",
            ("UiHostActionPlan", "UiHostActionSpec"),
        ),
        (
            root / "crates/katana-ui-core/tests/host_action_plan_contract.rs",
            (
                "generic_host_action_plan_collects_action_ids_and_enabled_state",
                "app.toolbar.",
                "ui.surface.",
                "UI_IMAGE_HIGHLIGHT_ACTION_ID",
            ),
        ),
    )
    failures: list[str] = []
    for path, tokens in required_sources:
        if not path.exists():
            failures.append(f"{path_label(path, root)}: viewer consumer event contract file is missing")
            continue
        source = path.read_text(encoding="utf-8")
        failures.extend(
            f"{path_label(path, root)}: viewer consumer event contract missing `{token}`"
            for token in tokens
            if token not in source
        )

    return failures


def preset_tab_scroll_contract_failures(root: Path = ROOT) -> list[str]:
    required_sources = (
        (
            root / "crates/katana-ui-core-storybook/src/visual/preset_tab_scroll.rs",
            (
                "pub(super) fn viewport_rect",
                "pub(super) fn max_scroll_x_for_page",
                "pub(super) fn ensure_index_visible",
                "pub(super) fn hit_index_at",
                "visible_index_range",
                "visual_rect_for_index",
            ),
        ),
        (
            root / "crates/katana-ui-core-storybook/src/visual/preset_tab_label.rs",
            (
                "pub(super) fn fit",
                "TRUNCATION_MARKER",
                "measured_width_for_test",
            ),
        ),
        (
            root / "crates/katana-ui-core-storybook/src/visual/visual_preset_tab_scroll_tests.rs",
            (
                "overflowing_preset_tabs_have_horizontal_scroll_range",
                "rendered_preset_tabs_are_clipped_at_preview_right_edge",
                "external_preset_selection_scrolls_current_tab_into_view",
                "clicking_scrolled_preset_tab_uses_logical_tab_index",
                "preset_tab_hit_bounds_reject_gap_and_clipped_edges",
            ),
        ),
        (
            root / "crates/katana-ui-core-storybook/src/visual/visual_tests.rs",
            (
                "every_required_page_preset_tab_labels_fit_clip_width",
                "StoryRequirements::required_pages()",
                "measured_width_for_test",
            ),
        ),
    )
    failures: list[str] = []
    for path, tokens in required_sources:
        if not path.exists():
            failures.append(f"{path_label(path, root)}: preset tab scroll contract file is missing")
            continue
        source = path.read_text(encoding="utf-8")
        failures.extend(
            f"{path_label(path, root)}: preset tab scroll contract missing `{token}`"
            for token in tokens
            if token not in source
        )
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
        storybook_task = active_root / "openspec/changes/storybook-page-text/tasks.md"
        storybook_task.parent.mkdir(parents=True)
        storybook_task.write_text("- [ ] 1. Storybook 未完了 task\n", encoding="utf-8")
        feedback_task = active_root / "openspec/changes/02-add-drag-drop-primitive/tasks.md"
        feedback_task.parent.mkdir(parents=True)
        feedback_task.write_text("- [/] 1. 対応完了 feedback\n", encoding="utf-8")
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
    active_task_bad_passed = (
        len(active_task_bad) != 2
        or "archive" in active_task_bad[0]
        or not any("storybook-page-text" in line for line in active_task_bad)
        or any("02-add-drag-drop-primitive" in line for line in active_task_bad)
    )
    active_task_missing_passed = not any("tasks.md is missing" in line for line in active_task_missing)
    with tempfile.TemporaryDirectory() as tmp:
        consumer_root = Path(tmp)
        write_consumer_app_self_test_files(consumer_root, include_member=True)
        consumer_good_failed = bool(consumer_app_failures(consumer_root))
    with tempfile.TemporaryDirectory() as tmp:
        consumer_root = Path(tmp)
        write_consumer_app_self_test_files(consumer_root, include_member=False)
        consumer_bad_passed = not any(
            "workspace member missing" in line for line in consumer_app_failures(consumer_root)
        )
    with tempfile.TemporaryDirectory() as tmp:
        consumer_root = Path(tmp)
        write_consumer_app_self_test_files(
            consumer_root,
            include_member=True,
            include_dynamic_actions=False,
        )
        consumer_dynamic_bad_passed = not any(
            "UiAction::invoke_callback" in line for line in consumer_app_failures(consumer_root)
        )
    with tempfile.TemporaryDirectory() as tmp:
        gate_root = Path(tmp)
        write_storybook_requirement_gate_self_test_files(gate_root, use_target_rustc=True)
        requirement_gate_good_failed = bool(
            storybook_requirement_gate_command_failures(gate_root)
        )
    with tempfile.TemporaryDirectory() as tmp:
        gate_root = Path(tmp)
        write_storybook_requirement_gate_self_test_files(gate_root, use_target_rustc=False)
        requirement_gate_bad_passed = not any(
            "must not apply KUC warning policy to path dependencies" in line
            for line in storybook_requirement_gate_command_failures(gate_root)
        )
    with tempfile.TemporaryDirectory() as tmp:
        fmt_root = Path(tmp)
        write_justfile_fmt_scope_self_test_file(fmt_root, use_scoped_packages=True)
        justfile_fmt_good_failed = bool(justfile_fmt_scope_failures(fmt_root))
    with tempfile.TemporaryDirectory() as tmp:
        fmt_root = Path(tmp)
        write_justfile_fmt_scope_self_test_file(fmt_root, use_scoped_packages=False)
        justfile_fmt_bad_passed = not any(
            "must not format path dependencies" in line
            for line in justfile_fmt_scope_failures(fmt_root)
        )
    with tempfile.TemporaryDirectory() as tmp:
        lint_root = Path(tmp)
        write_justfile_lint_scope_self_test_file(lint_root, use_scoped_packages=True)
        justfile_lint_good_failed = bool(justfile_lint_scope_failures(lint_root))
    with tempfile.TemporaryDirectory() as tmp:
        lint_root = Path(tmp)
        write_justfile_lint_scope_self_test_file(lint_root, use_scoped_packages=False)
        justfile_lint_bad_passed = not any(
            "must not deny warnings for path dependencies" in line
            for line in justfile_lint_scope_failures(lint_root)
        )
    with tempfile.TemporaryDirectory() as tmp:
        test_root = Path(tmp)
        write_justfile_test_scope_self_test_file(test_root, use_scoped_packages=True)
        justfile_test_good_failed = bool(justfile_test_scope_failures(test_root))
    with tempfile.TemporaryDirectory() as tmp:
        test_root = Path(tmp)
        write_justfile_test_scope_self_test_file(test_root, use_scoped_packages=False)
        justfile_test_bad_passed = not any(
            "must not execute path dependency tests" in line
            for line in justfile_test_scope_failures(test_root)
        )
    with tempfile.TemporaryDirectory() as tmp:
        storybook_root = Path(tmp)
        write_justfile_storybook_command_self_test_file(
            storybook_root,
            use_target_rustc=True,
        )
        justfile_storybook_good_failed = bool(
            justfile_storybook_command_scope_failures(storybook_root)
        )
    with tempfile.TemporaryDirectory() as tmp:
        storybook_root = Path(tmp)
        write_justfile_storybook_command_self_test_file(
            storybook_root,
            use_target_rustc=False,
        )
        justfile_storybook_bad_passed = not any(
            "must not deny warnings for path dependencies" in line
            for line in justfile_storybook_command_scope_failures(storybook_root)
        )
    with tempfile.TemporaryDirectory() as tmp:
        preset_root = Path(tmp)
        write_preset_tab_scroll_self_test_files(preset_root, include_hit_bounds=True)
        preset_tab_good_failed = bool(preset_tab_scroll_contract_failures(preset_root))
    with tempfile.TemporaryDirectory() as tmp:
        preset_root = Path(tmp)
        write_preset_tab_scroll_self_test_files(preset_root, include_hit_bounds=False)
        preset_tab_bad_passed = not any(
            "preset_tab_hit_bounds_reject_gap_and_clipped_edges" in line
            for line in preset_tab_scroll_contract_failures(preset_root)
        )
    storybook_good_manifest = json.dumps(
        {
            "operation_kinds": list(STORYBOOK_MANIFEST_OPERATION_KINDS),
            "defaults_by_engine": {
                "clickable": {
                    "public_props_options": ["source:storybook options"],
                    "state": ["clickable_state"],
                    "action": ["clickable_action"],
                    "event": ["clickable_event"],
                    "callback": ["clickable_callback"],
                    "required_operations": ["pointer", "keyboard"],
                    "evidence": ["contract evidence"],
                    "tests": {
                        "window_interaction": ["shared:window_interaction"],
                        "visual_interaction": ["shared:visual_interaction"],
                        "guard": ["shared:guard"],
                    },
                }
            },
            "ui": [
                {
                    "page": f"page-{index:02}",
                    "engine": "clickable",
                    "audit_status": "verified",
                    "gaps": [],
                }
                for index in range(1, 78)
            ]
        },
        ensure_ascii=False,
    )
    storybook_good_ledger = "\n".join(
        [
            "| No | UI | 不足 | あるべき姿 | 設計/階層監査観点 | 現判定 |",
            "| ---: | --- | --- | --- | --- | --- |",
            *(
                f"| {index:02} | page-{index:02} | done | expected | design | 実証済み |"
                for index in range(1, 78)
            ),
        ]
    )
    storybook_bad_manifest = json.dumps(
        {
            "operation_kinds": list(STORYBOOK_MANIFEST_OPERATION_KINDS),
            "ui": [
                {
                    "page": "text" if index == 1 else f"page-{index:02}",
                    "engine": "clickable",
                    "audit_status": "partial" if index == 1 else "verified",
                    "gaps": [
                        "manual_acceptance_pending: Storybook user confirmation is required"
                    ]
                    if index == 1
                    else [],
                }
                for index in range(1, 78)
            ]
        },
        ensure_ascii=False,
    )
    storybook_bad_ledger = storybook_good_ledger.replace(
        "| 01 | page-01 | done | expected | design | 実証済み |",
        "| 01 | text | pending | expected | design | manual_acceptance_pending |",
    )
    storybook_missing_contract_manifest = json.dumps(
        {
            "operation_kinds": list(STORYBOOK_MANIFEST_OPERATION_KINDS),
            "ui": [
                {
                    "page": f"page-{index:02}",
                    "engine": "clickable",
                    "audit_status": "verified",
                    "gaps": [],
                }
                for index in range(1, 78)
            ],
        },
        ensure_ascii=False,
    )
    storybook_release_good_failed = bool(
        storybook_release_gate_failures(
            manifest_source=storybook_good_manifest,
            ledger_source=storybook_good_ledger,
        )
    )
    storybook_release_bad_passed = not any(
        "manual_acceptance_pending" in line or "audit_status" in line
        for line in storybook_release_gate_failures(
            manifest_source=storybook_bad_manifest,
            ledger_source=storybook_bad_ledger,
        )
    )
    storybook_missing_contract_passed = not any(
        "must have effective `public_props_options`" in line
        or "must have effective `tests`" in line
        for line in storybook_release_gate_failures(
            manifest_source=storybook_missing_contract_manifest,
            ledger_source=storybook_good_ledger,
        )
    )
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
        or consumer_good_failed
        or consumer_bad_passed
        or consumer_dynamic_bad_passed
        or requirement_gate_good_failed
        or requirement_gate_bad_passed
        or justfile_fmt_good_failed
        or justfile_fmt_bad_passed
        or justfile_lint_good_failed
        or justfile_lint_bad_passed
        or justfile_test_good_failed
        or justfile_test_bad_passed
        or justfile_storybook_good_failed
        or justfile_storybook_bad_passed
        or preset_tab_good_failed
        or preset_tab_bad_passed
        or storybook_release_good_failed
        or storybook_release_bad_passed
        or storybook_missing_contract_passed
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
        if consumer_good_failed:
            print("- valid consumer app contract rejected", file=sys.stderr)
        if consumer_bad_passed:
            print("- missing consumer app workspace member allowed", file=sys.stderr)
        if consumer_dynamic_bad_passed:
            print("- missing consumer app dynamic action contract allowed", file=sys.stderr)
        if requirement_gate_good_failed:
            print("- valid Storybook requirement gate command rejected", file=sys.stderr)
        if requirement_gate_bad_passed:
            print("- dependency-wide Storybook requirement gate warning policy allowed", file=sys.stderr)
        if justfile_fmt_good_failed:
            print("- valid KUC-scoped Justfile fmt gate rejected", file=sys.stderr)
        if justfile_fmt_bad_passed:
            print("- dependency-wide Justfile fmt gate allowed", file=sys.stderr)
        if justfile_lint_good_failed:
            print("- valid KUC-scoped Justfile lint gate rejected", file=sys.stderr)
        if justfile_lint_bad_passed:
            print("- dependency-wide Justfile lint warning policy allowed", file=sys.stderr)
        if justfile_test_good_failed:
            print("- valid KUC-scoped Justfile test gate rejected", file=sys.stderr)
        if justfile_test_bad_passed:
            print("- dependency-wide Justfile test gate allowed", file=sys.stderr)
        if justfile_storybook_good_failed:
            print("- valid target-scoped Storybook Justfile commands rejected", file=sys.stderr)
        if justfile_storybook_bad_passed:
            print("- dependency-wide Storybook Justfile warning policy allowed", file=sys.stderr)
        if preset_tab_good_failed:
            print("- valid preset tab scroll contract rejected", file=sys.stderr)
        if preset_tab_bad_passed:
            print("- missing preset tab hit bounds contract allowed", file=sys.stderr)
        if storybook_release_good_failed:
            print("- valid Storybook release ledger/manifest rejected", file=sys.stderr)
        if storybook_release_bad_passed:
            print("- partial Storybook release ledger/manifest allowed", file=sys.stderr)
        if storybook_missing_contract_passed:
            print("- Storybook manifest without effective contract fields allowed", file=sys.stderr)
        return 1
    return 0


def write_consumer_app_self_test_files(
    root: Path,
    include_member: bool,
    include_dynamic_actions: bool = True,
) -> None:
    root.mkdir(parents=True, exist_ok=True)
    members = '"examples/kuc-consumer-app"' if include_member else '"crates/katana-ui-core"'
    (root / "Cargo.toml").write_text(f"[workspace]\nmembers = [{members}]\n", encoding="utf-8")
    (root / "Justfile").write_text(
        "consumer-app-contract:\n"
        "    cargo test -p kuc-consumer-app --locked\n"
        "    cargo test -p katana-ui-core --test generic_rust_app_contract --locked\n"
        "    cargo test -p katana-ui-core --test generic_rust_app_layout_contract --locked\n"
        "    cargo test -p katana-ui-core --test generic_rust_app_action_contract --locked\n"
        "integration-test: consumer-app-contract\n"
        "e2e-test:\n"
        "    bash scripts/storybook-requirement-gate.sh\n"
        "smoke-test: storybook-smoke storybook-interaction-smoke\n"
        "kuc-guardrails: consumer-app-contract\n"
        "release-readiness-check: integration-test e2e-test smoke-test\n",
        encoding="utf-8",
    )
    app = root / "examples/kuc-consumer-app"
    (app / "src").mkdir(parents=True)
    (app / "Cargo.toml").write_text(
        "[package]\nname = \"kuc-consumer-app\"\n"
        "[dependencies]\nkatana-ui-core.workspace = true\n",
        encoding="utf-8",
    )
    source_tokens = [
        "ComponentTree::new Panel::new SplitPane::new ScrollArea::new",
        "CloseableTabStrip::new Input::new TextArea::new SelectionList::new",
        "Toolbar::new apply_action CloseableTabStripAction::AddTab",
        "add_fixed_tab consumer_app_keeps_endpoint_closes_noop_and_non_closeable_tabs",
        "consumer_app_handles_input_textarea_scroll_split_and_tabs",
    ]
    if include_dynamic_actions:
        source_tokens.extend(
            (
                "UiAction::invoke_callback invoke_search_callback",
                "CloseableTabStripAction::CloseOthers",
                "CloseableTabStripAction::CloseToRight",
                "CloseableTabStripAction::CloseToLeft",
                "CloseableTabStripAction::CloseAll",
                "CloseableTabStripAction::PinTab",
                "CloseableTabStripAction::MoveToGroup",
                "CloseableTabStripEvent::TabPinChanged",
                "CloseableTabStripEvent::TabGroupChanged",
                "CloseableTabStripEvent::GroupCreated",
                "CloseableTabContextCommand::CloseOthers",
                "consumer_app_handles_workspace_tab_bulk_actions",
                "consumer_app_handles_workspace_tab_context_commands",
                "consumer_app_observes_tab_pin_and_group_events",
            )
        )
    source = " ".join(source_tokens)
    (app / "src/lib.rs").write_text(source, encoding="utf-8")
    (app / "src/fixtures.rs").write_text(source, encoding="utf-8")
    (app / "src/tests.rs").write_text(source, encoding="utf-8")
    (app / "src/main.rs").write_text(source, encoding="utf-8")


def write_storybook_requirement_gate_self_test_files(
    root: Path,
    use_target_rustc: bool,
) -> None:
    scripts = root / "scripts"
    scripts.mkdir(parents=True, exist_ok=True)
    requirement_command = (
        "cargo rustc --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings"
        if use_target_rustc
        else 'RUSTFLAGS="-D warnings" cargo build --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked'
    )
    smoke_command = (
        "cargo rustc -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings"
        if use_target_rustc
        else 'RUSTFLAGS="-D warnings" cargo build -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked'
    )
    menu_command = (
        '"${CARGO_CMD[@]}" rustc -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings'
        if use_target_rustc
        else 'RUSTFLAGS="-D warnings" "${CARGO_CMD[@]}" build -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked'
    )
    (scripts / "storybook-requirement-gate.sh").write_text(
        "#!/usr/bin/env bash\nset -euo pipefail\n" f"{requirement_command}\n",
        encoding="utf-8",
    )
    (scripts / "storybook-headless-smoke.sh").write_text(
        "#!/usr/bin/env bash\nset -euo pipefail\n" f"{smoke_command}\n",
        encoding="utf-8",
    )
    (scripts / "assert-menu-button-contract.sh").write_text(
        "#!/usr/bin/env bash\nset -euo pipefail\nread -r -a CARGO_CMD <<<\"${CARGO:-cargo}\"\n"
        f"{menu_command}\n",
        encoding="utf-8",
    )


def write_justfile_fmt_scope_self_test_file(root: Path, use_scoped_packages: bool) -> None:
    root.mkdir(parents=True, exist_ok=True)
    scoped_packages = (
        'KUC_FORMAT_PACKAGES := "-p katana-ui-core -p katana-ui-core-text-raster -p katana-ui-core-svg-raster -p katana-ui-core-egui-adapter -p katana-ui-core-storybook -p kuc-consumer-app"\n'
        "\n"
        "fmt:\n"
        "    {{CARGO}} fmt {{KUC_FORMAT_PACKAGES}}\n"
        "\n"
        "fmt-check:\n"
        "    {{CARGO}} fmt {{KUC_FORMAT_PACKAGES}} -- --check\n"
    )
    dependency_wide = (
        "fmt:\n"
        "    {{CARGO}} fmt --all\n"
        "\n"
        "fmt-check:\n"
        "    {{CARGO}} fmt --all -- --check\n"
    )
    source = scoped_packages if use_scoped_packages else dependency_wide
    (root / "Justfile").write_text(source, encoding="utf-8")


def write_justfile_lint_scope_self_test_file(root: Path, use_scoped_packages: bool) -> None:
    root.mkdir(parents=True, exist_ok=True)
    lint_flags = (
        "-D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::todo "
        "-D clippy::unimplemented -D clippy::dbg_macro -D clippy::panic -D clippy::wildcard_imports"
    )
    scoped_packages = (
        'KUC_WORKSPACE_PACKAGES := "-p katana-ui-core -p katana-ui-core-storybook -p kuc-consumer-app"\n'
        "\n"
        "lint:\n"
        f"    {{{{CARGO}}}} clippy -j {{{{JOBS}}}} {{{{KUC_WORKSPACE_PACKAGES}}}} --all-targets --all-features --locked -- {lint_flags}\n"
    )
    dependency_wide = (
        "lint:\n"
        f"    RUSTFLAGS=\"-D warnings\" {{{{CARGO}}}} clippy -j {{{{JOBS}}}} --workspace --all-targets --all-features --locked -- {lint_flags}\n"
    )
    source = scoped_packages if use_scoped_packages else dependency_wide
    (root / "Justfile").write_text(source, encoding="utf-8")


def write_justfile_test_scope_self_test_file(root: Path, use_scoped_packages: bool) -> None:
    root.mkdir(parents=True, exist_ok=True)
    scoped_packages = (
        'KUC_WORKSPACE_PACKAGES := "-p katana-ui-core -p katana-ui-core-storybook -p kuc-consumer-app"\n'
        "\n"
        "unit-test:\n"
        "    {{CARGO}} test {{KUC_WORKSPACE_PACKAGES}} --all-targets --all-features --locked\n"
        "\n"
        "coverage: fmt-check ast-lint\n"
        "    just coverage-container\n"
        "\n"
        "coverage-iterate: fmt-check ast-lint\n"
        "    just coverage-container-iterate\n"
        "\n"
        "coverage-linux:\n"
        "    CARGO=\"{{CARGO}}\" bash scripts/run-strict-coverage.sh\n"
        "\n"
        "coverage-linux-iterate:\n"
        "    KUC_COVERAGE_REUSE=1 CARGO=\"{{CARGO}}\" bash scripts/run-strict-coverage.sh\n"
        "\n"
        "cargo-test:\n"
        "    {{CARGO}} test {{KUC_WORKSPACE_PACKAGES}} --all-targets --locked\n"
        "\n"
        "check: fmt-check ast-lint check-types lint unit-test\n"
        "\n"
        "release-check: release-target-check fmt-check ast-lint release-readiness-check release-verify\n"
    )
    dependency_wide = (
        "unit-test:\n"
        "    {{CARGO}} test --workspace --all-targets --all-features --locked\n"
        "\n"
        "coverage:\n"
        "    {{CARGO}} llvm-cov --workspace --all-features --locked --summary-only --fail-under-lines {{COVERAGE_MIN_LINES}}\n"
        "\n"
        "cargo-test:\n"
        "    RUSTFLAGS=\"-D warnings\" cargo test --workspace --all-targets\n"
    )
    source = scoped_packages if use_scoped_packages else dependency_wide
    (root / "Justfile").write_text(source, encoding="utf-8")
    scripts = root / "scripts"
    scripts.mkdir(parents=True, exist_ok=True)
    coverage_source = (
        "export CARGO_PROFILE_TEST_OPT_LEVEL=0\n"
        'coverage_target_dir="${CARGO_TARGET_DIR:-target}/llvm-cov-target"\n'
        'coverage_reuse="${KUC_COVERAGE_REUSE:-0}"\n'
        'run_cargo clean --target-dir "$coverage_target_dir"\n'
        "run_cargo llvm-cov clean --profraw-only\n"
        'coverage_min_free_gib="${KUC_COVERAGE_MIN_FREE_GIB:-2}"\n'
        'df -Pk "${CARGO_TARGET_DIR:-target}"\n'
        "run_cargo llvm-cov \\\n"
        "  -p katana-ui-core \\\n"
        "  -p katana-ui-core-storybook \\\n"
        "  -p kuc-consumer-app \\\n"
        "  --all-targets --all-features --locked --no-report \\\n"
        "  -- --include-ignored\n"
        "run_cargo llvm-cov report \\\n"
        "  --summary-only \\\n"
        "  --fail-under-functions 100 \\\n"
        "  --fail-under-lines 100 \\\n"
        "  --fail-uncovered-functions 0 \\\n"
        "  --fail-uncovered-lines 0\n"
    )
    (scripts / "run-strict-coverage.sh").write_text(
        coverage_source,
        encoding="utf-8",
    )


def write_justfile_storybook_command_self_test_file(
    root: Path,
    use_target_rustc: bool,
) -> None:
    root.mkdir(parents=True, exist_ok=True)
    target_scoped = (
        "storybook:\n"
        "    {{CARGO}} rustc --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings\n"
        "    {{CARGO}} run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 0\n"
        "\n"
        "storybook-summary:\n"
        "    {{CARGO}} rustc -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings\n"
        "    {{CARGO}} run -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked\n"
        "\n"
        "storybook-modal:\n"
        "    {{CARGO}} rustc --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings\n"
        "    {{CARGO}} run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-modal-window 0\n"
        "\n"
        "storybook-check:\n"
        "    {{CARGO}} rustc -p katana-ui-core-storybook --lib --locked -- -D warnings\n"
        "    {{CARGO}} rustc -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings\n"
        "\n"
        "storybook-visual-snapshot:\n"
        "    {{CARGO}} rustc --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- -D warnings\n"
        "    {{CARGO}} run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --visual-snapshot target/storybook-panel.png\n"
    )
    dependency_wide = (
        "storybook:\n"
        "    RUSTFLAGS=\"-D warnings\" {{CARGO}} run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 0\n"
        "\n"
        "storybook-check:\n"
        "    RUSTFLAGS=\"-D warnings\" {{CARGO}} check -p katana-ui-core-storybook --all-targets --locked\n"
    )
    source = target_scoped if use_target_rustc else dependency_wide
    (root / "Justfile").write_text(source, encoding="utf-8")


def write_preset_tab_scroll_self_test_files(root: Path, include_hit_bounds: bool) -> None:
    visual = root / "crates/katana-ui-core-storybook/src/visual"
    visual.mkdir(parents=True, exist_ok=True)
    (visual / "preset_tab_scroll.rs").write_text(
        "pub(super) fn viewport_rect() {}\n"
        "pub(super) fn max_scroll_x_for_page() {}\n"
        "pub(super) fn ensure_index_visible() {}\n"
        "pub(super) fn hit_index_at() {}\n"
        "visible_index_range visual_rect_for_index\n",
        encoding="utf-8",
    )
    (visual / "preset_tab_label.rs").write_text(
        "pub(super) fn fit() {}\nTRUNCATION_MARKER\nmeasured_width_for_test\n",
        encoding="utf-8",
    )
    hit_bounds = (
        "fn preset_tab_hit_bounds_reject_gap_and_clipped_edges() {}\n"
        if include_hit_bounds
        else ""
    )
    (visual / "visual_preset_tab_scroll_tests.rs").write_text(
        "fn overflowing_preset_tabs_have_horizontal_scroll_range() {}\n"
        "fn rendered_preset_tabs_are_clipped_at_preview_right_edge() {}\n"
        "fn external_preset_selection_scrolls_current_tab_into_view() {}\n"
        "fn clicking_scrolled_preset_tab_uses_logical_tab_index() {}\n"
        f"{hit_bounds}",
        encoding="utf-8",
    )
    (visual / "visual_tests.rs").write_text(
        "fn every_required_page_preset_tab_labels_fit_clip_width() {}\n"
        "StoryRequirements::required_pages()\n"
        "measured_width_for_test\n",
        encoding="utf-8",
    )


def main() -> int:
    if "--self-test" in sys.argv:
        return self_test()
    failures = release_track_task_failures()
    failures.extend(dod_failures())
    failures.extend(no_image_policy_failures(REPOSITORY_POLICY_FILES))
    failures.extend(storybook_role_failures())
    failures.extend(image_gate_failures())
    failures.extend(consumer_app_failures())
    failures.extend(adapter_consumer_app_failures())
    failures.extend(storybook_requirement_gate_command_failures())
    failures.extend(justfile_fmt_scope_failures())
    failures.extend(justfile_lint_scope_failures())
    failures.extend(justfile_test_scope_failures())
    failures.extend(justfile_storybook_command_scope_failures())
    failures.extend(viewer_consumer_event_contract_failures())
    failures.extend(preset_tab_scroll_contract_failures())
    failures.extend(storybook_release_gate_failures())
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
