use super::super::{assertions, fixtures, harness};
use katana_ui_core::molecule::selection::{ContextMenuEvent, ContextMenuItemKind};
use katana_ui_core::text_surface::TextSurfacePresentation;
use katana_ui_core_egui_adapter::context_menu::{
    ContextMenuPaintOperationKind, ContextMenuPresentation, ContextMenuPresentationItem,
};
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurface, EguiTextCommandSurfaceChild, EguiTextCommandSurfaceOutput,
    EguiTextCommandSurfacePresentation,
};

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = harness::adapter()?;
    let mut surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture());
    let presentation = controlled_presentation(&surface, menu_presentation(true));
    assert!(surface.synchronize_presentation(presentation.clone()));
    assert_eq!(
        surface.context_menu_presentation(),
        presentation.context_menu.as_ref()
    );

    let mut hidden = presentation.clone();
    hidden
        .context_menu
        .as_mut()
        .expect("controlled context menu")
        .visible = false;
    assert!(surface.synchronize_presentation(hidden));
    let (_, hidden_frame) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &harness::style(),
        Vec::new(),
    )?;
    assert_menu_closed(&hidden_frame)?;
    assert!(surface.synchronize_presentation(presentation));

    let style = harness::style();
    let (initial_full, initial) =
        harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    assert_menu_closed(&initial)?;
    let text_point = egui::pos2(
        initial.text.record.frame.content_bounds.x as f32 + 16.0,
        initial.text.record.frame.content_bounds.y as f32 + 12.0,
    );
    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(text_point, true)],
    )?;
    let (_, focused) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(text_point, false)],
    )?;
    assert_focus_restored(&surface, &focused);

    let right_click = egui::pos2(
        initial.text.record.frame.content_bounds.x as f32 + 48.0,
        initial.text.record.frame.content_bounds.y as f32 + 8.0,
    );
    assert_menu_closed(&focused)?;
    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::secondary_button(right_click, true)],
    )?;
    let (pointer_full, pointer_open) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::secondary_button(right_click, false)],
    )?;
    assert_context_menu_opened(&pointer_open)?;
    let pointer_menu = context_menu_record(&pointer_open);
    assert_eq!(
        pointer_menu.viewport_bounds,
        pointer_open.text.record.frame.viewport_bounds
    );
    assert_context_artifact(&pointer_open)?;
    assertions::assert_accesskit(&pointer_full, pointer_open.root_bounds, &["整形 ⭐️"], &[]);

    let disabled = pointer_menu
        .items
        .last()
        .expect("disabled context menu item");
    assert!(disabled.disabled);
    let disabled_point = center(disabled.bounds);
    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(disabled_point, true)],
    )?;
    let (_, disabled_release) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(disabled_point, false)],
    )?;
    assert!(context_menu_output(&disabled_release).events.is_empty());

    let text_before_typeahead = surface.text().state().text_area.value.clone();
    let (_, typeahead) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![egui::Event::Text("整".to_string())],
    )?;
    assert!(context_menu_output(&typeahead).events.iter().any(|event| {
        matches!(event, ContextMenuEvent::TypeAheadMatched { prefix, .. } if prefix == "整")
    }));
    assert_eq!(
        surface.text().state().text_area.value,
        text_before_typeahead
    );
    let _ = harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::key(egui::Key::ArrowRight, false)],
    )?;
    let (_, nested) = harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    assert_eq!(context_menu_record(&nested).items[0].id, "nested");

    let (_, escaped) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::key(egui::Key::Escape, false)],
    )?;
    assert_context_menu_closed(&escaped)?;
    let (_, escape_restored) =
        harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    assert_focus_restored(&surface, &escape_restored);

    assert_menu_closed(&escape_restored)?;
    let (_, keyboard_open) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::key(egui::Key::F10, true)],
    )?;
    assert_context_menu_opened(&keyboard_open)?;
    let outside = outside_point(&keyboard_open);
    let (_, outside_closed) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(outside, true)],
    )?;
    assert_context_menu_closed(&outside_closed)?;
    let (_, outside_restored) =
        harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    assert_focus_restored(&surface, &outside_restored);

    assert_menu_closed(&outside_restored)?;
    let root_id = text_surface_accesskit_id(&initial_full)?;
    let (_, accesskit_open) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::ShowContextMenu,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: root_id,
                data: None,
            },
        )],
    )?;
    assert_context_menu_opened(&accesskit_open)?;
    let (_, repeat) = harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    assert_eq!(
        assertions::composite_hash(&accesskit_open)?,
        assertions::composite_hash(&repeat)?
    );
    assert_root_clamps_measured_menu()?;
    Ok(())
}

fn assert_root_clamps_measured_menu() -> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    let mut adapter = harness::adapter()?;
    let mut surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture());
    let presentation = controlled_presentation(&surface, menu_presentation(true));
    assert!(surface.synchronize_presentation(presentation));
    let style = harness::style();
    let screen = egui::vec2(960.0, 220.0);
    let (_, initial) = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        Vec::new(),
    )?;
    assert_menu_closed(&initial)?;
    let pointer = egui::pos2(
        initial
            .text
            .record
            .frame
            .content_bounds
            .x
            .saturating_add_unsigned(initial.text.record.frame.content_bounds.width)
            .saturating_sub(2) as f32,
        initial.text.record.frame.content_bounds.y as f32 + 8.0,
    );
    let _ = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        vec![harness::secondary_button(pointer, true)],
    )?;
    let (_, opened) = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        vec![harness::secondary_button(pointer, false)],
    )?;
    assert_context_menu_opened(&opened)?;
    let menu = context_menu_record(&opened);
    assert!(menu.bounds.width <= menu.viewport_bounds.width);
    assert!(menu.bounds.height <= menu.viewport_bounds.height);
    Ok(())
}

