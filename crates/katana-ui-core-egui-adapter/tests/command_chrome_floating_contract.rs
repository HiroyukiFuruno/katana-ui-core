use katana_ui_core::interaction::placement::{Rect, Size};
use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeDisplayMode, CommandChromeDropdown,
    CommandChromeDropdownCloseReason, CommandChromeDropdownItem, CommandChromeDropdownTrigger,
    CommandChromeToolbar, CommandChromeToolbarEvent, FloatingCommandToolbar,
    FloatingCommandToolbarAction, FloatingCommandToolbarCloseReason, FloatingCommandToolbarEvent,
    FloatingCommandToolbarLayout, FloatingCommandToolbarPresentation,
    FloatingCommandToolbarVisibility,
};
use katana_ui_core::render_model::{UiIconProps, UiNodeId};
use katana_ui_core::theme::{FontFamily, FontToken};
use katana_ui_core_egui_adapter::command_chrome::{
    CommandChromePaintOperationKind, CommandChromePaintStyle, CommandChromePaintTexture,
    CommandChromeRasterStyle, EguiCommandChromeAdapter, EguiCommandChromeDrawLayer,
    EguiCommandChromeFloatingArtifactFrame, EguiCommandChromeFloatingOutput,
};

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 160.0;

#[test]
fn floating_toolbar_fails_closed_when_its_action_icon_is_invalid() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let toolbar = CommandChromeToolbar::new().action(
        CommandChromeAction::new("invalid", "Invalid").icon(UiIconProps::new("not-an-svg")),
    );
    let mut floating = FloatingCommandToolbar::new(
        toolbar,
        FloatingCommandToolbarLayout::new(
            Rect::new(32, 32, 8, 8),
            Size::new(160, 48),
            Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        ),
    );
    let _ = floating.apply_action(FloatingCommandToolbarAction::Open);
    let mut output = None;

    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut output,
    );

    assert!(output.is_some_and(|result| result.is_err()));
}

#[test]
fn floating_toolbar_fails_closed_when_a_dropdown_item_icon_is_invalid() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let toolbar = CommandChromeToolbar::new().action(
        CommandChromeAction::new("menu", "Menu").dropdown(
            CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary).item(
                CommandChromeDropdownItem::new("invalid", "Invalid")
                    .icon(UiIconProps::new("not-an-svg")),
            ),
        ),
    );
    let mut floating = FloatingCommandToolbar::new(
        toolbar,
        FloatingCommandToolbarLayout::new(
            Rect::new(32, 32, 8, 8),
            Size::new(160, 48),
            Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        ),
    );
    let _ = floating.apply_action(FloatingCommandToolbarAction::Open);
    let mut output = None;

    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut output,
    );

    assert!(output.is_some_and(|result| result.is_err()));
}

