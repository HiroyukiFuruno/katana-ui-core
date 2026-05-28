//! Core-only Storybook model for KUC verification.

mod catalog;
mod katana_icons;
mod panel;
mod requirements;
mod visual;

pub use catalog::{
    StoryCatalog, StoryCatalogReport, StoryDetailContent, StoryExample,
    StorybookPanelInteractionReport, StorybookPanelReport, StorybookStyleSheet,
};
use katana_ui_core::theme::ThemeSnapshot;
pub use panel::StorybookPanel;
pub use visual::{
    Canvas, StorybookRuntimeReport, StorybookVisual, StorybookVisualError, StorybookWindowRun,
};

/// 起動直後に Storybook の操作性が見える代表ページを開く。
pub const DEFAULT_STORYBOOK_PAGE: &str = "text-input";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorybookRoute {
    pub page: &'static str,
    pub source_crate: &'static str,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StorybookRoutes;

impl StorybookRoutes {
    #[must_use]
    pub fn default_routes(self) -> Vec<StorybookRoute> {
        requirements::StoryRequirements::required_pages()
            .iter()
            .copied()
            .map(Self::route)
            .collect()
    }

    fn route(page: &'static str) -> StorybookRoute {
        StorybookRoute {
            page,
            source_crate: "katana-ui-core",
        }
    }
}

#[must_use]
fn render_summary() -> String {
    let catalog = StoryCatalog;
    let examples = catalog.examples();
    let catalog_report = StoryCatalog.verify();
    let panel_report = StorybookPanel::verify_theme_variants(
        &examples,
        &[ThemeSnapshot::light(), ThemeSnapshot::dark()],
    );
    format!("{} {}", catalog_report.summary(), panel_report.summary())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StorybookSummary;

impl StorybookSummary {
    #[must_use]
    pub fn render(self) -> String {
        render_summary()
    }
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_STORYBOOK_PAGE, StorybookRoutes, StorybookSummary};

    #[test]
    fn storybook_routes_use_core_crate() {
        assert!(
            StorybookRoutes
                .default_routes()
                .iter()
                .all(|route| route.source_crate == "katana-ui-core")
        );
    }

    #[test]
    fn storybook_routes_cover_core_and_legacy_targets() {
        let routes = StorybookRoutes.default_routes();
        assert_eq!(77, routes.len());
        assert!(
            routes
                .iter()
                .any(|route| route.page == DEFAULT_STORYBOOK_PAGE)
        );
        assert!(routes.iter().any(|route| route.page == "panel"));
        assert!(routes.iter().any(|route| route.page == "code-diff"));
        assert!(routes.iter().any(|route| route.page == "context-menu"));
        assert!(routes.iter().any(|route| route.page == "banner"));
        assert!(routes.iter().any(|route| route.page == "settings-list"));
        assert!(routes.iter().any(|route| route.page == "collapsible-panel"));
        assert!(
            routes
                .iter()
                .any(|route| route.page == "window-control-button-group")
        );
        assert!(
            routes
                .iter()
                .any(|route| route.page == "search-control-strip")
        );
        assert!(routes.iter().any(|route| route.page == "grid"));
    }

    #[test]
    fn default_storybook_page_is_representative_input_playground() {
        assert_eq!("text-input", DEFAULT_STORYBOOK_PAGE);
    }

    #[test]
    fn summary_reports_panel_theme_and_style_gates() {
        let summary = StorybookSummary.render();

        assert!(summary.contains("panel_theme_configured=true"));
        assert!(summary.contains("panel_scroll_configured=true"));
        assert!(summary.contains("independent_panel_scrolls=4"));
        assert!(summary.contains("styled_story_roots=1"));
    }
}
