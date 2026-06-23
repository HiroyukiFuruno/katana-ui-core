mod panel_build;
mod panel_verify;

use crate::catalog::{
    StoryDetailContent, StoryExample, StoryPresetLabels, StorybookPanelInteractionReport,
    StorybookStyleSheet,
};
use katana_ui_core::atom::Text;
use katana_ui_core::molecule::{Card, Tabs};
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::UiNode;
use katana_ui_core::style::StyleSheet;
use katana_ui_core::theme::ThemeSnapshot;

const ROOT_SCROLL_VIEWPORT: u32 = 920;
const ROOT_SCROLL_CONTENT: u32 = 3840;
const NAV_SCROLL_VIEWPORT: u32 = 760;
const NAV_SCROLL_CONTENT: u32 = 1480;
const PREVIEW_SCROLL_VIEWPORT: u32 = 520;
const PREVIEW_SCROLL_CONTENT: u32 = 1260;
const DETAILS_SCROLL_VIEWPORT: u32 = 760;
const DETAILS_SCROLL_CONTENT: u32 = 1320;

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
        let mut panel = Panel::new("Navigation", PanelRegion::Navigation, self.theme.clone())
            .vertical_scroll(0, NAV_SCROLL_VIEWPORT, NAV_SCROLL_CONTENT, true);
        for example in examples {
            panel = panel.child(Text::new(example.page));
        }
        panel
    }

    fn preview_panel(&self, examples: &[StoryExample], selected_page: &str) -> Panel {
        let mut panel = Panel::new("Preview", PanelRegion::Preview, self.theme.clone())
            .vertical_scroll(0, PREVIEW_SCROLL_VIEWPORT, PREVIEW_SCROLL_CONTENT, true);
        if let Some(example) = selected_example(examples, selected_page) {
            panel = panel.child(story_root(example, &self.theme));
        }
        panel
    }

    fn details_panel(&self, examples: &[StoryExample], selected_page: &str) -> Panel {
        let mut panel = Panel::new("Details", PanelRegion::Details, self.theme.clone())
            .vertical_scroll(0, DETAILS_SCROLL_VIEWPORT, DETAILS_SCROLL_CONTENT, true);
        if let Some(example) = examples.iter().find(|it| it.page == selected_page) {
            let content = StoryDetailContent::from_example(example);
            let mut preset_tabs = Tabs::new("Preset tabs");
            for label in StoryPresetLabels::for_page(example.page).iter().copied() {
                preset_tabs = preset_tabs.child(Text::new(label));
            }
            panel = panel
                .child(preset_tabs)
                .child(Card::new("Settings").child(Text::new(content.settings)))
                .child(Card::new("State").child(Text::new(content.state)))
                .child(Card::new("Event history").child(Text::new(content.event)))
                .child(Card::new("Action history").child(Text::new(content.action)))
                .child(Card::new("Requirement status").child(Text::new(content.quality)));
        }
        panel
    }
}

fn selected_example<'a>(
    examples: &'a [StoryExample],
    selected_page: &str,
) -> Option<&'a StoryExample> {
    examples
        .iter()
        .find(|it| it.page == selected_page)
        .or_else(|| examples.first())
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

        assert_eq!(4, report.panel_nodes);
        assert!(report.panel_theme_configured);
        assert!(report.details_panel_configured);
        assert_eq!(6, report.detail_sections);
        assert!(report.panel_scroll_configured);
        assert_eq!(4, report.independent_panel_scrolls);
        assert_eq!(1, report.panel_theme_variants);
        assert_eq!(1, report.themed_story_roots);
        assert_eq!(1, report.styled_story_roots);
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
        assert!(report.panel_scroll_configured);
        assert_eq!(4, report.independent_panel_scrolls);
        assert_eq!(1, report.themed_story_roots);
        assert_eq!(1, report.styled_story_roots);
    }

    #[test]
    fn storybook_panel_reports_selection_theme_and_callback_log() {
        let examples = StoryCatalog.examples();
        let report = StorybookPanel::interaction_report(&examples);

        assert_eq!("text-input", report.story_selection.selected_page);
        assert_eq!("Text input", report.story_selection.preview_page);
        assert_eq!("light", report.theme_switch.before_theme_id);
        assert_eq!("dark", report.theme_switch.after_theme_id);
        assert!(report.theme_switch.theme_control);
        assert_eq!("dark", report.theme_switch.root_theme_id);
        assert_eq!(1, report.operation_sequence.len());
        assert_eq!(1, report.callback_log.len());
    }
}