#[test]
fn actual_egui_floating_toolbar_uses_core_placement_rasters_tooltip_and_accesskit() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut floating = floating(false);
    let open = floating.apply_action(FloatingCommandToolbarAction::Open);
    assert!(matches!(
        open.as_slice(),
        [FloatingCommandToolbarEvent::Opened { placement }]
            if placement.clamped
    ));

    let mut output = None;
    let mut full_output = run_frame_preserving_textures(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut output,
    );
    let output = expect_output(output);
    let emitted_textures = !full_output.textures_delta.set.is_empty();
    full_output.textures_delta.clear();
    let Some(record) = output.record else {
        panic!("the opened floating toolbar did not produce a frame record");
    };
    assert_eq!(
        floating.bounds_model().map(|bounds| (bounds.x, bounds.y)),
        Some((record.panel_bounds.x, record.panel_bounds.y))
    );
    let artifact = output
        .artifact
        .as_ref()
        .expect("the open floating toolbar did not produce an artifact");
    assert_eq!(record, artifact.record);
    assert_eq!(output.events, artifact.events);
    assert_eq!(1, record.toolbar.actions.len());
    assert!(
        contains(record.panel_bounds, record.toolbar.actions[0].bounds),
        "floating action bounds {:?} were not contained by panel {:?}",
        record.toolbar.actions[0].bounds,
        record.panel_bounds,
    );
    assert!(record.toolbar.actions[0].icon_raster_identity.is_some());
    assert!(record.toolbar.actions[0].label_raster_identity.is_some());
    assert!(contains(
        artifact.paint_plan.surface_bounds,
        record.panel_bounds
    ));
    assert!(contains(
        artifact.paint_plan.surface_bounds,
        record.toolbar.bounds
    ));
    assert!(plan_has_fill(
        artifact,
        EguiCommandChromeDrawLayer::PanelBorder,
        record.panel_bounds,
    ));
    let panel_fill = plan_fill_bounds(artifact, EguiCommandChromeDrawLayer::PanelFill)
        .expect("floating panel fill was absent from the composed plan");
    assert!(contains(record.panel_bounds, panel_fill));
    assert!(panel_fill.x > record.panel_bounds.x && panel_fill.y > record.panel_bounds.y);
    assert!(plan_has_fill(
        artifact,
        EguiCommandChromeDrawLayer::ActionFill,
        record.toolbar.actions[0].bounds,
    ));
    assert!(emitted_textures);
    let Some(update) = full_output.platform_output.accesskit_update else {
        panic!("the enabled egui context did not emit an AccessKit tree update");
    };
    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Button && node.label() == Some("太字 ⭐️")
    }));

    let action_bounds = record.toolbar.actions[0].bounds;
    let action_point = center(action_bounds);
    let mut hovered = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![egui::Event::PointerMoved(action_point)],
        &mut hovered,
    );
    let hovered = expect_output(hovered);
    let Some(hovered_record) = hovered.record else {
        panic!("the open floating toolbar disappeared while hovering");
    };
    let tooltip_bounds = hovered_record
        .tooltip_bounds
        .expect("hovering an action did not produce tooltip bounds");
    let hovered_artifact = hovered
        .artifact
        .as_ref()
        .expect("hovering an action did not produce an artifact");
    assert_eq!(hovered_record, hovered_artifact.record);
    assert_eq!(hovered.events, hovered_artifact.events);
    assert!(hovered_record.tooltip_raster_identity.is_some());
    assert!(contains(
        hovered_artifact.paint_plan.surface_bounds,
        hovered_record.panel_bounds,
    ));
    assert!(contains(
        hovered_artifact.paint_plan.surface_bounds,
        hovered_record.toolbar.bounds,
    ));
    assert!(contains(
        hovered_artifact.paint_plan.surface_bounds,
        tooltip_bounds,
    ));
    assert!(plan_has_fill(
        hovered_artifact,
        EguiCommandChromeDrawLayer::TooltipFill,
        tooltip_bounds,
    ));
    let (tooltip_texture_bounds, tooltip_texture) =
        plan_texture(hovered_artifact, EguiCommandChromeDrawLayer::TooltipTexture)
            .expect("tooltip platform text texture was absent from the composed plan");
    assert!(contains(tooltip_bounds, tooltip_texture_bounds));
    assert!(tooltip_texture.identity.contains("太字 ⭐️"));
    assert!(
        tooltip_texture
            .rgba_pixels
            .chunks_exact(4)
            .any(|rgba| rgba[3] > 0 && (rgba[0] != rgba[1] || rgba[1] != rgba[2]))
    );
}

