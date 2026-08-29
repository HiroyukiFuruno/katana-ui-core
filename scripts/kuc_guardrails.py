#!/usr/bin/env python3
import json
from pathlib import Path
import re
import sys

from kuc_openspec_guardrails import KucOpenSpecGuardrails
from kuc_workspace_tab_guardrails import WorkspaceTabGuardrails

RUNTIME_KEYWORDS = (
    "drag",
    "reset",
    "resize",
    "clamp",
    "ratio",
    "move",
    "compute",
    "position",
    "bounds",
    "cursor",
    "scroll",
)
TEST_HELPER_SUFFIXES = ("_for_test",)
LONG_HELPER_ONLY_VIEW_LINES = 220

CALLBACK_RULES = (
    (("composite", "selector", "toggle"), ("on_change",)),
    (("composite", "selector", "segmented"), ("on_change",)),
    (("composite", "selector", "select"), ("on_change", "on_select")),
    (("composite", "selector", "color"), ("on_change", "on_click", "on_input")),
    (("composite", "selector", "color_picker"), ("on_change",)),
    (("composite", "combo_box"), ("on_change", "on_select")),
    (("composite", "menu_button"), ("on_open", "on_close")),
    (("composite", "input", "text"), ("on_input", "on_change", "on_submit")),
    (("composite", "input", "search"), ("on_submit", "on_change", "on_input")),
    (("layout", "accordion"), ("on_toggle",)),
)

