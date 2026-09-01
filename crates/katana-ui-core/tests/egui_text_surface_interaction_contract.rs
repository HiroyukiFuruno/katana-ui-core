#![cfg(feature = "egui")]
use katana_ui_core::atom::{
    TextArea, TextAreaAction, TextAreaCompositionPhase, TextAreaEvent, TextAreaSelection,
};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::egui::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceDrawLayer, EguiTextSurfaceError,
    EguiTextSurfaceInputPolicy, EguiTextSurfaceKey, EguiTextSurfaceOutput, TextSurfaceGutterPaint,
    TextSurfacePaintOperationKind, TextSurfacePaintStyle, TextSurfaceRasterStyle,
};
use katana_ui_core::interaction::UiAction;
use katana_ui_core::render_model::{UiIconProps, UiTextSpan};
use katana_ui_core::text_raster::{PlatformTextRasterConfig, PlatformTextRasterError};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAccessibilityTarget, TextSurfaceAction,
    TextSurfaceAutomaticGutterPresentation, TextSurfaceAutomaticGutterRangeOverride,
    TextSurfaceClipboardOperation, TextSurfaceEvent, TextSurfaceGutter,
    TextSurfaceGutterRangeStartAnchor, TextSurfaceGutterRow, TextSurfaceHistoryOperation,
    TextSurfacePresentation, TextSurfaceProps, TextSurfaceScrollAlignment,
    TextSurfaceScrollRequest, TextSurfaceScrollRequestRejection, TextSurfaceScrollRequestToken,
    TextSurfaceScrollTarget, TextSurfaceViewport,
};
use katana_ui_core::theme::{FontFamily, FontToken};

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 360.0;
const SCROLL_SCREEN_HEIGHT: f32 = 64.0;
const GUTTER_WIDTH: u32 = 32;
const FONT_SIZE: f32 = 16.0;
const LINE_HEIGHT: f32 = 24.0;
const FONT_WEIGHT: u16 = 400;
const TEXT_COLOR: [u8; 4] = [235, 235, 235, 255];
const BACKGROUND_COLOR: [u8; 4] = [24, 24, 24, 255];
const GUTTER_COLOR: [u8; 4] = [32, 32, 32, 255];
const ACTIVE_GUTTER_BACKGROUND: [u8; 4] = [90, 130, 190, 220];
const HOVERED_GUTTER_BACKGROUND: [u8; 4] = [70, 110, 170, 220];
const SELECTION_COLOR: [u8; 4] = [64, 96, 160, 180];
const PREEDIT_COLOR: [u8; 4] = [255, 196, 64, 255];
const CARET_COLOR: [u8; 4] = [255, 255, 255, 255];
const TEXT_START_X: f32 = 40.0;
const TEXT_Y: f32 = 8.0;
const SCROLL_DELTA_Y: f32 = 24.0;

#[test]
fn real_egui_pointer_keyboard_scroll_and_clipboard_history_are_typed()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = surface();

    let initial = run_frame(&context, &mut adapter, &mut surface, Vec::new())?;
    let caret = initial.record.frame.selection.caret;
    let content = initial.record.frame.content_bounds;
    let start = egui::pos2(caret.x.saturating_sub(1) as f32, (caret.y + 1) as f32);
    let end = egui::pos2((content.x + 1) as f32, (caret.y + 1) as f32);
    let drag_start = egui::pos2((start.x + end.x) / 2.0, start.y);
    let keyboard_target = initial.keyboard_context_target();
    assert_eq!(
        keyboard_target.selection(),
        initial.record.frame.selection.range
    );
    assert_eq!(
        keyboard_target.viewport_bounds(),
        initial.record.frame.viewport_bounds
    );
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(start, true)],
    )?;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::PointerMoved(drag_start)],
    )?;
    let drag = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::PointerMoved(end)],
    )?;
    let released = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(end, false)],
    )?;

    assert!(
        drag.events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::SelectionChanged { .. })),
        "drag events={:?} start={start:?} end={end:?} content={content:?} caret={caret:?}",
        drag.events
    );
    assert!(
        released
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::SelectionChanged { .. }))
    );
    let after_release = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::PointerMoved(start)],
    )?;
    assert!(
        !after_release
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::SelectionChanged { .. })),
        "release must stop the physical drag"
    );
    let keyboard = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![key_event(egui::Key::ArrowRight, egui::Modifiers::default())],
    )?;
    assert!(
        keyboard
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::SelectionChanged { .. }))
    );
    let scroll = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![
            egui::Event::PointerMoved(end),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, SCROLL_DELTA_Y),
                phase: egui::TouchPhase::Move,
                modifiers: egui::Modifiers::default(),
            },
        ],
    )?;
    assert!(
        scroll
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::Scrolled { .. }))
    );

    let select_all = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![key_event(
            egui::Key::A,
            egui::Modifiers {
                command: true,
                ..egui::Modifiers::default()
            },
        )],
    )?;
    assert!(
        select_all
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::SelectionChanged { .. }))
    );
    let before_paste = surface.state().text_area.value.clone();
    let cut = run_frame(&context, &mut adapter, &mut surface, vec![egui::Event::Cut])?;
    assert!(cut.events.contains(&TextSurfaceEvent::ClipboardRequested {
        operation: TextSurfaceClipboardOperation::Cut,
        selection_start: 0,
        selection_end: before_paste.len(),
    }));
    let paste = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Paste("host-owned clipboard value".to_string())],
    )?;
    assert!(
        paste
            .events
            .contains(&TextSurfaceEvent::ClipboardRequested {
                operation: TextSurfaceClipboardOperation::Paste,
                selection_start: 0,
                selection_end: before_paste.len(),
            })
    );
    assert_eq!(before_paste, surface.state().text_area.value);
    let redo = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![key_event(
            egui::Key::Z,
            egui::Modifiers {
                command: true,
                shift: true,
                ..egui::Modifiers::default()
            },
        )],
    )?;
    assert!(redo.events.contains(&TextSurfaceEvent::HistoryRequested(
        TextSurfaceHistoryOperation::Redo,
    )));
    Ok(())
}

#[test]
fn real_egui_enter_uses_the_text_surface_submit_key_contract() -> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = surface();
    let _ = surface.apply_action(katana_ui_core::text_surface::TextSurfaceAction::SetFocus(
        true,
    ));

    let submit = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![key_event(egui::Key::Enter, egui::Modifiers::default())],
    )?;

    assert!(
        submit
            .events
            .contains(&TextSurfaceEvent::TextArea(TextAreaEvent::Submit(
                "日本語 ⭐️".to_string()
            )))
    );
    assert!(
        !submit
            .events
            .contains(&TextSurfaceEvent::TextArea(TextAreaEvent::InsertNewline))
    );
    assert_eq!("日本語 ⭐️", surface.state().text_area.value);
    Ok(())
}

#[test]
fn actual_egui_input_policy_suppresses_command_keys_before_text_selection()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = surface();
    let _ = surface.apply_action(katana_ui_core::text_surface::TextSurfaceAction::SetFocus(
        true,
    ));
    let selection = surface.state().text_area.selection;
    let output = run_frame_with_policy(
        &context,
        &mut adapter,
        &mut surface,
        vec![key_event(egui::Key::ArrowDown, egui::Modifiers::default())],
        &EguiTextSurfaceInputPolicy::default().suppress(EguiTextSurfaceKey::ArrowDown),
    )?;

    assert!(
        !output
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::SelectionChanged { .. }))
    );
    assert_eq!(selection, surface.state().text_area.selection);
    Ok(())
}

