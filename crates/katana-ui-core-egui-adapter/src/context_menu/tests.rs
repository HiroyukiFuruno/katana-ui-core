use super::types::{ContextMenuPaintPlan, EguiContextMenuFrameRecord, EguiContextMenuItemFrame};
use super::{
    ContextMenuPaintStyle, ContextMenuPresentation, ContextMenuPresentationItem,
    ContextMenuRasterStyle, EguiContextMenuAdapter,
};
use crate::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use crate::context_menu::ContextMenuAdapterError;
use crate::text_surface::TextSurfaceContextTargetAnchor;
use katana_ui_core::molecule::selection::{
    ContextMenuAction, ContextMenuItemKind, ContextMenuTypeAheadBuffer,
};
use katana_ui_core::molecule::selection::{
    ContextMenuAnchor, ContextMenuSize, ContextMenuViewport,
};
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_selection::UiTextSelectionRange;
use katana_ui_core::theme::{FontFamily, FontToken};

const FRAME_WIDTH_PX: f32 = 640.0;
const FRAME_HEIGHT_PX: f32 = 360.0;
const CONTEXT_X: i32 = 620;
const CONTEXT_Y: i32 = 340;
const FONT_SIZE_PX: f32 = 15.0;
const FONT_WEIGHT: u16 = 400;
const LINE_HEIGHT_PX: f32 = 22.0;
const TEXT_RGBA: [u8; 4] = [230, 230, 230, 255];
const MENU_BACKGROUND_RGBA: [u8; 4] = [30, 30, 32, 255];
const MENU_HIGHLIGHTED_RGBA: [u8; 4] = [60, 80, 112, 255];
const MENU_DISABLED_RGBA: [u8; 4] = [45, 45, 48, 255];

fn pressed_key_event(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }
}

fn collect_keyboard_actions(
    context: &egui::Context,
    events: Vec<egui::Event>,
    items: &[ContextMenuPresentationItem],
    submenu_path: &mut Vec<usize>,
    highlighted_path: &[usize],
    type_ahead: &mut ContextMenuTypeAheadBuffer,
) -> Vec<ContextMenuAction> {
    let mut actions = Vec::new();
    let mut frame_output = context.run_ui(
        egui::RawInput {
            events,
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(160.0, 40.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            actions = ui.input(|input| {
                super::interaction::keyboard_actions(
                    input,
                    items,
                    submenu_path,
                    highlighted_path,
                    type_ahead,
                )
            });
        },
    );
    frame_output.textures_delta.clear();
    actions
}

#[test]
fn actual_context_menu_adapter_keeps_opaque_tree_and_composites_repeatably()
-> Result<(), Box<dyn std::error::Error>> {
    let first = run_menu()?;
    let second = run_menu()?;
    assert_eq!(first, second);
    assert!(first.0.width > 0);
    assert!(first.0.x < CONTEXT_X);
    assert!(first.1 > 0);
    Ok(())
}

#[test]
fn actual_context_menu_artifact_route_emits_both_content_hashes()
-> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    let mut adapter = EguiContextMenuAdapter::new(
        katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
    )?;
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: vec![ContextMenuPresentationItem::action("artifact", "Artifact")],
    });
    adapter.request_open(TextSurfaceContextTargetAnchor::pointer(
        CONTEXT_X,
        CONTEXT_Y,
        UiTextSelectionRange::caret(0),
        UiRect::new(0, 0, FRAME_WIDTH_PX as u32, FRAME_HEIGHT_PX as u32),
    ));

    let mut output = None;
    let mut frame_output = context.run_ui(frame_input(), |ui| {
        output = Some(adapter.show(ui, &raster_style(), &paint_style()));
    });
    frame_output.textures_delta.clear();
    let artifact = output
        .ok_or_else(|| std::io::Error::other("actual egui frame did not run"))??
        .artifact
        .ok_or_else(|| std::io::Error::other("visible menu artifact was absent"))?;

    assert_eq!(artifact.frame_record_hash.len(), 64);
    assert_eq!(artifact.paint_plan_hash.len(), 64);
    assert_ne!(artifact.frame_record_hash, artifact.paint_plan_hash);
    assert!(!artifact.paint_plan.operations.is_empty());
    Ok(())
}