#[test]
fn actual_egui_floating_toolbar_maps_click_and_close_to_core_events() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut floating = floating(false);
    let _ = floating.apply_action(FloatingCommandToolbarAction::Open);
    let mut initial = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut initial,
    );
    let initial = expect_output(initial);
    let action_point = center(
        initial
            .record
            .as_ref()
            .expect("the open toolbar had no frame record")
            .toolbar
            .actions[0]
            .bounds,
    );

    let mut press = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![pointer_button(action_point, true)],
        &mut press,
    );
    let mut release = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![pointer_button(action_point, false)],
        &mut release,
    );
    let release = expect_output(release);
    let release_artifact = release
        .artifact
        .as_ref()
        .expect("an open floating toolbar did not produce an artifact");
    assert_eq!(release.record.as_ref(), Some(&release_artifact.record));
    assert_eq!(release.events, release_artifact.events);
    assert!(
        release
            .events
            .contains(&FloatingCommandToolbarEvent::FocusRetained)
    );
    assert!(release.events.contains(&FloatingCommandToolbarEvent::Toolbar {
        event: katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent::CommandActivated {
            action_id: "bold".into(),
        },
    }));

    let mut outside = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![pointer_button(
            egui::pos2(SCREEN_WIDTH - 8.0, SCREEN_HEIGHT - 8.0),
            true,
        )],
        &mut outside,
    );
    let outside = expect_output(outside);
    assert!(outside.record.is_none());
    assert!(outside.artifact.is_none());
    assert!(
        outside
            .events
            .contains(&FloatingCommandToolbarEvent::Closed {
                reason: FloatingCommandToolbarCloseReason::OutsideClick,
            })
    );
    assert!(
        outside
            .events
            .contains(&FloatingCommandToolbarEvent::FocusReturnRequested {
                target: UiNodeId::new("editor-surface"),
            })
    );
}

#[test]
fn controlled_floating_presentation_uses_actual_raw_input_measurement_and_close_events() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiCommandChromeAdapter::default();
    let toolbar = CommandChromeToolbar::new()
        .display_mode(CommandChromeDisplayMode::IconLeading)
        .action(CommandChromeAction::new("controlled", "制御 ⭐️").icon(icon()));
    let mut floating = FloatingCommandToolbar::new_adapter_measured(
        toolbar,
        Rect::new(520, 120, 8, 8),
        Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
    )
    .focus_return_target(UiNodeId::new("controlled-return"));
    assert_eq!(floating.layout_model().panel_size, Size::new(0, 0));
    assert!(
        floating.synchronize_presentation(FloatingCommandToolbarPresentation::new(
            Rect::new(520, 120, 8, 8),
            Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
            FloatingCommandToolbarVisibility::Visible,
        ))
    );

    let mut first = None;
    let full = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut first,
    );
    let first = expect_output(first);
    let first_record = first
        .record
        .expect("controlled visible toolbar must render");
    assert!(first_record.panel_bounds.width > 0 && first_record.panel_bounds.height > 0);
    assert_eq!(
        floating.layout_model().panel_size,
        Size::new(
            first_record.panel_bounds.width,
            first_record.panel_bounds.height
        )
    );
    assert!(
        first.events.is_empty(),
        "controlled visibility must not synthesize Opened"
    );
    let update = full
        .platform_output
        .accesskit_update
        .expect("controlled floating toolbar must publish AccessKit");
    assert!(update.nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Button && node.label() == Some("制御 ⭐️")
    }));

    assert!(
        floating.synchronize_presentation(FloatingCommandToolbarPresentation::new(
            Rect::new(104, 40, 8, 8),
            Rect::new(0, 0, 160, 100),
            FloatingCommandToolbarVisibility::Visible,
        ))
    );
    let mut resized = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut resized,
    );
    let resized = expect_output(resized);
    let resized_record = resized.record.expect("resized toolbar must render");
    assert!(
        resized.events.is_empty(),
        "controlled frame facts must not reposition by event"
    );
    assert!(resized_record.panel_bounds.x >= 0);
    assert!(
        resized_record
            .panel_bounds
            .x
            .saturating_add_unsigned(resized_record.panel_bounds.width)
            <= 160
    );

    let mut outside = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![pointer_button(egui::pos2(1.0, 1.0), true)],
        &mut outside,
    );
    let outside = expect_output(outside);
    assert!(
        outside
            .events
            .contains(&FloatingCommandToolbarEvent::Closed {
                reason: FloatingCommandToolbarCloseReason::OutsideClick,
            })
    );
    assert!(
        outside
            .events
            .contains(&FloatingCommandToolbarEvent::FocusReturnRequested {
                target: UiNodeId::new("controlled-return"),
            })
    );

    assert!(
        floating.synchronize_presentation(FloatingCommandToolbarPresentation::new(
            Rect::new(104, 40, 8, 8),
            Rect::new(0, 0, 160, 100),
            FloatingCommandToolbarVisibility::Visible,
        ))
    );
    let mut reopened = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut reopened,
    );
    let reopened = expect_output(reopened);
    assert!(reopened.events.is_empty());
    let mut escaped = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![key_event(egui::Key::Escape)],
        &mut escaped,
    );
    let escaped = expect_output(escaped);
    assert!(
        escaped
            .events
            .contains(&FloatingCommandToolbarEvent::Closed {
                reason: FloatingCommandToolbarCloseReason::Escape,
            })
    );
    assert!(
        escaped
            .events
            .contains(&FloatingCommandToolbarEvent::FocusReturnRequested {
                target: UiNodeId::new("controlled-return"),
            })
    );
}

