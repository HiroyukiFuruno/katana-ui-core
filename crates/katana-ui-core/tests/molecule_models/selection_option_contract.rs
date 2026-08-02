use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{
    Breadcrumb, ChoiceItem, ComboBox, MenuButton, SelectBox, SelectionList, SideMenu, Tabs,
};
use katana_ui_core::render_model::UiNode;

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
    let focus = select.apply_action(&UiAction::focus(select.state_id().clone()));

    assert!(result.handled);
    assert!(focus.handled);
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
    assert!(focus.after.focused);
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

#[test]
fn combo_box_filters_focuses_and_selects_through_core_actions() {
    let mut combo = ComboBox::new("Command")
        .open(true)
        .input_value("tw")
        .filter_result(ChoiceItem::new("two", "Two"))
        .free_input(true)
        .keyboard_navigation("arrow-down moves highlight")
        .highlighted_index(LIGHT_INDEX)
        .item(ChoiceItem::new("one", "One"))
        .item(ChoiceItem::new("two", "Two"));
    let target = combo.state_id().clone();

    let input = combo.apply_action(&UiAction::input_value(target.clone(), "two"));
    let focus = combo.apply_action(&UiAction::focus(target.clone()));
    let hover = combo.apply_action(&UiAction::hover(target.clone(), true));
    let selected = combo.apply_action(&UiAction::select_box_selected(target, LIGHT_INDEX));

    assert!(input.handled);
    assert!(focus.handled);
    assert!(hover.handled);
    assert!(selected.handled);
    assert_eq!("tw", combo.input_model());
    assert_eq!(1, combo.filter_results().len());
    assert!(combo.allows_free_input());
    assert_eq!(
        "arrow-down moves highlight",
        combo.keyboard_navigation_summary()
    );
    assert_eq!(LIGHT_INDEX, combo.highlighted_index_model());
    assert_eq!(
        Some("two"),
        combo.selected_option().map(|it| it.value.as_str())
    );
    assert_eq!("two", selected.after.value);
    assert!(!selected.after.open);
    assert!(focus.after.focused);
    assert!(hover.after.hovered);
}

#[test]
fn every_choice_molecule_exposes_disabled_and_readonly_state() {
    let nodes = [
        UiNode::from(SelectBox::new("select").disabled(true).readonly(true)),
        UiNode::from(ComboBox::new("combo").disabled(true).readonly(true)),
        UiNode::from(MenuButton::new("menu").disabled(true).readonly(true)),
        UiNode::from(SelectionList::new("list").disabled(true).readonly(true)),
        UiNode::from(SideMenu::new("side").disabled(true).readonly(true)),
        UiNode::from(Tabs::new("tabs").disabled(true).readonly(true)),
        UiNode::from(Breadcrumb::new("breadcrumb").disabled(true).readonly(true)),
    ];

    assert!(
        nodes
            .iter()
            .all(|node| node.props().disabled && node.props().readonly)
    );
}

#[test]
fn selection_components_ignore_unrelated_actions_and_normalize_missing_items() {
    let mut select = SelectBox::new("Empty");
    let id = select.state_id().clone();
    let mut disabled = SelectBox::new("Disabled").disabled(true);

    let ignored = disabled.apply_action(&UiAction::focus(disabled.state_id().clone()));
    let missing = select.apply_action(&UiAction::select_box_selected(id, 99));

    assert!(!ignored.handled);
    assert!(missing.handled);
    assert!(select.selected_option().is_none());
}