#[test]
fn actual_egui_wheel_scroll_moves_the_shared_surface_layout_inside_a_stable_viewport()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("scroll").value("一行目\n二行目\n三行目"),
        UiTextSpan::emoji_marked_spans("一行目\n二行目\n三行目", Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, LINE_HEIGHT as u32),
    ));
    let _ = surface.apply_action(katana_ui_core::text_surface::TextSurfaceAction::SetFocus(
        true,
    ));
    let first = run_frame(&context, &mut adapter, &mut surface, Vec::new())?;
    let scrolled = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![
            egui::Event::PointerMoved(egui::pos2(TEXT_START_X, TEXT_Y)),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, LINE_HEIGHT),
                phase: egui::TouchPhase::Move,
                modifiers: egui::Modifiers::default(),
            },
        ],
    )?;

    let offset = surface.state().scroll_y;
    assert!(offset > 0);
    assert!(scrolled.events.contains(&TextSurfaceEvent::Scrolled {
        scroll_x: 0,
        scroll_y: offset,
    }));
    assert_eq!(
        first.record.frame.surface_bounds,
        scrolled.record.frame.surface_bounds
    );
    assert_eq!(
        first.record.frame.viewport_bounds,
        scrolled.record.frame.viewport_bounds
    );
    assert_eq!(
        scrolled.record.frame.content_bounds.y,
        first.record.frame.content_bounds.y.saturating_sub(offset),
    );
    assert_eq!(
        scrolled.record.frame.content_bounds.y,
        scrolled.record.texture_bounds.y
    );
    assert_eq!(offset, surface.state().scroll_y);
    Ok(())
}

#[test]
fn actual_egui_wheel_scroll_clamps_to_content_extents_without_moving_the_viewport()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("scroll-bounds").value("一行目\n二行目\n三行目"),
        UiTextSpan::emoji_marked_spans("一行目\n二行目\n三行目", Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, LINE_HEIGHT as u32),
    ));
    let _ = surface.apply_action(katana_ui_core::text_surface::TextSurfaceAction::SetFocus(
        true,
    ));
    let first = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;
    let lower = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, 4_000.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::default(),
        }],
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;
    let maximum_y = i32::try_from(
        lower
            .record
            .frame
            .content_bounds
            .height
            .saturating_sub(lower.record.frame.viewport_bounds.height),
    )
    .unwrap_or(i32::MAX);

    assert!(maximum_y > 0);
    assert_eq!(maximum_y, surface.state().scroll_y);
    assert!(lower.events.contains(&TextSurfaceEvent::Scrolled {
        scroll_x: 0,
        scroll_y: maximum_y,
    }));
    assert_eq!(
        first.record.frame.surface_bounds,
        lower.record.frame.surface_bounds
    );
    assert_eq!(
        first.record.frame.viewport_bounds,
        lower.record.frame.viewport_bounds
    );

    let upper = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -4_000.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::default(),
        }],
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;

    assert_eq!(0, surface.state().scroll_y);
    assert!(upper.events.contains(&TextSurfaceEvent::Scrolled {
        scroll_x: 0,
        scroll_y: 0,
    }));
    assert_eq!(
        first.record.frame.content_bounds.y,
        upper.record.frame.content_bounds.y
    );
    Ok(())
}

#[test]
fn actual_egui_scroll_shares_coordinates_with_ime_preedit() -> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let text = "一行目\n二行目\n三行目";
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("scroll-coordinates").value(text),
        UiTextSpan::emoji_marked_spans(text, Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, LINE_HEIGHT as u32),
    ));
    let _ = surface.apply_action(katana_ui_core::text_surface::TextSurfaceAction::SetFocus(
        true,
    ));
    let _ = run_frame(&context, &mut adapter, &mut surface, Vec::new())?;
    let scrolled = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, 4_000.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::default(),
        }],
    )?;
    assert!(surface.state().scroll_y > 0);

    let preedit = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな".to_string(),
            active_range_chars: None,
        })],
    )?;
    let Some(preedit_frame) = preedit.record.frame.preedit else {
        panic!("the scrolled text surface did not expose its IME preedit frame");
    };
    assert_eq!(
        scrolled.record.frame.selection.caret.y,
        preedit_frame.caret.y
    );
    assert_eq!(
        scrolled.record.frame.viewport_bounds,
        preedit.record.frame.viewport_bounds
    );
    Ok(())
}

#[test]
fn actual_egui_viewport_measurement_uses_available_region_and_preserves_interaction_state()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let text = "一行目\n二行目\n三行目\n四行目\n五行目\n六行目\n七行目\n八行目\n九行目\n十行目\n";
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("viewport-measurement").value(text),
            UiTextSpan::emoji_marked_spans(text, Default::default()),
            TextSurfaceViewport::new(0, 0, 1, 1),
        )
        .adapter_measured_viewport(),
    );
    let _ = surface.apply_action(katana_ui_core::text_surface::TextSurfaceAction::SetFocus(
        true,
    ));

    let (compact_full, compact) = run_frame_with_full_output_and_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(640.0, 64.0)),
    )?;

    assert!(compact.record.frame.surface_bounds.width > 1);
    assert!(compact.record.frame.surface_bounds.height > 1);
    assert!(compact.record.frame.viewport_bounds.width > 1);
    assert!(compact.record.frame.viewport_bounds.height > 1);
    assert_eq!(compact.record.frame.surface_bounds.width, 640);
    assert_eq!(compact.record.frame.viewport_bounds.width, 640);
    assert_eq!(surface.props().viewport.width, 640);
    assert_eq!(surface.props().viewport.height, 64);
    let compact_accesskit = compact_full
        .platform_output
        .accesskit_update
        .expect("adapter measured viewport should emit AccessKit");
    assert!(compact_accesskit.nodes.iter().any(|(node_id, node)| {
        node.role() == egui::accesskit::Role::MultilineTextInput
            && *node_id == compact_accesskit.focus
    }));
    assert!(compact.record.frame.accessibility.root.focused);

    let scrolled = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, 4_000.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::default(),
        }],
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(640.0, 64.0)),
    )?;

    let scrolled_y = surface.state().scroll_y;
    assert!(
        scrolled.events.iter().any(
            |event| matches!(event, TextSurfaceEvent::Scrolled { scroll_y, .. } if *scroll_y > 0)
        ),
        "wheel input should emit a typed scroll event"
    );
    assert!(
        scrolled.record.frame.viewport_bounds.width == compact.record.frame.viewport_bounds.width
    );
    assert_eq!(
        compact.record.frame.viewport_bounds.y,
        scrolled.record.frame.viewport_bounds.y
    );
    assert!(scrolled_y > 0);

    let preedit = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな⭐️".to_string(),
            active_range_chars: None,
        })],
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(640.0, 64.0)),
    )?;

    let Some(preedit_frame) = preedit.record.frame.preedit else {
        panic!("the resized text surface did not expose its IME preedit frame");
    };
    assert_eq!(preedit_frame.text, "かな⭐️");
    assert_eq!(preedit.record.frame.surface_bounds.width, 640);
    assert_eq!(preedit.record.frame.viewport_bounds.width, 640);
    assert_eq!(surface.props().viewport.width, 640);
    assert_eq!(surface.props().viewport.height, 64);
    assert!(surface.state().text_area.focused);
    assert_eq!(surface.state().scroll_y, scrolled_y);
    Ok(())
}

