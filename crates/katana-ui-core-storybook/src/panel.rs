use crate::catalog::StoryExample;
use katana_ui_core::atom::Text;
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::{UiNode, UiNodeKind, UiTree};
use katana_ui_core::style::{StyleDeclaration, StyleProperty, StyleRule, StyleSheet, StyleValue};
use katana_ui_core::theme::ThemeSnapshot;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorybookPanelReport {
    pub panel_nodes: usize,
    pub panel_theme_configured: bool,
    pub panel_theme_variants: usize,
    pub themed_story_roots: usize,
    pub styled_story_roots: usize,
    panel_theme_ids: BTreeSet<String>,
}

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

    pub fn build(&self, examples: &[StoryExample]) -> UiTree {
        UiTree::new(
            Panel::new(
                "katana-ui-core Storybook",
                PanelRegion::Root,
                self.theme.clone(),
            )
            .child(self.navigation_panel(examples))
            .child(self.preview_panel(examples)),
        )
    }

    pub fn verify(&self, examples: &[StoryExample]) -> StorybookPanelReport {
        let tree = self.build(examples);
        let mut theme_ids = BTreeSet::new();
        Self::collect_panel_theme_ids(tree.root(), &mut theme_ids);
        StorybookPanelReport {
            panel_nodes: Self::panel_count(tree.root()),
            panel_theme_configured: Self::panel_theme_is_configured(tree.root()),
            panel_theme_variants: theme_ids.len(),
            themed_story_roots: Self::themed_story_root_count(tree.root()),
            styled_story_roots: self.styled_story_root_count(tree.root()),
            panel_theme_ids: theme_ids,
        }
    }

    pub fn verify_theme_variants(
        examples: &[StoryExample],
        themes: &[ThemeSnapshot],
    ) -> StorybookPanelReport {
        let mut panel_nodes = 0;
        let mut theme_ids = BTreeSet::new();
        let mut panel_theme_configured = true;
        let mut themed_story_roots = 0;
        let mut styled_story_roots = 0;

        for theme in themes {
            let report = Self::new(theme.clone()).verify(examples);
            panel_nodes = panel_nodes.max(report.panel_nodes);
            panel_theme_configured = panel_theme_configured && report.panel_theme_configured;
            themed_story_roots = themed_story_roots.max(report.themed_story_roots);
            styled_story_roots = styled_story_roots.max(report.styled_story_roots);
            theme_ids.extend(report.panel_theme_ids);
        }

        StorybookPanelReport {
            panel_nodes,
            panel_theme_configured,
            panel_theme_variants: theme_ids.len(),
            themed_story_roots,
            styled_story_roots,
            panel_theme_ids: theme_ids,
        }
    }

    fn navigation_panel(&self, examples: &[StoryExample]) -> Panel {
        let mut panel = Panel::new("Navigation", PanelRegion::Navigation, self.theme.clone());
        for example in examples {
            panel = panel.child(Text::new(example.page));
        }
        panel
    }

    fn preview_panel(&self, examples: &[StoryExample]) -> Panel {
        let mut panel = Panel::new("Preview", PanelRegion::Preview, self.theme.clone());
        for example in examples {
            panel = panel.child(
                example
                    .tree
                    .root()
                    .clone()
                    .theme(&self.theme)
                    .style_class("story-root")
                    .style_class(format!("story-{}", example.page)),
            );
        }
        panel
    }

    fn panel_count(node: &UiNode) -> usize {
        let current = usize::from(node.kind() == UiNodeKind::Panel);
        current + node.children().iter().map(Self::panel_count).sum::<usize>()
    }

    fn panel_theme_is_configured(node: &UiNode) -> bool {
        if node.kind() == UiNodeKind::Panel && node.props().theme_id.is_empty() {
            return false;
        }
        node.children().iter().all(Self::panel_theme_is_configured)
    }

    fn collect_panel_theme_ids(node: &UiNode, theme_ids: &mut BTreeSet<String>) {
        if node.kind() == UiNodeKind::Panel && !node.props().theme_id.is_empty() {
            theme_ids.insert(node.props().theme_id.clone());
        }
        for child in node.children() {
            Self::collect_panel_theme_ids(child, theme_ids);
        }
    }

    fn themed_story_root_count(node: &UiNode) -> usize {
        preview_panel(node)
            .map(|it| {
                it.children()
                    .iter()
                    .filter(|child| !child.props().theme_id.is_empty())
                    .count()
            })
            .unwrap_or_default()
    }

    fn styled_story_root_count(&self, node: &UiNode) -> usize {
        preview_panel(node)
            .map(|it| {
                it.children()
                    .iter()
                    .filter(|child| !self.style_sheet.resolve(child).declarations().is_empty())
                    .count()
            })
            .unwrap_or_default()
    }
}

impl StorybookPanelReport {
    pub fn summary(&self) -> String {
        format!(
            "panel_nodes={} panel_theme_configured={} panel_theme_variants={} themed_story_roots={} styled_story_roots={}",
            self.panel_nodes,
            self.panel_theme_configured,
            self.panel_theme_variants,
            self.themed_story_roots,
            self.styled_story_roots
        )
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StorybookStyleSheet;

impl StorybookStyleSheet {
    pub fn default_sheet() -> StyleSheet {
        StyleSheet::new().rule(StyleRule::class(
            "story-root",
            vec![
                StyleDeclaration::new(
                    StyleProperty::Background,
                    StyleValue::ColorToken("surface".to_string()),
                ),
                StyleDeclaration::new(StyleProperty::Padding, StyleValue::Px(STORY_PADDING)),
                StyleDeclaration::new(StyleProperty::Radius, StyleValue::Px(STORY_RADIUS)),
            ],
        ))
    }
}

const STORY_PADDING: f32 = 12.0;
const STORY_RADIUS: f32 = 6.0;

fn preview_panel(root: &UiNode) -> Option<&UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == "Preview")
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
}
