use super::{StoryExample, StorybookOperationSequences};
use crate::panel::StorybookPanel;
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use katana_ui_core::theme::ThemeSnapshot;
use serde::{Deserialize, Serialize};

const SELECTED_PAGE: &str = "button";
const NAVIGATION_LABEL: &str = "Navigation";
const PREVIEW_LABEL: &str = "Preview";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorybookPanelInteractionReport {
    pub story_selection: StorySelectionReport,
    pub theme_switch: ThemeSwitchReport,
    pub operation_sequence: Vec<OperationStepReport>,
    pub selector_operations: Vec<OperationStepReport>,
    pub overlay_dismissals: Vec<OperationStepReport>,
    pub color_picker_updates: Vec<OperationStepReport>,
    pub callback_log: Vec<CallbackLogReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorySelectionReport {
    pub selected_page: String,
    pub preview_page: String,
    pub navigation_items: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeSwitchReport {
    pub before_theme_id: String,
    pub after_theme_id: String,
    pub theme_control: bool,
    pub root_theme_id: String,
    pub navigation_theme_id: String,
    pub preview_theme_id: String,
    pub story_root_theme_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationStepReport {
    pub action: String,
    pub target_state_id: String,
    pub before_summary: String,
    pub after_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackLogReport {
    pub action: String,
    pub target_state_id: String,
    pub before_summary: String,
    pub after_summary: String,
}

impl StorybookPanelInteractionReport {
    #[must_use]
    pub fn build(examples: &[StoryExample]) -> Self {
        let before_theme = ThemeSnapshot::light();
        let after_theme = ThemeSnapshot::dark();
        let tree = StorybookPanel::new(after_theme.clone()).build_selected(examples, SELECTED_PAGE);
        let root = tree.root();
        let preview = panel_child(root, PREVIEW_LABEL);
        let selected_story = examples
            .iter()
            .find(|it| it.page == SELECTED_PAGE)
            .or_else(|| examples.first());
        let callback_log = selected_story
            .map(|it| report_callback_logs(&it.callback_logs))
            .unwrap_or_default();

        Self {
            story_selection: StorySelectionReport {
                selected_page: SELECTED_PAGE.to_string(),
                preview_page: preview_page(preview).unwrap_or_default(),
                navigation_items: examples.len(),
            },
            theme_switch: ThemeSwitchReport {
                before_theme_id: before_theme.id.as_str().to_string(),
                after_theme_id: after_theme.id.as_str().to_string(),
                theme_control: true,
                root_theme_id: root.props().theme_id.clone(),
                navigation_theme_id: panel_theme(root, NAVIGATION_LABEL).unwrap_or_default(),
                preview_theme_id: panel_theme(root, PREVIEW_LABEL).unwrap_or_default(),
                story_root_theme_id: story_root_theme(preview).unwrap_or_default(),
            },
            operation_sequence: operation_sequence(&callback_log),
            selector_operations: StorybookOperationSequences::selector_operations(examples),
            overlay_dismissals: StorybookOperationSequences::overlay_dismissals(examples),
            color_picker_updates: StorybookOperationSequences::color_picker_updates(examples),
            callback_log,
        }
    }

    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "story_selection={} theme_switch={}->{} theme_control={} operation_sequence={} selector_operations={} overlay_dismissals={} color_picker_updates={} callback_log={}",
            self.story_selection.selected_page,
            self.theme_switch.before_theme_id,
            self.theme_switch.after_theme_id,
            self.theme_switch.theme_control,
            self.operation_sequence.len(),
            self.selector_operations.len(),
            self.overlay_dismissals.len(),
            self.color_picker_updates.len(),
            self.callback_log.len()
        )
    }
}

fn operation_sequence(callback_log: &[CallbackLogReport]) -> Vec<OperationStepReport> {
    callback_log
        .iter()
        .map(|it| OperationStepReport {
            action: it.action.clone(),
            target_state_id: it.target_state_id.clone(),
            before_summary: it.before_summary.clone(),
            after_summary: it.after_summary.clone(),
        })
        .collect()
}

fn panel_child<'a>(root: &'a UiNode, label: &str) -> Option<&'a UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == label)
}

fn panel_theme(root: &UiNode, label: &str) -> Option<String> {
    panel_child(root, label).map(|it| it.props().theme_id.clone())
}

fn preview_page(preview: Option<&UiNode>) -> Option<String> {
    preview
        .and_then(|it| it.children().first())
        .map(|it| it.props().label.clone())
}

fn story_root_theme(preview: Option<&UiNode>) -> Option<String> {
    preview
        .and_then(|it| it.children().first())
        .map(|it| it.props().theme_id.clone())
}

fn report_callback_logs(callback_logs: &[UiCallbackLog]) -> Vec<CallbackLogReport> {
    callback_logs
        .iter()
        .map(|it| CallbackLogReport {
            action: it.action.clone(),
            target_state_id: it.target.as_str().to_string(),
            before_summary: it.before.clone(),
            after_summary: it.after.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::StorybookPanelInteractionReport;
    use crate::catalog::StoryCatalog;

    #[test]
    fn report_covers_selector_overlay_and_color_picker_sequences() {
        let examples = StoryCatalog.examples();
        let report = StorybookPanelInteractionReport::build(&examples);

        assert_eq!(7, report.selector_operations.len());
        assert_eq!(3, report.overlay_dismissals.len());
        assert_eq!(3, report.color_picker_updates.len());
        assert!(
            report
                .selector_operations
                .iter()
                .any(|it| it.action == "combo-box-arrow-down")
        );
        assert!(
            report
                .overlay_dismissals
                .iter()
                .any(|it| it.action == "modal-overlay-escape")
        );
        assert!(
            report
                .color_picker_updates
                .iter()
                .any(|it| it.action == "color-picker-drag-alpha")
        );
    }
}
