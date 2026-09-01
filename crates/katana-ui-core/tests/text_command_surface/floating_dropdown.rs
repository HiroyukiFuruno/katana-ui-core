use super::super::{assertions, fixtures, harness};
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter,
    EguiTextCommandSurfaceFloatingPresentation, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceSearchPresentation,
};
use katana_ui_core::molecule::command_chrome::FloatingCommandToolbarVisibility;
use katana_ui_core::text_surface::TextSurfacePresentation;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextCommandSurfaceAdapter::with_text_raster_config(
        katana_ui_core::text_raster::PlatformTextRasterConfig::default(),
    )
    .expect("text command adapter");
    let mut surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture())
        .with_toolbar(fixtures::toolbar_fixture())
        .with_floating_toolbar(
            fixtures::floating_toolbar_fixture(),
            FloatingCommandToolbarVisibility::Closed,
        )
        .with_search_strip(fixtures::search_fixture(false));
    let style = harness::style();

    let (_, initial) =
        harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    let bounds = initial.text.record.frame.content_bounds;
    let text_point = egui::pos2(bounds.x as f32 + 48.0, bounds.y as f32 + 8.0);
    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(text_point, true)],
    )?;
    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(text_point, false)],
    )?;
    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![egui::Event::Text("日本語".into())],
    )?;
    let (_, selected) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::key(egui::Key::ArrowLeft, true)],
    )?;
    let trigger = assertions::floating_dropdown_trigger(&selected)
        .expect("actual floating toolbar must expose dropdown trigger");

    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(trigger, true)],
    )?;
    let (_, dropdown_opened) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(trigger, false)],
    )?;
    assert!(
        dropdown_opened
            .floating
            .as_ref()
            .expect("floating")
            .record
            .as_ref()
            .is_some_and(|record| record.toolbar.dropdown.is_some())
    );

    let before_move = dropdown_opened
        .floating
        .as_ref()
        .expect("floating")
        .record
        .as_ref()
        .expect("floating record");
    let before_anchor = before_move.anchor_bounds;
    let before_panel = before_move.panel_bounds;
    let before_surface_id = before_move.surface_id.clone();
    let mut text = TextSurfacePresentation::from_props(surface.text().props());
    let selection_start = text
        .value
        .find("日本語")
        .expect("fixture must contain Japanese text");
    text.selection_start = selection_start;
    text.selection_end = selection_start.saturating_add("日本".len());
    assert!(
        surface.synchronize_presentation(EguiTextCommandSurfacePresentation {
            text_state_id: None,
            text,
            toolbar: Some(fixtures::toolbar_presentation()),
            floating: Some(EguiTextCommandSurfaceFloatingPresentation {
                toolbar: fixtures::floating_toolbar_presentation(),
                visibility: FloatingCommandToolbarVisibility::Visible,
            }),
            search: Some(EguiTextCommandSurfaceSearchPresentation {
                state_id: fixtures::search_presentation_state_id(),
                label: "検索と置換".to_string(),
                value: fixtures::search_presentation(),
            }),
            context_menu: None,
        })
    );
    let (_, moved) = harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    let moved_record = moved
        .floating
        .as_ref()
        .expect("floating")
        .record
        .as_ref()
        .expect("floating record");
    assert_ne!(before_anchor, moved_record.anchor_bounds);
    assert_ne!(before_panel, moved_record.panel_bounds);
    assert_eq!(before_surface_id, moved_record.surface_id);
    assert!(moved_record.toolbar.dropdown.is_some());

    let (_, dropdown_closed) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }],
    )?;
    assert!(
        dropdown_closed
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_some()
    );
    assert!(
        dropdown_closed
            .floating
            .as_ref()
            .expect("floating")
            .record
            .as_ref()
            .is_some_and(|record| record.toolbar.dropdown.is_none())
    );

    let (_, closed) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }],
    )?;
    assert!(
        closed
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_none()
    );
    assert!(surface.text().state().text_area.focused);

    let _ = closed;
    Ok(())
}

