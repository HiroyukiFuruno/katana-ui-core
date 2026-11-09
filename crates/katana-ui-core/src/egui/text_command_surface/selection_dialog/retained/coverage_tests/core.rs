use super::super::super::super::types::TextCommandSurfaceStyle;
use super::super::super::model::{
    FileSelectionProjection, KucDialogControls, KucDialogTarget, SearchFilterValidity,
    SearchModePresentation, SearchResultMode, SearchResultProjection, SelectionDialogProjection,
    SelectionDialogProjectionLease,
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
fn search_presentation_sync_updates_input_hint_and_invalid_theme_state() {
    let mode_target = KucDialogTarget::from_opaque_bytes(b"mode");
    let mode = SearchResultMode {
        target: mode_target.clone(),
        label: "All".into(),
        query: String::new(),
        include: Some(String::new()),
        exclude: Some(String::new()),
        options: Vec::new(),
        history: Vec::new(),
        results: Vec::new(),
        searched: false,
    };
    let projection = || {
        SelectionDialogProjection::Search(SearchResultProjection {
            modes: vec![mode.clone()],
            active_mode: mode_target.clone(),
        })
    };
    let lease = SelectionDialogProjectionLease::new(1, "Search", controls(), projection())
        .expect("lease")
        .search_mode_presentation(
            mode_target.clone(),
            SearchModePresentation::new()
                .query_hint("query hint")
                .include_hint("include hint")
                .exclude_hint("exclude hint")
                .include_validity(SearchFilterValidity::Invalid)
                .exclude_validity(SearchFilterValidity::Valid),
        )
        .expect("presentation");
    let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
    let mut state = SelectionDialogRetainedState::new(lease, &resources);
    let context = egui::Context::default();
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let mode_key = mode_target.interaction_key();
    let render = |state: &mut SelectionDialogRetainedState| {
        let mut result = None;
        let mut output = context.run_ui(egui::RawInput::default(), |ui| {
            let mut search = match state.lease.projection.clone() {
                SelectionDialogProjection::Search(search) => search,
                SelectionDialogProjection::File(_) => unreachable!(),
            };
            result = Some(state.search(
                ui,
                &mut search,
                &style,
                true,
                &mut Vec::new(),
                &mut Vec::new(),
            ));
        });
        output.textures_delta.clear();
        result.expect("search rendered").expect("valid search");
    };

    render(&mut state);
    let query = &state.inputs[&format!("kuc.dialog.query.{mode_key}")].surface;
    assert_eq!(query.props().text_area.options().placeholder, "query hint");
    let include_key = format!("kuc.dialog.filter.{mode_key}.true");
    let exclude_key = format!("kuc.dialog.filter.{mode_key}.false");
    let include = &state.inputs[&include_key].surface;
    let exclude = &state.inputs[&exclude_key].surface;
    assert_eq!(
        include.props().text_area.options().placeholder,
        "include hint"
    );
    assert!(include.props().text_area.options().invalid);
    assert!(include.state().text_area.invalid);
    assert_eq!(
        exclude.props().text_area.options().placeholder,
        "exclude hint"
    );
    assert!(!exclude.props().text_area.options().invalid);

    let updated = SelectionDialogProjectionLease::new(2, "Search", controls(), projection())
        .expect("updated lease")
        .search_mode_presentation(
            mode_target,
            SearchModePresentation::new()
                .query_hint("")
                .include_hint("include hint 2")
                .exclude_hint("")
                .include_validity(SearchFilterValidity::Valid)
                .exclude_validity(SearchFilterValidity::Invalid),
        )
        .expect("updated presentation");
    state.synchronize(updated).expect("synchronize");
    render(&mut state);
    assert_eq!(
        state.inputs[&format!("kuc.dialog.query.{mode_key}")]
            .surface
            .props()
            .text_area
            .options()
            .placeholder,
        ""
    );
    assert_eq!(
        state.inputs[&include_key]
            .surface
            .props()
            .text_area
            .options()
            .placeholder,
        "include hint 2"
    );
    assert!(
        !state.inputs[&include_key]
            .surface
            .props()
            .text_area
            .options()
            .invalid
    );
    assert!(
        state.inputs[&exclude_key]
            .surface
            .props()
            .text_area
            .options()
            .invalid
    );
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