#[test]
fn inconsistent_open_projection_without_bounds_fails_closed() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut floating = floating(false);
    let _ = floating.apply_action(FloatingCommandToolbarAction::Open);
    let mut measured = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut measured,
    );
    assert!(expect_output(measured).record.is_some());

    let mut serialized = serde_json::to_value(&floating).expect("serialize floating projection");
    serialized["bounds"] = serde_json::Value::Null;
    let mut inconsistent: FloatingCommandToolbar =
        serde_json::from_value(serialized).expect("deserialize inconsistent projection fixture");
    let mut output = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut inconsistent,
        Vec::new(),
        &mut output,
    );
    let output = expect_output(output);
    assert!(output.record.is_none());
    assert!(output.events.is_empty());
    assert!(output.artifact.is_none());
}

#[test]
fn controlled_floating_raw_input_sequence_has_deterministic_adapter_artifact_hashes() {
    assert_eq!(
        controlled_floating_artifact_hashes(),
        controlled_floating_artifact_hashes()
    );
}

#[test]
fn actual_egui_floating_toolbar_keeps_disabled_actions_inert_and_esc_closes() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut floating = floating(true);
    let _ = floating.apply_action(FloatingCommandToolbarAction::Open);
    let mut initial = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut initial,
    );
    let initial = expect_output(initial);
    let action_point = center(
        initial
            .record
            .as_ref()
            .expect("the open toolbar had no frame record")
            .toolbar
            .actions[0]
            .bounds,
    );

    let mut press = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![pointer_button(action_point, true)],
        &mut press,
    );
    let mut release = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![pointer_button(action_point, false)],
        &mut release,
    );
    assert!(expect_output(release).events.is_empty());

    let mut escape = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![key_event(egui::Key::Escape)],
        &mut escape,
    );
    let escape = expect_output(escape);
    assert!(escape.record.is_none());
    assert!(escape.artifact.is_none());
    assert!(
        escape
            .events
            .contains(&FloatingCommandToolbarEvent::Closed {
                reason: FloatingCommandToolbarCloseReason::Escape,
            })
    );
    assert!(
        escape
            .events
            .contains(&FloatingCommandToolbarEvent::FocusReturnRequested {
                target: UiNodeId::new("editor-surface"),
            })
    );
}

#[test]
fn actual_egui_floating_dropdown_keeps_menu_pointer_inside_and_escape_closes_menu_first() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut floating = floating_menu();
    let _ = floating.apply_action(FloatingCommandToolbarAction::Open);
    let mut initial_output = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut initial_output,
    );
    let initial = expect_output(initial_output);
    let action_point = center(
        initial
            .record
            .as_ref()
            .expect("floating menu toolbar did not render")
            .toolbar
            .actions[0]
            .bounds,
    );

    let mut press = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![pointer_button(action_point, true)],
        &mut press,
    );
    let mut release = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![pointer_button(action_point, false)],
        &mut release,
    );
    let opened = expect_output(release);
    let item_point = center(
        opened
            .record
            .as_ref()
            .and_then(|record| record.toolbar.dropdown.as_ref())
            .expect("floating dropdown did not render")
            .items[0]
            .bounds,
    );

    let mut item_press = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![pointer_button(item_point, true)],
        &mut item_press,
    );
    let item_press = expect_output(item_press);
    assert!(
        !item_press
            .events
            .iter()
            .any(|event| matches!(event, FloatingCommandToolbarEvent::Closed { .. }))
    );

    let mut escape = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![key_event(egui::Key::Escape)],
        &mut escape,
    );
    let escape = expect_output(escape);
    assert!(floating.is_open());
    assert!(
        escape
            .events
            .contains(&FloatingCommandToolbarEvent::Toolbar {
                event: CommandChromeToolbarEvent::DropdownClosed {
                    action_id: "code-block".into(),
                    reason: CommandChromeDropdownCloseReason::Escape,
                },
            })
    );
    assert!(
        !escape
            .events
            .iter()
            .any(|event| matches!(event, FloatingCommandToolbarEvent::Closed { .. }))
    );

    let mut second_escape = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        vec![key_event(egui::Key::Escape)],
        &mut second_escape,
    );
    let second_escape = expect_output(second_escape);
    assert!(!floating.is_open());
    assert!(
        second_escape
            .events
            .contains(&FloatingCommandToolbarEvent::Closed {
                reason: FloatingCommandToolbarCloseReason::Escape,
            })
    );
}