#[test]
fn actual_egui_viewport_measurement_with_pointer_focus_preserves_scroll_ime_accesskit()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let text = "一行目\n二行目\n三行目\n四行目\n五行目";
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("viewport-measurement-pointer").value(text),
            UiTextSpan::emoji_marked_spans(text, Default::default()),
            TextSurfaceViewport::new(0, 0, 1, 1),
        )
        .adapter_measured_viewport(),
    );
    let screen_rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(640.0, 64.0));
    let initial = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        screen_rect,
    )?;
    let focus_point = center_bounds(initial.record.frame.viewport_bounds);

    let focused = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![
            egui::Event::PointerMoved(focus_point),
            pointer_button(focus_point, true),
            pointer_button(focus_point, false),
        ],
        screen_rect,
    )?;
    assert!(focused.record.frame.accessibility.root.focused);
    assert!(
        focused.record.frame.surface_bounds.width > 1,
        "focus frame should use measured width"
    );
    assert!(
        focused.record.frame.surface_bounds.height > 1,
        "focus frame should use measured height"
    );
    let scrolled = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![
            egui::Event::PointerMoved(focus_point),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, SCROLL_DELTA_Y),
                phase: egui::TouchPhase::Move,
                modifiers: egui::Modifiers::default(),
            },
        ],
        screen_rect,
    )?;

    assert!(
        scrolled
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::Scrolled { .. }))
    );
    let scrolled_y = surface.state().scroll_y;
    assert!(scrolled_y > 0);
    assert!(scrolled.record.frame.surface_bounds.width > 1);
    assert!(scrolled.record.frame.surface_bounds.height > 1);
    assert_eq!(scrolled.record.frame.surface_bounds.width, 640);
    assert_eq!(scrolled.record.frame.surface_bounds.height, 64);
    assert_eq!(scrolled.record.frame.viewport_bounds.width, 640);
    assert_eq!(scrolled.record.frame.viewport_bounds.height, 64);
    assert_eq!(surface.props().viewport.width, 640);
    assert_eq!(surface.props().viewport.height, 64);

    let (compact_full, preedit) = run_frame_with_full_output_and_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな⭐️".to_string(),
            active_range_chars: None,
        })],
        screen_rect,
    )?;

    let preedit_frame = preedit
        .record
        .frame
        .preedit
        .as_ref()
        .expect("the resized text surface did not expose its IME preedit frame");
    assert_eq!(preedit_frame.text, "かな⭐️");
    assert_eq!(preedit.record.frame.surface_bounds.width, 640);
    assert_eq!(preedit.record.frame.surface_bounds.height, 64);
    assert_eq!(preedit.record.frame.viewport_bounds.width, 640);
    assert_eq!(preedit.record.frame.viewport_bounds.height, 64);
    let compact_accesskit = compact_full
        .platform_output
        .accesskit_update
        .expect("adapter measured viewport should emit AccessKit");
    assert!(compact_accesskit.nodes.iter().any(|(node_id, node)| {
        node.role() == egui::accesskit::Role::MultilineTextInput
            && *node_id == compact_accesskit.focus
    }));
    assert!(preedit.record.frame.accessibility.root.focused);

    Ok(())
}

#[test]
fn actual_egui_scrolled_pointer_hit_test_and_ime_commit_share_one_surface_layout()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let text = "一行目\n二行目\n三行目";
    let third_line_start = "一行目\n二行目\n".len();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("scroll-pointer-ime").value(text),
        UiTextSpan::emoji_marked_spans(text, Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, LINE_HEIGHT as u32),
    ));
    let _ = surface.apply_action(katana_ui_core::text_surface::TextSurfaceAction::SetFocus(
        true,
    ));
    let _ = run_frame(&context, &mut adapter, &mut surface, Vec::new())?;
    let scrolled = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, 4_000.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::default(),
        }],
    )?;
    assert!(surface.state().scroll_y > 0);
    let visible_line_start = egui::pos2(
        scrolled.record.frame.viewport_bounds.x as f32 + 1.0,
        scrolled.record.frame.viewport_bounds.y as f32 + 1.0,
    );
    let pressed = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(visible_line_start, true)],
    )?;
    let released = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(visible_line_start, false)],
    )?;

    assert!(pressed.events.iter().chain(&released.events).any(|event| {
        matches!(
            event,
            TextSurfaceEvent::SelectionChanged {
                selection_start,
                selection_end,
            } if *selection_start == third_line_start && *selection_end == third_line_start
        )
    }));
    assert_eq!(third_line_start, surface.state().text_area.selection.start);
    assert_eq!(third_line_start, surface.state().text_area.selection.end);
    assert_eq!(
        scrolled.record.frame.viewport_bounds,
        released.record.frame.viewport_bounds
    );

    let preedit = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "⭐️".to_string(),
            active_range_chars: None,
        })],
    )?;
    let Some(preedit_frame) = preedit.record.frame.preedit else {
        panic!("the pointer-selected scrolled text surface did not expose its IME preedit");
    };
    assert!(
        preedit_frame.caret.y >= preedit.record.frame.viewport_bounds.y
            && preedit_frame.caret.y
                < preedit
                    .record
                    .frame
                    .viewport_bounds
                    .y
                    .saturating_add(preedit.record.frame.viewport_bounds.height as i32)
    );
    let committed = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    )?;

    assert_eq!("一行目\n二行目\n⭐️三行目", surface.state().text_area.value);
    assert!(committed.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::TextArea(TextAreaEvent::EmojiInput { grapheme_count: 1 })
    )));
    Ok(())
}

#[test]
fn controlled_text_surface_sync_preserves_raw_input_ime_focus_scroll_and_accesskit()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let text = "一行目\n二行目\n三行目";
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("controlled-surface").value(text),
            UiTextSpan::emoji_marked_spans(text, Default::default()),
            TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, LINE_HEIGHT as u32),
        )
        .accessibility_label("controlled surface"),
    );
    let initial = run_frame(&context, &mut adapter, &mut surface, Vec::new())?;
    let point = center_bounds(initial.record.frame.viewport_bounds);
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(point, true)],
    )?;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(point, false)],
    )?;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, 4_000.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::default(),
        }],
    )?;
    assert!(surface.state().text_area.focused);
    assert!(surface.state().scroll_y > 0);
    let preedit_value = "かな⭐️👩‍💻";
    let preedit = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: preedit_value.to_string(),
            active_range_chars: None,
        })],
    )?;
    assert_eq!(
        preedit
            .record
            .frame
            .preedit
            .as_ref()
            .map(|value| value.text.as_str()),
        Some(preedit_value)
    );
    let scroll_y = surface.state().scroll_y;
    let external_value = "一行目\n二行目\nかな⭐️👩‍💻".to_string();
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.value = external_value.clone();
    presentation.spans = UiTextSpan::emoji_marked_spans(&external_value, Default::default());
    assert!(surface.synchronize_presentation(presentation));
    let (full_output, synchronized) =
        run_frame_with_full_output(&context, &mut adapter, &mut surface, Vec::new())?;
    assert!(surface.state().text_area.focused);
    assert_eq!(surface.state().scroll_y, scroll_y);
    assert_eq!(
        synchronized
            .record
            .frame
            .preedit
            .as_ref()
            .map(|value| value.text.as_str()),
        Some(preedit_value)
    );
    assert!(synchronized.record.frame.accessibility.root.focused);
    assert_eq!(
        synchronized.record.frame.accessibility.root.label.as_str(),
        "controlled surface"
    );
    let update = full_output
        .platform_output
        .accesskit_update
        .expect("controlled synchronized surface must emit AccessKit");
    assert!(update.nodes.iter().any(|(node_id, node)| {
        node.role() == egui::accesskit::Role::MultilineTextInput && *node_id == update.focus
    }));
    let committed = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    )?;
    assert_eq!(
        surface.state().text_area.value.match_indices("⭐️").count(),
        2,
        "external value and IME commit must both preserve VS16"
    );
    assert!(committed.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::TextArea(TextAreaEvent::ImeCommit(value)) if value == "⭐️"
    )));
    assert!(committed.record.frame.layout_identity.contains("⭐️"));
    Ok(())
}

