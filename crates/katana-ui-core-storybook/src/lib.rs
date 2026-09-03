//! Core-only Storybook model for KUC verification.

mod catalog;
mod panel;
#[cfg(test)]
mod raster_host_parity_tests;
mod requirements;
mod storybook_svg_fixtures;
#[cfg(target_os = "macos")]
mod system;
#[cfg(test)]
mod test_assert;
mod visual;

/// Private Storybook's thin bridge to the registry-safe raster host.
pub mod raster_host {
    pub use katana_ui_core::raster_host::*;
}

pub use catalog::{
    StoryCatalog, StoryCatalogReport, StoryDetailContent, StoryExample,
    StorybookPanelInteractionReport, StorybookPanelReport, StorybookStyleSheet,
};
use katana_ui_core::theme::ThemeSnapshot;
pub use panel::StorybookPanel;
pub use visual::FullRootArtifactError;
pub use visual::TextSurfaceArtifactError;
pub use visual::{
    Canvas, CanvasBlitRequest, RgbaBlitRequest, SelectableTextRun, StorybookKeyboardRuntimeReport,
    StorybookPresentation, StorybookRuntimeReport, StorybookVisual, StorybookVisualError,
    StorybookWindowRun, TextRenderer, UiTreeCanvasRenderer, UiTreeHitRect, UiTreeHostActionHit,
    UiTreeHostActionHitQuery, UiTreeInteractionSurface, UiTreeInteractionTarget, UiTreeNodeHit,
    UiTreeRenderArea, UiTreeStorybookHost, UiTreeSurfaceHost,
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
        let mut pages = Vec::new();
        for page in requirements::StoryRequirements::required_pages()
            .iter()
            .chain(requirements::StoryRequirements::interactive_runtime_pages().iter())
        {
            if !pages.contains(page) {
                pages.push(*page);
            }
        }

        pages.iter().copied().map(Self::route).collect()
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
    use super::{
        Canvas, DEFAULT_STORYBOOK_PAGE, StorybookPresentation, StorybookRoutes, StorybookSummary,
    };

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
        assert_eq!(79, routes.len());
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
        assert!(routes.iter().any(|route| route.page == "command-chrome"));
        assert_eq!(
            1,
            routes
                .iter()
                .filter(|route| route.page == "command-chrome")
                .count()
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

    #[test]
    fn crate_root_exports_window_presentation_type() {
        let canvas = Canvas::new(4, 4, 0xff_11_22_33);

        let frame = StorybookPresentation::present_frame_for_window(&canvas, 8, 8, 0xff_ff_ff_ff);

        assert_eq!(8, frame.width());
        assert_eq!(8, frame.height());
    }
}