pub(crate) fn last_item_run() -> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextCommandSurfaceAdapter::with_text_raster_config(
        katana_ui_core::text_raster::PlatformTextRasterConfig::default(),
    )
    .expect("text command adapter");
    let mut surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture())
        .with_floating_toolbar(
            fixtures::toolbar_fixture(),
            FloatingCommandToolbarVisibility::Closed,
        );
    let style = harness::style();
    let screen = egui::vec2(fixtures::FRAME_WIDTH, 960.0);

    let (_, initial) = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        Vec::new(),
    )?;
    let bounds = initial.text.record.frame.content_bounds;
    let text_point = egui::pos2(bounds.x as f32 + 48.0, bounds.y as f32 + 8.0);
    let _ = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        vec![harness::button(text_point, true)],
    )?;
    let _ = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        vec![harness::button(text_point, false)],
    )?;
    let _ = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        vec![egui::Event::Text("日本語 ⭐️".into())],
    )?;
    let (_, selected) = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        vec![harness::key(egui::Key::ArrowLeft, true)],
    )?;
    let trigger = selected
        .floating
        .as_ref()
        .and_then(|value| value.record.as_ref())
        .and_then(|record| {
            record
                .toolbar
                .actions
                .iter()
                .find(|action| action.action_id == "code-block")
                .and_then(|action| action.secondary_trigger_bounds)
        })
        .map(center)
        .expect("floating toolbar must expose the generic code dropdown trigger");

    let _ = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        vec![harness::button(trigger, true)],
    )?;
    let (opened_full, opened) = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        vec![harness::button(trigger, false)],
    )?;
    let dropdown = opened
        .floating
        .as_ref()
        .and_then(|value| value.record.as_ref())
        .and_then(|record| record.toolbar.dropdown.as_ref())
        .expect("17-item floating dropdown must render");
    assert_eq!(dropdown.items.len(), 17);
    let last = dropdown.items.last().expect("last visible dropdown item");
    assertions::assert_inside(last.bounds, opened.root_bounds);
    assert!(
        opened_full
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| update
                .nodes
                .iter()
                .any(|(_, node)| node.label() == Some("候補 17 ⭐️")))
    );
    let last_point = center(last.bounds);

    let (_, press) = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        vec![harness::button(last_point, true)],
    )?;
    assert!(
        press
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .and_then(|record| record.toolbar.dropdown.as_ref())
            .is_some()
    );
    assert!(!press.floating.as_ref().is_some_and(|value| value.events.iter().any(|event| {
        matches!(event, katana_ui_core::molecule::command_chrome::FloatingCommandToolbarEvent::Closed { .. })
    })));

    let (_, release) = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        vec![harness::button(last_point, false)],
    )?;
    let activations = release.floating.as_ref().map_or(0, |value| {
        value
            .events
            .iter()
            .filter(|event| matches!(
                event,
                katana_ui_core::molecule::command_chrome::FloatingCommandToolbarEvent::Toolbar {
                    event: katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent::DropdownItemActivated {
                        action_id,
                        item_id,
                    },
                } if action_id.as_str() == "code-block" && item_id.as_str() == "code-17"
            ))
            .count()
    });
    assert_eq!(activations, 1);
    assert_eq!(
        release.artifact_order(),
        vec![
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceChild::Text,
            katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceChild::Floating,
        ]
    );
    let (_, settled) = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        Vec::new(),
    )?;
    let (_, repeat) = harness::run_frame_sized(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        screen,
        Vec::new(),
    )?;
    assert_eq!(
        assertions::composite_hash(&settled)?,
        assertions::composite_hash(&repeat)?
    );
    Ok(())
}

fn center(bounds: katana_ui_core::render_model::UiRect) -> egui::Pos2 {
    egui::pos2(
        bounds.x as f32 + bounds.width as f32 / 2.0,
        bounds.y as f32 + bounds.height as f32 / 2.0,
    )
}
