#![cfg(feature = "egui")]
use katana_ui_core::egui::command_chrome::{
    CommandChromePaintOperationKind, CommandChromePaintStyle, CommandChromeRasterStyle,
    EguiCommandChromeAdapter,
};
use katana_ui_core::interaction::placement::{Rect, Size};
use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::molecule::command_chrome::CommandChromeIcon;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeDisplayMode, CommandChromeDropdown,
    CommandChromeDropdownItem, CommandChromeDropdownTrigger, CommandChromeToolbar,
    CommandChromeToolbarAction, CommandChromeToolbarEvent,
};
use katana_ui_core::render_model::RGBA_CHANNEL_COUNT;
use katana_ui_core::render_model::UiIconProps;
use katana_ui_core::theme::{FontFamily, FontToken};

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 160.0;
const FONT_SIZE: f32 = 16.0;
const LINE_HEIGHT: f32 = 24.0;
const FONT_WEIGHT: u16 = 400;

#[test]
fn invalid_action_icon_fails_closed() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut toolbar = CommandChromeToolbar::new().action(
        CommandChromeAction::new("invalid", "Invalid").icon(UiIconProps::new("not-an-svg")),
    );
    let mut output = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        Vec::new(),
        &mut output,
    );
    assert!(output.is_some_and(|result| result.is_err()));
}

#[test]
fn invalid_dropdown_item_icon_fails_closed_during_layout() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let dropdown = CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary).item(
        CommandChromeDropdownItem::new("invalid", "Invalid").icon(UiIconProps::new("not-an-svg")),
    );
    let mut toolbar = CommandChromeToolbar::new()
        .action(CommandChromeAction::new("menu", "Menu").dropdown(dropdown));
    let mut output = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        Vec::new(),
        &mut output,
    );
    assert!(output.is_some_and(|result| result.is_err()));
}

#[test]
fn invalid_open_dropdown_item_fails_closed_during_presentation() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let dropdown = CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary)
        .item(CommandChromeDropdownItem::new("invalid", "Invalid"));
    let mut toolbar = CommandChromeToolbar::new()
        .action(CommandChromeAction::new("menu", "Menu").dropdown(dropdown));
    let _ = toolbar.apply_action(CommandChromeToolbarAction::update_dropdown_layout(
        "menu",
        katana_ui_core::molecule::command_chrome::CommandChromeDropdownLayout::new(
            Rect::new(0, 0, 20, 20),
            Rect::new(0, 0, 200, 100),
            Size::new(80, 30),
        ),
    ));
    let _ = toolbar.apply_action(CommandChromeToolbarAction::activate("menu"));
    let mut value = serde_json::to_value(&toolbar).expect("toolbar serialization");
    value["actions"][0]["dropdown"]["items"][0]["icon"] =
        serde_json::to_value(UiIconProps::new("not-an-svg")).expect("invalid icon serialization");
    let mut toolbar: CommandChromeToolbar =
        serde_json::from_value(value).expect("open invalid dropdown fixture");
    let mut output = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        Vec::new(),
        &mut output,
    );
    assert!(output.is_some_and(|result| result.is_err()));
}

#[test]
fn stale_open_dropdown_without_its_action_fails_closed_without_presentation() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let dropdown = CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary)
        .item(CommandChromeDropdownItem::new("item", "Item"));
    let mut toolbar = CommandChromeToolbar::new()
        .action(CommandChromeAction::new("menu", "Menu").dropdown(dropdown));
    let _ = toolbar.apply_action(CommandChromeToolbarAction::update_dropdown_layout(
        "menu",
        katana_ui_core::molecule::command_chrome::CommandChromeDropdownLayout::new(
            Rect::new(0, 0, 20, 20),
            Rect::new(0, 0, 200, 100),
            Size::new(80, 30),
        ),
    ));
    let _ = toolbar.apply_action(CommandChromeToolbarAction::activate("menu"));
    let mut value = serde_json::to_value(&toolbar).expect("toolbar serialization");
    value["actions"] = serde_json::json!([]);
    let mut toolbar: CommandChromeToolbar =
        serde_json::from_value(value).expect("stale open dropdown fixture");
    let mut output = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        Vec::new(),
        &mut output,
    );
    let output = output
        .expect("toolbar output")
        .expect("stale dropdown fails closed");
    assert!(output.record.dropdown.is_none());
}

