use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeDisplayMode, CommandChromeDropdown,
    CommandChromeDropdownCloseReason, CommandChromeDropdownItem, CommandChromeDropdownTrigger,
    CommandChromeToolbar, CommandChromeToolbarEvent,
};
use katana_ui_core::molecule::toolbar::SplitAction;
use katana_ui_core::render_model::UiIconProps;
use katana_ui_core::theme::{FontFamily, FontToken};
use katana_ui_core_egui_adapter::command_chrome::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeAdapter,
    EguiCommandChromeOutput,
};

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 240.0;

#[test]
fn actual_egui_menu_only_dropdown_uses_platform_raster_accesskit_and_typed_item_event() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut toolbar = menu_only_toolbar();
    let initial = frame(&context, &mut adapter, &mut toolbar, Vec::new());
    assert!(initial.record.dropdown.is_none());

    let action_point = center(initial.record.actions[0].bounds);
    let _ = frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(action_point, true)],
    );
    let opened_frame = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(action_point, false)],
    );
    let opened = expect_output(opened_frame.1);
    let dropdown = opened
        .record
        .dropdown
        .as_ref()
        .expect("menu-only action did not render a dropdown");
    assert_eq!("code-block", dropdown.action_id);
    assert_eq!(3, dropdown.items.len());
    assert!(dropdown.items[0].icon_raster_identity.is_some());
    assert!(
        dropdown.items[1]
            .label_raster_identity
            .starts_with("command-label:⭐️ Rust:")
    );
    assert!(opened.events.iter().any(|event| {
        matches!(
            event,
            CommandChromeToolbarEvent::DropdownOpened { action_id, .. }
                if action_id.as_str() == "code-block"
        )
    }));
    assert!(
        !opened
            .events
            .iter()
            .any(|event| { matches!(event, CommandChromeToolbarEvent::CommandActivated { .. }) })
    );
    assert_eq!(opened.record, opened.artifact.record);
    assert_eq!(opened.events, opened.artifact.events);
    assert!(contains_bounds(
        opened.artifact.paint_plan.surface_bounds,
        opened.record.bounds,
    ));
    assert!(contains_bounds(
        opened.artifact.paint_plan.surface_bounds,
        dropdown.bounds,
    ));
    let Some(update) = opened_frame.0.platform_output.accesskit_update else {
        panic!("the opened dropdown did not publish an AccessKit tree");
    };
    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Button && node.label() == Some("⭐️ Rust")
    }));

    let item_point = center(dropdown.items[2].bounds);
    let _ = frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(item_point, true)],
    );
    let selected = frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(item_point, false)],
    );
    assert!(
        selected
            .events
            .contains(&CommandChromeToolbarEvent::DropdownItemActivated {
                action_id: "code-block".into(),
                item_id: "markdown".into(),
            })
    );
    assert!(
        selected
            .events
            .contains(&CommandChromeToolbarEvent::DropdownClosed {
                action_id: "code-block".into(),
                reason: CommandChromeDropdownCloseReason::ItemActivated,
            })
    );
    assert!(
        frame(&context, &mut adapter, &mut toolbar, Vec::new())
            .record
            .dropdown
            .is_none()
    );
}

#[test]
fn actual_egui_dropdown_icon_raster_failure_propagates_from_item_rendering() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut toolbar = CommandChromeToolbar::new()
        .display_mode(CommandChromeDisplayMode::LabelOnly)
        .action(
            CommandChromeAction::new("invalid-icon", "Invalid icon").dropdown(
                CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary).item(
                    CommandChromeDropdownItem::new("broken", "Broken").icon(UiIconProps::new("")),
                ),
            ),
        );

    let (_, result) = run_frame(&context, &mut adapter, &mut toolbar, Vec::new());
    let error = result
        .expect("dropdown rendering should produce a result")
        .expect_err("an empty item SVG must fail through the real raster route");
    assert!(matches!(
        error,
        katana_ui_core_egui_adapter::command_chrome::EguiCommandChromeError::Svg(_)
    ));
}

