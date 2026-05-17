use katana_ui_core::adapter_contract::{AdapterExtension, EventSink, HostHandle};
use katana_ui_core::atom::{
    Badge, Button, ColorSwatch, IconTextButton, Input, KeyCap, LoadingDots, SlideControl,
    SvgButton, Text, TextButton, Toggle,
};
use katana_ui_core::component::{Component, ComponentTree};
use katana_ui_core::event::{CommandEvent, EventRoute, PointerEvent, PointerEventKind, UiEvent};
use katana_ui_core::layout::{Alignment, Column, Length, Row, SplitPane};
use katana_ui_core::molecule::{
    Accordion, Breadcrumb, Card, CodeDiff, ColorPicker, ComboBox, CommandPalette,
    DynamicArrayEditor, MenuButton, Modal, NotificationToast, Popover, SearchBox, SegmentedToggle,
    SelectBox, SelectionList, SideMenu, StatusBar, Tabs, Toolbar, Tooltip, TreeView,
};
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::{RenderContext, UiNodeKind, UiTree};
use katana_ui_core::runtime::{AppConfig, AppHandle, AppLifecycle, Application, RuntimeAdapter};
use katana_ui_core::style::{StyleDeclaration, StyleProperty, StyleRule, StyleSheet, StyleValue};
use katana_ui_core::surface::{PaintRequest, SurfaceMetrics};
use katana_ui_core::theme::{ThemeId, ThemeSnapshot};
use katana_ui_core::window::{WindowCommand, WindowConfig, WindowId};

#[derive(Default)]
struct NoopRuntime {
    events: Vec<AppLifecycle>,
}

impl RuntimeAdapter for NoopRuntime {
    fn run(&mut self, config: AppConfig, windows: Vec<WindowConfig>) -> AppHandle {
        self.events.push(AppLifecycle::Started);
        AppHandle::new(
            config.app_id,
            windows.into_iter().map(WindowConfig::into_id).collect(),
        )
    }
}

#[test]
fn application_builds_without_framework_types() {
    let runtime = NoopRuntime::default();
    let handle = Application::new(AppConfig::new("katana-ui-core"))
        .window(WindowConfig::new("Main"))
        .run_with(runtime);

    assert_eq!("katana-ui-core", handle.app_id());
    assert_eq!(1, handle.window_ids().len());
}

#[test]
fn neutral_tree_can_represent_atoms_and_layout() {
    let button = Button::new("Save").disabled(false).focusable(true);
    let tree = UiTree::new(Row::new().child(Text::new("Title")).child(button));

    assert_eq!(UiNodeKind::Row, tree.root().kind());
    assert_eq!(2, tree.root().children().len());
}

#[test]
fn duplicate_ui_instances_have_unique_state_identity() {
    let tree = UiTree::new(
        Row::new()
            .child(Button::new("Save"))
            .child(Button::new("Save")),
    );
    let first = &tree.root().children()[0];
    let second = &tree.root().children()[1];

    assert_ne!(first.id(), second.id());
    assert_ne!(first.props().state_id, second.props().state_id);
}

#[test]
fn button_variants_keep_unique_state_identity() {
    let tree = UiTree::new(
        Row::new()
            .child(SvgButton::new("Action"))
            .child(TextButton::new("Action"))
            .child(IconTextButton::new("Action")),
    );
    let first = &tree.root().children()[0];
    let second = &tree.root().children()[1];
    let third = &tree.root().children()[2];

    assert_ne!(first.props().state_id, second.props().state_id);
    assert_ne!(second.props().state_id, third.props().state_id);
    assert_ne!(first.props().state_id, third.props().state_id);
}

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

#[test]
fn runtime_window_surface_values_are_kuc_owned() {
    let window_id = WindowId::new("main");
    let command = WindowCommand::SetTitle {
        window_id: window_id.clone(),
        title: "KUC".to_string(),
    };
    let metrics = SurfaceMetrics::new(800.0, 600.0, 2.0, 192.0);
    let request = PaintRequest::new(window_id, metrics);

    assert_eq!(Some("KUC"), command.title());
    assert_eq!(1600.0, request.metrics().physical_width());
}

