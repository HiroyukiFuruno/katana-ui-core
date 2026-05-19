use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{
    Accordion, Breadcrumb, ChoiceItem, ComboBox, DisclosureTriggerArea, MenuButton, ModalOverlay,
    Popover, SelectBox, SelectionList, SideMenu, SlideControl, Tabs, Tooltip,
};
use katana_ui_core::render_model::UiTree;

const SELECTED_INDEX: usize = 1;
const ITEM_COUNT: usize = 2;
const OFFSET_X: i16 = 8;
const OFFSET_Y: i16 = 12;
const TOOLTIP_OFFSET_Y: i16 = 6;
const RANGE_MINIMUM: i32 = 0;
const RANGE_MAXIMUM: i32 = 100;
const RANGE_STEP: i32 = 5;

#[test]
fn choice_molecules_keep_typed_items_and_apply_selection_actions() {
    let mut select = SelectBox::new("Theme")
        .placeholder("Select theme")
        .open(true)
        .item(ChoiceItem::new("dark", "Dark"))
        .item(ChoiceItem::new("light", "Light"));

    let result = select.apply_action(&UiAction::set_selected_index(
        select.state_id().clone(),
        SELECTED_INDEX,
    ));
    let tree = UiTree::new(select);

    assert!(result.handled);
    assert!(!tree.root().props().interaction.open);
    assert_eq!(
        SELECTED_INDEX,
        tree.root().props().interaction.selected_index
    );
    assert_eq!(ITEM_COUNT, tree.root().props().interaction.item_count);
    assert_eq!("Select theme", tree.root().props().placeholder);

    let combo = ComboBox::new("Command")
        .input_value("for")
        .free_input(true)
        .keyboard_navigation("arrow-down selects next filtered command")
        .filter_result(ChoiceItem::new("format", "Format"))
        .item(ChoiceItem::new("format", "Format"))
        .selected_index(0);
    let menu = MenuButton::new("Menu")
        .framed(false)
        .trigger_summary("icon button trigger")
        .select_action("open")
        .item(ChoiceItem::new("open", "Open"));
    let tabs = Tabs::new("Tabs")
        .icon_action("pin-tab")
        .item(
            ChoiceItem::new("preview", "Preview")
                .pinned(true)
                .closeable(true)
                .dirty(true)
                .group("work")
                .svg_icon("<svg data-icon=\"markdown\"/>"),
        )
        .selected_index(0);
    let breadcrumb = Breadcrumb::new("Path")
        .crumb_action("navigate-root")
        .item(ChoiceItem::new("/", "Root"));
    let side = SideMenu::new("Side")
        .hover_expansion(true)
        .item(ChoiceItem::new("files", "Files"))
        .selected_index(0);
    let list = SelectionList::new("List")
        .section("Recent")
        .marker("check")
        .more_row(true)
        .item(ChoiceItem::new("one", "One"));

    assert_eq!("format", combo.items()[0].value);
    assert_eq!("for", combo.input_model());
    assert_eq!("format", combo.filter_results()[0].value);
    assert!(combo.allows_free_input());
    assert_eq!(
        Some("format"),
        combo
            .selected_option()
            .map(|selected| selected.value.as_str())
    );
    assert_eq!(
        "arrow-down selects next filtered command",
        combo.keyboard_navigation_summary()
    );
    assert_eq!("open", menu.items()[0].value);
    assert!(!menu.framed_model());
    assert_eq!("icon button trigger", menu.trigger_model());
    assert_eq!("open", menu.select_action_model());
    assert_eq!("preview", tabs.items()[0].value);
    assert!(tabs.items()[0].pinned);
    assert!(tabs.items()[0].closeable);
    assert!(tabs.items()[0].dirty);
    assert_eq!("work", tabs.items()[0].group);
    assert_eq!("<svg data-icon=\"markdown\"/>", tabs.items()[0].svg_icon);
    assert_eq!("pin-tab", tabs.icon_action_model());
    assert_eq!("/", breadcrumb.items()[0].value);
    assert_eq!("navigate-root", breadcrumb.crumb_action_model());
    assert_eq!("files", side.items()[0].value);
    assert!(side.hover_expansion_model());
    assert_eq!("one", list.items()[0].value);
    assert_eq!("Recent", list.section_model());
    assert_eq!("check", list.marker_model());
    assert!(list.has_more_row());
}

#[test]
fn disclosure_molecules_update_open_value_and_dismiss_state() {
    let mut popover = Popover::new("Actions").open(true);
    let dismiss = UiAction::dismiss(popover.state_id().clone());
    let result = popover.apply_action(&dismiss);

    assert!(result.handled);
    assert!(!result.after.open);

    let popover_model = Popover::new("Actions")
        .placement("bottom-start")
        .offset(OFFSET_X, OFFSET_Y)
        .outside_click_dismiss(true)
        .escape_dismiss(true)
        .anchor_summary("toolbar action");
    let tooltip_model = Tooltip::new("Hint")
        .placement("top")
        .offset(0, TOOLTIP_OFFSET_Y)
        .anchor_summary("status icon");
    let overlay_model = ModalOverlay::new("Overlay")
        .backdrop("dim")
        .escape_dismiss(true)
        .focus_return("settings-button")
        .dismiss_policy("outside-disabled");
    let accordion_model = Accordion::new("Section")
        .controlled(true)
        .disabled(true)
        .multiple(true)
        .indicator_position("start")
        .trigger_area(DisclosureTriggerArea::IconAndText)
        .toggle_icon("<svg data-icon=\"chevron\"/>")
        .tree_mode(true);
    let slide_model = SlideControl::new("Opacity")
        .value("0.75")
        .range(RANGE_MINIMUM, RANGE_MAXIMUM, RANGE_STEP)
        .binding("opacity");

    assert_eq!("bottom-start", popover_model.placement_model());
    assert_eq!((OFFSET_X, OFFSET_Y), popover_model.offset_model());
    assert!(popover_model.dismisses_on_outside_click());
    assert!(popover_model.dismisses_on_escape());
    assert_eq!("toolbar action", popover_model.anchor_model());
    assert_eq!("top", tooltip_model.placement_model());
    assert_eq!("status icon", tooltip_model.anchor_model());
    assert_eq!("dim", overlay_model.backdrop_model());
    assert_eq!("settings-button", overlay_model.focus_return_model());
    assert_eq!("outside-disabled", overlay_model.dismiss_policy_model());
    assert!(accordion_model.is_controlled());
    assert!(UiTree::new(accordion_model.clone()).root().props().disabled);
    assert!(accordion_model.allows_multiple());
    assert_eq!("start", accordion_model.indicator_position_model());
    assert_eq!(
        DisclosureTriggerArea::IconAndText,
        accordion_model.trigger_area_model()
    );
    assert_eq!(
        "<svg data-icon=\"chevron\"/>",
        accordion_model.toggle_icon_model()
    );
    assert!(accordion_model.is_tree_mode());
    assert_eq!(
        (RANGE_MINIMUM, RANGE_MAXIMUM, RANGE_STEP),
        slide_model.range_model()
    );
    assert_eq!("opacity", slide_model.binding_model());
    assert!(
        UiTree::new(Tooltip::new("Hint").open(true))
            .root()
            .props()
            .interaction
            .open
    );
    assert!(
        UiTree::new(ModalOverlay::new("Overlay").open(true))
            .root()
            .props()
            .interaction
            .open
    );
    assert_eq!(
        "0.75",
        UiTree::new(SlideControl::new("Opacity").value("0.75"))
            .root()
            .props()
            .interaction
            .value
    );
}