#[test]
fn actual_egui_floating_toolbar_artifact_hashes_are_deterministic() {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let mut floating = floating(false);
    let _ = floating.apply_action(FloatingCommandToolbarAction::Open);

    let mut first = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut first,
    );
    let first = expect_output(first);
    let first_artifact = first
        .artifact
        .as_ref()
        .expect("the first floating toolbar frame did not produce an artifact");

    let mut second = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut second,
    );
    let second = expect_output(second);
    let second_artifact = second
        .artifact
        .as_ref()
        .expect("the second floating toolbar frame did not produce an artifact");

    assert_eq!(first.record, second.record);
    assert_eq!(first.events, second.events);
    assert_eq!(first_artifact.record, second_artifact.record);
    assert_eq!(first_artifact.paint_plan, second_artifact.paint_plan);
    assert_eq!(
        first_artifact.frame_record_hash,
        second_artifact.frame_record_hash
    );
    assert_eq!(
        first_artifact.paint_plan_hash,
        second_artifact.paint_plan_hash
    );
}

fn controlled_floating_artifact_hashes() -> Vec<(String, String)> {
    let context = egui::Context::default();
    let mut adapter = EguiCommandChromeAdapter::default();
    let toolbar = CommandChromeToolbar::new()
        .display_mode(CommandChromeDisplayMode::IconLeading)
        .action(CommandChromeAction::new("controlled", "制御 ⭐️").icon(icon()));
    let mut floating = FloatingCommandToolbar::new_adapter_measured(
        toolbar,
        Rect::new(520, 120, 8, 8),
        Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
    );
    let _ = floating.synchronize_presentation(FloatingCommandToolbarPresentation::new(
        Rect::new(520, 120, 8, 8),
        Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        FloatingCommandToolbarVisibility::Visible,
    ));
    let mut first = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut first,
    );
    let first = expect_output(first)
        .artifact
        .expect("controlled initial frame must have an adapter artifact");
    let _ = floating.synchronize_presentation(FloatingCommandToolbarPresentation::new(
        Rect::new(104, 40, 8, 8),
        Rect::new(0, 0, 160, 100),
        FloatingCommandToolbarVisibility::Visible,
    ));
    let mut resized = None;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut floating,
        Vec::new(),
        &mut resized,
    );
    let resized = expect_output(resized)
        .artifact
        .expect("controlled resized frame must have an adapter artifact");
    vec![
        (first.frame_record_hash, first.paint_plan_hash),
        (resized.frame_record_hash, resized.paint_plan_hash),
    ]
}

fn floating(disabled: bool) -> FloatingCommandToolbar {
    let toolbar = CommandChromeToolbar::new()
        .display_mode(CommandChromeDisplayMode::IconLeading)
        .action(
            CommandChromeAction::new("bold", "太字 ⭐️")
                .icon(icon())
                .tooltip("太字 ⭐️")
                .disabled(disabled),
        );
    FloatingCommandToolbar::new(
        toolbar,
        FloatingCommandToolbarLayout::new(
            Rect::new(630, 130, 8, 8),
            Size::new(160, 48),
            Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        ),
    )
    .focus_return_target(UiNodeId::new("editor-surface"))
}