#[test]
fn actual_egui_toolbar_uses_kuc_rasters_accesskit_and_typed_events() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut toolbar = toolbar(false);
    let mut output = None;
    let mut full_output = run_frame_preserving_textures(
        &context,
        &mut adapter,
        &mut toolbar,
        Vec::new(),
        &mut output,
    );

    assert!(output.is_some());
    let Some(output) = output else {
        return;
    };
    assert!(output.is_ok());
    let Ok(output) = output else {
        return;
    };
    assert_eq!(1, output.record.actions.len());
    assert!(output.record.actions[0].icon_raster_identity.is_some());
    assert!(output.record.actions[0].label_raster_identity.is_some());

    let Some(icon_identity) = output.record.actions[0].icon_raster_identity.as_deref() else {
        return;
    };
    assert!(
        output
            .artifact
            .paint_plan
            .operations
            .iter()
            .any(|operation| match &operation.kind {
                CommandChromePaintOperationKind::Texture { texture, .. } => {
                    texture.identity == icon_identity && texture.width > 0 && texture.height > 0
                }
                CommandChromePaintOperationKind::Fill { .. }
                | CommandChromePaintOperationKind::RoundedFill { .. } => false,
            })
    );
    assert!(!full_output.textures_delta.set.is_empty());
    full_output.textures_delta.clear();
    let Some(update) = full_output.platform_output.accesskit_update else {
        panic!("the enabled egui context did not emit an AccessKit tree update");
    };
    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Button && node.label() == Some("太字 ⭐️")
    }));

    let bounds = output.record.actions[0].bounds;
    let point = egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    );
    let mut press = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(point, true)],
        &mut press,
    );
    let mut release = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(point, false)],
        &mut release,
    );
    let Some(release) = release else {
        return;
    };
    assert!(release.is_ok());
    let Ok(release) = release else {
        return;
    };
    assert!(
        release
            .events
            .contains(&CommandChromeToolbarEvent::CommandActivated {
                action_id: "bold".into(),
            })
    );

    let mut keyboard = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![key_event(egui::Key::Home), key_event(egui::Key::Enter)],
        &mut keyboard,
    );
    let Some(keyboard) = keyboard else {
        return;
    };
    assert!(keyboard.is_ok());
    let Ok(keyboard) = keyboard else {
        return;
    };
    assert!(
        keyboard
            .events
            .contains(&CommandChromeToolbarEvent::CommandActivated {
                action_id: "bold".into(),
            })
    );
}

#[test]
fn actual_egui_toolbar_rejects_disabled_command_clicks() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut toolbar = toolbar(true);
    let mut output = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        Vec::new(),
        &mut output,
    );
    let Some(output) = output else {
        return;
    };
    let Ok(output) = output else {
        return;
    };
    let bounds = output.record.actions[0].bounds;
    let point = egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    );
    let mut press = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(point, true)],
        &mut press,
    );
    let mut release = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        vec![pointer_button(point, false)],
        &mut release,
    );
    let Some(release) = release else {
        return;
    };
    assert!(release.is_ok());
    let Ok(release) = release else {
        return;
    };
    assert!(release.events.is_empty());
}