#[test]
fn actual_egui_dropdown_keyboard_skips_disabled_item_and_escape_closes_it() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut toolbar = menu_only_toolbar();
    let opened = open_dropdown(&context, &mut adapter, &mut toolbar);
    assert_eq!(Some("plain"), focused_item(&opened));

    let moved = frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![key_event(egui::Key::ArrowDown)],
    );
    assert!(
        moved
            .events
            .contains(&CommandChromeToolbarEvent::DropdownFocusChanged {
                action_id: "code-block".into(),
                item_id: "markdown".into(),
            })
    );
    let focused = frame(&context, &mut adapter, &mut toolbar, Vec::new());
    assert_eq!(Some("markdown"), focused_item(&focused));

    let escaped = frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![key_event(egui::Key::Escape)],
    );
    assert!(
        escaped
            .events
            .contains(&CommandChromeToolbarEvent::DropdownClosed {
                action_id: "code-block".into(),
                reason: CommandChromeDropdownCloseReason::Escape,
            })
    );
    assert!(
        frame(&context, &mut adapter, &mut toolbar, Vec::new())
            .record
            .dropdown
            .is_none()
    );
}

#[test]
fn actual_egui_dropdown_outside_pointer_closes_without_command_activation() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut toolbar = menu_only_toolbar();
    let opened = open_dropdown(&context, &mut adapter, &mut toolbar);
    assert!(opened.record.dropdown.is_some());

    let outside = frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(
            egui::pos2(SCREEN_WIDTH - 4.0, SCREEN_HEIGHT - 4.0),
            true,
        )],
    );

    assert!(
        outside
            .events
            .contains(&CommandChromeToolbarEvent::DropdownClosed {
                action_id: "code-block".into(),
                reason: CommandChromeDropdownCloseReason::OutsideClick,
            })
    );
    assert!(
        !outside
            .events
            .iter()
            .any(|event| { matches!(event, CommandChromeToolbarEvent::CommandActivated { .. }) })
    );
    assert!(
        frame(&context, &mut adapter, &mut toolbar, Vec::new())
            .record
            .dropdown
            .is_none()
    );
}

#[test]
fn disabled_dropdown_item_pointer_keeps_the_menu_open_without_activation() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut toolbar = menu_only_toolbar();
    let opened = open_dropdown(&context, &mut adapter, &mut toolbar);
    let disabled_item = opened
        .record
        .dropdown
        .as_ref()
        .and_then(|dropdown| dropdown.items.iter().find(|item| item.disabled))
        .expect("the fixture must expose a disabled dropdown item");

    let unchanged = frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(center(disabled_item.bounds), true)],
    );

    assert!(unchanged.record.dropdown.is_some());
    assert!(!unchanged.events.iter().any(|event| matches!(
        event,
        CommandChromeToolbarEvent::DropdownItemActivated { item_id, .. }
            if item_id.as_str() == "rust"
    )));
}

#[test]
fn actual_egui_split_secondary_keeps_primary_command_and_opens_the_generic_menu() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut toolbar = CommandChromeToolbar::new()
        .display_mode(CommandChromeDisplayMode::LabelOnly)
        .action(
            CommandChromeAction::new("insert", "Insert")
                .split(SplitAction::new(Default::default(), Default::default()))
                .dropdown(
                    CommandChromeDropdown::new(CommandChromeDropdownTrigger::SplitSecondary)
                        .item(CommandChromeDropdownItem::new("table", "Table")),
                ),
        );
    let initial = frame(&context, &mut adapter, &mut toolbar, Vec::new());
    let action = &initial.record.actions[0];
    let secondary = action
        .secondary_trigger_bounds
        .expect("split-secondary action did not expose a trigger rect");

    let primary_point = center(action.bounds);
    let _ = frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(primary_point, true)],
    );
    let primary_release = frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(primary_point, false)],
    );
    assert!(
        primary_release
            .events
            .contains(&CommandChromeToolbarEvent::CommandActivated {
                action_id: "insert".into(),
            })
    );

    let secondary_point = center(secondary);
    let _ = frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(secondary_point, true)],
    );
    let secondary_release = frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(secondary_point, false)],
    );
    assert!(secondary_release.record.dropdown.is_some());
    assert!(secondary_release.events.iter().any(|event| {
        matches!(
            event,
            CommandChromeToolbarEvent::DropdownOpened { action_id, .. }
                if action_id.as_str() == "insert"
        )
    }));
}

