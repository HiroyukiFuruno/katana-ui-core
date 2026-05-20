use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{
    Accordion, AccordionGroup, AccordionGroupItem, DisclosureIndicatorPosition,
    DisclosureTriggerArea,
};
use katana_ui_core::render_model::{
    UiDisclosureIndicatorPosition, UiDisclosureTriggerArea, UiNodeKind, UiTree,
};

#[test]
fn accordion_trigger_area_limits_which_click_source_toggles() {
    let mut accordion = Accordion::new("Section").trigger_area(DisclosureTriggerArea::IconOnly);

    let text = accordion.apply_action(&UiAction::accordion_text_toggle(
        accordion.state_id().clone(),
    ));
    assert!(!text.handled);
    assert!(!text.after.open);

    let icon = accordion.apply_action(&UiAction::accordion_icon_toggle(
        accordion.state_id().clone(),
    ));
    assert!(icon.handled);
    assert!(icon.after.open);
}

#[test]
fn controlled_accordion_emits_request_without_mutating_internal_open_state() {
    let mut accordion = Accordion::new("Section").controlled(true).open(false);

    let result = accordion.apply_action(&UiAction::accordion_toggle(accordion.state_id().clone()));

    assert!(result.handled);
    assert!(!result.after.open);
    assert_eq!("requested_open=true", result.after.value);
}

#[test]
fn accordion_render_props_expose_disclosure_contract() {
    let tree = UiTree::new(
        Accordion::new("Tree section")
            .open(true)
            .controlled(true)
            .multiple(true)
            .indicator_position(DisclosureIndicatorPosition::Leading)
            .trigger_area(DisclosureTriggerArea::IconAndText)
            .toggle_icon("<svg data-icon=\"chevron\"/>")
            .tree_mode(true)
            .reduced_motion(true)
            .body_border(true)
            .selected(true)
            .depth(2)
            .show_lines(true),
    );
    let props = tree.root().props();

    assert_eq!(UiNodeKind::Accordion, tree.root().kind());
    assert!(props.disclosure.controlled);
    assert!(props.disclosure.multiple);
    assert_eq!(
        UiDisclosureIndicatorPosition::Leading,
        props.disclosure.indicator_position
    );
    assert_eq!(
        UiDisclosureTriggerArea::IconAndText,
        props.disclosure.trigger_area
    );
    assert_eq!("<svg data-icon=\"chevron\"/>", props.disclosure.toggle_icon);
    assert!(props.disclosure.tree_mode);
    assert!(props.disclosure.reduced_motion);
    assert!(props.disclosure.body_border);
    assert!(props.disclosure.selected);
    assert_eq!(2, props.disclosure.depth);
    assert!(props.disclosure.show_lines);
}

#[test]
fn accordion_group_single_mode_closes_previous_item() {
    let mut group = AccordionGroup::new("Sections")
        .multiple(false)
        .item(AccordionGroupItem::new("one", "One").open(true))
        .item(AccordionGroupItem::new("two", "Two"));

    let result = group.apply_action(&UiAction::set_selected_index(group.state_id().clone(), 1));

    assert!(result.handled);
    assert_eq!(vec!["two"], group.open_item_ids());
    assert_eq!("opened=two closed=one", result.after.value);
}

#[test]
fn accordion_group_multiple_mode_keeps_existing_items_open() {
    let mut group = AccordionGroup::new("Sections")
        .multiple(true)
        .item(AccordionGroupItem::new("one", "One").open(true))
        .item(AccordionGroupItem::new("two", "Two"));

    let result = group.apply_action(&UiAction::set_selected_index(group.state_id().clone(), 1));

    assert!(result.handled);
    assert_eq!(vec!["one", "two"], group.open_item_ids());
    assert_eq!(2, UiTree::new(group).root().children().len());
}