#[test]
fn controlled_automatic_gutter_uses_raw_input_layout_for_scroll_and_accesskit()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let text = "一行目\n二行目\n三行目\n四行目\n五行目\n六行目\n七行目\n八行目\n九行目\n十行目\n十一行目\n十二行目";
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("controlled-gutter").value(text),
        UiTextSpan::emoji_marked_spans(text, Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, LINE_HEIGHT as u32),
    ));
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.automatic_gutter = Some(TextSurfaceAutomaticGutterPresentation::new());
    assert!(surface.synchronize_presentation(presentation));
    let first = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;
    let point = center_bounds(first.record.frame.viewport_bounds);
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(point, true)],
    )?;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(point, false)],
    )?;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Text("追加".to_string())],
    )?;
    let (full_output, scrolled) = run_frame_with_full_output_and_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, 4_000.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::default(),
        }],
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;
    assert!(surface.state().scroll_y > 0);
    assert!(
        scrolled
            .record
            .frame
            .gutter
            .iter()
            .all(|row| { row.display_label == row.logical_row.saturating_add(1).to_string() })
    );
    for row in &scrolled.record.frame.gutter {
        let target = scrolled
            .record
            .frame
            .accessibility
            .gutter_targets
            .iter()
            .find(|target| target.label.as_str() == row.display_label)
            .expect("automatic gutter row must have a renderer-neutral accessibility target");
        assert_eq!(target.bounds, row.bounds);
    }
    let update = full_output
        .platform_output
        .accesskit_update
        .expect("controlled automatic gutter must emit AccessKit");
    for row in &scrolled.record.frame.gutter {
        let node = update
            .nodes
            .iter()
            .map(|(_, node)| node)
            .find(|node| {
                node.role() == egui::accesskit::Role::Button
                    && node.label() == Some(row.display_label.as_str())
            })
            .expect("automatic gutter row must produce an AccessKit button");
        let bounds = node
            .bounds()
            .expect("automatic gutter node must expose bounds");
        assert_eq!(bounds.y0, f64::from(row.bounds.y));
        assert_eq!(
            bounds.y1,
            f64::from(row.bounds.y.saturating_add_unsigned(row.bounds.height))
        );
    }
    Ok(())
}

#[test]
fn controlled_automatic_gutter_handles_input_crossing_a_digit_boundary()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let text = (1..=9)
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("controlled-gutter-digit-boundary").value(text.clone()),
        UiTextSpan::emoji_marked_spans(&text, Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, 900),
    ));
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.automatic_gutter = Some(TextSurfaceAutomaticGutterPresentation::new());
    assert!(surface.synchronize_presentation(presentation));

    let compact_style = TextSurfaceRasterStyle::new(
        FontToken {
            name: "compact-editor".to_string(),
            family: FontFamily::Monospace,
            size: 80.0,
            weight: FONT_WEIGHT,
        },
        TEXT_COLOR,
        72.0,
    );
    let initial = run_frame_with_raster_style(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        &compact_style,
    )?;
    let first_row = initial
        .record
        .frame
        .gutter
        .first()
        .expect("nine-line source must expose its first gutter row");
    let point = egui::pos2(
        initial
            .record
            .frame
            .viewport_bounds
            .x
            .saturating_add_unsigned(initial.record.frame.viewport_bounds.width) as f32
            - 10.0,
        first_row.bounds.y as f32 + first_row.bounds.height as f32 / 2.0,
    );
    let _ = run_frame_with_raster_style(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(point, true), pointer_button(point, false)],
        &compact_style,
    )?;
    let end = surface.state().text_area.value.len();
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::Select(
        TextAreaSelection { start: end, end },
    )));
    let updated = run_frame_with_raster_style(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Text("\n10".to_string())],
        &compact_style,
    )?;

    assert_eq!(surface.state().text_area.value.lines().count(), 10);
    assert!(surface.state().text_area.value.contains("10"));
    assert!(
        updated.record.frame.viewport_bounds.x > initial.record.frame.viewport_bounds.x,
        "value={:?} selection={:?} initial={:?} updated={:?} initial_rows={:?} updated_rows={:?}",
        surface.state().text_area.value,
        surface.state().text_area.selection,
        initial.record.frame.viewport_bounds,
        updated.record.frame.viewport_bounds,
        initial
            .record
            .frame
            .gutter
            .iter()
            .map(|row| row.display_label.as_str())
            .collect::<Vec<_>>(),
        updated
            .record
            .frame
            .gutter
            .iter()
            .map(|row| row.display_label.as_str())
            .collect::<Vec<_>>()
    );
    assert!(
        updated.record.frame.viewport_bounds.width < initial.record.frame.viewport_bounds.width,
        "initial={:?} updated={:?}",
        initial.record.frame.viewport_bounds,
        updated.record.frame.viewport_bounds
    );
    assert!(!updated.record.frame.layout_identity.is_empty());
    Ok(())
}

