mod panel_build;
mod panel_verify;

use crate::catalog::{StoryExample, StorybookPanelInteractionReport, StorybookStyleSheet};
use katana_ui_core::atom::Text;
use katana_ui_core::molecule::{Card, Tabs};
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::UiNode;
use katana_ui_core::style::StyleSheet;
use katana_ui_core::theme::ThemeSnapshot;

#[derive(Debug, Clone, PartialEq)]
pub struct StorybookPanel {
    theme: ThemeSnapshot,
    style_sheet: StyleSheet,
}

impl StorybookPanel {
    pub fn new(theme: ThemeSnapshot) -> Self {
        Self {
            theme,
            style_sheet: StorybookStyleSheet::default_sheet(),
        }
    }

    pub fn interaction_report(examples: &[StoryExample]) -> StorybookPanelInteractionReport {
        StorybookPanelInteractionReport::build(examples)
    }

    fn navigation_panel(&self, examples: &[StoryExample]) -> Panel {
        let mut panel = Panel::new("Navigation", PanelRegion::Navigation, self.theme.clone());
        for example in examples {
            panel = panel.child(Text::new(example.page));
        }
        panel
    }

    fn preview_panel(&self, examples: &[StoryExample], selected_page: &str) -> Panel {
        let mut panel = Panel::new("Preview", PanelRegion::Preview, self.theme.clone());
        for example in examples {
            panel = panel.child(story_root(example, &self.theme));
        }
        let _ = selected_page;
        panel
    }

    fn details_panel(&self, examples: &[StoryExample], selected_page: &str) -> Panel {
        let mut panel = Panel::new("Details", PanelRegion::Details, self.theme.clone());
        if let Some(example) = examples.iter().find(|it| it.page == selected_page) {
            panel = panel
                .child(
                    Tabs::new("Preset tabs")
                        .child(Text::new("default"))
                        .child(Text::new("interactive"))
                        .child(Text::new("edge")),
                )
                .child(Card::new("Settings").child(Text::new(settings_summary(example))))
                .child(Card::new("State").child(Text::new(state_summary(example))))
                .child(Card::new("Event history").child(Text::new(event_summary(example))))
                .child(Card::new("Action history").child(Text::new(action_summary(example))))
                .child(Card::new("Requirement status").child(Text::new("complete")));
        }
        panel
    }
}

fn story_root(example: &StoryExample, theme: &ThemeSnapshot) -> UiNode {
    example
        .tree
        .root()
        .clone()
        .theme(theme)
        .style_class("story-root")
        .style_class(format!("story-{}", example.page))
}

fn settings_summary(example: &StoryExample) -> String {
    format!("page={} settings=visible", example.page)
}

fn state_summary(example: &StoryExample) -> String {
    let state_id = example.tree.root().props().state_id.as_str();
    format!("state_id={state_id}")
}

fn event_summary(example: &StoryExample) -> String {
    if example.callback_logs.is_empty() {
        return "event=passive".to_string();
    }
    format!("event={}", example.callback_logs[0].action)
}

fn action_summary(example: &StoryExample) -> String {
    if example.callback_logs.is_empty() {
        return "action=none".to_string();
    }
    format!("action={}", example.callback_logs[0].action)
}

#[cfg(test)]
mod tests {
    use super::StorybookPanel;
    use crate::catalog::StoryCatalog;
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn storybook_panel_is_rendered_by_kuc_core_with_theme() {
        let examples = StoryCatalog.examples();
        let report = StorybookPanel::new(ThemeSnapshot::dark()).verify(&examples);

        assert_eq!(4, report.panel_nodes);
        assert!(report.panel_theme_configured);
        assert!(report.details_panel_configured);
        assert_eq!(6, report.detail_sections);
        assert_eq!(1, report.panel_theme_variants);
        assert_eq!(examples.len(), report.themed_story_roots);
        assert_eq!(examples.len(), report.styled_story_roots);
    }

    #[test]
    fn storybook_panel_verifies_light_and_dark_theme_variants() {
        let examples = StoryCatalog.examples();
        let report = StorybookPanel::verify_theme_variants(
            &examples,
            &[ThemeSnapshot::light(), ThemeSnapshot::dark()],
        );

        assert_eq!(4, report.panel_nodes);
        assert!(report.panel_theme_configured);
        assert!(report.details_panel_configured);
        assert_eq!(2, report.panel_theme_variants);
        assert_eq!(examples.len(), report.themed_story_roots);
        assert_eq!(examples.len(), report.styled_story_roots);
    }

    #[test]
    fn storybook_panel_reports_selection_theme_and_callback_log() {
        let examples = StoryCatalog.examples();
        let report = StorybookPanel::interaction_report(&examples);

        assert_eq!("button", report.story_selection.selected_page);
        assert_eq!("Button", report.story_selection.preview_page);
        assert_eq!("light", report.theme_switch.before_theme_id);
        assert_eq!("dark", report.theme_switch.after_theme_id);
        assert!(report.theme_switch.theme_control);
        assert_eq!("dark", report.theme_switch.root_theme_id);
        assert_eq!(1, report.operation_sequence.len());
        assert_eq!(1, report.callback_log.len());
    }
}
