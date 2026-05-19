use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{ChoiceItem, SelectBox};

const LIGHT_INDEX: usize = 1;
const DISABLED_INDEX: usize = 2;

#[test]
fn select_box_tracks_options_highlight_long_list_and_placement() {
    let mut select = SelectBox::new("Theme")
        .placeholder("Select theme")
        .placement("bottom-start")
        .highlighted_index(LIGHT_INDEX)
        .long_list(true)
        .outside_click_dismiss(true)
        .keyboard_navigation("arrow-down moves highlight")
        .item(ChoiceItem::new("dark", "Dark"))
        .item(ChoiceItem::new("light", "Light"))
        .item(ChoiceItem::new("disabled", "Disabled").disabled(true));

    let result = select.apply_action(&UiAction::select_box_selected(
        select.state_id().clone(),
        LIGHT_INDEX,
    ));

    assert!(result.handled);
    assert_eq!("bottom-start", select.placement_model());
    assert_eq!(LIGHT_INDEX, select.highlighted_index_model());
    assert!(select.is_long_list());
    assert!(select.dismisses_on_outside_click());
    assert_eq!(
        "arrow-down moves highlight",
        select.keyboard_navigation_summary()
    );
    assert_eq!(
        Some("light"),
        select.selected_option().map(|it| it.value.as_str())
    );
    assert_eq!("light", result.after.value);
    assert!(!result.after.open);
}

#[test]
fn select_box_ignores_disabled_option_selection() {
    let mut select = SelectBox::new("Theme")
        .item(ChoiceItem::new("dark", "Dark"))
        .item(ChoiceItem::new("light", "Light"))
        .item(ChoiceItem::new("disabled", "Disabled").disabled(true));

    let result = select.apply_action(&UiAction::select_box_selected(
        select.state_id().clone(),
        DISABLED_INDEX,
    ));

    assert!(!result.handled);
    assert!(select.selected_option().is_none());
}
