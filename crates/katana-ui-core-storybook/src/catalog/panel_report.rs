use katana_ui_core::style::{StyleDeclaration, StyleProperty, StyleRule, StyleSheet, StyleValue};
use std::collections::BTreeSet;

const STORY_PADDING: f32 = 12.0;
const STORY_RADIUS: f32 = 6.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorybookPanelReport {
    pub panel_nodes: usize,
    pub panel_theme_configured: bool,
    pub panel_theme_variants: usize,
    pub themed_story_roots: usize,
    pub styled_story_roots: usize,
    pub details_panel_configured: bool,
    pub detail_sections: usize,
    pub(crate) panel_theme_ids: BTreeSet<String>,
}

pub(crate) struct StorybookPanelReportFields {
    pub(crate) panel_nodes: usize,
    pub(crate) panel_theme_configured: bool,
    pub(crate) panel_theme_variants: usize,
    pub(crate) themed_story_roots: usize,
    pub(crate) styled_story_roots: usize,
    pub(crate) details_panel_configured: bool,
    pub(crate) detail_sections: usize,
    pub(crate) panel_theme_ids: BTreeSet<String>,
}

impl StorybookPanelReport {
    #[must_use]
    pub(crate) fn new(fields: StorybookPanelReportFields) -> Self {
        Self {
            panel_nodes: fields.panel_nodes,
            panel_theme_configured: fields.panel_theme_configured,
            panel_theme_variants: fields.panel_theme_variants,
            themed_story_roots: fields.themed_story_roots,
            styled_story_roots: fields.styled_story_roots,
            details_panel_configured: fields.details_panel_configured,
            detail_sections: fields.detail_sections,
            panel_theme_ids: fields.panel_theme_ids,
        }
    }

    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "panel_nodes={} panel_theme_configured={} panel_theme_variants={} themed_story_roots={} styled_story_roots={} details_panel_configured={} detail_sections={}",
            self.panel_nodes,
            self.panel_theme_configured,
            self.panel_theme_variants,
            self.themed_story_roots,
            self.styled_story_roots,
            self.details_panel_configured,
            self.detail_sections
        )
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StorybookStyleSheet;

impl StorybookStyleSheet {
    #[must_use]
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