#[test]
fn context_menu_wheel_scroll_only_affects_hovered_bounds() {
    let context = egui::Context::default();
    let mut adapter = EguiContextMenuAdapter::new(
        katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
    )
    .expect("context menu adapter should be created");
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: vec![ContextMenuPresentationItem::action("a", "A")],
    });
    adapter.request_open(TextSurfaceContextTargetAnchor::pointer(
        0,
        0,
        UiTextSelectionRange::caret(0),
        UiRect::new(0, 0, 40, 20),
    ));

    let hover_bounds = UiRect::new(0, 0, 40, 20);
    let menu_height = 80;

    let run = |events: Vec<egui::Event>, adapter: &mut EguiContextMenuAdapter| {
        let mut frame_output = context.run_ui(
            egui::RawInput {
                events,
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(80.0, 40.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| {
                adapter.apply_wheel_scroll(ui, hover_bounds, menu_height);
            },
        );
        frame_output.textures_delta.clear();
        adapter.vertical_scroll_offset
    };

    let offset_without_hover = run(
        vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, 20.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::NONE,
        }],
        &mut adapter,
    );

    let offset_with_hover = run(
        vec![
            egui::Event::PointerMoved(egui::pos2(10.0, 10.0)),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, 20.0),
                phase: egui::TouchPhase::Move,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        &mut adapter,
    );

    assert_eq!(0.0, offset_without_hover);
    assert!(offset_with_hover > 0.0);
}

#[test]
fn context_menu_reveal_keyboard_highlight_keeps_scrolled_path_visible() {
    let context = egui::Context::default();
    let mut adapter = EguiContextMenuAdapter::new(
        katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
    )
    .expect("context menu adapter should be created");
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: vec![
            ContextMenuPresentationItem::action("0", "0"),
            ContextMenuPresentationItem::action("1", "1"),
            ContextMenuPresentationItem::action("2", "2"),
        ],
    });
    adapter.apply_actions([
        ContextMenuAction::OpenWithLayout {
            anchor: ContextMenuAnchor::Pointer { x: 0, y: 0 },
            menu_size: ContextMenuSize::new(60, 120),
            viewport: ContextMenuViewport::new(320, 80),
        },
        ContextMenuAction::Highlight { path: vec![2] },
    ]);
    adapter.reveal_keyboard_highlight(UiRect::new(0, 0, 80, 30), 300, 3);
    let index = adapter
        .menu
        .current_highlighted_path()
        .last()
        .cloned()
        .expect("highlighted item should exist");
    let row_top = super::types::MENU_PADDING_PX.saturating_add(
        u32::try_from(index)
            .ok()
            .unwrap_or(u32::MAX)
            .saturating_mul(super::types::ROW_HEIGHT_PX),
    ) as f32;
    let row_bottom = row_top + super::types::ROW_HEIGHT_PX as f32;
    let expected_offset = (row_bottom - 30.0).clamp(0.0, 300.0 - 30.0);
    assert_eq!(expected_offset, adapter.vertical_scroll_offset);
    let mut frame_output = context.run_ui(frame_input(), |_| {
        adapter.apply_actions([ContextMenuAction::Close {
            reason: katana_ui_core::molecule::selection::ContextMenuCloseReason::Escape,
        }]);
    });
    frame_output.textures_delta.clear();
    assert!(!adapter.menu.is_open());
}

#[test]
fn context_menu_local_anchor_returns_node_id_without_translation() {
    let viewport = UiRect::new(1, 2, 100, 200);
    let anchor = TextSurfaceContextTargetAnchor {
        anchor: katana_ui_core::render_model::UiContextMenuAnchor::NodeId(
            "menu-target".to_string(),
        ),
        selection: UiTextSelectionRange::caret(0),
        viewport_bounds: viewport,
    };

    assert_eq!(
        katana_ui_core::render_model::UiContextMenuAnchor::NodeId("menu-target".to_string()),
        super::surface::local_anchor(&anchor, viewport)
    );
}

#[test]
fn context_menu_local_anchor_translates_virtual_rect_into_viewport_space() {
    let viewport = UiRect::new(10, 20, 100, 200);
    let anchor = TextSurfaceContextTargetAnchor {
        anchor: katana_ui_core::render_model::UiContextMenuAnchor::VirtualRect(
            katana_ui_core::render_model::UiContextMenuRect::new(35, 55, 8, 12),
        ),
        selection: UiTextSelectionRange::caret(0),
        viewport_bounds: viewport,
    };

    assert_eq!(
        katana_ui_core::render_model::UiContextMenuAnchor::VirtualRect(
            katana_ui_core::render_model::UiContextMenuRect::new(25, 35, 8, 12),
        ),
        super::surface::local_anchor(&anchor, viewport)
    );
}

