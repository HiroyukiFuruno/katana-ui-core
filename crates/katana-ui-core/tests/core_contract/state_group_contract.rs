use katana_ui_core::atom::{
    Badge, Button, ColorSwatch, Input, KeyCap, LoadingDots, SlideControl, Text, Toggle,
};
use katana_ui_core::layout::SplitPane;
use katana_ui_core::molecule::{
    Accordion, Breadcrumb, Card, CodeDiff, ColorPicker, ComboBox, CommandPalette,
    DynamicArrayEditor, MenuButton, Modal, NotificationToast, Popover, SearchBox, SegmentedToggle,
    SelectBox, SelectionList, SideMenu, StatusBar, Tabs, Toolbar, Tooltip, TreeView,
};
use katana_ui_core::render_model::UiTree;

#[test]
fn ui_state_is_owned_by_the_component_model() {
    let tree = UiTree::new(Button::new("Save").disabled(true).focusable(true));

    assert!(tree.root().props().disabled);
    assert!(tree.root().props().focusable);
}

#[test]
fn complex_ui_state_is_owned_by_the_component_model() {
    let tree = UiTree::new(
        CommandPalette::new("Commands")
            .open(true)
            .selected_index(1)
            .item_count(2)
            .value("format"),
    );

    assert!(tree.root().props().interaction.open);
    assert!(tree.root().props().interaction.has_selection);
    assert_eq!(1, tree.root().props().interaction.selected_index);
    assert_eq!(2, tree.root().props().interaction.item_count);
    assert_eq!("format", tree.root().props().interaction.value);
}

#[test]
fn input_and_selection_state_is_owned_by_the_component_model() {
    let input = UiTree::new(Input::new("Text input").value("typed"));
    let toggle = UiTree::new(Toggle::new("Enabled").selected(true));
    let color = UiTree::new(ColorSwatch::new("Accent").value("rgba(64, 128, 255, 1)"));
    let segmented = UiTree::new(SegmentedToggle::new("Mode").selected_index(1).item_count(2));
    let select = UiTree::new(SelectBox::new("Theme").selected_index(0).item_count(2));

    assert_eq!("typed", input.root().props().interaction.value);
    assert!(toggle.root().props().interaction.has_selection);
    assert_eq!(
        "rgba(64, 128, 255, 1)",
        color.root().props().interaction.value
    );
    assert_eq!(1, segmented.root().props().interaction.selected_index);
    assert_eq!(2, segmented.root().props().interaction.item_count);
    assert_eq!(0, select.root().props().interaction.selected_index);
    assert_eq!(2, select.root().props().interaction.item_count);
}

#[test]
fn auxiliary_ui_state_and_structure_are_owned_by_the_component_model() {
    let search = UiTree::new(SearchBox::new("Search").value("query"));
    let tooltip = UiTree::new(Tooltip::new("Help").open(true).child(Text::new("Hint")));
    let badge = UiTree::new(Badge::new("Ready").accessibility_label("Status badge"));
    let key_cap = UiTree::new(KeyCap::new("Cmd K").accessibility_label("Shortcut"));
    let card = UiTree::new(Card::new("Summary").child(Text::new("Body")));

    assert_eq!("query", search.root().props().interaction.value);
    assert!(tooltip.root().props().interaction.open);
    assert_eq!("Status badge", badge.root().props().accessibility_label);
    assert_eq!("Shortcut", key_cap.root().props().accessibility_label);
    assert_eq!(1, card.root().children().len());
}

#[test]
fn disclosure_and_split_state_are_owned_by_the_component_model() {
    let accordion = UiTree::new(Accordion::new("Section").open(true));
    let modal = UiTree::new(Modal::new("Dialog").open(true));
    let popover = UiTree::new(Popover::new("Menu").open(true));
    let split = UiTree::new(SplitPane::new().value("0.5"));

    assert!(accordion.root().props().interaction.open);
    assert!(modal.root().props().interaction.open);
    assert!(popover.root().props().interaction.open);
    assert_eq!("0.5", split.root().props().interaction.value);
}

#[test]
fn color_picker_and_code_diff_state_are_owned_by_the_component_model() {
    let color_picker = UiTree::new(
        ColorPicker::new("Color")
            .open(true)
            .value("rgba(64, 128, 255, 1)"),
    );
    let code_diff = UiTree::new(CodeDiff::new("Diff").item_count(2));

    assert!(color_picker.root().props().interaction.open);
    assert_eq!(
        "rgba(64, 128, 255, 1)",
        color_picker.root().props().interaction.value
    );
    assert_eq!(2, code_diff.root().props().interaction.item_count);
}

#[test]
fn additional_ui_group_has_kuc_model_state_and_structure() {
    let tabs = UiTree::new(Tabs::new("Tabs").child(Text::new("Tab")));
    let breadcrumb = UiTree::new(Breadcrumb::new("Path").child(Text::new("Root")));
    let side_menu = UiTree::new(SideMenu::new("Side").child(Button::new("Files")));
    let selection_list = UiTree::new(SelectionList::new("Selection").child(Text::new("Item")));
    let slide_control = UiTree::new(SlideControl::new("Opacity").value("0.8"));
    let loading_dots = UiTree::new(LoadingDots::new("Loading").value("active"));
    let dynamic_array = UiTree::new(DynamicArrayEditor::new("Rows").item_count(1));
    let tree_view = UiTree::new(TreeView::new("Tree").open(true).item_count(2));
    let combo_box = UiTree::new(
        ComboBox::new("Combo")
            .open(true)
            .selected_index(0)
            .item_count(1),
    );
    let menu_button = UiTree::new(MenuButton::new("Menu").open(true).item_count(1));
    let command_palette = UiTree::new(
        CommandPalette::new("Commands")
            .open(true)
            .selected_index(0)
            .item_count(1),
    );
    let status_bar = UiTree::new(StatusBar::new("Status").child(Badge::new("Ready")));
    let toolbar = UiTree::new(Toolbar::new("Toolbar").child(Button::new("Save")));
    let toast = UiTree::new(NotificationToast::new("Toast").open(true));

    assert_eq!(1, tabs.root().children().len());
    assert_eq!(1, breadcrumb.root().children().len());
    assert_eq!(1, side_menu.root().children().len());
    assert_eq!(1, selection_list.root().children().len());
    assert_eq!("0.8", slide_control.root().props().interaction.value);
    assert_eq!("active", loading_dots.root().props().interaction.value);
    assert_eq!(1, dynamic_array.root().props().interaction.item_count);
    assert!(tree_view.root().props().interaction.open);
    assert_eq!(2, tree_view.root().props().interaction.item_count);
    assert!(combo_box.root().props().interaction.open);
    assert!(menu_button.root().props().interaction.open);
    assert!(command_palette.root().props().interaction.open);
    assert_eq!(1, status_bar.root().children().len());
    assert_eq!(1, toolbar.root().children().len());
    assert!(toast.root().props().interaction.open);
}
