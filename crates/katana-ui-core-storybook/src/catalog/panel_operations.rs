use super::StoryExample;
use super::panel_interaction::{
    LegacyDodReports, LegacyUiMarkerReport, OperationStepReport, PresetDifferenceReport,
    SettingsMutationReport,
};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::molecule::{DisclosureTriggerArea, TreeLineStyle, TreeNode, TreeView};

const SELECTOR_PAGES: &[&str] = &["select-box", "combo-box", "menu-button", "segmented-toggle"];
const OVERLAY_PAGES: &[&str] = &[
    "popover",
    "tooltip",
    "modal",
    "modal-overlay",
    "notification-toast",
];
const COLOR_PICKER_PAGES: &[&str] = &["color-picker-rgba"];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StorybookOperationSequences;

impl StorybookOperationSequences {
    pub(crate) fn selector_operations(examples: &[StoryExample]) -> Vec<OperationStepReport> {
        Self::operations_from_pages(examples, SELECTOR_PAGES)
    }

    pub(crate) fn overlay_dismissals(examples: &[StoryExample]) -> Vec<OperationStepReport> {
        Self::primary_operations_from_pages(examples, OVERLAY_PAGES)
    }

    pub(crate) fn color_picker_updates(examples: &[StoryExample]) -> Vec<OperationStepReport> {
        Self::operations_from_pages(examples, COLOR_PICKER_PAGES)
    }

    pub(crate) fn settings_mutations(examples: &[StoryExample]) -> Vec<SettingsMutationReport> {
        LegacyDodReports::settings_mutations(examples)
    }

    pub(crate) fn legacy_ui_markers(examples: &[StoryExample]) -> Vec<LegacyUiMarkerReport> {
        LegacyDodReports::ui_markers(examples)
    }

    pub(crate) fn preset_differences(examples: &[StoryExample]) -> Vec<PresetDifferenceReport> {
        LegacyDodReports::preset_differences(examples)
    }

    pub(crate) fn tree_view_option_mutations() -> Vec<OperationStepReport> {
        let mut tree = TreeView::new("Tree settings")
            .item(TreeNode::new("root", "Root", 0).directory())
            .line_display(false)
            .icons_visible(false)
            .default_open(false);
        let target = tree.state_id().clone();
        let mut steps = Vec::new();

        let before = tree_options_summary(&tree);
        tree = tree.line_display(true);
        push_step(&mut steps, "tree_line_display", &target, before, &tree);
        let before = tree_options_summary(&tree);
        tree = tree.line_style(TreeLineStyle::Dashed);
        push_step(&mut steps, "tree_line_style", &target, before, &tree);
        let before = tree_options_summary(&tree);
        tree = tree.line_width(2);
        push_step(&mut steps, "tree_line_width", &target, before, &tree);
        let before = tree_options_summary(&tree);
        tree = tree.icons_visible(true);
        push_step(&mut steps, "tree_icons_visible", &target, before, &tree);
        let before = tree_options_summary(&tree);
        tree = tree.directory_icon("<svg data-icon=\"branch\"/>");
        push_step(
            &mut steps,
            "tree_branch_marker_icon",
            &target,
            before,
            &tree,
        );
        let before = tree_options_summary(&tree);
        tree = tree.file_icon("<svg data-icon=\"leaf\"/>");
        push_step(&mut steps, "tree_leaf_marker_icon", &target, before, &tree);
        let before = tree_options_summary(&tree);
        tree = tree.tree_font_role("body").tree_theme_id("dark");
        push_step(&mut steps, "tree_font_theme", &target, before, &tree);
        let before = tree_options_summary(&tree);
        tree = tree.empty_area_context_menu(true);
        push_step(&mut steps, "tree_context_menu", &target, before, &tree);
        let before = tree_options_summary(&tree);
        tree = tree.default_open(true);
        push_step(&mut steps, "tree_default_open", &target, before, &tree);
        let before = tree_options_summary(&tree);
        tree = tree.toggle_icon("<svg data-icon=\"chevron\"/>");
        push_step(&mut steps, "tree_toggle_icon", &target, before, &tree);
        let before = tree_options_summary(&tree);
        tree = tree.toggle_trigger_area(DisclosureTriggerArea::IconAndText);
        push_step(&mut steps, "tree_toggle_trigger", &target, before, &tree);

        let result = tree.apply_action(&UiAction::click(target));
        steps.push(OperationStepReport {
            action: "tree_click_toggle".to_string(),
            target_state_id: result.target.as_str().to_string(),
            before_summary: result.before.summary(),
            after_summary: result.after.summary(),
        });
        steps
    }

    fn operations_from_pages(
        examples: &[StoryExample],
        pages: &[&str],
    ) -> Vec<OperationStepReport> {
        examples
            .iter()
            .filter(|example| pages.contains(&example.page))
            .flat_map(|example| example.callback_logs.iter().map(Self::operation_from_log))
            .collect()
    }

    fn primary_operations_from_pages(
        examples: &[StoryExample],
        pages: &[&str],
    ) -> Vec<OperationStepReport> {
        examples
            .iter()
            .filter(|example| pages.contains(&example.page))
            .filter_map(|example| example.callback_logs.first().map(Self::operation_from_log))
            .collect()
    }

    fn operation_from_log(log: &UiCallbackLog) -> OperationStepReport {
        OperationStepReport {
            action: log.action.clone(),
            target_state_id: log.target.as_str().to_string(),
            before_summary: log.before.clone(),
            after_summary: log.after.clone(),
        }
    }
}

fn push_step(
    steps: &mut Vec<OperationStepReport>,
    action: &str,
    target: &katana_ui_core::render_model::UiStateId,
    before_summary: String,
    tree: &TreeView,
) {
    steps.push(OperationStepReport {
        action: action.to_string(),
        target_state_id: target.as_str().to_string(),
        before_summary,
        after_summary: tree_options_summary(tree),
    });
}

fn tree_options_summary(tree: &TreeView) -> String {
    format!(
        "lines={} style={:?} width={} icons={} branch_marker={} leaf_marker={} font={} theme={} context_menu={} default_open={} toggle_icon={} trigger={:?}",
        tree.line_display_model(),
        tree.line_style_model(),
        tree.line_width_model(),
        tree.icons_visible_model(),
        tree.directory_icon_model(),
        tree.file_icon_model(),
        tree.tree_font_role_model(),
        tree.tree_theme_id_model(),
        tree.empty_area_context_menu_model(),
        tree.default_open_model(),
        tree.toggle_icon_model(),
        tree.toggle_trigger_area_model()
    )
}
