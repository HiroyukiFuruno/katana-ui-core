use super::super::super::super::types::TextCommandSurfaceStyle;
use super::super::super::model::{
    FileSelectionItem, FileSelectionProjection, KucDialogTarget, SearchResultMode, SearchResultRow,
    SelectionDialogProjection, SelectionDialogProjectionLease,
};
use super::super::SelectionDialogRetainedState;
use super::{controls, file_lease, mode_with_row, search_lease, show_rejects};
use crate::text_raster::{PlatformTextRasterConfig, PlatformTextRasterResources};

#[test]
fn empty_file_failure_message_is_rejected_by_real_paint() {
    let lease = SelectionDialogProjectionLease::new(
        1,
        "Files",
        controls(),
        SelectionDialogProjection::File(FileSelectionProjection {
            items: Vec::new(),
            failure: Some(String::new()),
        }),
    )
    .expect("valid file lease");
    show_rejects(lease);
}

#[test]
fn empty_search_controls_and_labels_propagate_real_paint_errors() {
    let mut search_control = controls();
    search_control.search.clear();
    show_rejects(search_lease(
        search_control,
        SearchResultMode {
            target: KucDialogTarget::from_opaque_bytes(b"search"),
            label: "All".into(),
            query: String::new(),
            include: None,
            exclude: None,
            options: Vec::new(),
            history: Vec::new(),
            results: Vec::new(),
            searched: false,
        },
    ));

    let mut empty_control = controls();
    empty_control.empty.clear();
    show_rejects(search_lease(
        empty_control,
        SearchResultMode {
            target: KucDialogTarget::from_opaque_bytes(b"empty"),
            label: "All".into(),
            query: String::new(),
            include: None,
            exclude: None,
            options: Vec::new(),
            history: Vec::new(),
            results: Vec::new(),
            searched: true,
        },
    ));

    show_rejects(search_lease(
        controls(),
        SearchResultMode {
            target: KucDialogTarget::from_opaque_bytes(b"history-label"),
            label: "All".into(),
            query: String::new(),
            include: None,
            exclude: None,
            options: Vec::new(),
            history: vec![SearchResultRow {
                target: KucDialogTarget::from_opaque_bytes(b"history-row"),
                label: String::new(),
                auxiliary_label: None,
                snippet: None,
                highlights: Vec::new(),
                disabled: false,
            }],
            results: Vec::new(),
            searched: false,
        },
    ));
    show_rejects(search_lease(
        controls(),
        mode_with_row(SearchResultRow {
            target: KucDialogTarget::from_opaque_bytes(b"result-label"),
            label: String::new(),
            auxiliary_label: None,
            snippet: None,
            highlights: Vec::new(),
            disabled: false,
        }),
    ));
}

#[test]
fn initial_show_preserves_an_existing_restore_focus_owner() {
    let lease = file_lease(controls(), Vec::new());
    let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
    let mut state = SelectionDialogRetainedState::new(lease, &resources);
    let owner = egui::Id::new("coverage.focus.owner");
    state.restore_focus = Some(owner);
    state.mounted = false;
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let (mut output, dialog) = context_with_input(egui::RawInput::default(), &mut state, &style);
    output.textures_delta.clear();
    assert!(dialog.events.is_empty());
    assert_eq!(state.restore_focus, Some(owner));
    assert!(state.mounted);
}

#[test]
fn arrow_up_from_no_selection_selects_last_selectable_file() {
    let first = KucDialogTarget::from_opaque_bytes(b"first");
    let last = KucDialogTarget::from_opaque_bytes(b"last");
    let lease = file_lease(
        controls(),
        vec![
            FileSelectionItem {
                target: first,
                label: "first".into(),
                depth: 0,
                navigable: false,
                selectable: true,
                disabled: false,
            },
            FileSelectionItem {
                target: KucDialogTarget::from_opaque_bytes(b"disabled"),
                label: "disabled".into(),
                depth: 0,
                navigable: false,
                selectable: true,
                disabled: true,
            },
            FileSelectionItem {
                target: last.clone(),
                label: "last".into(),
                depth: 0,
                navigable: false,
                selectable: true,
                disabled: false,
            },
        ],
    );
    let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
    let mut state = SelectionDialogRetainedState::new(lease, &resources);
    let before = state.lease.projection.clone();
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let mut input = egui::RawInput::default();
    input.events.push(egui::Event::Key {
        key: egui::Key::ArrowUp,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    });
    let (mut output, dialog) = context_with_input(input, &mut state, &style);
    output.textures_delta.clear();
    assert!(dialog.events.is_empty());
    assert_eq!(state.selected.as_ref(), Some(&last));
    assert_eq!(state.lease.projection, before);
}

fn context_with_input(
    input: egui::RawInput,
    state: &mut SelectionDialogRetainedState,
    style: &TextCommandSurfaceStyle,
) -> (egui::FullOutput, super::super::SelectionDialogOutput) {
    let context = egui::Context::default();
    let mut result = None;
    let output = context.run_ui(input, |ui| {
        result = Some(state.show(ui.ctx(), style));
    });
    let dialog = result
        .expect("dialog was shown")
        .expect("file dialog frame")
        .expect("open dialog output");
    (output, dialog)
}