#[test]
fn context_menu_keyboard_actions_follows_navigation_and_submenu_control_paths() {
    let context = egui::Context::default();
    let items = vec![
        ContextMenuPresentationItem::action("first", "first"),
        ContextMenuPresentationItem::action("second", "second"),
    ];

    let mut type_ahead = ContextMenuTypeAheadBuffer::new(1000);
    let mut submenu_path = vec![0];
    let down = collect_keyboard_actions(
        &context,
        vec![pressed_key_event(egui::Key::ArrowDown)],
        &items,
        &mut submenu_path,
        &[0],
        &mut type_ahead,
    );
    assert!(!down.is_empty());

    for (key, expected_index) in [
        (egui::Key::ArrowUp, 1),
        (egui::Key::Home, 0),
        (egui::Key::End, 1),
    ] {
        let actions = collect_keyboard_actions(
            &context,
            vec![pressed_key_event(key)],
            &items,
            &mut submenu_path,
            &[0],
            &mut type_ahead,
        );
        assert!(matches!(
            actions.first(),
            Some(ContextMenuAction::Highlight { path }) if path.last() == Some(&expected_index)
        ));
    }

    let mut type_ahead = ContextMenuTypeAheadBuffer::new(1000);
    let enter = collect_keyboard_actions(
        &context,
        vec![pressed_key_event(egui::Key::Enter)],
        &items,
        &mut submenu_path,
        &[0],
        &mut type_ahead,
    );
    assert!(matches!(
        enter.first(),
        Some(ContextMenuAction::Activate { path }) if path == &vec![0]
    ));

    let space = collect_keyboard_actions(
        &context,
        vec![pressed_key_event(egui::Key::Space)],
        &items,
        &mut submenu_path,
        &[0],
        &mut type_ahead,
    );
    assert!(matches!(
        space.first(),
        Some(ContextMenuAction::Activate { path }) if path == &vec![0]
    ));

    let activate_without_highlight = collect_keyboard_actions(
        &context,
        vec![pressed_key_event(egui::Key::Enter)],
        &items,
        &mut submenu_path,
        &[],
        &mut type_ahead,
    );
    assert!(activate_without_highlight.is_empty());

    let open_submenu = collect_keyboard_actions(
        &context,
        vec![pressed_key_event(egui::Key::ArrowRight)],
        &items,
        &mut submenu_path,
        &[0],
        &mut type_ahead,
    );
    assert!(matches!(
        open_submenu.first(),
        Some(ContextMenuAction::OpenSubmenu { path }) if path == &vec![0]
    ));

    let mut submenu_path = vec![0, 1];
    let type_ahead = &mut ContextMenuTypeAheadBuffer::new(1000);
    let close_submenu = collect_keyboard_actions(
        &context,
        vec![pressed_key_event(egui::Key::ArrowLeft)],
        &items,
        &mut submenu_path,
        &[0],
        type_ahead,
    );
    assert!(close_submenu.is_empty());
    assert_eq!(vec![0], submenu_path);

    let mut type_ahead = ContextMenuTypeAheadBuffer::new(1000);
    let close = collect_keyboard_actions(
        &context,
        vec![pressed_key_event(egui::Key::Escape)],
        &items,
        &mut Vec::new(),
        &[0],
        &mut type_ahead,
    );
    assert!(matches!(
        close.first(),
        Some(ContextMenuAction::Close {
            reason: katana_ui_core::molecule::selection::ContextMenuCloseReason::Escape
        })
    ));

    let typed = collect_keyboard_actions(
        &context,
        vec![egui::Event::Text("sec".to_owned())],
        &items,
        &mut Vec::new(),
        &[],
        &mut ContextMenuTypeAheadBuffer::new(1000),
    );
    assert_eq!(
        typed,
        vec![ContextMenuAction::TypeAhead {
            prefix: "sec".to_owned(),
        }]
    );

    let ignored = collect_keyboard_actions(
        &context,
        vec![pressed_key_event(egui::Key::Tab)],
        &items,
        &mut Vec::new(),
        &[],
        &mut ContextMenuTypeAheadBuffer::new(1000),
    );
    assert!(ignored.is_empty());
}