fn menu_only_toolbar() -> CommandChromeToolbar {
    CommandChromeToolbar::new()
        .display_mode(CommandChromeDisplayMode::LabelOnly)
        .action(
            CommandChromeAction::new("code-block", "Code block").dropdown(
                CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary)
                    .item(CommandChromeDropdownItem::new("plain", "Plain Text").icon(
                        UiIconProps::new(
                            "<svg viewBox=\"0 0 8 8\"><path d=\"M1 1h6v6H1z\"/></svg>",
                        ),
                    ))
                    .item(CommandChromeDropdownItem::new("rust", "⭐️ Rust").disabled(true))
                    .item(CommandChromeDropdownItem::new("markdown", "Markdown")),
            ),
        )
}

fn open_dropdown(
    context: &egui::Context,
    adapter: &mut EguiCommandChromeAdapter,
    toolbar: &mut CommandChromeToolbar,
) -> EguiCommandChromeOutput {
    let initial = frame(context, adapter, toolbar, Vec::new());
    let point = center(initial.record.actions[0].bounds);
    let _ = frame(context, adapter, toolbar, vec![pointer_button(point, true)]);
    frame(
        context,
        adapter,
        toolbar,
        vec![pointer_button(point, false)],
    )
}

fn focused_item(output: &EguiCommandChromeOutput) -> Option<&str> {
    output
        .record
        .dropdown
        .as_ref()?
        .items
        .iter()
        .find(|item| item.focused)
        .map(|item| item.item_id.as_str())
}

fn raster_style() -> CommandChromeRasterStyle {
    CommandChromeRasterStyle {
        font: FontToken {
            name: "command".to_string(),
            family: FontFamily::Monospace,
            size: 16.0,
            weight: 400,
        },
        text_color_rgba: [235, 235, 235, 255],
        icon_color: RgbaColor::new(235, 235, 235, 255),
        line_height_px: 24.0,
        icon_size_px: 16,
    }
}

fn paint_style() -> CommandChromePaintStyle {
    CommandChromePaintStyle {
        action_rgba: [32, 32, 32, 255],
        hovered_action_rgba: [56, 72, 96, 255],
        disabled_action_rgba: [24, 24, 24, 255],
    }
}

fn run_frame(
    context: &egui::Context,
    adapter: &mut EguiCommandChromeAdapter,
    toolbar: &mut CommandChromeToolbar,
    events: Vec<egui::Event>,
) -> (
    egui::FullOutput,
    Option<
        Result<
            EguiCommandChromeOutput,
            katana_ui_core_egui_adapter::command_chrome::EguiCommandChromeError,
        >,
    >,
) {
    let mut output = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            output = Some(adapter.show_toolbar(ui, toolbar, &raster_style(), &paint_style()));
        },
    );
    full_output.textures_delta.clear();
    (full_output, output)
}

fn frame(
    context: &egui::Context,
    adapter: &mut EguiCommandChromeAdapter,
    toolbar: &mut CommandChromeToolbar,
    events: Vec<egui::Event>,
) -> EguiCommandChromeOutput {
    expect_output(run_frame(context, adapter, toolbar, events).1)
}

fn expect_output(
    output: Option<
        Result<
            EguiCommandChromeOutput,
            katana_ui_core_egui_adapter::command_chrome::EguiCommandChromeError,
        >,
    >,
) -> EguiCommandChromeOutput {
    output
        .expect("the command chrome adapter did not produce a result")
        .expect("the command chrome adapter returned an error")
}

fn center(bounds: katana_ui_core::render_model::UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}

fn contains_bounds(
    outer: katana_ui_core::render_model::UiRect,
    inner: katana_ui_core::render_model::UiRect,
) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.x.saturating_add_unsigned(inner.width)
            <= outer.x.saturating_add_unsigned(outer.width)
        && inner.y.saturating_add_unsigned(inner.height)
            <= outer.y.saturating_add_unsigned(outer.height)
}

fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

fn key_event(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }
}
