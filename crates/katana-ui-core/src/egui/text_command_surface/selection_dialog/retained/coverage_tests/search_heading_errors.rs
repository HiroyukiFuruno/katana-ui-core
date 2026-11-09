use super::super::super::super::types::TextCommandSurfaceStyle;
use super::super::super::model::{KucDialogTarget, SearchResultRow};
use super::super::super::search_model::SearchModePresentation;
use super::super::SelectionDialogRetainedState;
use super::{controls, mode_with_row, search_lease};
use crate::text_raster::{PlatformTextRasterConfig, PlatformTextRasterResources};

#[test]
fn history_heading_label_raster_error_propagates_without_consuming_projection() {
    let mode = mode_with_row(SearchResultRow {
        target: KucDialogTarget::from_opaque_bytes(b"history-heading-result"),
        label: "result".into(),
        auxiliary_label: None,
        snippet: None,
        highlights: Vec::new(),
        disabled: false,
    });
    let mode_target = mode.target.clone();
    let lease = search_lease(controls(), mode)
        .search_mode_presentation(
            mode_target,
            SearchModePresentation::new().history_heading("M".repeat(180_000)),
        )
        .expect("history heading presentation");
    let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
    let mut state = SelectionDialogRetainedState::new(lease, &resources);
    let before = state.lease.projection.clone();
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let context = egui::Context::default();
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(state.show(ui.ctx(), &style));
    });
    output.textures_delta.clear();

    let error = result
        .expect("dialog should be shown")
        .expect_err("oversized history heading should reject rasterization");
    assert!(
        error.contains("exceeds 16777216 pixel limit"),
        "unexpected history-heading error: {error}"
    );
    assert_eq!(state.lease.projection, before);
    assert!(state.lease.open);
}