#[test]
fn context_menu_reveal_keyboard_highlight_covers_scrolling_boundaries() {
    let mut adapter = EguiContextMenuAdapter::new(
        katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
    )
    .expect("context menu adapter should be created");
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: vec![
            ContextMenuPresentationItem::action("first", "first"),
            ContextMenuPresentationItem::action("second", "second"),
        ],
    });

    adapter.vertical_scroll_offset = 48.0;
    adapter.reveal_keyboard_highlight(UiRect::new(0, 0, 80, 30), 220, 2);
    assert_eq!(48.0, adapter.vertical_scroll_offset);

    adapter.apply_actions([ContextMenuAction::Highlight { path: vec![0] }]);

    adapter.vertical_scroll_offset = 80.0;
    adapter.submenu_path.clear();
    adapter.reveal_keyboard_highlight(UiRect::new(0, 0, 80, 30), 220, 2);
    assert_eq!(6.0, adapter.vertical_scroll_offset);

    adapter.vertical_scroll_offset = 80.0;
    adapter.submenu_path = vec![0, 1];
    adapter.reveal_keyboard_highlight(UiRect::new(0, 0, 80, 30), 220, 2);
    assert_eq!(80.0, adapter.vertical_scroll_offset);

    adapter.reveal_keyboard_highlight(UiRect::new(0, 0, 80, 30), 220, 0);
    assert_eq!(80.0, adapter.vertical_scroll_offset);
}

#[test]
fn context_menu_paint_plan_captures_icon_texture_branch() -> Result<(), Box<dyn std::error::Error>>
{
    let operations = run_menu_with_items(vec![ContextMenuPresentationItem {
        id: "icon".to_string(),
        label: "icon".to_string(),
        accessibility_label: "icon".to_string(),
        icon: Some(katana_ui_core::render_model::UiIconProps::new("<svg/>")),
        enabled: true,
        checked: false,
        kind: ContextMenuItemKind::Action,
        children: Vec::new(),
    }])?;
    assert!(
        operations.iter().any(|operation| matches!(
            &operation.kind,
            crate::context_menu::types::ContextMenuPaintOperationKind::Texture { texture, .. }
                if texture.identity.starts_with("context-menu-icon:")
        )),
        "icon item must produce icon texture operation",
    );
    Ok(())
}

#[test]
fn context_menu_adapter_error_display_and_conversion_cover_paths() {
    use katana_ui_core_svg_raster::UiSvgRasterError;
    use katana_ui_core_text_raster::PlatformTextRasterError;

    let raster = ContextMenuAdapterError::from(PlatformTextRasterError::EmptyText);
    let svg = ContextMenuAdapterError::from(UiSvgRasterError::EmptySource);
    let artifact = ContextMenuAdapterError::ArtifactSerialization("artifact".to_string());

    assert!(matches!(raster, ContextMenuAdapterError::Raster(_)));
    assert!(matches!(svg, ContextMenuAdapterError::Svg(_)));
    assert!(matches!(
        artifact,
        ContextMenuAdapterError::ArtifactSerialization(_)
    ));
    assert!(format!("{raster}").contains("context menu raster failed"));
    assert!(format!("{svg}").contains("context menu SVG raster failed"));
    assert!(format!("{artifact}").contains("context menu artifact serialization failed"));
}

#[test]
fn context_menu_artifact_frame_generates_separate_payload_hashes()
-> Result<(), ContextMenuAdapterError> {
    let record = EguiContextMenuFrameRecord {
        bounds: UiRect::new(0, 0, 80, 40),
        viewport_bounds: UiRect::new(10, 10, 120, 200),
        highlighted_path: vec![0],
        focused: false,
        items: vec![EguiContextMenuItemFrame {
            id: "item".to_string(),
            bounds: UiRect::new(0, 0, 80, 20),
            disabled: false,
            checked: false,
        }],
    };
    let paint_plan = ContextMenuPaintPlan {
        surface_bounds: UiRect::new(0, 0, 120, 200),
        operations: vec![],
    };

    let frame = super::artifact::artifact_frame(record.clone(), paint_plan, vec![])?;

    assert_eq!(frame.frame_record_hash.len(), 64);
    assert_eq!(frame.paint_plan_hash.len(), 64);
    assert_ne!(frame.frame_record_hash, frame.paint_plan_hash);
    assert_eq!(frame.record, record);
    assert_eq!(frame.paint_plan.surface_bounds, UiRect::new(0, 0, 120, 200));
    Ok(())
}

