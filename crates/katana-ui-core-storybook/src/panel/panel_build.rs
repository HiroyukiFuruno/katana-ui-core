use super::StorybookPanel;
use crate::catalog::StoryExample;
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::UiTree;

impl StorybookPanel {
    pub fn build(&self, examples: &[StoryExample]) -> UiTree {
        self.build_selected(examples, examples.first().map_or("", |it| it.page))
    }

    pub fn build_selected(&self, examples: &[StoryExample], selected_page: &str) -> UiTree {
        UiTree::new(
            Panel::new(
                "katana-ui-core Storybook",
                PanelRegion::Root,
                self.theme.clone(),
            )
            .child(self.navigation_panel(examples))
            .child(self.preview_panel(examples, selected_page)),
        )
    }
}