#[test]
fn theme_event_and_adapter_contract_are_serializable_models() {
    fn assert_serializable<T: serde::Serialize + for<'de> serde::Deserialize<'de>>() {}

    assert_serializable::<ThemeSnapshot>();
    assert_serializable::<ThemeId>();
    assert_serializable::<UiEvent>();
    assert_serializable::<CommandEvent>();
    assert_serializable::<RenderContext>();
    assert_serializable::<EventSink>();
    assert_serializable::<HostHandle>();
    assert_serializable::<AdapterExtension>();
}

#[test]
fn panel_theme_is_configurable_in_the_core_model() {
    let tree = UiTree::new(
        Panel::new("Storybook", PanelRegion::Root, ThemeSnapshot::dark())
            .child(Text::new("Navigation"))
            .child(Button::new("Preview")),
    );

    assert_eq!(UiNodeKind::Panel, tree.root().kind());
    assert_eq!("dark", tree.root().props().theme_id);
    assert_eq!(2, tree.root().children().len());
}

#[test]
fn layout_models_keep_stable_dimensions() {
    let row = Row::new()
        .gap(Length::px(8.0))
        .align(Alignment::Center)
        .child(Text::new("A"));
    let column = Column::new().child(row);

    assert_eq!(1, column.children().len());
}

#[test]
fn theme_serialization_and_diff_are_stable() -> serde_json::Result<()> {
    let encoded = serde_json::to_string(&ThemeSnapshot::light())?;
    let decoded: ThemeSnapshot = serde_json::from_str(&encoded)?;
    let diff = decoded.diff(&ThemeSnapshot::dark());

    assert_eq!("light", decoded.id.as_str());
    assert_eq!(&["colors".to_string()], diff.changed_sections());
    Ok(())
}

#[test]
fn pure_rust_components_can_be_composed_with_late_bound_style() {
    let tree = ComponentTree::new(
        Panel::new("Storybook", PanelRegion::Root, ThemeSnapshot::dark())
            .child(Card::new("Actions").child(Button::new("Save").class("primary-action"))),
    )
    .into_tree();
    let button = &tree.root().children()[0].children()[0];
    let calm = StyleSheet::new().rule(StyleRule::class(
        "primary-action",
        vec![StyleDeclaration::new(
            StyleProperty::Background,
            StyleValue::ColorToken("accent".to_string()),
        )],
    ));
    let warning = StyleSheet::new().rule(StyleRule::class(
        "primary-action",
        vec![StyleDeclaration::new(
            StyleProperty::Background,
            StyleValue::ColorToken("warning".to_string()),
        )],
    ));

    assert_eq!(UiNodeKind::Button, button.kind());
    assert_eq!(
        Some(&StyleValue::ColorToken("accent".to_string())),
        calm.resolve(button).value(StyleProperty::Background)
    );
    assert_eq!(
        Some(&StyleValue::ColorToken("warning".to_string())),
        warning.resolve(button).value(StyleProperty::Background)
    );
}

#[test]
fn layout_and_render_model_serialize_as_neutral_tree() -> serde_json::Result<()> {
    let tree = UiTree::new(
        Row::new()
            .child(Text::new("Title"))
            .child(Button::new("Save").focusable(true)),
    );
    let encoded = serde_json::to_string(&tree)?;
    let decoded: UiTree = serde_json::from_str(&encoded)?;

    assert_eq!(UiNodeKind::Row, decoded.root().kind());
    assert_eq!(2, decoded.root().children().len());
    Ok(())
}

#[test]
fn event_serialization_and_ordering_are_neutral() -> serde_json::Result<()> {
    let target = katana_ui_core::render_model::UiNodeId::new("button");
    let event = UiEvent::Pointer(PointerEvent {
        target: target.clone(),
        x: 1.0,
        y: 2.0,
        kind: PointerEventKind::Down,
    });
    let encoded = serde_json::to_string(&event)?;
    let decoded: UiEvent = serde_json::from_str(&encoded)?;
    let route = EventRoute::bubble(
        target,
        vec![katana_ui_core::render_model::UiNodeId::new("root")],
        false,
    );

    assert_eq!(event, decoded);
    assert_eq!("button", route.order()[0].as_str());
    assert_eq!("root", route.order()[1].as_str());
    Ok(())
}