fn controlled_presentation(
    surface: &EguiTextCommandSurface,
    context_menu: ContextMenuPresentation,
) -> EguiTextCommandSurfacePresentation {
    EguiTextCommandSurfacePresentation {
        text_state_id: None,
        text: TextSurfacePresentation::from_props(surface.text().props()),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: Some(context_menu),
    }
}

fn menu_presentation(visible: bool) -> ContextMenuPresentation {
    ContextMenuPresentation {
        visible,
        items: vec![
            ContextMenuPresentationItem {
                kind: ContextMenuItemKind::Submenu,
                ..ContextMenuPresentationItem::action("format", "整形 ⭐️")
                    .child(ContextMenuPresentationItem::action("nested", "入れ子 ⭐️"))
            },
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
    }
}

fn assert_menu_closed(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(context_menu_output(output).record.is_none());
    assert!(context_menu_output(output).artifact.is_none());
    assert!(
        !output
            .artifact_order()
            .contains(&EguiTextCommandSurfaceChild::ContextMenu)
    );
    let plans = output.artifact_paint_plans()?;
    assert_eq!(plans.len(), output.artifact_order().len());
    Ok(())
}

fn assert_context_menu_opened(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<(), Box<dyn std::error::Error>> {
    let menu = context_menu_record(output);
    assertions::assert_inside(menu.bounds, output.root_bounds);
    assert_eq!(
        output.artifact_order().last(),
        Some(&EguiTextCommandSurfaceChild::ContextMenu)
    );
    assertions::assert_artifact_output_contract(output)?;
    Ok(())
}

fn assert_context_menu_closed(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(
        context_menu_output(output)
            .events
            .iter()
            .any(|event| { matches!(event, ContextMenuEvent::Closed { .. }) })
    );
    assert_menu_closed(output)?;
    Ok(())
}

fn assert_context_artifact(
    output: &EguiTextCommandSurfaceOutput,
) -> Result<(), Box<dyn std::error::Error>> {
    let artifact = context_menu_output(output)
        .artifact
        .as_ref()
        .expect("context-menu artifact");
    assert!(artifact.paint_plan.operations.iter().any(|operation| {
        matches!(
            operation.kind,
            ContextMenuPaintOperationKind::Texture { ref texture, .. }
                if !texture.rgba_pixels.is_empty()
        )
    }));
    let plans = output.artifact_paint_plans()?;
    assert!(matches!(
        plans.last(),
        Some(
            katana_ui_core_egui_adapter::artifact_compositor::ArtifactPaintPlanRef::ContextMenu(_)
        )
    ));
    Ok(())
}

fn assert_focus_restored(surface: &EguiTextCommandSurface, output: &EguiTextCommandSurfaceOutput) {
    assert!(surface.text().state().text_area.focused);
    assert!(output.text.record.frame.accessibility.root.focused);
}

fn context_menu_output(
    output: &EguiTextCommandSurfaceOutput,
) -> &katana_ui_core_egui_adapter::context_menu::EguiContextMenuOutput {
    output
        .context_menu
        .as_ref()
        .expect("root context-menu output")
}

fn context_menu_record(
    output: &EguiTextCommandSurfaceOutput,
) -> &katana_ui_core_egui_adapter::context_menu::EguiContextMenuFrameRecord {
    context_menu_output(output)
        .record
        .as_ref()
        .expect("root-owned context-menu record")
}

fn outside_point(output: &EguiTextCommandSurfaceOutput) -> egui::Pos2 {
    let root = output.root_bounds;
    let menu = context_menu_record(output).bounds;
    for point in [
        egui::pos2(root.x as f32 + 2.0, root.y as f32 + 2.0),
        egui::pos2(
            root.x.saturating_add_unsigned(root.width).saturating_sub(2) as f32,
            root.y as f32 + 2.0,
        ),
        egui::pos2(
            root.x as f32 + 2.0,
            root.y
                .saturating_add_unsigned(root.height)
                .saturating_sub(2) as f32,
        ),
        egui::pos2(
            root.x.saturating_add_unsigned(root.width).saturating_sub(2) as f32,
            root.y
                .saturating_add_unsigned(root.height)
                .saturating_sub(2) as f32,
        ),
    ] {
        if !contains(menu, point) {
            return point;
        }
    }
    panic!("root did not provide an outside point for the context menu");
}

fn contains(bounds: katana_ui_core::render_model::UiRect, point: egui::Pos2) -> bool {
    point.x >= bounds.x as f32
        && point.x < bounds.x.saturating_add_unsigned(bounds.width) as f32
        && point.y >= bounds.y as f32
        && point.y < bounds.y.saturating_add_unsigned(bounds.height) as f32
}

fn center(bounds: katana_ui_core::render_model::UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}

fn text_surface_accesskit_id(
    full: &egui::FullOutput,
) -> Result<egui::accesskit::NodeId, Box<dyn std::error::Error>> {
    full.platform_output
        .accesskit_update
        .as_ref()
        .into_iter()
        .flat_map(|update| update.nodes.iter())
        .find_map(|(id, node)| {
            (node.role() == egui::accesskit::Role::MultilineTextInput).then_some(*id)
        })
        .ok_or_else(|| std::io::Error::other("TextSurface AccessKit root was absent").into())
}
