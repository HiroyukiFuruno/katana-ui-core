use super::StorybookPanel;
use crate::catalog::{StoryExample, StorybookPanelReport, StorybookPanelReportFields};
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use katana_ui_core::theme::ThemeSnapshot;
use std::collections::BTreeSet;

impl StorybookPanel {
    pub fn verify(&self, examples: &[StoryExample]) -> StorybookPanelReport {
        let tree = self.build(examples);
        let mut theme_ids = BTreeSet::new();
        Self::collect_panel_theme_ids(tree.root(), &mut theme_ids);
        StorybookPanelReport::new(StorybookPanelReportFields {
            panel_nodes: Self::panel_count(tree.root()),
            panel_theme_configured: Self::panel_theme_is_configured(tree.root()),
            panel_theme_variants: theme_ids.len(),
            themed_story_roots: Self::themed_story_root_count(tree.root()),
            styled_story_roots: self.styled_story_root_count(tree.root()),
            details_panel_configured: details_panel(tree.root()).is_some(),
            detail_sections: details_panel(tree.root()).map_or(0, |it| it.children().len()),
            panel_theme_ids: theme_ids,
        })
    }

    pub fn verify_theme_variants(
        examples: &[StoryExample],
        themes: &[ThemeSnapshot],
    ) -> StorybookPanelReport {
        let mut summary = PanelVerificationSummary::default();
        for theme in themes {
            summary.merge(Self::new(theme.clone()).verify(examples));
        }
        summary.into_report()
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct PanelVerificationSummary {
    panel_nodes: usize,
    theme_ids: BTreeSet<String>,
    panel_theme_configured: bool,
    themed_story_roots: usize,
    styled_story_roots: usize,
    details_panel_configured: bool,
    detail_sections: usize,
}

impl Default for PanelVerificationSummary {
    fn default() -> Self {
        Self {
            panel_nodes: 0,
            theme_ids: BTreeSet::new(),
            panel_theme_configured: true,
            themed_story_roots: 0,
            styled_story_roots: 0,
            details_panel_configured: true,
            detail_sections: 0,
        }
    }
}

impl PanelVerificationSummary {
    fn merge(&mut self, report: StorybookPanelReport) {
        self.panel_nodes = self.panel_nodes.max(report.panel_nodes);
        self.panel_theme_configured = self.panel_theme_configured && report.panel_theme_configured;
        self.themed_story_roots = self.themed_story_roots.max(report.themed_story_roots);
        self.styled_story_roots = self.styled_story_roots.max(report.styled_story_roots);
        self.details_panel_configured =
            self.details_panel_configured && report.details_panel_configured;
        self.detail_sections = self.detail_sections.max(report.detail_sections);
        self.theme_ids.extend(report.panel_theme_ids);
    }

    fn into_report(self) -> StorybookPanelReport {
        StorybookPanelReport::new(StorybookPanelReportFields {
            panel_nodes: self.panel_nodes,
            panel_theme_configured: self.panel_theme_configured,
            panel_theme_variants: self.theme_ids.len(),
            themed_story_roots: self.themed_story_roots,
            styled_story_roots: self.styled_story_roots,
            details_panel_configured: self.details_panel_configured,
            detail_sections: self.detail_sections,
            panel_theme_ids: self.theme_ids,
        })
    }
}

fn preview_panel(root: &UiNode) -> Option<&UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == "Preview")
}

fn details_panel(root: &UiNode) -> Option<&UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == "Details")
}