#[test]
fn actual_egui_automatic_gutter_active_hover_state_tracks_caret_controlled_hover_scroll_and_source_replacement()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let source = "一行目\n⭐️\n三行目\n四行目\n五行目\n六行目";
    let source_marker_start = source.find("⭐️").expect("marker source text must exist");
    let marker = TextSurfaceAutomaticGutterRangeOverride {
        byte_start: source_marker_start,
        byte_end: source_marker_start + "⭐️".len(),
        start_anchor: TextSurfaceGutterRangeStartAnchor::ContainingLine,
        marker_id: "gutter.star-marker".to_string(),
        priority: 10,
        accessibility_label: "スター行 marker".to_string(),
        accessibility_description: Some("marker line for raw input proof".to_string()),
        visual_role: "star-marker".to_string(),
        icon: Some(UiIconProps::new(
            "<svg viewBox=\"0 0 8 8\"><path fill=\"#E53935\" d=\"M0 0h4v8H0z\"/><path fill=\"#1E88E5\" d=\"M4 0h4v8H4z\"/></svg>",
        )),
    };
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("controlled-gutter-proof").value(source),
        UiTextSpan::emoji_marked_spans(source, Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, LINE_HEIGHT as u32 * 2),
    ));
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.automatic_gutter =
        Some(TextSurfaceAutomaticGutterPresentation::new().override_range(marker.clone()));
    if let Some(gutter) = presentation.automatic_gutter.as_mut() {
        gutter.hovered_rows = vec![1, 1, 99];
    }
    assert!(surface.synchronize_presentation(presentation));

    let paint_style = {
        let mut style = paint_style();
        style.gutter_paints = vec![
            TextSurfaceGutterPaint::new("star-marker", TEXT_COLOR)
                .background([48, 64, 88, 255])
                .active_background([90, 130, 190, 220])
                .hovered_background([70, 110, 170, 220]),
        ];
        style
    };

    let initial = run_frame_with_paint_style(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        &paint_style,
    )?;
    assert!(initial.record.frame.layout_identity.contains("⭐️"));
    assert!(
        initial.record.frame.content_bounds.height > initial.record.frame.viewport_bounds.height,
        "wheel proof requires content taller than the fixed surface viewport"
    );
    let initial_row = initial
        .record
        .frame
        .gutter
        .iter()
        .find(|row| row.logical_row == 1)
        .expect("初期フレームに marker row があるはず");
    assert!(initial_row.hovered);
    assert!(!initial_row.active);
    assert_eq!(
        1,
        initial
            .record
            .frame
            .gutter
            .iter()
            .filter(|row| row.hovered)
            .count(),
        "controlled hover input must be deduplicated"
    );
    assert!(
        initial
            .record
            .frame
            .gutter
            .iter()
            .all(|row| row.logical_row != 99),
        "invalid controlled hover rows must not synthesize a gutter row"
    );

    let first_row = initial
        .record
        .frame
        .gutter
        .iter()
        .find(|row| row.logical_row == 0)
        .expect("automatic gutter must expose the first KUC-derived row");
    let focus_point = egui::pos2(
        initial.record.frame.viewport_bounds.x as f32 + 1.0,
        first_row.bounds.y as f32 + first_row.bounds.height as f32 / 2.0,
    );
    let _ = run_frame_with_paint_style(
        &context,
        &mut adapter,
        &mut surface,
        vec![
            egui::Event::PointerMoved(focus_point),
            pointer_button(focus_point, true),
        ],
        &paint_style,
    )?;
    let _ = run_frame_with_paint_style(
        &context,
        &mut adapter,
        &mut surface,
        vec![
            egui::Event::PointerMoved(focus_point),
            pointer_button(focus_point, false),
        ],
        &paint_style,
    )?;
    let focused = run_frame_with_paint_style(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        &paint_style,
    )?;
    assert!(surface.state().text_area.focused);
    assert!(focused.record.frame.accessibility.root.focused);

    let (full, moved) = run_frame_with_full_output_and_paint_style(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Key {
            key: egui::Key::ArrowDown,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }],
        &paint_style,
    )?;
    let active_row = moved
        .record
        .frame
        .gutter
        .iter()
        .find(|row| row.logical_row == 1)
        .expect("ArrowDown 後に marker 行が存在するはず");
    assert!(
        active_row.active,
        "ArrowDown must activate the marker row; caret={}, selection=({}, {}), events={:?}",
        moved.record.frame.caret,
        moved.record.frame.selection_start,
        moved.record.frame.selection_end,
        moved.events
    );
    assert_eq!(Some("gutter.star-marker"), active_row.marker_id.as_deref());
    assert_gutter_fill_color(&moved, active_row.bounds, ACTIVE_GUTTER_BACKGROUND);
    let active_target = moved
        .record
        .frame
        .accessibility
        .gutter_targets
        .iter()
        .find(|target| {
            matches!(
                target.target,
                TextSurfaceAccessibilityTarget::GutterRow { logical_row }
                    if logical_row == active_row.logical_row
            )
        })
        .expect("active gutter row must have a KUC accessibility target");
    assert_eq!(active_target.bounds, active_row.bounds);
    assert!(active_target.active);
    assert!(active_target.hovered);
    let update = full
        .platform_output
        .accesskit_update
        .expect("raw input with AccessKit must emit update");
    let matching_node = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| {
            node.role() == egui::accesskit::Role::Button
                && node.label() == Some(active_target.label.as_str())
                && node.bounds().is_some_and(|bounds| {
                    bounds.x0 == f64::from(active_row.bounds.x)
                        && bounds.y0 == f64::from(active_row.bounds.y)
                        && bounds.x1
                            == f64::from(
                                active_row
                                    .bounds
                                    .x
                                    .saturating_add_unsigned(active_row.bounds.width),
                            )
                        && bounds.y1
                            == f64::from(
                                active_row
                                    .bounds
                                    .y
                                    .saturating_add_unsigned(active_row.bounds.height),
                            )
                })
        })
        .expect("active gutter row must be published as an AccessKit node");
    let matching_bounds = matching_node
        .bounds()
        .expect("active gutter accesskit node has bounds");
    assert_eq!(matching_bounds.y0, f64::from(active_row.bounds.y));

    let (_, scrolled) = run_frame_with_full_output_and_paint_style(
        &context,
        &mut adapter,
        &mut surface,
        vec![
            egui::Event::PointerMoved(center_bounds(moved.record.frame.viewport_bounds)),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, 48.0),
                phase: egui::TouchPhase::Move,
                modifiers: egui::Modifiers::default(),
            },
        ],
        &paint_style,
    )?;
    let scroll_y = scrolled.record.frame.viewport.scroll_y;
    assert!(scroll_y > 0, "wheel RawInput must change KUC scroll_y");
    assert!(scrolled.events.iter().any(|event| {
        matches!(
            event,
            TextSurfaceEvent::Scrolled {
                scroll_x: 0,
                scroll_y: event_scroll_y,
            } if *event_scroll_y == scroll_y
        )
    }));

    let marker = scrolled
        .record
        .frame
        .gutter
        .iter()
        .find(|row| row.marker_id.as_deref() == Some("gutter.star-marker"))
        .expect("range marker row must still exist");
    let marker_bounds = marker
        .marker_bounds
        .expect("range marker row must expose marker bounds");
    let marker_click = run_frame_with_paint_style(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(center_bounds(marker_bounds), true)],
        &paint_style,
    )?;
    assert!(marker_click
        .events
        .iter()
        .any(|event| matches!(event, TextSurfaceEvent::GutterMarkerActivated { logical_row: 1, marker_id } if marker_id == "gutter.star-marker")));

    let replaced_source = format!("先頭\n{source}");
    let mut replacement = TextSurfacePresentation::from_props(surface.props());
    replacement.value = replaced_source.clone();
    replacement.spans = UiTextSpan::emoji_marked_spans(&replacement.value, Default::default());
    let marker_start = replaced_source
        .find("⭐️")
        .expect("marker source text must exist");
    replacement.automatic_gutter = Some(
        TextSurfaceAutomaticGutterPresentation::new()
            .override_range(TextSurfaceAutomaticGutterRangeOverride {
                byte_start: marker_start,
                byte_end: marker_start + "⭐️".len(),
                marker_id: "gutter.star-marker".to_string(),
                priority: 10,
                accessibility_label: "スター行 marker".to_string(),
                accessibility_description: Some("marker line for raw input proof".to_string()),
                visual_role: "star-marker".to_string(),
                icon: Some(UiIconProps::new(
                    "<svg viewBox=\"0 0 8 8\"><path fill=\"#E53935\" d=\"M0 0h4v8H0z\"/><path fill=\"#1E88E5\" d=\"M4 0h4v8H4z\"/></svg>",
                )),
                start_anchor: TextSurfaceGutterRangeStartAnchor::ContainingLine,
            }),
    );
    replacement
        .automatic_gutter
        .as_mut()
        .expect("gutter should be configured")
        .hovered_rows = vec![100, 2, 2];
    assert!(surface.synchronize_presentation(replacement));
    let (replacement_full, replaced) = run_frame_with_full_output_and_paint_style(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        &paint_style,
    )?;
    assert_eq!(replaced_source, surface.state().text_area.value);
    assert!(replaced.record.frame.layout_identity.contains("⭐️"));
    let hovered_marker = replaced
        .record
        .frame
        .gutter
        .iter()
        .find(|row| row.marker_id.as_deref() == Some("gutter.star-marker"))
        .expect("source replacement must resolve the marker against its new KUC row");
    assert_eq!(2, hovered_marker.logical_row);
    assert!(hovered_marker.hovered);
    assert_gutter_fill_color(&replaced, hovered_marker.bounds, HOVERED_GUTTER_BACKGROUND);
    let hovered_target = replaced
        .record
        .frame
        .accessibility
        .gutter_targets
        .iter()
        .find(|target| {
            matches!(
                target.target,
                TextSurfaceAccessibilityTarget::GutterRow { logical_row }
                    if logical_row == hovered_marker.logical_row
            )
        })
        .expect("hovered gutter row must have a KUC accessibility target");
    assert_eq!(hovered_target.bounds, hovered_marker.bounds);
    assert_eq!(hovered_target.active, hovered_marker.active);
    assert!(hovered_target.hovered);
    assert!(
        replaced
            .record
            .frame
            .gutter
            .iter()
            .all(|row| row.logical_row != 100),
        "invalid source-replacement hover row must not synthesize geometry"
    );
    let replacement_update = replacement_full
        .platform_output
        .accesskit_update
        .expect("source replacement must publish AccessKit");
    let replacement_node = replacement_update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| {
            node.role() == egui::accesskit::Role::Button
                && node.label() == Some(hovered_target.label.as_str())
        })
        .expect("hovered gutter row must be published as an AccessKit node");
    let replacement_bounds = replacement_node
        .bounds()
        .expect("hovered gutter accesskit node has bounds");
    assert_eq!(replacement_bounds.y0, f64::from(hovered_marker.bounds.y));
    assert_eq!(replaced.record, replaced.artifact.record);
    Ok(())
}

