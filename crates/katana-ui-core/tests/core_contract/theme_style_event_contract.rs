use katana_ui_core::adapter_contract::{AdapterExtension, EventSink, HostHandle};
use katana_ui_core::atom::{Button, Text};
use katana_ui_core::component::{Component, ComponentTree};
use katana_ui_core::event::{CommandEvent, EventRoute, PointerEvent, PointerEventKind, UiEvent};
use katana_ui_core::layout::{AlignCenter, Alignment, Column, Length, Row};
use katana_ui_core::molecule::Card;
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::{RenderContext, UiNodeId, UiNodeKind, UiTree};
use katana_ui_core::style::{StyleDeclaration, StyleProperty, StyleRule, StyleSheet, StyleValue};
use katana_ui_core::surface::{PaintRequest, SurfaceMetrics};
use katana_ui_core::theme::{ThemeId, ThemeSnapshot};
use katana_ui_core::window::{WindowCommand, WindowId};

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
fn text_and_align_center_are_core_building_blocks() {
    let tree = UiTree::new(AlignCenter::new().child(Text::new("日本語 Text 🔷").font_role("body")));

    assert_eq!(UiNodeKind::AlignCenter, tree.root().kind());
    assert_eq!(UiNodeKind::Text, tree.root().children()[0].kind());
    assert_eq!("body", tree.root().children()[0].props().font_role);
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
    let target = UiNodeId::new("button");
    let event = UiEvent::Pointer(PointerEvent {
        target: target.clone(),
        x: 1.0,
        y: 2.0,
        kind: PointerEventKind::Down,
    });
    let encoded = serde_json::to_string(&event)?;
    let decoded: UiEvent = serde_json::from_str(&encoded)?;
    let route = EventRoute::bubble(target, vec![UiNodeId::new("root")], false);

    assert_eq!(event, decoded);
    assert_eq!("button", route.order()[0].as_str());
    assert_eq!("root", route.order()[1].as_str());
    Ok(())
}
