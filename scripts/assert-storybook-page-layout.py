#!/usr/bin/env python3
from pathlib import Path
import sys

STORYBOOK_FILES = (
    Path("storybook/Cargo.toml"),
    Path("storybook/src/main.rs"),
    Path("crates/katana-ui-core-storybook/Cargo.toml"),
    Path("crates/katana-ui-core-storybook/src/lib.rs"),
    Path("crates/katana-ui-core-storybook/src/main.rs"),
    Path("crates/katana-ui-core-storybook/src/snapshot_output.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/mod.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/atoms.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/molecules.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/molecules/molecule_basic.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/molecules/molecule_heavy.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/molecules/molecule_interaction.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/layouts.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_interaction.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_operations.rs"),
    Path("crates/katana-ui-core-storybook/src/catalog/panel_report.rs"),
    Path("crates/katana-ui-core-storybook/src/panel.rs"),
    Path("crates/katana-ui-core-storybook/src/panel/panel_build.rs"),
    Path("crates/katana-ui-core-storybook/src/panel/panel_verify.rs"),
    Path("crates/katana-ui-core-storybook/src/requirements.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/card.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/coverage.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_basic.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_common.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_complex.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/dedicated_feedback.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/inspector.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/layout_metrics.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/modal.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/mod.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/navigation.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/navigation_icons.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/navigation_tree.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/palette.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/preset_tabs.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/preview.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/preview_contract.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/preview_contract_rows.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/preview_detail.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/render.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/render_context.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/runtime.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/scrollbar.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/shell.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/canvas.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/text.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/text_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/types.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/visual_tests.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_interaction.rs"),
    Path("crates/katana-ui-core-storybook/src/visual/window_options.rs"),
)

DOC_FILES = (
    Path("docs/architecture/ui-separation/ui-core-parity-gap.md"),
    Path("docs/architecture/ui-separation/owned-ui-task-map.md"),
    Path("tmp/reports/2026-05-17-overnight-residual-scope.md"),
)


def main() -> int:
    source = "\n".join(path.read_text(encoding="utf-8") for path in STORYBOOK_FILES)
    docs = "\n".join(path.read_text(encoding="utf-8") for path in DOC_FILES)
    required = (
        "StorybookPanel::verify_theme_variants",
        "ThemeSnapshot::light()",
        "ThemeSnapshot::dark()",
        "StorybookStyleSheet",
        "styled_story_roots",
        "StorybookVisual",
        "--visual-snapshot",
        "SnapshotOutput::prepare",
        "SnapshotOutput::evidence",
        "modified_unix={}",
        "--open-modal-window",
        "--runtime-regression",
        "modal_window_opened={}",
        "state_reflected={}",
        "overlay_rendered={}",
        "modal_plan_same_display",
        "render_summary()",
        "katana-ui-core-storybook:",
    )
    current_panel_tokens = (
        "PanelRegion::Navigation",
        "PanelRegion::Preview",
        "PanelRegion::Details",
        "navigation_panel",
        "preview_panel",
        "details_panel",
    )
    source_evidence_tokens = (
        "StorybookPanelInteractionReport",
        "CallbackLogReport",
        "selector_operations",
        "overlay_dismissals",
        "color_picker_updates",
        "theme_control",
        "modal_required",
        "theme_difference_pixels",
        "operation_difference_pixels",
        "required_ui_fallbacks",
        "PRESET_ACTIVE_BOTTOM_BORDER_HEIGHT",
        "KATANA_TAB_BORDER_WIDTH",
        "TreeExpansionState",
        "scrollbar_visible",
        "PreviewContract",
        "main_window_options",
        "ScaleMode::AspectRatioStretch",
        "operation_preset_changes_tab_and_canvas_pixels",
        "click_mapping_can_select_visible_story_and_change_rendered_scene",
    )
    evidence_tokens = (
        "storybook-panel-interaction-report.json",
        "callback log",
        "required_ui_fallbacks=0",
        "generic `node` fallback",
    )
    missing = [token for token in required if token not in source]
    missing.extend(token for token in current_panel_tokens if token not in source)
    missing.extend(token for token in source_evidence_tokens if token not in source)
    missing.extend(token for token in evidence_tokens if token not in docs)
    forbidden = (
        "katana-ui-core-floem",
        "katana_ui_core_floem",
        "floem",
        "floem::",
        "Application::new()",
    )
    leaked = []
    for path in STORYBOOK_FILES:
        candidate = path.read_text(encoding="utf-8")
        for token in forbidden:
            if token in candidate:
                leaked.append(f"{path}:{token}")
    if missing:
        print("storybook core-only layout lint failed", file=sys.stderr)
        for token in missing:
            print(f"- missing token: {token}", file=sys.stderr)
        return 1
    if leaked:
        print("storybook must not render through Floem", file=sys.stderr)
        for token in leaked:
            print(f"- forbidden token: {token}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
