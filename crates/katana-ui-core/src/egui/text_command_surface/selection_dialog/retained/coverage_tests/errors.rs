use super::super::super::super::types::TextCommandSurfaceStyle;
use super::super::super::model::{
    FileSelectionProjection, KucDialogTarget, SearchResultMode, SearchResultOption,
    SearchResultRow, SelectionDialogProjection, SelectionDialogProjectionLease,
};
use super::super::SelectionDialogRetainedState;
use super::{controls, file_lease, mode_with_row, search_lease, show_rejects};
use crate::text_raster::{PlatformTextRasterConfig, PlatformTextRasterResources};

#[test]
fn empty_dialog_title_propagates_raster_error_without_consuming_projection() {
    let lease = SelectionDialogProjectionLease::new(
        1,
        "",
        controls(),
        SelectionDialogProjection::File(FileSelectionProjection {
            items: Vec::new(),
            failure: None,
        }),
    )
    .expect("empty title is valid");
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
    assert!(result.expect("dialog was shown").is_err());
    assert_eq!(state.lease.projection, before);
    assert!(state.lease.open);
}

#[test]
fn file_helpers_propagate_empty_label_and_control_errors() {
    let target = KucDialogTarget::from_opaque_bytes(b"file");
    let item =
        |label: &str, target: KucDialogTarget| super::super::super::model::FileSelectionItem {
            target,
            label: label.into(),
            depth: 0,
            navigable: false,
            selectable: true,
            disabled: false,
        };
    show_rejects(file_lease(controls(), vec![item("", target.clone())]));
    let mut select = controls();
    select.select.clear();
    show_rejects(file_lease(select, vec![item("file", target.clone())]));
    let mut cancel = controls();
    cancel.cancel.clear();
    show_rejects(file_lease(cancel, Vec::new()));
}

#[test]
fn search_helpers_propagate_empty_option_auxiliary_snippet_and_history_controls() {
    show_rejects(search_lease(
        controls(),
        SearchResultMode {
            target: KucDialogTarget::from_opaque_bytes(b"empty-mode"),
            label: String::new(),
            query: String::new(),
            include: None,
            exclude: None,
            options: Vec::new(),
            history: Vec::new(),
            results: Vec::new(),
            searched: false,
        },
    ));
    let row = |auxiliary_label, snippet| SearchResultRow {
        target: KucDialogTarget::from_opaque_bytes(b"row"),
        label: "result".into(),
        auxiliary_label,
        snippet,
        highlights: Vec::new(),
        disabled: false,
    };
    show_rejects(search_lease(
        controls(),
        mode_with_row(row(Some(String::new()), None)),
    ));
    show_rejects(search_lease(
        controls(),
        mode_with_row(row(None, Some(String::new()))),
    ));
    show_rejects(search_lease(
        controls(),
        SearchResultMode {
            target: KucDialogTarget::from_opaque_bytes(b"option-mode"),
            label: "All".into(),
            query: String::new(),
            include: None,
            exclude: None,
            options: vec![SearchResultOption {
                target: KucDialogTarget::from_opaque_bytes(b"option"),
                label: String::new(),
                enabled: true,
                selected: false,
            }],
            history: Vec::new(),
            results: Vec::new(),
            searched: false,
        },
    ));
    let history = SearchResultRow {
        target: KucDialogTarget::from_opaque_bytes(b"history"),
        label: "history".into(),
        auxiliary_label: None,
        snippet: None,
        highlights: Vec::new(),
        disabled: false,
    };
    let mut delete = controls();
    delete.delete_history.clear();
    show_rejects(search_lease(
        delete,
        SearchResultMode {
            target: KucDialogTarget::from_opaque_bytes(b"history-mode"),
            label: "All".into(),
            query: String::new(),
            include: None,
            exclude: None,
            options: Vec::new(),
            history: vec![history],
            results: Vec::new(),
            searched: false,
        },
    ));
    let mut clear = controls();
    clear.clear_history.clear();
    show_rejects(search_lease(
        clear,
        SearchResultMode {
            target: KucDialogTarget::from_opaque_bytes(b"clear-mode"),
            label: "All".into(),
            query: String::new(),
            include: None,
            exclude: None,
            options: Vec::new(),
            history: vec![SearchResultRow {
                target: KucDialogTarget::from_opaque_bytes(b"clear-history"),
                label: "history".into(),
                auxiliary_label: None,
                snippet: None,
                highlights: Vec::new(),
                disabled: false,
            }],
            results: Vec::new(),
            searched: false,
        },
    ));
}