#[test]
fn actual_egui_range_icon_marker_owns_bounds_hits_accesskit_and_artifact()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let source = "日本語\n⭐️\n第三行";
    let marker = TextSurfaceAutomaticGutterRangeOverride {
        byte_start: "日本語\n".len(),
        byte_end: "日本語\n⭐️".len(),
        start_anchor: TextSurfaceGutterRangeStartAnchor::ContainingLine,
        marker_id: "range.marker".to_string(),
        priority: 10,
        accessibility_label: "範囲 marker ⭐️".to_string(),
        accessibility_description: Some("KUC marker bounds".to_string()),
        visual_role: "range-marker".to_string(),
        icon: Some(UiIconProps::new(
            "<svg viewBox=\"0 0 8 8\"><path fill=\"#E53935\" d=\"M0 0h4v8H0z\"/><path fill=\"#1E88E5\" d=\"M4 0h4v8H4z\"/></svg>",
        )),
    };
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("range-marker").value(source),
        UiTextSpan::emoji_marked_spans(source, Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
    ));
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.automatic_gutter =
        Some(TextSurfaceAutomaticGutterPresentation::new().override_range(marker.clone()));
    assert!(surface.synchronize_presentation(presentation));

    let (full_output, first) =
        run_frame_with_full_output(&context, &mut adapter, &mut surface, Vec::new())?;
    let gutter = first
        .record
        .frame
        .gutter
        .iter()
        .find(|row| row.marker_id.as_deref() == Some("range.marker"))
        .expect("range marker did not resolve against the actual UTF-8 layout");
    let marker_bounds = gutter
        .marker_bounds
        .expect("icon marker must provide KUC-derived marker bounds");
    assert_ne!(gutter.bounds, marker_bounds);
    assert!(marker_bounds.x >= gutter.bounds.x);
    assert!(marker_bounds.y >= gutter.bounds.y);
    let marker_texture = first
        .artifact
        .paint_plan
        .operations
        .iter()
        .find_map(|operation| match (&operation.layer, &operation.kind) {
            (
                EguiTextSurfaceDrawLayer::Gutter,
                TextSurfacePaintOperationKind::Texture { bounds, texture },
            ) if *bounds == marker_bounds => Some(texture),
            _ => None,
        })
        .expect("KUC paint plan must place SVG texture at marker bounds");
    assert!(
        marker_texture
            .rgba_pixels
            .chunks_exact(4)
            .any(|pixel| { pixel[3] == u8::MAX && pixel[0] != pixel[1] && pixel[1] != pixel[2] })
    );
    assert_eq!(first.record, first.artifact.record);
    assert_eq!(64, first.artifact.frame_record_hash.len());
    assert_eq!(64, first.artifact.paint_plan_hash.len());
    let update = full_output
        .platform_output
        .accesskit_update
        .expect("actual egui frame must publish the marker AccessKit node");
    let accesskit_marker = update
        .nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| {
            node.role() == egui::accesskit::Role::Button
                && node.label() == Some("範囲 marker ⭐️")
                && node.bounds().is_some_and(|bounds| {
                    bounds.x0 == f64::from(marker_bounds.x)
                        && bounds.y0 == f64::from(marker_bounds.y)
                        && bounds.x1
                            == f64::from(
                                marker_bounds.x.saturating_add_unsigned(marker_bounds.width),
                            )
                        && bounds.y1
                            == f64::from(
                                marker_bounds
                                    .y
                                    .saturating_add_unsigned(marker_bounds.height),
                            )
                })
        })
        .expect("marker-specific AccessKit node was not published");
    let accesskit_bounds = accesskit_marker
        .bounds()
        .expect("marker-specific AccessKit node must expose marker bounds");
    assert_eq!(accesskit_bounds.x0, f64::from(marker_bounds.x));
    assert_eq!(
        accesskit_bounds.x1,
        f64::from(marker_bounds.x.saturating_add_unsigned(marker_bounds.width))
    );
    assert_eq!(accesskit_bounds.y0, f64::from(marker_bounds.y));
    assert_eq!(
        accesskit_bounds.y1,
        f64::from(
            marker_bounds
                .y
                .saturating_add_unsigned(marker_bounds.height)
        )
    );

    let marker_point = center_bounds(marker_bounds);
    let marker_click = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(marker_point, true)],
    )?;
    assert_eq!(
        [TextSurfaceEvent::GutterMarkerActivated {
            logical_row: gutter.logical_row,
            marker_id: "range.marker".to_string(),
        }],
        marker_click.events.as_slice()
    );
    let row_point = egui::pos2(gutter.bounds.x as f32 + 1.0, gutter.bounds.y as f32 + 1.0);
    let row_click = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(row_point, true)],
    )?;
    assert_eq!(
        [TextSurfaceEvent::GutterRowActivated {
            logical_row: gutter.logical_row,
        }],
        row_click.events.as_slice()
    );

    let repeated = run_frame(&context, &mut adapter, &mut surface, Vec::new())?;
    assert_eq!(
        first.artifact.frame_record_hash,
        repeated.artifact.frame_record_hash
    );
    assert_eq!(
        first.artifact.paint_plan_hash,
        repeated.artifact.paint_plan_hash
    );

    let updated = "前置\n日本語\n⭐️\n第三行";
    let mut updated_presentation = TextSurfacePresentation::from_props(surface.props());
    updated_presentation.value = updated.to_string();
    updated_presentation.spans = UiTextSpan::emoji_marked_spans(updated, Default::default());
    updated_presentation.automatic_gutter = Some(
        TextSurfaceAutomaticGutterPresentation::new().override_range(
            TextSurfaceAutomaticGutterRangeOverride {
                byte_start: "前置\n日本語\n".len(),
                byte_end: "前置\n日本語\n⭐️".len(),
                ..marker
            },
        ),
    );
    assert!(surface.synchronize_presentation(updated_presentation));
    let updated_frame = run_frame(&context, &mut adapter, &mut surface, Vec::new())?;
    assert_eq!(
        Some(2),
        updated_frame
            .record
            .frame
            .gutter
            .iter()
            .find(|row| row.marker_id.as_deref() == Some("range.marker"))
            .map(|row| row.logical_row)
    );
    Ok(())
}

#[test]
fn controlled_scroll_requests_are_layout_resolved_idempotent_and_artifacted()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let source = "一行目 日本語\n⭐️ 二行目\n三行目\n四行目\n五行目";
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("controlled-scroll").value(source),
            UiTextSpan::emoji_marked_spans(source, Default::default()),
            TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, SCROLL_SCREEN_HEIGHT as u32),
        )
        .gutter(TextSurfaceGutter::new(GUTTER_WIDTH).row(TextSurfaceGutterRow::new(0, "1"))),
    );

    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("logical-row"),
        TextSurfaceScrollTarget::LogicalRow { logical_row: 4 },
        TextSurfaceScrollAlignment::End,
    ));
    assert!(surface.synchronize_presentation(presentation));
    let (accesskit_output, logical_row) = run_frame_with_full_output_and_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;
    assert!(logical_row.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::ScrollRequestAcknowledged(value) if value.token.as_str() == "logical-row" && value.target_bounds.is_some()
    )));
    assert!(logical_row.artifact.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::ScrollRequestAcknowledged(value)
            if value.token.as_str() == "logical-row" && value.target_bounds.is_some()
    )));
    assert!(matches!(
        logical_row.record.scroll_request,
        Some(katana_ui_core::text_surface::TextSurfaceScrollRequestResult::Acknowledged(_))
    ));
    assert_eq!(logical_row.record, logical_row.artifact.record);
    assert_eq!(64, logical_row.artifact.frame_record_hash.len());
    assert!(logical_row.record.frame.viewport.scroll_y > 0);
    let update = accesskit_output
        .platform_output
        .accesskit_update
        .expect("scroll request frame must publish AccessKit");
    assert!(
        update
            .nodes
            .iter()
            .any(|(_, node)| node.role() == egui::accesskit::Role::MultilineTextInput)
    );

    let viewport_point = center_bounds(logical_row.record.frame.viewport_bounds);
    let _ = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(viewport_point, true)],
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;
    let _ = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(viewport_point, false)],
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;
    let mut same_token_presentation = TextSurfacePresentation::from_props(surface.props());
    same_token_presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("logical-row"),
        TextSurfaceScrollTarget::LogicalRow { logical_row: 4 },
        TextSurfaceScrollAlignment::End,
    ));
    assert!(!surface.synchronize_presentation(same_token_presentation));
    let wheel = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![
            egui::Event::PointerMoved(viewport_point),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, -LINE_HEIGHT),
                phase: egui::TouchPhase::Move,
                modifiers: egui::Modifiers::default(),
            },
        ],
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;
    let (wheel_x, wheel_y) = wheel
        .events
        .iter()
        .find_map(|event| match event {
            TextSurfaceEvent::Scrolled { scroll_x, scroll_y } => Some((*scroll_x, *scroll_y)),
            _ => None,
        })
        .expect("actual RawInput wheel must mutate the KUC scroll state");
    assert_eq!(wheel_x, wheel.record.frame.viewport.scroll_x);
    assert_eq!(wheel_y, wheel.record.frame.viewport.scroll_y);
    assert_ne!(logical_row.record.frame.viewport.scroll_y, wheel_y);
    assert!(
        !wheel
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::ScrollRequestAcknowledged(_)))
    );

    let star_start = source.find("⭐️").expect("VS16 star source offset");
    let star_end = star_start + "⭐️".len();
    let mut byte_offset_presentation = TextSurfacePresentation::from_props(surface.props());
    byte_offset_presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("byte-offset"),
        TextSurfaceScrollTarget::ByteOffset {
            byte_offset: star_start,
        },
        TextSurfaceScrollAlignment::Start,
    ));
    assert!(surface.synchronize_presentation(byte_offset_presentation));
    let byte_offset = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;
    assert!(byte_offset.events.iter().any(|event| matches!(
        event, TextSurfaceEvent::ScrollRequestAcknowledged(value) if value.token.as_str() == "byte-offset" && value.target_bounds.is_some()
    )));
    let mut byte_presentation = TextSurfacePresentation::from_props(surface.props());
    byte_presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("byte-range"),
        TextSurfaceScrollTarget::ByteRange {
            byte_start: star_start,
            byte_end: star_end,
        },
        TextSurfaceScrollAlignment::Center,
    ));
    assert!(surface.synchronize_presentation(byte_presentation));
    let byte_range = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;
    assert!(byte_range.events.iter().any(|event| matches!(
        event, TextSurfaceEvent::ScrollRequestAcknowledged(value) if value.token.as_str() == "byte-range" && value.target_bounds.is_some()
    )));
    assert!(!byte_range.record.frame.visible_logical_rows.is_empty());

    let before_relative_y = surface.state().scroll_y;
    let mut relative_presentation = TextSurfacePresentation::from_props(surface.props());
    relative_presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("relative"),
        TextSurfaceScrollTarget::RelativePixels {
            delta_x: 0.into(),
            delta_y: 20.6.into(),
        },
        TextSurfaceScrollAlignment::Nearest,
    ));
    assert!(surface.synchronize_presentation(relative_presentation));
    let relative = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
        ),
    )?;
    assert!(
        surface.state().scroll_y <= surface.state().scroll_bounds.expect("adapter bounds").max_y
    );
    assert!(relative.events.iter().any(|event| matches!(
        event, TextSurfaceEvent::ScrollRequestAcknowledged(value) if value.token.as_str() == "relative" && value.target_bounds.is_none() && value.scroll_y == surface.state().scroll_y
    )));
    assert!(surface.state().scroll_y >= before_relative_y);
    assert_eq!(64, relative.artifact.frame_record_hash.len());
    assert_eq!(64, relative.artifact.paint_plan_hash.len());

    Ok(())
}