class KucGuardrails:
    def __init__(self, root: Path) -> None:
        self.root = root

    def run(self) -> list[str]:
        failures: list[str] = []
        failures.extend(self.runtime_api_failures())
        failures.extend(self.callback_failures())
        failures.extend(self.storybook_leak_failures())
        failures.extend(self.helper_only_view_failures())
        failures.extend(self.component_state_ownership_failures())
        failures.extend(self.typed_action_model_failures())
        failures.extend(self.storybook_panel_evidence_failures())
        failures.extend(self.storybook_live_harness_dor_failures())
        failures.extend(self.storybook_next_change_scope_failures())
        failures.extend(self.storybook_remaining_handoff_failures())
        failures.extend(self.visual_fallback_policy_failures())
        failures.extend(self.storybook_reflection_audit_policy_failures())
        failures.extend(self.repo_local_guardrail_policy_failures())
        failures.extend(self.generic_rust_ui_boundary_failures())
        failures.extend(self.adapter_svg_render_plan_failures())
        failures.extend(self.host_action_render_plan_failures())
        failures.extend(self.adapter_coverage_plan_failures())
        failures.extend(self.storybook_live_component_contract_failures())
        failures.extend(self.storybook_svg_runtime_boundary_failures())
        failures.extend(self.command_chrome_boundary_failures())
        failures.extend(self.controlled_presentation_boundary_failures())
        failures.extend(self.egui_text_surface_adapter_boundary_failures())
        failures.extend(self.text_surface_storybook_artifact_boundary_failures())
        failures.extend(self.artifact_compositor_boundary_failures())
        failures.extend(self.context_menu_adapter_boundary_failures())
        failures.extend(self.text_command_surface_artifact_order_ownership_failures())
        failures.extend(self.text_command_surface_adapter_boundary_failures())
        failures.extend(self.text_command_surface_context_menu_root_contract_failures())
        failures.extend(self.text_command_surface_context_menu_consumer_failures())
        failures.extend(self.egui_command_chrome_adapter_boundary_failures())
        failures.extend(self.choice_api_boundary_failures())
        failures.extend(WorkspaceTabGuardrails(self.root).failures())
        failures.extend(self.agent_stop_policy_failures())
        failures.extend(self.agent_hook_policy_failures())
        failures.extend(self.release_readiness_recipe_failures())
        failures.extend(self.storybook_regression_recipe_failures())
        failures.extend(self.public_app_shell_failures())
        failures.extend(self.openspec_evidence_failures())
        failures.extend(self.file_length_review_failures())
        return failures

    def rust_files(self, base: Path) -> list[Path]:
        if not base.exists():
            return []
        return sorted(base.rglob("*.rs"))

    def relative(self, path: Path) -> str:
        return path.relative_to(self.root).as_posix()

    def read(self, path: Path) -> str:
        return path.read_text(encoding="utf-8")

    def read_rust_dir(self, path: Path) -> str:
        return "\n".join(self.read(source) for source in self.rust_files(path))

    def runtime_api_failures(self) -> list[str]:
        failures: list[str] = []
        for path in self.rust_files(self.root / "crates"):
            if path.name not in {"ops.rs", "view.rs"}:
                continue
            pending_cfg_test = False
            for line_number, line in enumerate(self.read(path).splitlines(), start=1):
                stripped = line.strip()
                if stripped.startswith("#[cfg(test)]"):
                    pending_cfg_test = True
                    continue
                if pending_cfg_test and not stripped:
                    continue
                if pending_cfg_test:
                    name = self.public_fn_name(stripped)
                    if name and self.is_test_only_runtime_name(name):
                        failures.append(
                            f"{self.relative(path)}:{line_number}: runtime API `{name}` is test-only"
                        )
                    pending_cfg_test = False
        return failures

    def public_fn_name(self, line: str) -> str | None:
        pattern = r"\bpub(?:\([^)]*\))?\s+(?:const\s+)?fn\s+([A-Za-z0-9_]+)"
        match = re.search(pattern, line)
        if match:
            return match.group(1)
        return None

    def is_test_only_runtime_name(self, name: str) -> bool:
        lowered = name.lower()
        if lowered.endswith(TEST_HELPER_SUFFIXES):
            return False
        return any(keyword in lowered for keyword in RUNTIME_KEYWORDS)

    def callback_failures(self) -> list[str]:
        failures: list[str] = []
        for path in self.rust_files(self.root / "crates"):
            if path.name != "types.rs":
                continue
            required = self.required_callbacks(path.as_posix())
            if not required:
                continue
            source = self.read(path)
            if "Props" not in source:
                continue
            if not any(token in source for token in required):
                failures.append(
                    f"{self.relative(path)}: interactive Props missing callback: {'/'.join(required)}"
                )
        return failures

    def required_callbacks(self, path_text: str) -> tuple[str, ...]:
        for segments, callbacks in CALLBACK_RULES:
            if all(segment in path_text for segment in segments):
                return callbacks
        return ()

    def storybook_leak_failures(self) -> list[str]:
        failures: list[str] = []
        for path in self.rust_files(self.root / "storybook" / "src"):
            lines = self.read(path).splitlines()
            for index, line in enumerate(lines):
                if "Box::leak" not in line:
                    continue
                start = max(0, index - 2)
                end = min(len(lines), index + 3)
                allowed = any(
                    "WHY: allow(storybook_box_leak)" in candidate
                    for candidate in lines[start:end]
                )
                if not allowed:
                    failures.append(f"{self.relative(path)}:{index + 1}: Box::leak is not allowed")
        return failures

    def helper_only_view_failures(self) -> list[str]:
        failures: list[str] = []
        for path in self.rust_files(self.root / "crates"):
            if path.name != "view.rs":
                continue
            source = self.production_source(self.read(path))
            lines = source.splitlines()
            if len(lines) <= LONG_HELPER_ONLY_VIEW_LINES:
                continue
            if re.search(r"\b(struct|enum|impl|type)\b", source):
                continue
            if "fn " not in source and "const " not in source:
                continue
            view_pattern = r"->\s+(impl\s+IntoView|impl\s+View|Box<dyn\s+View>|[A-Za-z0-9_]*View)\b"
            if re.search(view_pattern, source):
                continue
            failures.append(f"{self.relative(path)}: view.rs is helper-only")
        return failures

    def production_source(self, source: str) -> str:
        marker = "\n#[cfg(test)]\nmod tests"
        if marker in source:
            return source.split(marker, maxsplit=1)[0]
        return source

    def openspec_evidence_failures(self) -> list[str]:
        return KucOpenSpecGuardrails(self.root).evidence_failures()

    def file_length_review_failures(self) -> list[str]:
        return KucOpenSpecGuardrails(self.root).file_length_review_failures()

    def repo_local_guardrail_policy_failures(self) -> list[str]:
        required_files = (
            self.root / "docs/architecture/ui-separation/owned-ui-task-map.md",
            self.root
            / "openspec/changes/establish-kuc-atoms-molecules-catalog/quality-gates-contract.md",
            self.root
            / "openspec/changes/establish-kuc-atoms-molecules-catalog/specs/kuc-quality-gates/spec.md",
        )
        missing_files = [path for path in required_files if not path.exists()]
        if missing_files:
            return [
                f"{self.relative(path)}: KUC repo-local guardrail policy file is missing"
                for path in missing_files
            ]

        combined = "\n".join(self.read(path) for path in required_files)
        required_tokens = (
            "KUC repo",
            "`scripts/`",
            "`kal` 側",
            "KUC-specific guards MUST live in this repository",
            "Storybook is an interactive feedback surface",
        )
        failures = [
            f"KUC repo-local guardrail policy missing token: {token}"
            for token in required_tokens
            if token not in combined
        ]
        forbidden_tokens = (
            "../kal",
            "../katana",
            "/works/private/katana/",
            "kal.json に追記",
            "kal repository changes are required",
        )
        failures.extend(
            f"KUC guardrail policy must not require kal-side edits: {token}"
            for token in forbidden_tokens
            if token in combined
        )
        return failures

    def generic_rust_ui_boundary_failures(self) -> list[str]:
        failures: list[str] = []
        core_src = self.root / "crates/katana-ui-core/src"
        forbidden_tokens = (
            "KatanaSvgIcon",
            "katana_icons",
            "katana-icons",
            "assets/icons/katana",
            "../katana",
            "/works/private/katana/",
            "crates/katana-ui",
        )
        for path in self.rust_files(core_src):
            source = self.read(path)
            for token in forbidden_tokens:
                if token in source:
                    failures.append(
                        f"{self.relative(path)}: KUC core must stay generic; forbidden Katana-specific token `{token}`"
                    )

        typed_icon = self.root / "crates/katana-ui-core/src/render_model/typed_icon.rs"
        atom_typed = self.root / "crates/katana-ui-core/src/atom/typed.rs"
        workspace_tab_options = (
            self.root
            / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/options.rs"
        )
        generic_app_contract = (
            self.root / "crates/katana-ui-core/tests/generic_rust_app_contract.rs"
        )
        generic_layout_contract = (
            self.root / "crates/katana-ui-core/tests/generic_rust_app_layout_contract.rs"
        )
        generic_action_contract = (
            self.root / "crates/katana-ui-core/tests/generic_rust_app_action_contract.rs"
        )
        required_sources = (
            (typed_icon, ("pub svg_source: String", "pub struct UiIconProps")),
            (
                atom_typed,
                (
                    "leading_svg_icon_slot",
                    "trailing_svg_icon_button",
                    "UiIconProps::new(svg_source)",
                ),
            ),
            (
                workspace_tab_options,
                (
                    "pub icon: Option<UiIconProps>",
                    "pub fn svg_icon(mut self, value: UiIconProps) -> Self",
                    "UiIconProps::new(value)",
                ),
            ),
            (
                generic_app_contract,
                (
                    "generic_rust_app_can_compose_shell_from_public_kuc_api",
                    "generic_app_inputs_keep_internal_state_per_instance",
                    "generic_app_readonly_input_rejects_write_actions",
                    "generic_app_readonly_input_allows_selection_without_write_mutation",
                    "generic_app_readonly_text_area_allows_selection_and_submit_without_write_mutation",
                    "generic_app_tabs_support_add_close_move_group_and_pin_contracts",
                ),
            ),
            (
                generic_layout_contract,
                (
                    "generic_app_can_build_resizable_scrollable_layout_from_public_kuc_api",
                    "generic_app_scroll_area_uses_typed_public_action_and_state",
                    "generic_app_split_pane_uses_typed_public_action_and_state",
                    "generic_app_facade_exposes_theme_state_and_render_context",
                ),
            ),
            (
                generic_action_contract,
                (
                    "generic_app_input_icon_button_invokes_callback_without_mutating_text",
                    "generic_app_disabled_input_blocks_icon_button_callback",
                    "generic_adapter_dispatch_targets_stable_state_id_after_redraw",
                    "generic_adapter_dispatches_closeable_tab_typed_actions",
                    "generic_adapter_dispatches_closeable_tab_context_bulk_actions",
                    "generic_adapter_dispatches_closeable_tab_add_and_group_actions",
                    "generic_adapter_dispatches_closeable_tab_typed_event_log",
                    "generic_adapter_dispatches_closeable_tab_visual_index_selection",
                    "generic_app_tabs_support_bulk_context_actions_from_public_api",
                    "generic_app_tabs_context_commands_map_to_typed_actions",
                    "CloseableTabContextMenu::menu",
                    "CloseableTabContextCommand::from_id",
                    "CloseableTabGroupContextCommand::from_id",
                    "to_group_action",
                    "ContextMenuItem::action",
                    "generic_app_tabs_emit_typed_events_for_pin_and_group_changes",
                ),
            ),
            (
                self.root / "examples/kuc-consumer-app/src/lib.rs",
                (
                    "quick_search: SearchBox",
                    "workspace_select: SelectBox",
                    "symbol_combo: ComboBox",
                ),
            ),
            (
                self.root / "examples/kuc-consumer-app/src/fixtures.rs",
                (
                    "SearchBox::new",
                    "SelectBox::new",
                    "ComboBox::new",
                    ".submit_on_enter(true)",
                    ".free_input(true)",
                ),
            ),
            (
                self.root / "examples/kuc-consumer-app/src/actions.rs",
                (
                    "set_quick_search",
                    "UiAction::search_submitted",
                    "select_workspace",
                    "UiAction::select_box_selected",
                    "select_symbol",
                ),
            ),
            (
                self.root / "examples/kuc-consumer-app/src/queries.rs",
                (
                    "quick_search_query",
                    "workspace_value",
                    "symbol_value",
                ),
            ),
            (
                self.root / "examples/kuc-consumer-app/src/tests.rs",
                (
                    "UiNodeKind::SearchBox",
                    "UiNodeKind::SelectBox",
                    "UiNodeKind::ComboBox",
                    "quick_search_log[0].action",
                    "workspace_value",
                    "symbol_value",
                ),
            ),
        )
        for path, tokens in required_sources:
            if not path.exists():
                failures.append(f"{self.relative(path)}: generic Rust UI boundary file is missing")
                continue
            source = self.read(path)
            failures.extend(
                f"{self.relative(path)}: generic Rust UI boundary missing token: {token}"
                for token in tokens
                if token not in source
            )

        docs = self.guard_docs_source() + "\n" + self.generic_contract_docs_source()
        required_doc_tokens = (
            "汎用 Rust app",
            "KUC 本体は Katana を知ってはならない",
            "Katana は参照実装",
            "外部から渡される `svg_source`",
            "framework 固有依存を排除",
        )
        failures.extend(
            f"generic Rust UI boundary docs missing token: {token}"
            for token in required_doc_tokens
            if token not in docs
        )
        return failures

    def adapter_svg_render_plan_failures(self) -> list[str]:
        core_plan = self.root / "crates/katana-ui-core/src/render_model/svg_icon_render_plan.rs"
        core_pixel_plan = self.root / "crates/katana-ui-core/src/render_model/svg_icon_pixel_plan.rs"
        render_model_mod = self.root / "crates/katana-ui-core/src/render_model/mod.rs"
        core_test = self.root / "crates/katana-ui-core/tests/svg_icon_render_plan_contract.rs"
        required_sources = (
            (
                core_plan,
                (
                    "pub struct UiSvgIconRenderPlan",
                    "pub svg_source: String",
                    "pub view_box: String",
                    "pub path_summary: String",
                    "pub paint_policy: super::UiSvgPaintPolicy",
                    "pub theme_token: String",
                    "pub callback: String",
                    "pub fn collect_from_tree",
                    "leading_slot",
                    "trailing_icon_buttons",
                ),
            ),
            (
                core_pixel_plan,
                (
                    "pub struct UiSvgIconPixelPlan",
                    "pub struct UiSvgIconViewBox",
                    "pub viewport: UiRect",
                    "pub scale_x_milli: u32",
                    "pub scale_y_milli: u32",
                    "pub pixel_ready: bool",
                    "UiSvgIconRenderPlan::collect_from_tree",
                    "DEFAULT_SVG_ICON_BOX_PX",
                ),
            ),
            (
                render_model_mod,
                (
                    "pub use svg_icon_pixel_plan",
                    "UiSvgIconPixelPlan",
                    "UiSvgIconViewBox",
                    "pub use svg_icon_render_plan::UiSvgIconRenderPlan",
                ),
            ),
            (
                core_test,
                (
                    "CALLER_SEARCH_SVG",
                    "CALLER_CLEAR_SVG",
                    "UiSvgIconRenderPlan::collect_from_tree",
                    "UiSvgIconPixelPlan::collect_from_tree",
                    "svg_icon_pixel_plan_preserves_viewbox_scale_and_paint_contract",
                    "semantic_fingerprint_changes_when_text_entry_svg_or_callback_changes",
                    "svg_icon_render_plan_preserves_external_svg_metadata_for_adapters",
                    "UiSvgPaintPolicy::StrokeOnly",
                ),
            ),
        )
        failures: list[str] = []
        for path, tokens in required_sources:
            if not path.exists():
                failures.append(f"{self.relative(path)}: SVG icon render plan contract file is missing")
                continue
            source = self.read(path)
            failures.extend(
                f"{self.relative(path)}: SVG icon render plan contract missing token: {token}"
                for token in tokens
                if token not in source
            )

        return failures

    def host_action_render_plan_failures(self) -> list[str]:
        core_plan = self.root / "crates/katana-ui-core/src/render_model/host_action_plan.rs"
        core_types = self.root / "crates/katana-ui-core/src/render_model/host_action_types.rs"
        common = self.root / "crates/katana-ui-core/src/render_model/common.rs"
        render_model_mod = self.root / "crates/katana-ui-core/src/render_model/mod.rs"
        core_test = self.root / "crates/katana-ui-core/tests/host_action_plan_contract.rs"
        required_sources = (
            (
                core_types,
                (
                    "pub struct UiHostActionPlan",
                    "pub action_id: String",
                    "pub enabled: bool",
                    "ui.link.open",
                    "ui.disclosure.",
                    "ui.image.highlight",
                ),
            ),
            (core_plan, ("pub fn collect_from_tree", "push_context_menu_item_plans")),
            (common, ("pub host_actions: Vec<UiHostActionSpec>",)),
            (render_model_mod, ("UiHostActionPlan", "UiHostActionSpec")),
            (
                core_test,
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
                failures.append(f"{self.relative(path)}: host action render plan contract file is missing")
                continue
            source = self.read(path)
            failures.extend(
                f"{self.relative(path)}: host action render plan missing token: {token}"
                for token in tokens
                if token not in source
            )

        return failures

    def adapter_coverage_plan_failures(self) -> list[str]:
        core_plan = self.root / "crates/katana-ui-core/src/render_model/adapter_coverage_plan.rs"
        render_model_mod = self.root / "crates/katana-ui-core/src/render_model/mod.rs"
        action_bridge = self.root / "crates/katana-ui-core/src/adapter_contract/action_bridge.rs"
        host_action_bridge = (
            self.root / "crates/katana-ui-core/src/adapter_contract/host_action_bridge.rs"
        )
        adapter_contract_mod = self.root / "crates/katana-ui-core/src/adapter_contract/mod.rs"
        core_test = self.root / "crates/katana-ui-core/tests/adapter_coverage_plan_contract.rs"
        host_action_test = (
            self.root / "crates/katana-ui-core/tests/adapter_host_action_bridge_contract.rs"
        )
        docs = self.root / "docs/dependency-policy.md"
        required_sources = (
            (
                core_plan,
                (
                    "pub struct UiAdapterCoveragePlan",
                    "pub input_count: usize",
                    "pub text_area_count: usize",
                    "pub tab_container_count: usize",
                    "pub selection_count: usize",
                    "pub split_pane_count: usize",
                    "pub scroll_area_count: usize",
                    "pub modal_count: usize",
                    "pub required_consumer_node_kind_count: usize",
                    "pub missing_required_consumer_node_kinds: Vec<UiNodeKind>",
                    "pub unsupported_node_count: usize",
                    "pub fn collect_from_tree",
                    "pub fn consumer_shell_ready",
                    "UiNodeKind::ImageSurface",
                ),
            ),
            (render_model_mod, ("pub use adapter_coverage_plan::UiAdapterCoveragePlan",)),
            (
                action_bridge,
                (
                    "pub struct AdapterActionBridge",
                    "ComponentAction",
                    "UiActionResult",
                    "component.apply_action(action)",
                ),
            ),
            (
                host_action_bridge,
                (
                    "pub struct AdapterHostActionBridge",
                    "UiHostActionPlan::collect_from_root",
                    "action.enabled",
                    "action.action_id == action_id",
                ),
            ),
            (
                adapter_contract_mod,
                (
                    "pub use action_bridge::AdapterActionBridge",
                    "pub use host_action_bridge::AdapterHostActionBridge",
                ),
            ),
            (
                core_test,
                (
                    "adapter_coverage_plan_reports_consumer_shell_surfaces",
                    "adapter_coverage_plan_blocks_consumer_ready_when_unsupported_nodes_exist",
                    "adapter_coverage_plan_requires_image_surface_for_native_raster_parity",
                    "ImageSurface::from_rgba",
                    "modal_count",
                    "consumer_shell_ready",
                ),
            ),
            (
                host_action_test,
                (
                    "adapter_host_action_bridge_triggers_enabled_button_command",
                    "adapter_host_action_bridge_triggers_text_entry_icon_callback",
                    "adapter_host_action_bridge_triggers_text_area_icon_callback",
                    "adapter_host_action_bridge_rejects_disabled_action",
                ),
            ),
            (
                docs,
                (
                    "UiAdapterCoveragePlan",
                    "AdapterActionBridge",
                    "AdapterHostActionBridge",
                    "core crate",
                    "outside core",
                ),
            ),
        )
        failures: list[str] = []
        for path, tokens in required_sources:
            if not path.exists():
                failures.append(f"{self.relative(path)}: adapter coverage plan contract file is missing")
                continue
            source = self.read(path)
            failures.extend(
                f"{self.relative(path)}: adapter coverage plan contract missing token: {token}"
                for token in tokens
                if token not in source
            )

        return failures

    def storybook_live_component_contract_failures(self) -> list[str]:
        failures: list[str] = []
        storybook_src = self.root / "crates/katana-ui-core-storybook/src"
        forbidden_tokens = (
            "KatanaSvgIcon",
            "katana_icons",
            "katana_svg_icons",
            "katana-icons",
        )
        for path in self.rust_files(storybook_src):
            source = self.read(path)
            for token in forbidden_tokens:
                if token in source:
                    failures.append(
                        f"{self.relative(path)}: Storybook must pass generic props into live KUC components; forbidden token `{token}`"
                    )

        asset_dir = self.root / "crates/katana-ui-core-storybook/assets/katana-icons"
        if asset_dir.exists() and any(asset_dir.rglob("*.svg")):
            failures.append(
                "crates/katana-ui-core-storybook/assets/katana-icons: Storybook fixtures must not create a Katana-namespaced icon pack"
            )
        failures.extend(self.storybook_tabs_core_bridge_failures(storybook_src))
        failures.extend(self.storybook_tabs_layout_order_failures(storybook_src))
        failures.extend(
            self.storybook_closeable_tab_strip_core_bridge_failures(storybook_src)
        )
        failures.extend(self.storybook_input_core_bridge_failures(storybook_src))
        failures.extend(self.storybook_search_core_bridge_failures(storybook_src))
        failures.extend(self.storybook_selection_core_bridge_failures(storybook_src))

        docs = self.generic_contract_docs_source()
        required_doc_tokens = (
            "Storybook は絵ではない",
            "KUC の実部品",
            "props / state / event / action / callback",
            "replay surface",
        )
        failures.extend(
            f"storybook live component contract docs missing token: {token}"
            for token in required_doc_tokens
            if token not in docs
        )
        return failures

    def storybook_svg_runtime_boundary_failures(self) -> list[str]:
        storybook_root = self.root / "crates/katana-ui-core-storybook"
        cargo_toml = storybook_root / "Cargo.toml"
        icon_raster = storybook_root / "src/visual/ui_tree_canvas_svg_icon.rs"
        if not cargo_toml.exists() and not icon_raster.exists():
            return []

        failures: list[str] = []
        if not cargo_toml.exists():
            failures.append(
                "crates/katana-ui-core-storybook/Cargo.toml: Storybook SVG runtime dependency file is missing"
            )
        else:
            cargo_source = self.read(cargo_toml)
            required_dependency = "katana-ui-core-svg-raster.workspace = true"
            if required_dependency not in cargo_source:
                failures.append(
                    "crates/katana-ui-core-storybook/Cargo.toml: Storybook must depend on the public katana-ui-core-svg-raster runtime"
                )
            for dependency in ("resvg", "tiny-skia"):
                if re.search(rf"(?m)^\s*{re.escape(dependency)}(?:\.|\s|=)", cargo_source):
                    failures.append(
                        "crates/katana-ui-core-storybook/Cargo.toml: "
                        f"Storybook must not directly depend on private SVG raster dependency `{dependency}`"
                    )

        if not icon_raster.exists():
            failures.append(
                "crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_svg_icon.rs: Storybook SVG runtime adapter is missing"
            )
            return failures

        source = self.read(icon_raster)
        required_tokens = (
            "katana_ui_core_svg_raster",
            "UiSvgRasterRequest",
            "UiSvgRasterizer",
            "rasterize(&request)",
        )
        failures.extend(
            f"{self.relative(icon_raster)}: Storybook SVG runtime adapter missing token `{token}`"
            for token in required_tokens
            if token not in source
        )
        forbidden_tokens = (
            "resvg::",
            "tiny_skia::",
            "usvg::",
            "Pixmap",
            "Tree::from_str",
            "fn apply_paint_policy",
            "fn unpremultiply",
            "HashMap<",
            "VecDeque<",
        )
        failures.extend(
            f"{self.relative(icon_raster)}: Storybook must not retain a private SVG raster path `{token}`"
            for token in forbidden_tokens
            if token in source
        )
        return failures

    def command_chrome_boundary_failures(self) -> list[str]:
        command_chrome = self.root / "crates/katana-ui-core/src/molecule/command_chrome"
        if not command_chrome.exists():
            return []

        failures: list[str] = []
        forbidden_tokens = (
            "KatanA",
            "KLE",
            "KDV",
            "Markdown",
            "katana_language_editor",
            "egui::",
            "TextEdit",
            "FontDefinitions",
            "load_system_fonts",
            "UiIconProps::new(",
            '"Match case"',
            '"Whole word"',
            '"Use regex"',
            '"Previous result"',
            '"Next result"',
            '"Replace all"',
            '"Search controls"',
        )
        for path in self.rust_files(command_chrome):
            source = self.read(path)
            failures.extend(
                f"{self.relative(path)}: command chrome must not contain forbidden token `{token}`"
                for token in forbidden_tokens
                if token in source
            )
        return failures

    def controlled_presentation_boundary_failures(self) -> list[str]:
        gutter_types_path = (
            self.root / "crates/katana-ui-core/src/text_surface/gutter_types.rs"
        )
        gutter_logic_path = self.root / "crates/katana-ui-core/src/text_surface/gutter.rs"
        props_path = self.root / "crates/katana-ui-core/src/text_surface/props.rs"
        controlled_path = self.root / "crates/katana-ui-core/src/text_surface/surface_controlled.rs"
        floating_path = (
            self.root
            / "crates/katana-ui-core/src/molecule/command_chrome/floating_model.rs"
        )
        paths = (
            gutter_types_path,
            gutter_logic_path,
            props_path,
            controlled_path,
            floating_path,
        )
        if not any(path.exists() for path in paths):
            return []

        failures: list[str] = []
        if gutter_types_path.exists():
            source = self.read(gutter_types_path)
            fields = self.public_struct_fields(
                source, "TextSurfaceAutomaticGutterPresentation"
            )
            forbidden = (
                "width",
                "display_label",
                "logical_row",
                "row_coordinate",
                "bounds",
                "coordinate",
                "panel_size",
                "UiRect",
                "TextSurfaceGutterRow",
            )
            failures.extend(
                f"{self.relative(gutter_types_path)}: controlled automatic gutter DTO must not accept `{token}`"
                for token in forbidden
                if (
                    bool(re.search(r"\bTextSurfaceGutterRow\b", fields))
                    if token == "TextSurfaceGutterRow"
                    else token in fields
                )
            )
            if "TextSurfaceGutterRowId" not in fields:
                failures.append(
                    f"{self.relative(gutter_types_path)}: controlled automatic gutter DTO must use KUC-issued row identities"
                )
            constructor_source = (
                self.read(gutter_logic_path) if gutter_logic_path.exists() else source
            )
            constructor = self.public_constructor_args(
                constructor_source, "TextSurfaceAutomaticGutterPresentation"
            )
            if constructor is None:
                failures.append(
                    f"{self.relative(gutter_types_path)}: controlled automatic gutter DTO must expose a zero-argument constructor"
                )
            elif constructor.strip():
                failures.append(
                    f"{self.relative(gutter_types_path)}: controlled automatic gutter constructor must not accept consumer geometry"
                )

        if props_path.exists():
            fields = self.public_struct_fields(
                self.read(props_path), "TextSurfacePresentation"
            )
            if not re.search(r"\bpub\s+automatic_gutter\s*:", fields):
                failures.append(
                    f"{self.relative(props_path)}: controlled TextSurface presentation must expose automatic_gutter"
                )
            if re.search(r"\bpub\s+gutter\s*:", fields):
                failures.append(
                    f"{self.relative(props_path)}: controlled TextSurface presentation must not expose legacy gutter props"
                )

        if controlled_path.exists():
            source = self.read(controlled_path)
            if "TextSurfaceGutter::from_controlled_automatic" not in source:
                failures.append(
                    f"{self.relative(controlled_path)}: controlled TextSurface synchronization must use KUC automatic gutter conversion"
                )
            if "TextSurfaceGutter::new(" in source:
                failures.append(
                    f"{self.relative(controlled_path)}: controlled TextSurface synchronization must not require consumer gutter geometry"
                )

        if floating_path.exists():
            source = self.read(floating_path)
            fields = self.public_struct_fields(source, "FloatingCommandToolbarPresentation")
            forbidden = ("panel_size", "width", "height", "bounds", "Size")
            failures.extend(
                f"{self.relative(floating_path)}: controlled floating toolbar DTO must not accept `{token}`"
                for token in forbidden
                if token in fields
            )
            constructor = self.public_constructor_args(
                source, "FloatingCommandToolbarPresentation"
            )
            if constructor is None:
                failures.append(
                    f"{self.relative(floating_path)}: controlled floating toolbar DTO must expose a presentation constructor"
                )
            elif any(token in constructor for token in forbidden):
                failures.append(
                    f"{self.relative(floating_path)}: controlled floating toolbar constructor must not accept panel dimensions"
                )

        measurement_path = (
            self.root
            / "crates/katana-ui-core-egui-adapter/src/text_surface/measurement.rs"
        )
        if measurement_path.exists():
            source = self.read(measurement_path)
            if "controlled_gutter_width" not in source:
                failures.append(
                    f"{self.relative(measurement_path)}: controlled gutter must be measured by KUC"
                )
            elif "rasterize_gutter_label" not in source:
                failures.append(
                    f"{self.relative(measurement_path)}: controlled gutter width must come from the KUC text raster"
                )
        return failures

    def public_struct_fields(self, source: str, name: str) -> str:
        match = re.search(
            rf"pub\s+struct\s+{re.escape(name)}\s*\{{(?P<fields>[^}}]*)\}}",
            source,
            re.DOTALL,
        )
        return match.group("fields") if match else ""

    def public_constructor_args(self, source: str, name: str) -> str | None:
        match = re.search(
            rf"impl\s+{re.escape(name)}\s*\{{.*?pub\s+(?:const\s+)?fn\s+new\s*\((?P<args>[^)]*)\)",
            source,
            re.DOTALL,
        )
        return match.group("args") if match else None

    def egui_text_surface_adapter_boundary_failures(self) -> list[str]:
        adapter = self.root / "crates/katana-ui-core-egui-adapter"
        if not adapter.exists():
            return []

        failures: list[str] = []
        manifest = adapter / "Cargo.toml"
        if not manifest.exists():
            return [
                "crates/katana-ui-core-egui-adapter/Cargo.toml: shared text surface adapter manifest is missing"
            ]
        manifest_source = self.read(manifest)
        required_dependencies = (
            "egui.workspace = true",
            "katana-ui-core.workspace = true",
            "katana-ui-core-text-raster.workspace = true",
            "katana-ui-core-svg-raster.workspace = true",
        )
        failures.extend(
            "crates/katana-ui-core-egui-adapter/Cargo.toml: "
            f"shared adapter dependency is missing `{dependency}`"
            for dependency in required_dependencies
            if dependency not in manifest_source
        )
        forbidden_dependencies = (
            "cosmic-text",
            "resvg",
            "tiny-skia",
            "katana-language-editor",
            "katana-document-viewer",
            "katana-render-runtime",
        )
        failures.extend(
            "crates/katana-ui-core-egui-adapter/Cargo.toml: "
            f"shared adapter must not directly depend on `{dependency}`"
            for dependency in forbidden_dependencies
            if re.search(rf"(?m)^\s*{re.escape(dependency)}(?:\.|\s|=)", manifest_source)
        )

        source_root = adapter / "src/text_surface"
        forbidden_tokens = (
            "egui::TextEdit",
            "TextEdit::",
            "egui::Popup",
            "Popup::",
            "MenuButton",
            "menu_button(",
            "egui::Label",
            "Label::",
            "ui.label(",
            "ui.button(",
            "FontDefinitions",
            "FontData",
            "FontFamily::Name",
            "load_system_fonts",
            "SystemSource",
            "/System/Library/Fonts",
            "/Library/Fonts",
            "/usr/share/fonts",
            "C:\\\\Windows\\\\Fonts",
            "cosmic_text::",
            "resvg::",
            "tiny_skia::",
            "painter().text(",
            "painter.text(",
            "draw_glyph",
            "rasterize_glyph",
            "GlyphAtlas",
            "UiIconProps::new(",
            "katana_language_editor",
            "katana_document_viewer",
            "KatanA",
            "KLE",
            "KDV",
            "Markdown",
            'replace("⭐',
            "replace('⭐",
            "replace(\"☆",
            "replace('☆",
        )
        for path in self.rust_files(source_root):
            source = self.read(path)
            failures.extend(
                f"{self.relative(path)}: shared text surface adapter must not contain `{token}`"
                for token in forbidden_tokens
                if token in source
            )
        return failures

    def text_surface_storybook_artifact_boundary_failures(self) -> list[str]:
        storybook_root = self.root / "crates/katana-ui-core-storybook/src/visual"
        runtime = storybook_root / "text_surface_runtime.rs"
        artifact = storybook_root / "text_surface_artifact.rs"
        if not runtime.exists() and not artifact.exists():
            return []
        if not runtime.exists() or not artifact.exists():
            missing = runtime if not runtime.exists() else artifact
            return [
                f"{self.relative(missing)}: TextSurface Storybook artifact path is incomplete"
            ]

        sources = tuple(sorted(storybook_root.glob("text_surface*.rs")))
        source_by_path = {path: self.read(path) for path in sources}
        combined = "\n".join(source_by_path.values())
        required_runtime_tokens = (
            "EguiTextSurfaceAdapter",
            "egui::RawInput",
            "run_scripted_sequence",
            "TextSurfaceArtifactFrame",
            "TextSurfaceEvent",
            "actual_egui_script_is_deterministic_and_covers_editor_surface_events",
            "scripted_artifact_writes_plan_only_png_gif_and_manifest",
        )
        required_artifact_tokens = (
            "TextSurfacePaintOperationKind",
            "render_artifact_frame",
            "write_png",
            "write_gif",
            "adapter-paint-plan-only",
            "actual-egui-raw-input",
            "color_emoji_texture_present",
            "star_variation_selector_present",
        )
        failures = [
            f"{self.relative(runtime)}: TextSurface Storybook actual-egui contract missing `{token}`"
            for token in required_runtime_tokens
            if token not in combined
        ]
        failures.extend(
            f"{self.relative(artifact)}: TextSurface Storybook artifact contract missing `{token}`"
            for token in required_artifact_tokens
            if token not in combined
        )
        if not re.search(r"adapter\s*\.show\(\s*ui,\s*surface", combined):
            failures.append(
                f"{self.relative(runtime)}: TextSurface Storybook actual-egui contract missing `adapter.show(ui, surface`"
            )

        forbidden_runtime_tokens = (
            "egui::Canvas",
            "TextRenderer",
            "render_storybook_canvas",
            "TextSurfaceAction::",
            "surface.apply_action(",
            "layout_for_surface",
            "rasterize_surface",
            "painter.text(",
            "painter().text(",
            "draw_glyph",
            "rasterize_glyph",
            "GlyphAtlas",
            "shape_count",
            "shapes.len()",
            "replace(\"⭐",
            "replace('⭐",
            "replace(\"☆",
            "replace('☆",
        )
        for path, source in source_by_path.items():
            failures.extend(
                f"{self.relative(path)}: TextSurface Storybook must not contain `{token}`"
                for token in forbidden_runtime_tokens
                if token in source
            )
        return failures

    def artifact_compositor_boundary_failures(self) -> list[str]:
        adapter_root = self.root / "crates/katana-ui-core-egui-adapter/src"
        public_entry = adapter_root / "artifact_compositor.rs"
        if not public_entry.exists():
            return []
        source = self.read(public_entry)
        required_entry_tokens = (
            "pub struct ArtifactCompositor",
            "impl ArtifactCompositor",
            "pub fn compose",
            "artifact_compositor_types",
            "artifact_compositor_paint",
        )
        failures = [
            f"{self.relative(public_entry)}: public artifact compositor missing `{token}`"
            for token in required_entry_tokens
            if token not in source
        ]
        if re.search(r"^pub\s+fn\s+compose_artifact_plans", source, re.MULTILINE):
            failures.append(
                f"{self.relative(public_entry)}: public free compositor function is forbidden"
            )

        storybook_root = self.root / "crates/katana-ui-core-storybook/src/visual"
        callers = (
            storybook_root / "text_surface_artifact.rs",
            storybook_root / "command_chrome_artifact.rs",
            storybook_root / "command_chrome_artifact_writer_composite.rs",
        )
        for caller in callers:
            if not caller.exists():
                continue
            caller_source = self.read(caller)
            if "ArtifactCompositor::compose" not in caller_source:
                failures.append(
                    f"{self.relative(caller)}: Storybook must call `ArtifactCompositor::compose`"
                )
            for token in (
                "blend_at(",
                "blend_fill(",
                "blend_texture(",
                "fn source_over(",
                "fn nearest_texture_pixel(",
                "fn validate_texture(",
                "overlay_plan_into(",
                "pixel_index(",
                "composite_canvas_dimensions(",
                "translate_rect(",
                "union_bounds(",
                "include_bounds(",
                "StorybookFallbackRenderer",
                "minifb",
                "fontdue",
                "fontdb",
            ):
                if token in caller_source:
                    failures.append(
                        f"{self.relative(caller)}: private compositor/fallback token `{token}` is forbidden"
                    )

        compositor_sources = self.rust_files(adapter_root)
        manual_tokens = ("fn source_over(", "fn nearest_texture_pixel(", "fn validate_texture(")
        for path in compositor_sources:
            source = self.read(path)
            if path.name == "artifact_compositor_blend.rs":
                continue
            for token in manual_tokens:
                if token in source:
                    failures.append(
                        f"{self.relative(path)}: manual compositor token `{token}` belongs only to KUC artifact compositor"
                    )
        return failures

    def egui_command_chrome_adapter_boundary_failures(self) -> list[str]:
        source_root = self.root / "crates/katana-ui-core-egui-adapter/src"
        paths = tuple(sorted(source_root.glob("command_chrome*.rs")))
        if not paths:
            return []
        forbidden_tokens = (
            "egui::TextEdit",
            "TextEdit::",
            "egui::Popup",
            "Popup::",
            "MenuButton",
            "menu_button(",
            "egui::Label",
            "Label::",
            "ui.label(",
            "ui.button(",
            "FontDefinitions",
            "FontData",
            "load_system_fonts",
            "SystemSource",
            "/System/Library/Fonts",
            "/Library/Fonts",
            "/usr/share/fonts",
            "C:\\\\Windows\\\\Fonts",
            "cosmic_text::",
            "painter().text(",
            "painter.text(",
            "draw_glyph",
            "rasterize_glyph",
            "UiIconProps::new(",
            "katana_language_editor",
            "katana_document_viewer",
            "KatanA",
            "KLE",
            "KDV",
            "Markdown",
            'replace("⭐',
            "replace('⭐",
        )
        failures: list[str] = []
        for path in paths:
            source = self.read(path)
            failures.extend(
                f"{self.relative(path)}: command chrome adapter must not contain `{token}`"
                for token in forbidden_tokens
                if token in source
            )
        return failures

    def context_menu_adapter_boundary_failures(self) -> list[str]:
        adapter_root = self.root / "crates/katana-ui-core-egui-adapter/src/context_menu"
        types_path = adapter_root / "types.rs"
        adapter_path = adapter_root / "adapter.rs"
        compositor_types = self.root / "crates/katana-ui-core-egui-adapter/src/artifact_compositor_types.rs"
        compositor_paint = self.root / "crates/katana-ui-core-egui-adapter/src/artifact_compositor_paint.rs"
        storybook_root = self.root / "crates/katana-ui-core-storybook/src/visual"
        storybook_sources = tuple(sorted(storybook_root.glob("context_menu_surface*.rs")))
        if not adapter_root.exists() and not storybook_sources:
            return []
        failures: list[str] = []
        required_paths = (types_path, adapter_path, compositor_types, compositor_paint)
        failures.extend(
            f"{self.relative(path)}: ContextMenu actual adapter path is incomplete"
            for path in required_paths
            if not path.exists()
        )
        if types_path.exists():
            fields = self.public_struct_fields(
                self.read(types_path), "ContextMenuPresentation"
            )
            forbidden_dto_tokens = ("UiRect", "anchor", "x:", "y:", "bounds", "viewport")
            failures.extend(
                f"{self.relative(types_path)}: controlled ContextMenu presentation must not expose pixel DTO `{token}`"
                for token in forbidden_dto_tokens
                if token in fields
            )
        if adapter_path.exists():
            adapter_source = self.read(adapter_path)
            required_tokens = (
                "EguiContextMenuAdapter",
                "egui::Area",
                "ContextMenuPlacementResolver",
                "ContextMenuTypeAheadBuffer",
                "TextSurfaceContextTargetAnchor",
                "request_open",
            )
            failures.extend(
                f"{self.relative(adapter_path)}: ContextMenu actual adapter missing `{token}`"
                for token in required_tokens
                if token not in adapter_source
            )
            forbidden_tokens = (
                "katana_language_editor",
                "katana_document_viewer",
                "KatanA",
                "KLE",
                "KDV",
                "Markdown",
                "clipboard",
                "egui::menu",
                "menu_button(",
            )
            failures.extend(
                f"{self.relative(adapter_path)}: ContextMenu actual adapter must not contain `{token}`"
                for token in forbidden_tokens
                if token in adapter_source
            )
        for path in (compositor_types, compositor_paint):
            if path.exists() and "ContextMenu" not in self.read(path):
                failures.append(
                    f"{self.relative(path)}: artifact compositor must include ContextMenu plan refs"
                )
        required_storybook_tokens = (
            "EguiContextMenuAdapter",
            "EguiTextSurfaceAdapter",
            "egui::RawInput",
            "ArtifactCompositor::compose",
            "actual_egui_context_menu_storybook_integration_is_repeatable",
            "AccessKitActionRequest",
        )
        storybook_source = "\n".join(self.read(path) for path in storybook_sources)
        failures.extend(
            "crates/katana-ui-core-storybook/src/visual/context_menu_surface: "
            f"ContextMenu actual Storybook evidence missing `{token}`"
            for token in required_storybook_tokens
            if token not in storybook_source
        )
        forbidden_storybook_tokens = (
            "egui::Area",
            "egui::menu",
            "menu_button(",
            "ContextMenuAction::",
            "ContextMenuAnchor::Pointer",
            "painter.text(",
            "painter().text(",
        )
        for token in forbidden_storybook_tokens:
            if token in storybook_source:
                failures.append(
                    "crates/katana-ui-core-storybook/src/visual/context_menu_surface: "
                    f"consumer ContextMenu geometry or direct core dispatch `{token}` is forbidden"
                )
        return failures

    def text_command_surface_artifact_order_ownership_failures(self) -> list[str]:
        """Keep root artifact order private and read-only to consumers."""
        types = self.root / "crates/katana-ui-core-egui-adapter/src/text_command_surface/types.rs"
        if not types.exists():
            return []
        source = self.read(types)
        failures: list[str] = []
        output_match = re.search(
            r"(?s)\bstruct\s+EguiTextCommandSurfaceOutput\s*\{(?P<body>.*?)\n\}",
            source,
        )
        output_body = output_match.group("body") if output_match else ""
        if re.search(
            r"(?m)^\s*pub(?:\([^)]*\))?\s+artifact_order\s*:\s*Vec<EguiTextCommandSurfaceChild>",
            output_body,
        ):
            failures.append(
                f"{self.relative(types)}: EguiTextCommandSurfaceOutput must not expose mutable public artifact_order storage"
            )
        required_accessor = "pub fn artifact_order(&self) -> &[EguiTextCommandSurfaceChild]"
        if required_accessor not in source:
            failures.append(
                f"{self.relative(types)}: EguiTextCommandSurfaceOutput must expose read-only `{required_accessor}`"
            )
        for token in ("pub fn artifact_order_mut", "pub fn set_artifact_order"):
            if token in source:
                failures.append(
                    f"{self.relative(types)}: EguiTextCommandSurfaceOutput must not expose mutable artifact order API `{token}`"
                )
        return failures

    def text_command_surface_adapter_boundary_failures(self) -> list[str]:
        adapter = self.root / "crates/katana-ui-core-egui-adapter/src/text_command_surface.rs"
        adapter_artifact = self.root / "crates/katana-ui-core-egui-adapter/src/text_command_surface/artifact.rs"
        adapter_composition = self.root / "crates/katana-ui-core-egui-adapter/src/text_command_surface/composition.rs"
        adapter_model = self.root / "crates/katana-ui-core-egui-adapter/src/text_command_surface/model.rs"
        adapter_synchronization = self.root / "crates/katana-ui-core-egui-adapter/src/text_command_surface/synchronization.rs"
        adapter_types = self.root / "crates/katana-ui-core-egui-adapter/src/text_command_surface/types.rs"
        floating_adapter = self.root / "crates/katana-ui-core-egui-adapter/src/command_chrome_floating.rs"
        dropdown_adapter = self.root / "crates/katana-ui-core-egui-adapter/src/command_chrome_dropdown.rs"
        storybook = self.root / "crates/katana-ui-core-storybook/src/visual/text_command_surface_integration_tests.rs"
        storybook_facts = self.root / "crates/katana-ui-core-storybook/src/visual/text_command_surface_integration_tests/facts.rs"
        storybook_harness = self.root / "crates/katana-ui-core-storybook/src/visual/text_command_surface_integration_tests/harness.rs"
        storybook_assertions = self.root / "crates/katana-ui-core-storybook/src/visual/text_command_surface_integration_tests/assertions.rs"
        storybook_scenario = self.root / "crates/katana-ui-core-storybook/src/visual/text_command_surface_integration_tests/scenario.rs"
        if not adapter.exists() and not storybook.exists():
            return []
        failures: list[str] = []
        for path in (
            adapter,
            adapter_artifact,
            adapter_composition,
            adapter_model,
            adapter_synchronization,
            adapter_types,
            storybook,
            storybook_facts,
            storybook_harness,
            storybook_assertions,
            storybook_scenario,
        ):
            if not path.exists():
                failures.append(f"{self.relative(path)}: text-command surface path is incomplete")
        if not adapter.exists() or not storybook.exists():
            return failures
        adapter_source = "\n".join(
            self.read(path)
            for path in (
                adapter,
                adapter_artifact,
                adapter_composition,
                adapter_model,
                adapter_synchronization,
                adapter_types,
            )
        )
        required_adapter_tokens = (
            "EguiTextCommandSurfaceAdapter",
            "EguiTextCommandSurface",
            "available_rect_before_wrap",
            "measure_toolbar",
            "show_floating_toolbar",
            "artifact_order_for_root",
            "show_search_strip",
            "root_bounds",
            "Vec<EguiTextCommandSurfaceChild>",
            "pub fn artifact_order(&self) -> &[EguiTextCommandSurfaceChild]",
            "EguiContextMenuAdapter",
            "with_context_menu",
            "synchronize_context_menu",
            "ArtifactPaintPlanRef::ContextMenu",
        )
        failures.extend(
            f"{self.relative(adapter)}: text-command adapter missing `{token}`"
            for token in required_adapter_tokens
            if token not in adapter_source
        )
        separate_children = (
            "toolbar: &mut CommandChromeToolbar",
            "floating: &mut FloatingCommandToolbar",
            "search: &mut CommandChromeSearchStrip",
        )
        failures.extend(
            f"{self.relative(adapter)}: public root API must retain `{token}` inside EguiTextCommandSurface"
            for token in separate_children
            if token in adapter_source
        )
        required_retained_tokens = (
            "pub struct EguiTextCommandSurface",
            "deferred_floating_toolbar",
            "with_floating_toolbar",
            "surface: &mut EguiTextCommandSurface",
            "EguiTextCommandSurfacePresentation",
            "synchronize_presentation",
            "synchronize_floating_for_frame",
            "floating_visibility_controlled",
        )
        failures.extend(
            f"{self.relative(adapter)}: consumer-safe retained composition missing `{token}`"
            for token in required_retained_tokens
            if token not in adapter_source
        )
        mutable_getters = (
            "pub fn text_mut",
            "pub fn toolbar_mut",
            "pub fn floating_toolbar_mut",
            "pub fn search_strip_mut",
        )
        failures.extend(
            f"{self.relative(adapter)}: retained text-command surface must not expose mutable child getter `{token}`"
            for token in mutable_getters
            if token in adapter_source
        )
        toolbar_index = adapter_source.find("EguiTextCommandSurfaceChild::Toolbar")
        search_index = (
            adapter_source.find("EguiTextCommandSurfaceChild::Search", toolbar_index + 1)
            if toolbar_index >= 0
            else -1
        )
        text_index = (
            adapter_source.find("EguiTextCommandSurfaceChild::Text", search_index + 1)
            if search_index >= 0
            else -1
        )
        if not (
            toolbar_index >= 0
            and search_index > toolbar_index
            and text_index > search_index
            and "artifact_order_for_root" in adapter_source
        ):
            failures.append(
                f"{self.relative(adapter)}: text-command adapter must define canonical child paint order Toolbar -> Search -> Text"
            )
        forbidden_adapter_tokens = (
            "katana_language_editor",
            "katana_document_viewer",
            "KatanA",
            "KLE",
            "KDV",
            "Markdown",
            "search-provider",
        )
        failures.extend(
            f"{self.relative(adapter)}: text-command adapter must not contain `{token}`"
            for token in forbidden_adapter_tokens
            if token in adapter_source
        )
        floating_source = self.read(floating_adapter) if floating_adapter.exists() else ""
        dropdown_source = self.read(dropdown_adapter) if dropdown_adapter.exists() else ""
        required_dropdown_precedence = (
            "outside_dismiss_events",
            "floating_interaction_contains",
            "!item.disabled && contains_ui_rect(item.bounds, point)",
        )
        failures.extend(
            f"{self.relative(floating_adapter)}: floating dropdown must resolve enabled item bounds before outside dismissal `{token}`"
            for token in required_dropdown_precedence
            if token not in floating_source
        )
        if "!item.disabled && contains(item.bounds, *pos)" not in dropdown_source:
            failures.append(
                f"{self.relative(dropdown_adapter)}: dropdown item hit resolution must precede outside dismissal"
            )
        storybook_source = "\n".join(
            self.read(path)
            for path in (
                storybook,
                storybook_facts,
                storybook_harness,
                storybook_assertions,
                storybook_scenario,
            )
        )
        required_storybook_tokens = (
            "EguiTextCommandSurfaceAdapter",
            "egui::RawInput",
            "actual_egui_text_command_surface_keeps_all_children_inside_root_repeatably",
            "⭐️",
            "assert_artifact_output_contract",
            "expected_artifact_order",
            "assert_inside",
        )
        failures.extend(
            f"{self.relative(storybook)}: text-command Storybook evidence missing `{token}`"
            for token in required_storybook_tokens
            if token not in storybook_source
        )
        forbidden_storybook_tokens = (
            "EguiTextSurfaceAdapter",
            "EguiCommandChromeAdapter",
            "new_child(",
            "available_height",
            "allocate_ui_with_layout",
            "FloatingCommandToolbarLayout",
            "previous TextSurface frame",
            "FloatingCommandToolbarPresentation",
            "text_mut()",
            "floating_dropdown_hit_test(",
            "retry_floating_dropdown_pointer(",
        )
        failures.extend(
            f"{self.relative(storybook)}: consumer layout bypass `{token}` is forbidden"
            for token in forbidden_storybook_tokens
            if token in storybook_source
        )
        return failures

    def text_command_surface_context_menu_root_contract_failures(self) -> list[str]:
        """Keep ContextMenu styling, controlled state, and AccessKit proof in the root."""
        adapter_root = self.root / "crates/katana-ui-core-egui-adapter"
        types = adapter_root / "src/text_command_surface/types.rs"
        synchronization = adapter_root / "src/text_command_surface/synchronization.rs"
        context_menu = adapter_root / "src/text_command_surface/context_menu.rs"
        test = adapter_root / "tests/text_command_surface/context_menu.rs"
        paths = (types, synchronization, context_menu, test)
        if not any(path.exists() for path in paths):
            return []
        failures: list[str] = []
        failures.extend(
            f"{self.relative(path)}: retained ContextMenu root contract path is incomplete"
            for path in paths
            if not path.exists()
        )
        if not all(path.exists() for path in paths):
            return failures
        types_source = self.read(types)
        synchronization_source = self.read(synchronization)
        context_source = self.read(context_menu)
        test_source = self.read(test)
        required_type_tokens = (
            "pub context_menu: Option<ContextMenuPresentation>",
            "context_menu_raster_style",
            "context_menu_paint_style",
        )
        failures.extend(
            f"{self.relative(types)}: retained ContextMenu presentation contract missing `{token}`"
            for token in required_type_tokens
            if token not in types_source
        )
        required_synchronization_tokens = (
            "value.context_menu",
            "synchronize_context_menu",
        )
        failures.extend(
            f"{self.relative(synchronization)}: retained ContextMenu synchronization missing `{token}`"
            for token in required_synchronization_tokens
            if token not in synchronization_source
        )
        forbidden_style_tokens = (
            "ContextMenuRasterStyle",
            "ContextMenuPaintStyle",
            "FontToken",
            "FontFamily",
            "RGBA",
        )
        failures.extend(
            f"{self.relative(context_menu)}: root ContextMenu style must come from TextCommandSurfaceStyle, not `{token}`"
            for token in forbidden_style_tokens
            if token in context_source
        )
        if re.search(r"\[\s*\d+(?:\s*,\s*\d+){2}", context_source):
            failures.append(
                f"{self.relative(context_menu)}: root ContextMenu style must not contain an in-module color literal"
            )
        required_test_tokens = (
            "context_menu: Some(context_menu)",
            "ContextMenuEvent::TypeAheadMatched",
            "assert_focus_restored",
            "AccessKitActionRequest",
            "assert_context_menu_opened(&accesskit_open)",
        )
        failures.extend(
            f"{self.relative(test)}: actual root ContextMenu test missing `{token}`"
            for token in required_test_tokens
            if token not in test_source
        )
        if not re.search(
            r"assert_menu_closed\(&outside_restored\)(?:\?|);[\s\S]*AccessKitActionRequest[\s\S]*assert_context_menu_opened\(&accesskit_open\)",
            test_source,
        ):
            failures.append(
                f"{self.relative(test)}: AccessKit ContextMenu proof must begin from a closed retained root menu"
            )
        return failures

    def text_command_surface_context_menu_consumer_failures(self) -> list[str]:
        """Reject prospective consumer ownership of root ContextMenu composition."""
        storybook_root = (
            self.root
            / "crates/katana-ui-core-storybook/src/visual/text_command_surface_integration_tests"
        )
        kle_root = (
            self.root.parent
            / "katana-language-editor/crates/katana-language-editor-egui/src"
        )
        consumer_roots = (
            ("KUC Storybook TextCommandSurface", storybook_root),
            ("KLE sibling TextCommandSurface", kle_root),
        )
        failures: list[str] = []
        for consumer_name, consumer_root in consumer_roots:
            sources = [self.read(path) for path in self.rust_files(consumer_root)]
            source = "\n".join(sources)
            if "EguiTextCommandSurfaceAdapter" not in source:
                continue
            if "EguiContextMenuAdapter" in source:
                failures.append(
                    f"{consumer_name}: consumer must use one EguiTextCommandSurfaceAdapter root show API, not sequential EguiContextMenuAdapter composition"
                )
            geometry_tokens = (
                "TextSurfaceContextTargetAnchor",
                "UiContextMenuAnchor",
                "UiContextMenuRect",
                "ContextMenuAnchor::",
                "ContextMenuPlacementResolver",
                "ContextMenuViewport",
                "ContextMenuSize",
                "request_open(",
                "egui::Area",
                ".fixed_pos(",
            )
            failures.extend(
                f"{consumer_name}: consumer ContextMenu target, anchor, rect, or geometry `{token}` is forbidden"
                for token in geometry_tokens
                if token in source
            )
            manual_artifact_tokens = (
                "plans.push(ArtifactPaintPlanRef::ContextMenu",
                ".artifact_order.push(",
                ".artifact_order.insert(",
                ".artifact_order.remove(",
                ".artifact_order.sort(",
                ".artifact_order.reverse(",
                ".artifact_order.extend(",
                ".artifact_order.splice(",
                "artifact_paint_plans().sort",
                "artifact_paint_plans().reverse",
            )
            failures.extend(
                f"{consumer_name}: consumer must not manually compose or reorder ContextMenu artifacts `{token}`"
                for token in manual_artifact_tokens
                if token in source
            )
        return failures

    def storybook_closeable_tab_strip_core_bridge_failures(
        self, storybook_src: Path
    ) -> list[str]:
        dedicated = storybook_src / "visual/dedicated_closeable_tab_strip.rs"
        tests = (
            storybook_src / "visual/visual_interaction_closeable_tab_strip_tests.rs"
        )
        if not dedicated.exists() and not tests.exists():
            return []

        sources = (
            dedicated,
            storybook_src / "visual/screen_state_tabs_bridge.rs",
            storybook_src / "visual/window_interaction/button_operation.rs",
            storybook_src / "visual/window_interaction/button_operation/tabs_operation.rs",
            storybook_src / "visual/window_interaction/context_click.rs",
            tests,
            storybook_src / "visual/visual_interaction_closeable_tab_strip_context_tests.rs",
        )
        combined = "\n".join(self.read(path) for path in sources if path.exists())
        required_tokens = (
            "dedicated_closeable_tab_strip::tab_hit_at",
            "register_closeable_tab_strip_select",
            "CloseableTabStripAction::SelectTab",
            "CloseableTabStripSelect",
            "context_menu_command_at",
            "closeable_tab_strip_context_target",
            "closeable_tab_strip_component_click_selects_real_core_tab",
            "closeable_tab_strip_context_menu_uses_real_core_commands",
            "closeable_tab_strip_tab_context_menu_applies_workspace_tab_commands",
            "closeable_tab_strip_context_menu_keeps_pinned_tabs_fixed_until_unpinned",
            "CLOSE_OTHERS_INDEX",
            "CLOSE_RIGHT_INDEX",
            "MOVE_TO_GROUP_INDEX",
        )
        return [
            "crates/katana-ui-core-storybook/src/visual: "
            f"closeable-tab-strip live core bridge missing token `{token}`"
            for token in required_tokens
            if token not in combined
        ]

    def storybook_tabs_core_bridge_failures(self, storybook_src: Path) -> list[str]:
        tabs_state = storybook_src / "visual/screen_state_tabs.rs"
        tabs_context = storybook_src / "visual/screen_state_tabs_context.rs"
        if not tabs_state.exists() and not tabs_context.exists():
            return []
        tabs_core = storybook_src / "visual/screen_state_tabs_core.rs"
        if not tabs_core.exists():
            return [
                "crates/katana-ui-core-storybook/src/visual/screen_state_tabs_core.rs: Storybook tabs must route through core CloseableTabStrip actions"
            ]
        source = self.read(tabs_core)
        required_tokens = (
            "CloseableTabStripAction",
            "CloseableTabStripEvent",
            "apply_core_tab_action",
            "apply_core_tab_action_confirming_dirty",
            "CloseableTabStripEvent::name",
        )
        failures = [
            f"{self.relative(tabs_core)}: Storybook tabs core bridge missing token `{token}`"
            for token in required_tokens
            if token not in source
        ]
        if tabs_context.exists():
            context_source = self.read(tabs_context)
            tabs_group_context = storybook_src / "visual/screen_state_tabs_group_context.rs"
            if tabs_group_context.exists():
                context_source = f"{context_source}\n{self.read(tabs_group_context)}"
            tabs_context_types = (
                storybook_src / "visual/screen_state_tabs_context_menu_types.rs"
            )
            if tabs_context_types.exists():
                context_source = f"{context_source}\n{self.read(tabs_context_types)}"
            required_context_tokens = (
                "CloseableTabContextMenu::menu",
                "TabsContextMenuCommand::for_group",
                "ContextMenuAnchor::Pointer",
                "context_node.props().context_menu.items",
                "CloseableTabContextCommand::from_id",
                "CloseableTabGroupContextCommand::from_id",
                "from_item_id",
                "open_context_menu_for_group",
            )
            failures.extend(
                f"{self.relative(tabs_context)}: Storybook tabs context menu bridge missing token `{token}`"
                for token in required_context_tokens
                if token not in context_source
            )
        failures.extend(self.storybook_tabs_direct_pin_icon_failures(storybook_src))
        return failures

    def storybook_tabs_direct_pin_icon_failures(
        self, storybook_src: Path
    ) -> list[str]:
        dedicated_tabs = storybook_src / "visual/dedicated_tabs.rs"
        if not dedicated_tabs.exists():
            return []

        sources = (
            storybook_src / "visual/dedicated_tabs.rs",
            storybook_src / "visual/dedicated_tabs_layout.rs",
            storybook_src / "visual/screen_state_tabs.rs",
            storybook_src / "visual/screen_state_tabs_bridge.rs",
            storybook_src / "visual/window_interaction/button_operation.rs",
            storybook_src / "visual/window_interaction/button_operation/tabs_operation.rs",
            storybook_src / "visual/visual_interaction_tabs_tests.rs",
            storybook_src / "visual/visual_interaction_tabs_parity_tests.rs",
        )
        combined = "\n".join(self.read(path) for path in sources if path.exists())
        required_tokens = (
            "pin_icon_hit_at",
            "pin_icon_rect_for_test",
            "TabsPinIcon",
            "register_tabs_pin_icon_unpin",
            "unpin_tab_by_icon",
            "CloseableTabStripAction::UnpinTab",
            "tab_pin_icon_unpin",
            "direct-icon",
            "tabs_pinned_icon_click_directly_unpins_tab",
        )
        return [
            f"{self.relative(dedicated_tabs)}: Storybook tabs direct pin icon contract missing token `{token}`"
            for token in required_tokens
            if token not in combined
        ]

    def storybook_tabs_layout_order_failures(self, storybook_src: Path) -> list[str]:
        layout = storybook_src / "visual/dedicated_tabs_layout.rs"
        core_bar = (
            self.root
            / "crates/katana-ui-core/src/molecule/structured/workspace_tab_bar/bar.rs"
        )
        if not layout.exists() and not core_bar.exists():
            return []

        failures: list[str] = []
        order_contract_active = False
        if layout.exists():
            source = self.read(layout)
            group_index = source.find("push_grouped_tabs(&mut items")
            pinned_index = source.find("push_pinned_tabs(&mut items")
            order_contract_active = group_index >= 0 or pinned_index >= 0
            if order_contract_active and (
                group_index < 0 or pinned_index < 0 or pinned_index > group_index
            ):
                failures.append(
                    f"{self.relative(layout)}: Storybook tabs must render pinned tabs before group blocks"
                )

        if core_bar.exists():
            source = self.read(core_bar)
            group_index = source.find("for group in &options.groups")
            pinned_index = source.find("for tab in options.tabs.iter().filter(|tab| tab.pinned)")
            ordering = core_bar.parent / "ordering.rs"
            ordering_source = self.read(ordering) if ordering.exists() else ""
            delegated_order = (
                "pub fn visual_tabs" in source
                and "ordered_visible_tabs(&self.options.tabs, &self.options.groups)" in source
            )
            if delegated_order:
                order_contract_active = True
                if not ordering_source or not self._ordering_has_pinned_before_groups(ordering_source):
                    failures.append(
                        f"{self.relative(core_bar)}: CloseableTabStrip delegated visual order must expose pinned tabs before group blocks"
                    )
            else:
                order_contract_active = order_contract_active or group_index >= 0 or pinned_index >= 0
                if group_index < 0 or pinned_index < 0 or pinned_index > group_index:
                    failures.append(
                        f"{self.relative(core_bar)}: CloseableTabStrip render tree must expose pinned tabs before group blocks"
                    )

        if not order_contract_active:
            return failures

        storybook_tests = (
            storybook_src / "visual/visual_interaction_tabs_tests.rs",
            storybook_src / "visual/visual_interaction_tabs_parity_tests.rs",
        )
        if any(path.exists() for path in storybook_tests) and not any(
            "tabs_pinned_tabs_render_before_group_block" in self.read(path)
            for path in storybook_tests
            if path.exists()
        ):
            failures.append(
                "crates/katana-ui-core-storybook/src/visual: Storybook tabs pinned-before-group interaction test is missing"
            )

        core_tests = self.root / "crates/katana-ui-core/tests/closeable_tab_strip_rendering_contract.rs"
        if core_tests.exists() and (
            "closeable_tab_strip_renders_pinned_tabs_before_group_blocks"
            not in self.read(core_tests)
        ):
            failures.append(
                f"{self.relative(core_tests)}: CloseableTabStrip pinned-before-group render contract test is missing"
            )
        return failures

    @staticmethod
    def _ordering_has_pinned_before_groups(source: str) -> bool:
        pinned = source.find("push_pinned_tabs")
        grouped = source.find("append_grouped_tabs")
        unknown = source.find("push_unknown_group_tabs")
        ungrouped = source.find("filter(|tab| !tab.pinned && tab.group_id.is_none())")
        declared_group_evidence = (
            "groups: &[WorkspaceTabGroup]" in source
            and "for group in root_groups" in source
            and "tab.group_id.as_ref() == Some(&group.id)" in source
        )
        return (
            pinned >= 0
            and grouped > pinned
            and unknown > grouped
            and ungrouped > unknown
            and declared_group_evidence
        )

    def storybook_input_core_bridge_failures(self, storybook_src: Path) -> list[str]:
        failures: list[str] = []
        text_input = storybook_src / "visual/screen_state_text_input.rs"
        if text_input.exists():
            source = self.read(text_input)
            required_tokens = (
                "Input::new",
                "UiAction::input_value",
                "ComponentAction",
                "apply_core_text_input_value",
            )
            failures.extend(
                f"{self.relative(text_input)}: Storybook text-input core bridge missing token `{token}`"
                for token in required_tokens
                if token not in source
            )
        text_area = storybook_src / "visual/screen_state_text_area_core.rs"
        text_area_state = storybook_src / "visual/screen_state_text_area.rs"
        if text_area_state.exists() and not text_area.exists():
            failures.append(
                "crates/katana-ui-core-storybook/src/visual/screen_state_text_area_core.rs: Storybook text-area must route through core TextAreaAction"
            )
        if text_area.exists():
            source = self.read(text_area)
            required_tokens = (
                "TextArea::new",
                "TextAreaAction",
                "TextAreaActionOutcome",
                "apply_text_area_action",
            )
            failures.extend(
                f"{self.relative(text_area)}: Storybook text-area core bridge missing token `{token}`"
                for token in required_tokens
                if token not in source
            )
        return failures

    def storybook_search_core_bridge_failures(self, storybook_src: Path) -> list[str]:
        search_box = storybook_src / "visual/search_box_screen_state.rs"
        if not search_box.exists():
            return []
        source = self.read(search_box)
        required_tokens = (
            "SearchBox::new",
            "UiAction::input_value",
            "UiAction::search_submitted",
            "UiAction::clear_value",
            "ComponentAction",
        )
        return [
            f"{self.relative(search_box)}: Storybook search-box core bridge missing token `{token}`"
            for token in required_tokens
            if token not in source
        ]

    def storybook_selection_core_bridge_failures(
        self, storybook_src: Path
    ) -> list[str]:
        selection_state = storybook_src / "visual/selection_screen_state.rs"
        selection_core = storybook_src / "visual/selection_screen_state_core.rs"
        if not selection_state.exists():
            return []
        if not selection_core.exists():
            return [
                "crates/katana-ui-core-storybook/src/visual/selection_screen_state_core.rs: Storybook selection controls must route through core select actions"
            ]
        source = self.read(selection_core)
        required_tokens = (
            "SelectBox::new",
            "ComboBox::new",
            "SelectionList::new",
            "UiAction::select_box_selected",
            "UiAction::set_selected_index",
            "ComponentAction",
        )
        return [
            f"{self.relative(selection_core)}: Storybook selection core bridge missing token `{token}`"
            for token in required_tokens
            if token not in source
        ]

    def choice_api_boundary_failures(self) -> list[str]:
        choice = self.root / "crates/katana-ui-core/src/molecule/selection/choice.rs"
        accessors = self.root / "crates/katana-ui-core/src/molecule/selection/accessors.rs"
        options = self.root / "crates/katana-ui-core/src/molecule/selection/options.rs"
        failures: list[str] = []
        if choice.exists():
            source = self.read(choice)
            macro_body = source.split("choice_molecule!(SelectBox", maxsplit=1)[0]
            combo_only_builders = (
                "pub fn input_value",
                "pub fn filter_result",
                "pub fn free_input",
            )
            failures.extend(
                f"{self.relative(choice)}: combo-only builder `{token}` must not be inside choice_molecule macro"
                for token in combo_only_builders
                if token in macro_body
            )
            combo_impl = source.split("impl ComboBox", maxsplit=1)[-1]
            failures.extend(
                f"{self.relative(choice)}: ComboBox is missing combo-only builder `{token}`"
                for token in combo_only_builders
                if token not in combo_impl
            )
            breadcrumb_only_builders = ("pub fn crumb_action",)
            failures.extend(
                f"{self.relative(choice)}: breadcrumb-only builder `{token}` must not be inside choice_molecule macro"
                for token in breadcrumb_only_builders
                if token in macro_body
            )
            breadcrumb_impl = source.split("impl Breadcrumb", maxsplit=1)[-1]
            failures.extend(
                f"{self.relative(choice)}: Breadcrumb is missing breadcrumb-only builder `{token}`"
                for token in breadcrumb_only_builders
                if token not in breadcrumb_impl
            )
        if options.exists():
            source = self.read(options)
            macro_body = source.split("selection_options!(Breadcrumb)", maxsplit=1)[0]
            specialized_builders = (
                ("Tabs", "tabs-only builder", "pub fn icon_action"),
                ("SideMenu", "side-menu-only builder", "pub fn hover_expansion"),
                ("SelectionList", "selection-list-only builder", "pub fn section"),
                ("SelectionList", "selection-list-only builder", "pub fn marker"),
                ("SelectionList", "selection-list-only builder", "pub fn more_row"),
            )
            failures.extend(
                f"{self.relative(options)}: {label} `{token}` must not be inside selection_options macro"
                for _, label, token in specialized_builders
                if token in macro_body
            )
            for target, label, token in specialized_builders:
                target_impl = source.split(f"impl {target}", maxsplit=1)[-1]
                if token not in target_impl:
                    failures.append(
                        f"{self.relative(options)}: {target} is missing {label} `{token}`"
                    )
        if accessors.exists():
            source = self.read(accessors)
            macro_body = source.split("selection_accessors!(Breadcrumb)", maxsplit=1)[0]
            combo_only_accessors = (
                "pub fn input_model",
                "pub fn filter_results",
                "pub fn allows_free_input",
            )
            failures.extend(
                f"{self.relative(accessors)}: combo-only accessor `{token}` must not be inside selection_accessors macro"
                for token in combo_only_accessors
                if token in macro_body
            )
            combo_impl = source.split("impl ComboBox", maxsplit=1)[-1]
            failures.extend(
                f"{self.relative(accessors)}: ComboBox is missing combo-only accessor `{token}`"
                for token in combo_only_accessors
                if token not in combo_impl
            )
            specialized_accessors = (
                ("Breadcrumb", "breadcrumb-only accessor", "pub fn crumb_action_model"),
                ("Tabs", "tabs-only accessor", "pub fn icon_action_model"),
                ("SideMenu", "side-menu-only accessor", "pub fn hover_expansion_model"),
                ("SelectionList", "selection-list-only accessor", "pub fn section_model"),
                ("SelectionList", "selection-list-only accessor", "pub fn marker_model"),
                ("SelectionList", "selection-list-only accessor", "pub fn has_more_row"),
            )
            failures.extend(
                f"{self.relative(accessors)}: {label} `{token}` must not be inside selection_accessors macro"
                for _, label, token in specialized_accessors
                if token in macro_body
            )
            for target, label, token in specialized_accessors:
                target_impl = source.split(f"impl {target}", maxsplit=1)[-1]
                if token not in target_impl:
                    failures.append(
                        f"{self.relative(accessors)}: {target} is missing {label} `{token}`"
                    )
        return failures

    def generic_contract_docs_source(self) -> str:
        paths = (
            self.root
            / "openspec/changes/establish-kuc-atoms-molecules-catalog/quality-gates-contract.md",
            self.root
            / "openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-catalog-contract.md",
            self.root
            / "openspec/changes/establish-kuc-atoms-molecules-catalog/specs/kuc-quality-gates/spec.md",
            self.root
            / "openspec/changes/establish-kuc-atoms-molecules-catalog/specs/kuc-storybook-catalog/spec.md",
        )
        return "\n".join(self.read(path) for path in paths if path.exists())

    def agent_stop_policy_failures(self) -> list[str]:
        agents = self.root / "AGENTS.md"
        if not agents.exists():
            return ["AGENTS.md: runner stop policy is missing"]

        source = self.read(agents)
        required_tokens = (
            "## runner 停止条件",
            "v0.1.0 release readiness が未達",
            "ローカル保存（commit）",
            "停止理由にしない",
            "push confirmation required",
            "release confirmation required",
            "destructive operation confirmation required",
            "次の未完了タスク",
        )
        return [
            f"agent runner stop policy missing token: {token}"
            for token in required_tokens
            if token not in source
        ]

    def agent_hook_policy_failures(self) -> list[str]:
        hook = self.root / ".githooks/pre-commit"
        push_hook = self.root / ".githooks/pre-push"
        installer = self.root / "scripts/install-git-hooks.sh"
        agents = self.root / "AGENTS.md"
        missing_files = [
            path for path in (hook, push_hook, installer, agents) if not path.exists()
        ]
        if missing_files:
            return [
                f"{self.relative(path)}: agent stop hook policy file is missing"
                for path in missing_files
            ]

        combined = "\n".join(
            self.read(path) for path in (hook, push_hook, installer, agents)
        )
        required_tokens = (
            "core.hooksPath .githooks",
            "just kuc-guardrails",
            "fix-and-continue",
            "KUC_PUSH_CONFIRMED",
            "push confirmation required",
            "release confirmation required",
            "destructive operation confirmation required",
            "ユーザー確認で止まらず",
        )
        failures = [
            f"agent stop hook policy missing token: {token}"
            for token in required_tokens
            if token not in combined
        ]
        forbidden_tokens = ("commit confirmation required",)
        failures.extend(
            f"local commit must not be a stop reason: {token}"
            for token in forbidden_tokens
            if token in combined
        )
        return failures

    def release_readiness_recipe_failures(self) -> list[str]:
        justfile = self.root / "Justfile"
        if not justfile.exists():
            return ["Justfile: kuc-guardrails release readiness recipe is missing"]

        source = self.read(justfile)
        if "kuc-guardrails:" not in source:
            return ["Justfile: kuc-guardrails recipe is missing"]

        lines = [line.strip() for line in source.splitlines()]
        has_consumer_contract_recipe = "consumer-app-contract:" in source
        has_integration_recipe = "integration-test: consumer-app-contract" in source
        has_e2e_recipe = "e2e-test:" in source and "storybook-requirement-gate.sh" in source
        has_smoke_recipe = "smoke-test: storybook-smoke storybook-interaction-smoke" in source
        has_consumer_app_test = any("test -p kuc-consumer-app --locked" in line for line in lines)
        generic_app_tests = (
            "--test generic_rust_app_contract --locked",
            "--test generic_rust_app_layout_contract --locked",
            "--test generic_rust_app_action_contract --locked",
        )
        missing_generic_app_tests = [
            target
            for target in generic_app_tests
            if not any(f"test -p katana-ui-core {target}" in line for line in lines)
        ]
        has_consumer_guardrail_dependency = any(
            line.startswith("kuc-guardrails:") and "consumer-app-contract" in line
            for line in lines
        )
        has_consumer_release_dependency = any(
            line.startswith("release-readiness-check:")
            and "integration-test" in line
            and "e2e-test" in line
            and "smoke-test" in line
            for line in lines
        )
        has_self_test = any(
            "scripts/assert-kuc-release-readiness.py --self-test" in line
            for line in lines
        )
        has_runtime_check = any(
            line.endswith("scripts/assert-kuc-release-readiness.py")
            and "--self-test" not in line
            for line in lines
        )
        failures: list[str] = []
        if not has_consumer_contract_recipe:
            failures.append("Justfile: consumer app contract recipe is missing")
        if not has_integration_recipe:
            failures.append("Justfile: integration-test must depend on consumer-app-contract")
        if not has_e2e_recipe:
            failures.append("Justfile: e2e-test must run storybook-requirement-gate")
        if not has_smoke_recipe:
            failures.append(
                "Justfile: smoke-test must cover storybook-smoke and storybook-interaction-smoke"
            )
        if not has_consumer_app_test:
            failures.append("Justfile: consumer app contract must run kuc-consumer-app tests")
        failures.extend(
            f"Justfile: consumer app contract must run generic_app tests {target}"
            for target in missing_generic_app_tests
        )
        if not has_consumer_guardrail_dependency:
            failures.append("Justfile: kuc-guardrails must depend on consumer-app-contract")
        if not has_consumer_release_dependency:
            failures.append(
                "Justfile: release-readiness-check must depend on integration-test, e2e-test, and smoke-test"
            )
        if not has_self_test:
            failures.append(
                "Justfile: kuc-guardrails must run release readiness guard self-test"
            )
        if not has_runtime_check:
            failures.append(
                "Justfile: kuc-guardrails must run release readiness guard runtime check"
            )
        return failures

    def storybook_panel_evidence_failures(self) -> list[str]:
        docs = self.guard_docs_source()
        required_tokens = (
            "storybook-panel-interaction-report.json",
            "story_selection",
            "theme_switch",
            "operation_sequence",
            "callback log",
            "target state id",
            "before / after summary",
        )
        return [
            f"storybook panel evidence missing token: {token}"
            for token in required_tokens
            if token not in docs
        ]

    def storybook_regression_recipe_failures(self) -> list[str]:
        justfile = self.root / "Justfile"
        if not justfile.exists():
            return ["Justfile: storybook-regression recipe is missing"]
        source = self.read(justfile)
        failures: list[str] = []
        if "storybook-manual-acceptance-smoke:" not in source:
            failures.append("Justfile: storybook-manual-acceptance-smoke recipe is missing")
        if "scripts/storybook_manual_acceptance_smoke.py" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-smoke must run the smoke script"
            )
        if "storybook-manual-acceptance-approval-template:" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-approval-template recipe is missing"
            )
        if "scripts/storybook_manual_acceptance_approval_template.py" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-approval-template must run the approval template script"
            )
        if "storybook-manual-acceptance-next:" not in source:
            failures.append("Justfile: storybook-manual-acceptance-next recipe is missing")
        if "scripts/storybook_manual_acceptance_next.py" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-next must run the next script"
            )
        if "storybook-manual-acceptance-status:" not in source:
            failures.append("Justfile: storybook-manual-acceptance-status recipe is missing")
        if "scripts/storybook_manual_acceptance_status.py" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-status must run the status script"
            )
        if "storybook-manual-acceptance-complete-next" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-complete-next recipe is missing"
            )
        if "scripts/storybook_manual_acceptance_complete_next.py" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-complete-next must run the complete-next script"
            )
        if "storybook-manual-acceptance-mark-approved" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-mark-approved recipe is missing"
            )
        if "scripts/storybook_manual_acceptance_mark_approved.py" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-mark-approved must run the mark-approved script"
            )
        if "storybook-manual-acceptance-approve page" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-approve recipe is missing"
            )
        if "scripts/storybook_manual_acceptance_approve.py" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-approve must run the approve script"
            )
        required_kuc_guardrail_tests = (
            "python3 scripts/test_next_storybook_page_change.py",
            "python3 scripts/test_storybook_manual_acceptance_queue.py",
            "python3 scripts/test_storybook_manual_acceptance_review.py",
            "python3 scripts/test_storybook_manual_acceptance_status.py",
            "python3 scripts/test_storybook_manual_acceptance_next.py",
            "python3 scripts/test_storybook_manual_acceptance_approval_template.py",
            "python3 scripts/test_storybook_manual_acceptance_complete_next.py",
            "python3 scripts/test_storybook_manual_acceptance_mark_approved.py",
            "python3 scripts/test_storybook_manual_acceptance_approve.py",
            "python3 scripts/test_storybook_manual_acceptance_smoke.py",
            "python3 scripts/test_storybook_manual_acceptance_final_gate.py",
            "python3 scripts/test_storybook_interaction_pending_only.py",
        )
        for command in required_kuc_guardrail_tests:
            if command not in source:
                failures.append(f"Justfile: kuc-guardrails must run {command}")
        if "storybook-manual-acceptance-final-gate:" not in source:
            failures.append("Justfile: storybook-manual-acceptance-final-gate recipe is missing")
        if "scripts/storybook_manual_acceptance_final_gate.py" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-final-gate must run the final gate"
            )
        if "storybook-kuc-dod-final:" not in source:
            failures.append("Justfile: storybook-kuc-dod-final recipe is missing")
        if not re.search(
            r"storybook-kuc-dod-final:.*storybook-manual-acceptance-final-gate.*storybook-interaction-smoke",
            source,
        ):
            failures.append(
                "Justfile: storybook-kuc-dod-final must require final gate and interaction smoke"
            )
        if "storybook-interaction-pending-only:" not in source:
            failures.append("Justfile: storybook-interaction-pending-only recipe is missing")
        if "scripts/storybook_interaction_pending_only.py" not in source:
            failures.append(
                "Justfile: storybook-interaction-pending-only must run the pending-only verifier"
            )
        pending_script = self.root / "scripts/storybook_interaction_pending_only.py"
        pending_test = self.root / "scripts/test_storybook_interaction_pending_only.py"
        if not pending_script.exists():
            failures.append(
                "scripts/storybook_interaction_pending_only.py: pending-only verifier is missing"
            )
        if not pending_test.exists():
            failures.append(
                "scripts/test_storybook_interaction_pending_only.py: pending-only verifier test is missing"
            )
        final_script = self.root / "scripts/storybook_manual_acceptance_final_gate.py"
        final_test = self.root / "scripts/test_storybook_manual_acceptance_final_gate.py"
        metadata_script = self.root / "scripts/storybook_manual_acceptance_metadata.py"
        if not final_script.exists():
            failures.append(
                "scripts/storybook_manual_acceptance_final_gate.py: manual acceptance final gate is missing"
            )
        if not final_test.exists():
            failures.append(
                "scripts/test_storybook_manual_acceptance_final_gate.py: manual acceptance final gate test is missing"
            )
        if not metadata_script.exists():
            failures.append(
                "scripts/storybook_manual_acceptance_metadata.py: manual acceptance metadata validator is missing"
            )
        if "--headless-interaction-audit" not in source:
            failures.append(
                "Justfile: storybook-manual-acceptance-smoke must regenerate live interaction audit"
            )
        regression_lines = [
            line.strip()
            for line in source.splitlines()
            if line.strip().startswith("storybook-regression:")
        ]
        if not regression_lines:
            failures.append("Justfile: storybook-regression recipe is missing")
        elif not any("storybook-manual-acceptance-smoke" in line for line in regression_lines):
            failures.append(
                "Justfile: storybook-regression must include storybook-manual-acceptance-smoke"
            )
        elif not any("storybook-interaction-pending-only" in line for line in regression_lines):
            failures.append(
                "Justfile: storybook-regression must include storybook-interaction-pending-only"
            )
        return failures

    def storybook_next_change_scope_failures(self) -> list[str]:
        script = self.root / "scripts/next-storybook-page-change.py"
        test = self.root / "scripts/test_next_storybook_page_change.py"
        required_sources = (
            (
                script,
                (
                    '"completion_scope": "storybook_page_leaf_changes"',
                    '"complete": kuc_dod_complete',
                    '"kuc_dod_complete": kuc_dod_complete',
                    "remaining_handoff_items = self.remaining_handoff_items()",
                    "kuc_dod_complete = not remaining_handoff_items",
                    "manual_acceptance_queue(manifest)",
                    '"pending_reason": "manual_acceptance_pending"',
                    '"next_manual_acceptance_page": next_page',
                    '"pending_manual_acceptance_pages": pending_pages',
                    "audit remaining P0/P1 handoff items",
                ),
            ),
            (
                test,
                (
                    "test_complete_payload_is_false_when_leaf_queue_is_done_but_kuc_dod_has_handoff_items",
                    "test_complete_payload_is_true_only_when_leaf_queue_and_kuc_dod_are_done",
                    "self.assertEqual(\"storybook_page_leaf_changes\", payload[\"completion_scope\"])",
                    "self.assertFalse(payload[\"kuc_dod_complete\"])",
                    "self.assertFalse(payload[\"complete\"])",
                    "self.assertTrue(payload[\"kuc_dod_complete\"])",
                    "test_payload_names_next_manual_acceptance_page_when_leaf_queue_is_done",
                    "payload[\"next_manual_acceptance_page\"]",
                    "payload[\"next_command\"]",
                    "remaining_handoff_items",
                ),
            ),
        )
        failures: list[str] = []
        for path, tokens in required_sources:
            if not path.exists():
                failures.append(f"{self.relative(path)}: Storybook next-change scope guard is missing")
                continue
            source = self.read(path)
            failures.extend(
                f"{self.relative(path)}: Storybook next-change scope guard missing token: {token}"
                for token in tokens
                if token not in source
            )
        return failures

    def storybook_remaining_handoff_failures(self) -> list[str]:
        candidates = sorted((self.root / "docs/reviews").glob("*kuc-remaining-work-handoff.md"))
        if not candidates:
            return ["docs/reviews/*kuc-remaining-work-handoff.md: KUC remaining work handoff is missing"]
        handoff = candidates[-1]
        source = self.read(handoff)
        required_tokens = (
            "P0",
            "P1",
            "manual_acceptance_pending",
            "text manual acceptance",
            "text_drag_selection",
            "text_keyboard_copy",
            "text_zero_distance_drag_no_selection",
            "progress-bar manual acceptance",
            "progress_timed_tick",
            "progress_timed_cycle",
            "progress_indeterminate_segment_motion",
            "storybook-interaction-smoke",
            "audit_status=verified",
        )
        failures = [
            f"{self.relative(handoff)}: remaining work handoff missing token `{token}`"
            for token in required_tokens
            if token not in source
        ]
        failures.extend(self.storybook_remaining_handoff_manifest_sync_failures(handoff, source))
        return failures

    def storybook_remaining_handoff_manifest_sync_failures(
        self,
        handoff: Path,
        handoff_source: str,
    ) -> list[str]:
        manifest = self.root / "docs/storybook-77ui-interaction-manifest.json"
        if not manifest.exists():
            return []
        payload = json.loads(self.read(manifest))
        entries = payload.get("ui", [])
        if not isinstance(entries, list):
            return []
        pending_pages = []
        for entry in entries:
            if not isinstance(entry, dict):
                continue
            page = entry.get("page")
            gaps = entry.get("gaps", [])
            if not isinstance(page, str) or not isinstance(gaps, list):
                continue
            if any("manual_acceptance_pending" in gap for gap in gaps if isinstance(gap, str)):
                pending_pages.append(page)
        return [
            f"{self.relative(handoff)}: manual pending page `{page}` missing from remaining work handoff"
            for page in pending_pages
            if page not in handoff_source
        ]

    def storybook_live_harness_dor_failures(self) -> list[str]:
        doc = self.root / "docs/storybook-live-harness-dor.md"
        if not doc.exists():
            return ["docs/storybook-live-harness-dor.md: Storybook live harness DoR is missing"]
        smoke = self.root / "scripts/storybook-interaction-smoke.sh"
        source = self.read(doc)
        required_tokens = (
            "解析レーン",
            "実作業レーン",
            "`storybook-interaction-smoke`",
            "interaction smoke として未成立",
            "checkbox / radio",
            "native window 経路",
            "screenshot を完了根拠にする",
        )
        return [
            f"{self.relative(doc)}: missing DoR token `{token}`"
            for token in required_tokens
            if token not in source
        ]
        if not smoke.exists():
            failures.append("scripts/storybook-interaction-smoke.sh: interaction smoke is missing")
            return failures
        smoke_source = self.read(smoke)
        smoke_tokens = (
            "--headless-interaction-audit",
            "storybook-live-interaction-audit.json",
            "checkbox_changed=true",
            "radio_changed=true",
            "body_pixel_diff",
        )
        failures.extend(
            f"{self.relative(smoke)}: missing live interaction smoke token `{token}`"
            for token in smoke_tokens
            if token not in smoke_source
        )
        return failures

    def visual_fallback_policy_failures(self) -> list[str]:
        docs = self.guard_docs_source()
        required_tokens = (
            "required_ui_fallbacks=0",
            "generic `node` fallback",
            "完了根拠にしない",
        )
        return [
            f"visual fallback policy missing token: {token}"
            for token in required_tokens
            if token not in docs
        ]

    def storybook_reflection_audit_policy_failures(self) -> list[str]:
        justfile = self.root / "Justfile"
        docs = self.guard_docs_source()
        if not justfile.exists():
            return ["Justfile: storybook reflection audit recipe is missing"]
        justfile_source = self.read(justfile)
        checks = (
            (justfile_source, "storybook-reflection-audit:", "Justfile"),
            (
                justfile_source,
                "scripts/assert-storybook-reflection-audit.py --strict",
                "Justfile",
            ),
            (
                justfile_source,
                "scripts/test_storybook_reflection_audit.py",
                "Justfile",
            ),
            (docs, "just storybook-reflection-audit", "guard docs"),
            (docs, "missing-*", "guard docs"),
            (docs, "page 固有 surface", "guard docs"),
        )
        return [
            f"{label}: storybook reflection audit missing token: {token}"
            for source, token, label in checks
            if token not in source
        ]

    def guard_docs_source(self) -> str:
        paths = (
            self.root / "docs/architecture/ui-separation/ui-core-parity-gap.md",
            self.root / "docs/architecture/ui-separation/owned-ui-task-map.md",
        )
        return "\n".join(self.read(path) for path in paths if path.exists())

    def typed_action_model_failures(self) -> list[str]:
        required_files = (
            self.root / "crates/katana-ui-core/src/interaction/mod.rs",
            self.root / "crates/katana-ui-core/src/component.rs",
            self.root / "crates/katana-ui-core/tests/interaction_contract.rs",
            self.root
            / "crates/katana-ui-core/tests/interaction_contract/callback_action_contract.rs",
        )
        missing_files = [path for path in required_files if not path.exists()]
        if missing_files:
            return [
                f"{self.relative(path)}: typed action model file is missing"
                for path in missing_files
            ]

        combined = "\n".join(
            (
                self.read_rust_dir(self.root / "crates/katana-ui-core/src/interaction"),
                self.read(self.root / "crates/katana-ui-core/src/component.rs"),
                self.read(self.root / "crates/katana-ui-core/tests/interaction_contract.rs"),
                self.read(
                    self.root
                    / "crates/katana-ui-core/tests/interaction_contract/callback_action_contract.rs"
                ),
            )
        )
        required_tokens = (
            "pub enum UiAction",
            "pub struct UiActionResult",
            "pub struct UiCallbackLog",
            "pub trait ComponentAction",
            "apply_action",
            "action_targets_only_the_matching_component_state",
            "action_result_is_serializable_snapshot",
            "callback_action_invokes_named_callback_without_mutating_value",
        )
        failures = [
            f"typed action model missing token: {token}"
            for token in required_tokens
            if token not in combined
        ]
        forbidden_tokens = (
            "ExternalStore",
            "GlobalStore",
            "external_store",
            "OnceLock",
            "static mut",
            "lazy_static",
        )
        failures.extend(
            f"typed action model must not require external store: {token}"
            for token in forbidden_tokens
            if token in combined
        )
        return failures

    def component_state_ownership_failures(self) -> list[str]:
        required_files = (
            self.root / "crates/katana-ui-core/src/state.rs",
            self.root / "crates/katana-ui-core/src/component.rs",
            self.root / "crates/katana-ui-core/src/atom/mod.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/window_interaction/state_store.rs",
            self.root / "crates/katana-ui-core-storybook/src/visual/window_interaction.rs",
            self.root
            / "crates/katana-ui-core-storybook/src/visual/window_interaction/tests/navigation_tests.rs",
            self.root / "crates/katana-ui-core/tests/interaction_contract.rs",
            self.root
            / "openspec/changes/establish-kuc-atoms-molecules-catalog/core-foundation-contract.md",
        )
        missing_files = [path for path in required_files if not path.exists()]
        if missing_files:
            return [
                f"{self.relative(path)}: component state ownership file is missing"
                for path in missing_files
            ]

        state_model = self.read(required_files[0])
        component_model = self.read(required_files[1])
        atom_model = self.read(required_files[2])
        state_store = self.read(required_files[3])
        window_state = self.read(required_files[4])
        navigation_tests = self.read(required_files[5])
        core_contract = "\n".join(
            (
                self.read(required_files[6]),
                self.read_rust_dir(self.root / "crates/katana-ui-core/tests/interaction_contract"),
                self.read_rust_dir(self.root / "crates/katana-ui-core/tests/core_contract"),
            )
        )
        foundation_contract = self.read(required_files[7])
        combined = "\n".join(
            (
                state_model,
                component_model,
                atom_model,
                state_store,
                window_state,
                navigation_tests,
                core_contract,
                foundation_contract,
            )
        )
        required_tokens = (
            "UiStateHandle",
            "UiComponentState",
            "ComponentStateBinding",
            "state_snapshot",
            "sync_state",
            "set/update",
            "component_id",
            "selected_component_presets",
            "preset_tab_selection_is_owned_by_component",
            "action_targets_only_the_matching_component_state",
            "complex_ui_state_is_owned_by_the_component_model",
            "app_global_state_updates_component_owned_state_via_handle",
            "state_handle_supports_react_like_get_set_and_update_without_global_store",
        )
        failures = [
            f"component state ownership missing token: {token}"
            for token in required_tokens
            if token not in combined
        ]
        forbidden_patterns = (
            (state_store, r"\bpage\s*:\s*&'static str", "storybook state key must not be page-owned"),
            (
                window_state,
                r"\bselected_presets\b",
                "storybook preset state must be component-owned",
            ),
            (
                window_state,
                r"\bglobal_(state|store)\b",
                "component state must not use global state/store",
            ),
        )
        for source, pattern, message in forbidden_patterns:
            if re.search(pattern, source):
                failures.append(message)
        return failures

    def public_app_shell_failures(self) -> list[str]:
        checked_paths = (
            self.root / "crates/katana-ui-core/src/molecule/mod.rs",
            self.root / "crates/katana-ui-core/src/widget/molecules.rs",
            self.root / "crates/katana-ui-core/src/render_model/kind.rs",
            self.root / "crates/katana-ui-core-storybook/src/catalog/preset_labels.rs",
        )
        failures: list[str] = []
        for path in checked_paths:
            if not path.exists():
                continue
            source = self.read(path)
            if "AppShell" in source or "app-shell" in source:
                failures.append(
                    f"{self.relative(path)}: AppShell is outside the public KUC molecule scope"
                )
        return failures


def main() -> int:
    failures = KucGuardrails(Path.cwd()).run()
    if failures:
        print("katana-ui-core guardrails failed", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
