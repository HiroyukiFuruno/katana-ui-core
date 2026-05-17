//! Core-only Storybook model for KUC verification.

mod catalog;
mod panel;
mod requirements;
mod visual;

pub use catalog::{StoryCatalog, StoryCatalogReport, StoryExample};
use katana_ui_core::theme::ThemeSnapshot;
pub use panel::{StorybookPanel, StorybookPanelReport, StorybookStyleSheet};
pub use visual::{
    Canvas, StorybookRuntimeReport, StorybookVisual, StorybookVisualError, StorybookWindowRun,
};

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
    use super::{StorybookRoutes, StorybookSummary};

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
        assert_eq!(53, routes.len());
        assert!(routes.iter().any(|route| route.page == "code-diff"));
        assert!(routes.iter().any(|route| route.page == "grid"));
    }

    #[test]
    fn summary_reports_panel_theme_and_style_gates() {
        let summary = StorybookSummary.render();

        assert!(summary.contains("panel_theme_configured=true"));
        assert!(summary.contains("styled_story_roots=53"));
    }
}
