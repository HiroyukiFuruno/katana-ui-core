use super::super::super::super::types::TextCommandSurfaceStyle;
use super::super::super::model::{
    FileSelectionProjection, KucDialogControls, KucDialogTarget, SearchResultMode,
    SearchResultProjection, SelectionDialogProjection, SelectionDialogProjectionLease,
};
use super::super::SelectionDialogRetainedState;
use crate::text_raster::{PlatformTextRasterConfig, PlatformTextRasterResources};

fn controls() -> KucDialogControls {
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

#[test]
fn search_rejects_an_active_mode_that_disappeared_without_mutating_state() {
    let mode_target = KucDialogTarget::from_opaque_bytes(b"mode");
    let lease = SelectionDialogProjectionLease::new(
        1,
        "Search",
        controls(),
        SelectionDialogProjection::Search(SearchResultProjection {
            active_mode: mode_target.clone(),
            modes: vec![SearchResultMode {
                target: mode_target,
                label: "All".into(),
                query: String::new(),
                include: None,
                exclude: None,
                options: Vec::new(),
                history: Vec::new(),
                results: Vec::new(),
                searched: false,
            }],
        }),
    )
    .expect("valid search lease");
    let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
    let mut state = SelectionDialogRetainedState::new(lease, &resources);
    if let SelectionDialogProjection::Search(search) = &mut state.lease.projection {
        search.active_mode = KucDialogTarget::from_opaque_bytes(b"missing");
    }
    let before = state.lease.projection.clone();
    let context = egui::Context::default();
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        if let SelectionDialogProjection::Search(mut search) = state.lease.projection.clone() {
            let mut operations = Vec::new();
            let mut events = Vec::new();
            result =
                Some(state.search(ui, &mut search, &style, true, &mut operations, &mut events));
        }
    });
    output.textures_delta.clear();
    assert!(matches!(result, Some(Err(message)) if message == "active dialog mode is absent"));
    assert_eq!(state.lease.projection, before);
}

#[test]
fn corrupted_internal_mode_cannot_preserve_a_history_draft_across_sync() {
    let row = super::super::super::model::SearchResultRow {
        target: KucDialogTarget::from_opaque_bytes(b"history"),
        label: "old".into(),
        auxiliary_label: None,
        snippet: None,
        highlights: Vec::new(),
        disabled: false,
    };
    let mode = super::mode_with_row(row);
    let old = super::search_lease(controls(), mode.clone());
    let mut next = super::search_lease(controls(), mode);
    next.revision = 2;
    if let SelectionDialogProjection::Search(search) = &mut next.projection {
        search.modes.clear();
    }
    let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
    let mut state = SelectionDialogRetainedState::new(old, &resources);
    state.history_traversal = Some(super::super::HistoryTraversal {
        draft: "draft".into(),
        index: 0,
    });
    assert_eq!(state.synchronize(next), Ok(true));
    assert!(state.history_traversal.is_none());
}

#[test]
fn file_projection_validation_rejects_duplicate_targets() {
    let target = KucDialogTarget::from_opaque_bytes(b"same");
    let item =
        |label: &str, target: KucDialogTarget| super::super::super::model::FileSelectionItem {
            target,
            label: label.into(),
            depth: 0,
            navigable: false,
            selectable: true,
            disabled: false,
        };
    let result = SelectionDialogProjectionLease::new(
        1,
        "Files",
        controls(),
        SelectionDialogProjection::File(FileSelectionProjection {
            items: vec![item("one", target.clone()), item("two", target)],
            failure: None,
        }),
    );
    assert!(matches!(
        result,
        Err(super::super::super::model::SelectionDialogProjectionError::DuplicateTarget)
    ));
}
