#[path = "coverage_tests/core.rs"]
mod core;
#[path = "coverage_tests/edges.rs"]
mod edges;
#[path = "coverage_tests/errors.rs"]
mod errors;

use super::super::super::types::TextCommandSurfaceStyle;
use super::super::model::{
    FileSelectionProjection, KucDialogControls, KucDialogTarget, SearchResultMode,
    SearchResultProjection, SelectionDialogProjection, SelectionDialogProjectionLease,
};
use super::SelectionDialogRetainedState;
use crate::text_raster::{PlatformTextRasterConfig, PlatformTextRasterResources};

pub(super) fn controls() -> KucDialogControls {
    KucDialogControls {
        cancel: "Cancel".into(),
        select: "Select".into(),
        search: "Search".into(),
        include: "Include".into(),
        exclude: "Exclude".into(),
        clear_history: "Clear".into(),
        delete_history: "Delete".into(),
        empty: "Empty".into(),
    }
}

pub(super) fn show_rejects(lease: SelectionDialogProjectionLease) {
    let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
    let mut state = SelectionDialogRetainedState::new(lease, &resources);
    let before = state.lease.projection.clone();
    let context = egui::Context::default();
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(state.show(ui.ctx(), &style));
    });
    output.textures_delta.clear();
    assert!(
        result.expect("dialog was shown").is_err(),
        "invalid dialog label must reject rasterization"
    );
    assert_eq!(state.lease.projection, before);
    assert!(state.lease.open);
}

pub(super) fn file_lease(
    controls: KucDialogControls,
    items: Vec<super::super::model::FileSelectionItem>,
) -> SelectionDialogProjectionLease {
    SelectionDialogProjectionLease::new(
        1,
        "Files",
        controls,
        SelectionDialogProjection::File(FileSelectionProjection {
            items,
            failure: None,
        }),
    )
    .expect("valid file lease")
}

pub(super) fn search_lease(
    controls: KucDialogControls,
    mode: SearchResultMode,
) -> SelectionDialogProjectionLease {
    let target = mode.target.clone();
    SelectionDialogProjectionLease::new(
        1,
        "Search",
        controls,
        SelectionDialogProjection::Search(SearchResultProjection {
            modes: vec![mode],
            active_mode: target,
        }),
    )
    .expect("valid search lease")
}

pub(super) fn mode_with_row(row: super::super::model::SearchResultRow) -> SearchResultMode {
    SearchResultMode {
        target: KucDialogTarget::from_opaque_bytes(b"mode"),
        label: "All".into(),
        query: String::new(),
        include: None,
        exclude: None,
        options: Vec::new(),
        history: Vec::new(),
        results: vec![row],
        searched: true,
    }
}