#[test]
fn invalid_controlled_scroll_request_preserves_actual_raw_input_interaction_and_raster_state()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let source = "一行目 日本語\n⭐️ 二行目\n三行目\n四行目\n五行目\n六行目";
    let viewport = TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, SCROLL_SCREEN_HEIGHT as u32)
        .scroll_offset(0, LINE_HEIGHT as i32);
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("invalid-controlled-scroll").value(source),
        UiTextSpan::emoji_marked_spans(source, Default::default()),
        viewport,
    ));
    let screen = egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(SCREEN_WIDTH, SCROLL_SCREEN_HEIGHT),
    );
    let initial =
        run_frame_with_screen_rect(&context, &mut adapter, &mut surface, Vec::new(), screen)?;
    let point = center_bounds(initial.record.frame.viewport_bounds);
    let _ = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(point, true)],
        screen,
    )?;
    let _ = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(point, false)],
        screen,
    )?;
    let preedit = run_frame_with_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな⭐️".to_string(),
            active_range_chars: None,
        })],
        screen,
    )?;
    let before = surface.state().clone();
    assert!(before.text_area.focused);
    assert_eq!(
        Some("かな⭐️"),
        preedit
            .record
            .frame
            .preedit
            .as_ref()
            .map(|value| value.text.as_str())
    );
    assert!(before.scroll_y > 0);

    let invalid_offset = source
        .find("⭐️")
        .expect("VS16 source offset")
        .saturating_add(1);
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("invalid-utf8-boundary"),
        TextSurfaceScrollTarget::ByteOffset {
            byte_offset: invalid_offset,
        },
        TextSurfaceScrollAlignment::Nearest,
    ));
    assert!(surface.synchronize_presentation(presentation));
    let (full_output, invalid) = run_frame_with_full_output_and_screen_rect(
        &context,
        &mut adapter,
        &mut surface,
        Vec::new(),
        screen,
    )?;

    let rejection = TextSurfaceEvent::ScrollRequestRejected {
        token: TextSurfaceScrollRequestToken::new("invalid-utf8-boundary"),
        reason: TextSurfaceScrollRequestRejection::InvalidUtf8Boundary,
    };
    assert_eq!(std::slice::from_ref(&rejection), invalid.events.as_slice());
    assert_eq!(
        std::slice::from_ref(&rejection),
        invalid.artifact.events.as_slice()
    );
    assert_eq!(before.text_area, surface.state().text_area);
    assert_eq!(before.pointer_anchor, surface.state().pointer_anchor);
    assert_eq!(before.scroll_x, surface.state().scroll_x);
    assert_eq!(before.scroll_y, surface.state().scroll_y);
    assert_eq!(before.scroll_bounds, surface.state().scroll_bounds);
    assert_eq!(preedit.record.frame, invalid.record.frame);
    assert_eq!(
        preedit.record.raster_identity,
        invalid.record.raster_identity
    );
    assert_eq!(preedit.record.texture_bounds, invalid.record.texture_bounds);
    assert_eq!(
        preedit.record.placeholder_raster_identity,
        invalid.record.placeholder_raster_identity
    );
    assert_eq!(
        preedit.record.placeholder_texture_bounds,
        invalid.record.placeholder_texture_bounds
    );
    assert_eq!(preedit.record.hit_target, invalid.record.hit_target);
    assert_eq!(preedit.record.layers, invalid.record.layers);
    assert_eq!(preedit.record.scroll_request, None);
    assert_eq!(
        invalid.record.scroll_request,
        Some(
            katana_ui_core::text_surface::TextSurfaceScrollRequestResult::Rejected {
                token: TextSurfaceScrollRequestToken::new("invalid-utf8-boundary"),
                reason: TextSurfaceScrollRequestRejection::InvalidUtf8Boundary,
            }
        )
    );
    assert_eq!(preedit.artifact.paint_plan, invalid.artifact.paint_plan);
    assert_ne!(
        preedit.artifact.frame_record_hash,
        invalid.artifact.frame_record_hash
    );
    assert_eq!(
        preedit.artifact.paint_plan_hash,
        invalid.artifact.paint_plan_hash
    );
    assert_eq!(invalid.record, invalid.artifact.record);
    let accesskit = full_output
        .platform_output
        .accesskit_update
        .expect("invalid request frame must publish AccessKit");
    assert!(accesskit.nodes.iter().any(|(node_id, node)| {
        node.role() == egui::accesskit::Role::MultilineTextInput && *node_id == accesskit.focus
    }));
    Ok(())
}

#[test]
fn actual_raw_input_shift_f10_requests_the_keyboard_context_target()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = surface();
    let output = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Key {
            key: egui::Key::F10,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers {
                shift: true,
                ..egui::Modifiers::NONE
            },
        }],
    )?;

    let target = output
        .context_target
        .expect("Shift+F10 RawInput must resolve a context target");
    assert_eq!(output.record.frame.selection.range, target.selection());
    assert_eq!(
        output.record.frame.viewport_bounds,
        target.viewport_bounds()
    );
    assert!(output.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::ContextTargetRequested { selection }
            if *selection == output.record.frame.selection.range
    )));
    Ok(())
}

