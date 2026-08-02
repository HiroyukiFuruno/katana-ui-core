use katana_ui_core::interaction::UiAction;
use katana_ui_core::layout::SplitPaneResizeSource;
use katana_ui_core::render_model::{UiRect, UiScrollbarVisibility, UiStateId};

#[test]
fn action_accessors_cover_scroll_split_pane_and_ungrouped_variants() {
    let target = UiStateId::new("target");
    let cases = [
        (
            UiAction::scroll_into_view(target.clone(), UiRect::new(1, 2, 3, 4)),
            "scroll_into_view",
        ),
        (
            UiAction::scrollbar_visibility(target.clone(), UiScrollbarVisibility::Always),
            "scrollbar_visibility_changed",
        ),
        (
            UiAction::split_pane_set_ratio(target.clone(), 45),
            "split_pane_set_ratio",
        ),
        (
            UiAction::split_pane_resize_by(target.clone(), 5, SplitPaneResizeSource::Keyboard),
            "split_pane_resize_by",
        ),
        (
            UiAction::split_pane_reset_ratio(target.clone()),
            "split_pane_reset_ratio",
        ),
        (
            UiAction::split_pane_start_resize(target.clone()),
            "split_pane_start_resize",
        ),
        (
            UiAction::split_pane_end_resize(target.clone()),
            "split_pane_end_resize",
        ),
        (
            UiAction::tab_move_to_ungrouped(target.clone(), "preview"),
            "tab_move_to_ungrouped",
        ),
        (UiAction::active(target.clone(), true), "active_start"),
    ];

    for (action, expected_name) in cases {
        assert_eq!(&target, action.target());
        assert_eq!(expected_name, action.name());
        assert_eq!(expected_name, action.callback_log_action());
    }

    let open_uri = UiAction::open_uri(target.clone(), "https://example.test/docs");
    assert_eq!("callback_invoked", open_uri.name());
    assert_eq!(
        "open-uri:https://example.test/docs",
        open_uri.callback_log_action()
    );
    assert!(matches!(
        UiAction::open_uri(target, "https://example.test/docs"),
        UiAction::InvokeCallback { callback, .. }
            if callback == "open-uri:https://example.test/docs"
    ));
}