#[test]
fn actual_egui_toolbar_emits_an_immutable_plan_from_its_final_rasters_and_bounds() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut toolbar = toolbar(false);
    let mut first = None;
    let mut first_full_output =
        run_frame_preserving_textures(&context, &mut adapter, &mut toolbar, Vec::new(), &mut first);
    let Some(first) = first else {
        panic!("the actual egui toolbar did not produce an output");
    };
    assert!(first.is_ok());
    let Ok(first) = first else {
        return;
    };
    assert_eq!(first.record, first.artifact.record);
    assert_eq!(
        first.record.bounds,
        first.artifact.paint_plan.surface_bounds
    );
    assert_eq!(64, first.artifact.frame_record_hash.len());
    assert_eq!(64, first.artifact.paint_plan_hash.len());
    let label_texture = first
        .artifact
        .paint_plan
        .operations
        .iter()
        .find_map(|operation| match &operation.kind {
            CommandChromePaintOperationKind::Texture { texture, .. }
                if texture.identity.contains("太字 ⭐️") =>
            {
                Some(texture)
            }
            _ => None,
        })
        .expect("the command chrome paint plan did not contain the platform text texture");
    assert_eq!(
        first.record.actions[0].label_raster_identity.as_deref(),
        Some(label_texture.identity.as_str())
    );
    assert_eq!(
        label_texture.rgba_pixels.len(),
        label_texture.width as usize * label_texture.height as usize * RGBA_CHANNEL_COUNT
    );
    assert!(
        label_texture
            .rgba_pixels
            .chunks_exact(RGBA_CHANNEL_COUNT)
            .any(|rgba| rgba[3] > 0 && (rgba[0] != rgba[1] || rgba[1] != rgba[2]))
    );
    assert!(!first_full_output.textures_delta.set.is_empty());
    first_full_output.textures_delta.clear();

    let mut second = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut toolbar,
        Vec::new(),
        &mut second,
    );
    let Some(second) = second else {
        panic!("the second actual egui toolbar frame did not produce an output");
    };
    assert!(second.is_ok());
    let Ok(second) = second else {
        return;
    };
    assert_eq!(first.record, second.record);
    assert_eq!(
        first.artifact.frame_record_hash,
        second.artifact.frame_record_hash
    );
    assert_eq!(
        first.artifact.paint_plan_hash,
        second.artifact.paint_plan_hash
    );
    assert_eq!(first.artifact.paint_plan, second.artifact.paint_plan);
}

fn toolbar(disabled: bool) -> CommandChromeToolbar {
    CommandChromeToolbar::new()
        .display_mode(CommandChromeDisplayMode::IconLeading)
        .action(
            CommandChromeAction::new("bold", "太字 ⭐️")
                .icon(icon())
                .tooltip("太字 ⭐️")
                .disabled(disabled),
        )
}

fn icon() -> UiIconProps {
    CommandChromeIcon::EmphasisStrong.icon_props()
}

fn raster_style() -> CommandChromeRasterStyle {
    CommandChromeRasterStyle {
        font: FontToken {
            name: "command".to_string(),
            family: FontFamily::Monospace,
            size: FONT_SIZE,
            weight: FONT_WEIGHT,
        },
        text_color_rgba: [235, 235, 235, 255],
        icon_color: RgbaColor::new(235, 235, 235, 255),
        line_height_px: LINE_HEIGHT,
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
    output: &mut Option<
        Result<
            katana_ui_core::egui::command_chrome::EguiCommandChromeOutput,
            katana_ui_core::egui::command_chrome::EguiCommandChromeError,
        >,
    >,
) -> egui::FullOutput {
    let mut full_output = run_frame_preserving_textures(context, adapter, toolbar, events, output);
    full_output.textures_delta.clear();
    full_output
}

fn run_frame_preserving_textures(
    context: &egui::Context,
    adapter: &mut EguiCommandChromeAdapter,
    toolbar: &mut CommandChromeToolbar,
    events: Vec<egui::Event>,
    output: &mut Option<
        Result<
            katana_ui_core::egui::command_chrome::EguiCommandChromeOutput,
            katana_ui_core::egui::command_chrome::EguiCommandChromeError,
        >,
    >,
) -> egui::FullOutput {
    context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            *output = Some(adapter.show_toolbar(ui, toolbar, &raster_style(), &paint_style()));
        },
    )
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
