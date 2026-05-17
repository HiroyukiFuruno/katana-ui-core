mod panel_build;
mod panel_verify;

use crate::catalog::{StoryExample, StorybookPanelInteractionReport, StorybookStyleSheet};
use katana_ui_core::atom::Text;
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
        if let Some(example) = examples.iter().find(|it| it.page == selected_page) {
            panel = panel.child(story_root(example, &self.theme));
        }
        for example in examples {
            if example.page != selected_page {
                panel = panel.child(story_root(example, &self.theme));
            }
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

#[cfg(test)]
mod tests {
    use super::StorybookPanel;
    use crate::catalog::StoryCatalog;
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn storybook_panel_is_rendered_by_kuc_core_with_theme() {
        let examples = StoryCatalog.examples();
        let report = StorybookPanel::new(ThemeSnapshot::dark()).verify(&examples);

        assert_eq!(3, report.panel_nodes);
        assert!(report.panel_theme_configured);
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

        assert_eq!(3, report.panel_nodes);
        assert!(report.panel_theme_configured);
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
