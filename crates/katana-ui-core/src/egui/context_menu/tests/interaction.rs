use super::*;

fn run_menu_with_items(
    items: Vec<ContextMenuPresentationItem>,
) -> Result<
    Vec<crate::egui::context_menu::types::ContextMenuPaintOperation>,
    Box<dyn std::error::Error>,
> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter =
        EguiContextMenuAdapter::new(crate::text_raster::PlatformTextRasterConfig::default())?;
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
    Ok(output
        .ok_or_else(|| std::io::Error::other("actual egui frame did not run"))??
        .artifact
        .ok_or_else(|| std::io::Error::other("visible menu artifact was absent"))?
        .paint_plan
        .operations)
}

#[test]
fn context_menu_local_anchor_returns_node_id_without_translation() {
    let viewport = UiRect::new(1, 2, 100, 200);
    let anchor = TextSurfaceContextTargetAnchor {
        anchor: crate::render_model::UiContextMenuAnchor::NodeId("menu-target".to_owned()),
        selection: UiTextSelectionRange::caret(0),
        viewport_bounds: viewport,
    };
    assert_eq!(
        crate::render_model::UiContextMenuAnchor::NodeId("menu-target".to_owned()),
        super::super::surface::local_anchor(&anchor, viewport)
    );
}

#[test]
fn context_menu_local_anchor_translates_virtual_rect_into_viewport_space() {
    let viewport = UiRect::new(10, 20, 100, 200);
    let anchor = TextSurfaceContextTargetAnchor {
        anchor: crate::render_model::UiContextMenuAnchor::VirtualRect(
            crate::render_model::UiContextMenuRect::new(35, 55, 8, 12),
        ),
        selection: UiTextSelectionRange::caret(0),
        viewport_bounds: viewport,
    };
    assert_eq!(
        crate::render_model::UiContextMenuAnchor::VirtualRect(
            crate::render_model::UiContextMenuRect::new(25, 35, 8, 12)
        ),
        super::super::surface::local_anchor(&anchor, viewport)
    );
}

#[test]
fn context_menu_keyboard_actions_follows_navigation_and_submenu_control_paths() {
    let context = egui::Context::default();
    let items = vec![
        ContextMenuPresentationItem::action("first", "first"),
        ContextMenuPresentationItem::action("second", "second"),
    ];
    let mut submenu_path = vec![0];
    let mut type_ahead = ContextMenuTypeAheadBuffer::new(1000);
    assert!(
        !collect_keyboard_actions(
            &context,
            vec![pressed_key_event(egui::Key::ArrowDown)],
            &items,
            &mut submenu_path,
            &[0],
            &mut type_ahead
        )
        .is_empty()
    );
    for (key, index) in [
        (egui::Key::ArrowUp, 1),
        (egui::Key::Home, 0),
        (egui::Key::End, 1),
    ] {
        assert!(
            matches!(collect_keyboard_actions(&context, vec![pressed_key_event(key)], &items, &mut submenu_path, &[0], &mut type_ahead).first(), Some(ContextMenuAction::Highlight { path }) if path.last() == Some(&index))
        );
    }
    assert!(
        matches!(collect_keyboard_actions(&context, vec![pressed_key_event(egui::Key::Enter)], &items, &mut submenu_path, &[0], &mut type_ahead).first(), Some(ContextMenuAction::Activate { path }) if path == &vec![0])
    );
    assert!(
        matches!(collect_keyboard_actions(&context, vec![pressed_key_event(egui::Key::Space)], &items, &mut submenu_path, &[0], &mut type_ahead).first(), Some(ContextMenuAction::Activate { path }) if path == &vec![0])
    );
    assert!(
        collect_keyboard_actions(
            &context,
            vec![pressed_key_event(egui::Key::Enter)],
            &items,
            &mut submenu_path,
            &[],
            &mut type_ahead
        )
        .is_empty()
    );
    assert!(
        matches!(collect_keyboard_actions(&context, vec![pressed_key_event(egui::Key::ArrowRight)], &items, &mut submenu_path, &[0], &mut type_ahead).first(), Some(ContextMenuAction::OpenSubmenu { path }) if path == &vec![0])
    );
    submenu_path = vec![0, 1];
    assert!(
        collect_keyboard_actions(
            &context,
            vec![pressed_key_event(egui::Key::ArrowLeft)],
            &items,
            &mut submenu_path,
            &[0],
            &mut type_ahead
        )
        .is_empty()
    );
    assert_eq!(vec![0], submenu_path);
    assert!(matches!(
        collect_keyboard_actions(
            &context,
            vec![pressed_key_event(egui::Key::Escape)],
            &items,
            &mut Vec::new(),
            &[0],
            &mut type_ahead
        )
        .first(),
        Some(ContextMenuAction::Close {
            reason: crate::molecule::selection::ContextMenuCloseReason::Escape
        })
    ));
    assert_eq!(
        collect_keyboard_actions(
            &context,
            vec![egui::Event::Text("sec".to_owned())],
            &items,
            &mut Vec::new(),
            &[],
            &mut ContextMenuTypeAheadBuffer::new(1000)
        ),
        vec![ContextMenuAction::TypeAhead {
            prefix: "sec".to_owned()
        }]
    );
    assert!(
        collect_keyboard_actions(
            &context,
            vec![pressed_key_event(egui::Key::Tab)],
            &items,
            &mut Vec::new(),
            &[],
            &mut ContextMenuTypeAheadBuffer::new(1000)
        )
        .is_empty()
    );
}

#[test]
fn context_menu_reveal_keyboard_highlight_covers_scrolling_boundaries() {
    let Some(mut adapter) = require_ok(
        EguiContextMenuAdapter::new(crate::text_raster::PlatformTextRasterConfig::default()),
        "context menu adapter should be created",
    ) else {
        return;
    };
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
        id: "icon".to_owned(),
        label: "icon".to_owned(),
        accessibility_label: "icon".to_owned(),
        icon: Some(crate::render_model::UiIconProps::new("<svg/>")),
        enabled: true,
        checked: false,
        kind: ContextMenuItemKind::Action,
        children: Vec::new(),
    }])?;
    assert!(operations.iter().any(|operation| matches!(&operation.kind, crate::egui::context_menu::types::ContextMenuPaintOperationKind::Texture { texture, .. } if texture.identity.starts_with("context-menu-icon:"))));
    Ok(())
}