#[test]
fn search_filter_input_error_propagates_after_query_input_succeeds() {
    let lease = search_lease(
        controls(),
        SearchResultMode {
            target: KucDialogTarget::from_opaque_bytes(b"filter-error"),
            label: "All".into(),
            query: "query".into(),
            /* WHY: 描画上限を十分に超える長さで、失敗伝播を検証するための確保量を抑える。 */
            include: Some("x".repeat(16_384)),
            exclude: None,
            options: Vec::new(),
            history: Vec::new(),
            results: Vec::new(),
            searched: false,
        },
    );
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
        .expect_err("oversized include input should reject after query succeeds");
    assert!(error.starts_with("text surface raster failed: platform text raster "));
    assert!(error.ends_with(" exceeds 16777216 pixel limit"));
    assert_eq!(
        state.inputs.len(),
        2,
        "query and include inputs were mounted"
    );
    assert_eq!(state.lease.projection, before);
}

#[test]
fn oversized_query_propagates_input_error_without_losing_projection() {
    let mut mode = mode_with_row(SearchResultRow {
        target: KucDialogTarget::from_opaque_bytes(b"result"),
        label: "result".into(),
        auxiliary_label: None,
        snippet: None,
        highlights: Vec::new(),
        disabled: false,
    });
    mode.query = "x".repeat(16_384);
    let lease = search_lease(controls(), mode);
    let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
    let mut state = SelectionDialogRetainedState::new(lease, &resources);
    let before = state.lease.projection.clone();
    let style = TextCommandSurfaceStyle::standard().expect("style");
    let context = egui::Context::default();
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(state.show(ui.ctx(), &style));
    });
    output.textures_delta.clear();
    let error = result
        .expect("frame")
        .expect_err("oversized query must fail");
    assert!(error.contains("pixel limit"));
    assert_eq!(state.inputs.len(), 1);
    assert_eq!(state.lease.projection, before);
    assert!(state.lease.open);
}

#[test]
fn search_filter_input_completes_include_branch() {
    let lease = search_lease(
        controls(),
        SearchResultMode {
            target: KucDialogTarget::from_opaque_bytes(b"filter-valid"),
            label: "All".into(),
            query: "query".into(),
            include: Some("scope".into()),
            exclude: None,
            options: Vec::new(),
            history: Vec::new(),
            results: Vec::new(),
            searched: false,
        },
    );
    let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
    let mut state = SelectionDialogRetainedState::new(lease, &resources);
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let context = egui::Context::default();
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(state.show(ui.ctx(), &style));
    });
    output.textures_delta.clear();
    assert!(result.expect("dialog should be shown").is_ok());
}

#[test]
fn input_propagates_raster_error_from_an_invalid_font_style() {
    let lease = search_lease(
        controls(),
        SearchResultMode {
            target: KucDialogTarget::from_opaque_bytes(b"input-mode"),
            label: "All".into(),
            query: String::new(),
            include: None,
            exclude: None,
            options: Vec::new(),
            history: Vec::new(),
            results: Vec::new(),
            searched: false,
        },
    );
    let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
    let mut state = SelectionDialogRetainedState::new(lease, &resources);
    let mut style = TextCommandSurfaceStyle::standard().expect("standard style");
    style.text_raster.font.size = f32::NAN;
    let context = egui::Context::default();
    let mut value = "query".to_owned();
    let mut operations = Vec::new();
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(state.input(
            ui,
            "coverage.invalid-input".into(),
            "Query",
            &mut value,
            &style,
            false,
            &mut operations,
        ));
    });
    output.textures_delta.clear();
    assert!(matches!(result, Some(Err(_))));
    assert_eq!(value, "query");
}