#[test]
fn context_menu_artifact_hash_propagates_serialization_failure() {
    use serde::{Serialize, Serializer};

    struct FailingSerialization;

    impl Serialize for FailingSerialization {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(serde::ser::Error::custom("intentional failure"))
        }
    }

    let error = super::artifact::artifact_hash(&FailingSerialization)
        .err()
        .map(|error| error.to_string());
    assert!(error.as_deref().is_some_and(|message| {
        message.contains("intentional")
            && message.contains("context menu artifact serialization failed")
    }));
}

fn run_menu() -> Result<(UiRect, usize, String), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiContextMenuAdapter::new(
        katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
    )?;
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: vec![
            ContextMenuPresentationItem::action("format", "整形 ⭐️"),
            ContextMenuPresentationItem::action("code", "コード種別").child(
                ContextMenuPresentationItem::action("opaque-code-kind", "code kind"),
            ),
            ContextMenuPresentationItem {
                id: "disabled".to_string(),
                label: "利用不可".to_string(),
                accessibility_label: "利用不可".to_string(),
                icon: None,
                enabled: false,
                checked: false,
                kind: ContextMenuItemKind::Action,
                children: Vec::new(),
            },
        ],
    });
    adapter.request_open(TextSurfaceContextTargetAnchor::pointer(
        CONTEXT_X,
        CONTEXT_Y,
        UiTextSelectionRange::caret(0),
        UiRect::new(0, 0, FRAME_WIDTH_PX as u32, FRAME_HEIGHT_PX as u32),
    ));
    let mut output = None;
    let mut frame_output = context.run_ui(frame_input(), |ui| {
        output = Some(adapter.show(ui, &raster_style(), &paint_style()));
    });
    frame_output.textures_delta.clear();
    let output = output.ok_or_else(|| std::io::Error::other("actual egui frame did not run"))??;
    let record = output
        .record
        .ok_or_else(|| std::io::Error::other("visible menu record was absent"))?;
    let artifact = output
        .artifact
        .ok_or_else(|| std::io::Error::other("visible menu artifact was absent"))?;
    let plans = [ArtifactPaintPlanRef::ContextMenu(&artifact.paint_plan)];
    let composite = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            0,
            0,
            FRAME_WIDTH_PX as u32,
            FRAME_HEIGHT_PX as u32,
        )),
        plans: &plans,
    })?;
    Ok((
        record.bounds,
        composite.non_transparent_pixel_count,
        composite.pixel_hash,
    ))
}

fn run_menu_with_items(
    items: Vec<ContextMenuPresentationItem>,
) -> Result<Vec<crate::context_menu::types::ContextMenuPaintOperation>, Box<dyn std::error::Error>>
{
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiContextMenuAdapter::new(
        katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
    )?;
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items,
    });
    adapter.request_open(TextSurfaceContextTargetAnchor::pointer(
        CONTEXT_X,
        CONTEXT_Y,
        UiTextSelectionRange::caret(0),
        UiRect::new(0, 0, FRAME_WIDTH_PX as u32, FRAME_HEIGHT_PX as u32),
    ));
    let mut output = None;
    let mut frame_output = context.run_ui(frame_input(), |ui| {
        output = Some(adapter.show(ui, &raster_style(), &paint_style()));
    });
    frame_output.textures_delta.clear();
    let output = output.ok_or_else(|| std::io::Error::other("actual egui frame did not run"))??;
    let artifact = output
        .artifact
        .ok_or_else(|| std::io::Error::other("visible menu artifact was absent"))?;
    Ok(artifact.paint_plan.operations)
}

fn frame_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(FRAME_WIDTH_PX, FRAME_HEIGHT_PX),
        )),
        ..egui::RawInput::default()
    }
}

fn raster_style() -> ContextMenuRasterStyle {
    ContextMenuRasterStyle {
        font: FontToken {
            name: "context-menu-test".to_string(),
            family: FontFamily::Proportional,
            size: FONT_SIZE_PX,
            weight: FONT_WEIGHT,
        },
        text_color_rgba: TEXT_RGBA,
        icon_color_rgba: TEXT_RGBA,
        line_height_px: LINE_HEIGHT_PX,
    }
}

const fn paint_style() -> ContextMenuPaintStyle {
    ContextMenuPaintStyle {
        background_rgba: MENU_BACKGROUND_RGBA,
        highlighted_rgba: MENU_HIGHLIGHTED_RGBA,
        disabled_rgba: MENU_DISABLED_RGBA,
    }
}