fn floating_menu() -> FloatingCommandToolbar {
    let toolbar = CommandChromeToolbar::new()
        .display_mode(CommandChromeDisplayMode::LabelOnly)
        .action(
            CommandChromeAction::new("code-block", "Code block").dropdown(
                CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary)
                    .item(CommandChromeDropdownItem::new("rust", "⭐️ Rust")),
            ),
        );
    FloatingCommandToolbar::new(
        toolbar,
        FloatingCommandToolbarLayout::new(
            Rect::new(64, 64, 16, 16),
            Size::new(160, 48),
            Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        ),
    )
}

fn icon() -> UiIconProps {
    UiIconProps::new("<svg viewBox=\"0 0 16 16\"><path d=\"M2 1h8a3 3 0 0 1 0 6H2z\"/></svg>")
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
    floating: &mut FloatingCommandToolbar,
    events: Vec<egui::Event>,
    output: &mut Option<
        Result<
            EguiCommandChromeFloatingOutput,
            katana_ui_core_egui_adapter::command_chrome::EguiCommandChromeError,
        >,
    >,
) -> egui::FullOutput {
    let mut full_output = run_frame_preserving_textures(context, adapter, floating, events, output);
    full_output.textures_delta.clear();
    full_output
}

fn run_frame_preserving_textures(
    context: &egui::Context,
    adapter: &mut EguiCommandChromeAdapter,
    floating: &mut FloatingCommandToolbar,
    events: Vec<egui::Event>,
    output: &mut Option<
        Result<
            EguiCommandChromeFloatingOutput,
            katana_ui_core_egui_adapter::command_chrome::EguiCommandChromeError,
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
            *output =
                Some(adapter.show_floating_toolbar(ui, floating, &raster_style(), &paint_style()));
        },
    )
}

fn expect_output(
    output: Option<
        Result<
            EguiCommandChromeFloatingOutput,
            katana_ui_core_egui_adapter::command_chrome::EguiCommandChromeError,
        >,
    >,
) -> EguiCommandChromeFloatingOutput {
    output
        .expect("the floating toolbar did not produce an adapter result")
        .expect("the floating toolbar adapter returned an error")
}

fn center(bounds: katana_ui_core::render_model::UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}

fn contains(
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

fn plan_has_fill(
    artifact: &EguiCommandChromeFloatingArtifactFrame,
    layer: EguiCommandChromeDrawLayer,
    bounds: katana_ui_core::render_model::UiRect,
) -> bool {
    artifact.paint_plan.operations.iter().any(|operation| {
        operation.layer == layer && plan_operation_fill_bounds(&operation.kind) == Some(bounds)
    })
}

fn plan_fill_bounds(
    artifact: &EguiCommandChromeFloatingArtifactFrame,
    layer: EguiCommandChromeDrawLayer,
) -> Option<katana_ui_core::render_model::UiRect> {
    artifact
        .paint_plan
        .operations
        .iter()
        .find(|operation| operation.layer == layer)
        .and_then(|operation| plan_operation_fill_bounds(&operation.kind))
}

fn plan_operation_fill_bounds(
    kind: &CommandChromePaintOperationKind,
) -> Option<katana_ui_core::render_model::UiRect> {
    match kind {
        CommandChromePaintOperationKind::Fill { bounds, .. }
        | CommandChromePaintOperationKind::RoundedFill { bounds, .. } => Some(*bounds),
        CommandChromePaintOperationKind::Texture { .. } => None,
    }
}

fn plan_texture(
    artifact: &EguiCommandChromeFloatingArtifactFrame,
    layer: EguiCommandChromeDrawLayer,
) -> Option<(
    katana_ui_core::render_model::UiRect,
    &CommandChromePaintTexture,
)> {
    artifact.paint_plan.operations.iter().find_map(|operation| {
        (operation.layer == layer)
            .then_some(&operation.kind)
            .and_then(|kind| match kind {
                CommandChromePaintOperationKind::Texture {
                    bounds: operation_bounds,
                    texture,
                } => Some((*operation_bounds, texture)),
                CommandChromePaintOperationKind::Fill { .. }
                | CommandChromePaintOperationKind::RoundedFill { .. } => None,
            })
    })
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