#[test]
fn actual_legacy_marker_without_icon_activates_from_the_whole_gutter_row()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let text = "marker row";
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("legacy-marker").value(text),
            UiTextSpan::emoji_marked_spans(text, Default::default()),
            TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        )
        .gutter(
            TextSurfaceGutter::new(GUTTER_WIDTH)
                .row(TextSurfaceGutterRow::new(0, "1").marker_id("legacy-marker")),
        ),
    );

    let first = run_frame(&context, &mut adapter, &mut surface, Vec::new())?;
    let gutter = first
        .record
        .frame
        .gutter
        .first()
        .expect("legacy marker gutter row must render");
    assert_eq!(Some("legacy-marker"), gutter.marker_id.as_deref());
    assert!(gutter.marker_bounds.is_none());
    let logical_row = gutter.logical_row;
    let point = center_bounds(gutter.bounds);
    let activated = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(point, true)],
    )?;

    assert!(activated.events.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::GutterMarkerActivated {
            logical_row: row,
            marker_id,
        } if *row == logical_row && marker_id == "legacy-marker"
    )));
    Ok(())
}

#[test]
fn actual_post_input_emoji_raster_failure_is_returned_without_painting()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    let mut config = PlatformTextRasterConfig::default();
    config.emoji_candidates.clear();
    config.emoji_candidate_sha256.clear();
    let mut adapter = EguiTextSurfaceAdapter::new(config);
    let text = "A";
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("post-input-raster").value(text),
        UiTextSpan::emoji_marked_spans(text, Default::default()),
        TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
    ));

    let first = run_frame(&context, &mut adapter, &mut surface, Vec::new())?;
    let point = center_bounds(first.record.frame.viewport_bounds);
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(point, true)],
    )?;
    let _ = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![pointer_button(point, false)],
    )?;
    assert!(surface.state().text_area.focused);

    let failed = run_frame(
        &context,
        &mut adapter,
        &mut surface,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_owned()))],
    );
    assert!(matches!(
        failed,
        Err(EguiTextSurfaceError::Raster(
            PlatformTextRasterError::ColorEmojiUnavailable { .. }
        ))
    ));
    Ok(())
}

#[test]
fn actual_generic_selection_with_invalid_utf8_boundaries_fails_closed_during_composition()
-> Result<(), EguiTextSurfaceError> {
    let context = egui::Context::default();
    for (start, end) in [(1, 2), (0, 1)] {
        let mut text_area = TextArea::new("invalid-selection").value("あ");
        let target = text_area.state_id().clone();
        let selected = text_area.apply_action(&UiAction::cursor_selection(target, end, start, end));
        assert!(selected.handled);
        let mut surface = TextSurface::new(TextSurfaceProps::new(
            text_area,
            UiTextSpan::emoji_marked_spans("あ", Default::default()),
            TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        ));
        let composition = surface.apply_action(TextSurfaceAction::TextArea(
            TextAreaAction::composition(TextAreaCompositionPhase::Update, "X", 1),
        ));
        assert!(composition.handled);

        let mut adapter = EguiTextSurfaceAdapter::default();
        let output = run_frame(&context, &mut adapter, &mut surface, Vec::new())?;
        assert_eq!("あ", surface.state().text_area.value);
        assert!(output.record.frame.preedit.is_none());
    }
    Ok(())
}

fn surface() -> TextSurface {
    let text = "日本語 ⭐️";
    TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("editor").value(text),
            UiTextSpan::emoji_marked_spans(text, Default::default()),
            TextSurfaceViewport::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        )
        .gutter(
            TextSurfaceGutter::new(GUTTER_WIDTH)
                .row(TextSurfaceGutterRow::new(0, "1").accessibility_label("1 行目")),
        ),
    )
}

fn center_bounds(bounds: katana_ui_core::render_model::UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}

fn raster_style() -> TextSurfaceRasterStyle {
    TextSurfaceRasterStyle::new(
        FontToken {
            name: "editor".to_string(),
            family: FontFamily::Monospace,
            size: FONT_SIZE,
            weight: FONT_WEIGHT,
        },
        TEXT_COLOR,
        LINE_HEIGHT,
    )
}

fn paint_style() -> TextSurfacePaintStyle {
    TextSurfacePaintStyle {
        background_rgba: BACKGROUND_COLOR,
        gutter_background_rgba: GUTTER_COLOR,
        gutter_paints: Vec::new(),
        selection_rgba: SELECTION_COLOR,
        preedit_rgba: PREEDIT_COLOR,
        caret_rgba: CARET_COLOR,
        annotation_paints: Vec::new(),
    }
}

fn run_frame(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
    run_frame_with_policy(
        context,
        adapter,
        surface,
        events,
        &EguiTextSurfaceInputPolicy::default(),
    )
}

fn run_frame_with_raster_style(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
    style: &TextSurfaceRasterStyle,
) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
    let mut result = None;
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
            result = Some(adapter.show(ui, surface, style, &paint_style()));
        },
    );
    full_output.textures_delta.clear();
    result.ok_or(EguiTextSurfaceError::FrameNotProduced)?
}

fn run_frame_with_paint_style(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
    style: &TextSurfacePaintStyle,
) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
    let mut result = None;
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
            result = Some(adapter.show(ui, surface, &raster_style(), style));
        },
    );
    full_output.textures_delta.clear();
    result.ok_or(EguiTextSurfaceError::FrameNotProduced)?
}

fn run_frame_with_policy(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
    input_policy: &EguiTextSurfaceInputPolicy,
) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
    let mut result = None;
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
            result = Some(adapter.show_with_input_policy(
                ui,
                surface,
                &raster_style(),
                &paint_style(),
                input_policy,
            ));
        },
    );
    full_output.textures_delta.clear();
    result.ok_or(EguiTextSurfaceError::FrameNotProduced)?
}

fn run_frame_with_screen_rect(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
    screen_rect: egui::Rect,
) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
    let mut result = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(screen_rect),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            result = Some(adapter.show_with_input_policy(
                ui,
                surface,
                &raster_style(),
                &paint_style(),
                &EguiTextSurfaceInputPolicy::default(),
            ));
        },
    );
    full_output.textures_delta.clear();
    result.ok_or(EguiTextSurfaceError::FrameNotProduced)?
}

fn run_frame_with_full_output(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
) -> Result<(egui::FullOutput, EguiTextSurfaceOutput), EguiTextSurfaceError> {
    let mut result = None;
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
            result = Some(adapter.show(ui, surface, &raster_style(), &paint_style()));
        },
    );
    full_output.textures_delta.clear();
    Ok((
        full_output,
        result.ok_or(EguiTextSurfaceError::FrameNotProduced)??,
    ))
}

fn run_frame_with_full_output_and_paint_style(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
    style: &TextSurfacePaintStyle,
) -> Result<(egui::FullOutput, EguiTextSurfaceOutput), EguiTextSurfaceError> {
    let mut result = None;
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
            result = Some(adapter.show(ui, surface, &raster_style(), style));
        },
    );
    full_output.textures_delta.clear();
    Ok((
        full_output,
        result.ok_or(EguiTextSurfaceError::FrameNotProduced)??,
    ))
}

fn assert_gutter_fill_color(
    output: &EguiTextSurfaceOutput,
    bounds: katana_ui_core::render_model::UiRect,
    color: [u8; 4],
) {
    assert!(
        output
            .artifact
            .paint_plan
            .operations
            .iter()
            .any(|operation| {
                matches!(
                    (&operation.layer, &operation.kind),
                    (
                        EguiTextSurfaceDrawLayer::Gutter,
                        TextSurfacePaintOperationKind::Fill {
                            bounds: operation_bounds,
                            color_rgba,
                        },
                    ) if *operation_bounds == bounds && *color_rgba == color
                )
            })
    );
}

fn run_frame_with_full_output_and_screen_rect(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
    screen_rect: egui::Rect,
) -> Result<(egui::FullOutput, EguiTextSurfaceOutput), EguiTextSurfaceError> {
    let mut result = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(screen_rect),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            result = Some(adapter.show(ui, surface, &raster_style(), &paint_style()));
        },
    );
    full_output.textures_delta.clear();
    Ok((
        full_output,
        result.ok_or(EguiTextSurfaceError::FrameNotProduced)??,
    ))
}

fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

fn key_event(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }
}
