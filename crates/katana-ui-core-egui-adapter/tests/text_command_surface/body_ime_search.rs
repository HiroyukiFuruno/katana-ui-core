use super::super::{assertions, fixtures, harness, support};
use katana_ui_core::molecule::command_chrome::FloatingCommandToolbarVisibility;
use katana_ui_core::text_surface::TextSurfacePresentation;
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter,
    EguiTextCommandSurfaceFloatingPresentation, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceSearchPresentation,
};

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiTextCommandSurfaceAdapter::with_text_raster_config(
        katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
    )
    .expect("text command adapter");
    let mut surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture())
        .with_toolbar(fixtures::toolbar_fixture())
        .with_floating_toolbar(
            fixtures::floating_toolbar_fixture(),
            FloatingCommandToolbarVisibility::Closed,
        )
        .with_search_strip(fixtures::search_fixture(false));
    let initial_text = surface.text().state().text_area.value.clone();
    let style = harness::style();

    assert!(fixtures::script_line_height() > 0.0);

    let (initial_full, initial) =
        harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    assert_eq!(
        initial.artifact_order(),
        assertions::expected_artifact_order(&initial)
    );
    assertions::assert_artifact_output_contract(&initial)?;
    assert_eq!(
        initial.root_bounds,
        katana_ui_core::render_model::UiRect::new(
            0,
            0,
            fixtures::FRAME_WIDTH as u32,
            fixtures::FRAME_HEIGHT as u32
        )
    );
    assert!(
        initial
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_none()
    );
    assert!(
        initial
            .floating
            .as_ref()
            .and_then(|value| value.artifact.as_ref())
            .is_none()
    );
    assertions::assert_inside(
        initial.text.record.frame.surface_bounds,
        initial.root_bounds,
    );
    assertions::assert_inside(
        initial.toolbar.as_ref().expect("toolbar").record.bounds,
        initial.root_bounds,
    );
    assertions::assert_inside(
        initial.search.as_ref().expect("search").record.bounds,
        initial.root_bounds,
    );
    assertions::assert_accesskit(
        &initial_full,
        initial.root_bounds,
        &["TextSurface story", "太字", "検索", "置換"],
        &[],
    );

    let text_point = egui::pos2(
        initial.text.record.frame.content_bounds.x as f32 + 48.0,
        initial.text.record.frame.content_bounds.y as f32 + 8.0,
    );
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
    assert!(surface.text().state().text_area.focused);

    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![egui::Event::Text("日本語 ⭐️".into())],
    )?;

    let before_preedit = surface.text().state().text_area.value.clone();
    let (_, preedit) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな".into(),
            active_range_chars: None,
        })],
    )?;
    assert!(preedit.text.record.frame.preedit.is_some());
    assert_eq!(surface.text().state().text_area.value, before_preedit);

    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".into()))],
    )?;
    assert!(surface.text().state().text_area.value.contains("⭐️"));

    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::key(egui::Key::ArrowRight, true)],
    )?;
    let (selected_full, selected) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::key(egui::Key::ArrowRight, true)],
    )?;

    assert!(
        selected
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_some(),
        "selection must open floating toolbar"
    );
    assert!(
        selected
            .floating
            .as_ref()
            .and_then(|value| value.artifact.as_ref())
            .is_some()
    );
    let selected_floating = selected
        .floating
        .as_ref()
        .expect("selection must create a floating output");
    let selected_floating_record = selected_floating
        .record
        .as_ref()
        .expect("selection must create a floating record");
    let selected_anchor = selected_floating_record.anchor_bounds;
    let selected_panel = selected_floating_record.panel_bounds;
    let selected_floating_id = selected_floating_record.surface_id.clone();
    let selected_surface_id = selected.text.record.hit_target.clone();
    let selected_toolbar_action_id = selected
        .toolbar
        .as_ref()
        .expect("root toolbar")
        .record
        .actions[0]
        .action_id
        .clone();
    let selected_search_id = selected
        .search
        .as_ref()
        .expect("search strip")
        .record
        .query
        .hit_target
        .clone();
    assertions::assert_floating_within_root(&selected, selected.root_bounds);
    assertions::assert_artifact_output_contract(&selected)?;
    assertions::assert_accesskit(
        &selected_full,
        selected.root_bounds,
        &[
            "TextSurface story",
            "太字",
            "検索",
            "置換",
            "選択コード",
            "選択ツール",
        ],
        &[],
    );

    let mut presentation = EguiTextCommandSurfacePresentation {
        text_state_id: Some(katana_ui_core::render_model::UiStateId::new(
            selected_surface_id.clone(),
        )),
        text: TextSurfacePresentation::from_props(surface.text().props()),
        toolbar: Some(fixtures::toolbar_presentation()),
        floating: Some(EguiTextCommandSurfaceFloatingPresentation {
            toolbar: fixtures::floating_toolbar_presentation(),
            visibility: FloatingCommandToolbarVisibility::Closed,
        }),
        search: Some(EguiTextCommandSurfaceSearchPresentation {
            state_id: fixtures::search_presentation_state_id(),
            label: "検索と置換".to_string(),
            value: fixtures::search_presentation(),
        }),
        context_menu: None,
    };
    assert!(surface.synchronize_presentation(presentation.clone()));
    assert_eq!(
        surface.toolbar().expect("retained root toolbar").actions()[0].label_model(),
        "同期太字"
    );
    assert_eq!(
        surface
            .search_strip()
            .expect("retained search strip")
            .query_model(),
        "同期検索 ⭐️"
    );
    let (_, policy_closed) =
        harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    assert!(
        policy_closed
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_none(),
        "controlled closed visibility must suppress the retained floating toolbar"
    );
    presentation
        .floating
        .as_mut()
        .expect("floating presentation")
        .visibility = FloatingCommandToolbarVisibility::Visible;
    assert!(surface.synchronize_presentation(presentation));
    let (_, policy_visible) =
        harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    assert!(
        policy_visible
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_some(),
        "controlled visible policy must reuse the retained floating toolbar"
    );

    let (moved_full, moved) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::key(egui::Key::ArrowRight, true)],
    )?;
    let moved_floating = moved
        .floating
        .as_ref()
        .expect("moved selection must retain floating output");
    let moved_floating_record = moved_floating
        .record
        .as_ref()
        .expect("moved selection must retain floating record");
    assert_ne!(selected_anchor, moved_floating_record.anchor_bounds);
    assert_ne!(selected_panel, moved_floating_record.panel_bounds);
    assert_eq!(selected_floating_id, moved_floating_record.surface_id);
    assert_eq!(selected_surface_id, moved.text.record.hit_target);
    assert_eq!(
        selected_toolbar_action_id,
        moved
            .toolbar
            .as_ref()
            .expect("retained root toolbar")
            .record
            .actions[0]
            .action_id
    );
    assert_eq!(
        selected_search_id,
        moved
            .search
            .as_ref()
            .expect("retained search strip")
            .record
            .query
            .hit_target
    );
    assert_eq!(
        moved_floating_record.anchor_bounds,
        moved_floating
            .artifact
            .as_ref()
            .expect("floating artifact")
            .record
            .anchor_bounds
    );
    assertions::assert_floating_within_root(&moved, moved.root_bounds);
    assertions::assert_accesskit(
        &moved_full,
        moved.root_bounds,
        &["同期太字", "検索", "置換", "選択コード"],
        &[],
    );
    let selected_trigger = assertions::floating_dropdown_trigger(&moved)
        .expect("moved floating toolbar should expose a dropdown trigger");

    let _ = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(selected_trigger, true)],
    )?;
    let (dropdown_full, dropdown_opened) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(selected_trigger, false)],
    )?;
    let floating_dropdown = dropdown_opened
        .floating
        .as_ref()
        .expect("floating")
        .record
        .as_ref()
        .and_then(|record| record.toolbar.dropdown.as_ref())
        .expect("actual floating toolbar should expose a dropdown");
    assertions::assert_inside(
        floating_dropdown.trigger_bounds,
        dropdown_opened.root_bounds,
    );
    assertions::assert_inside(floating_dropdown.bounds, dropdown_opened.root_bounds);
    assertions::assert_inside(
        dropdown_opened
            .floating
            .as_ref()
            .expect("floating")
            .record
            .as_ref()
            .expect("actual floating toolbar should expose panel bounds")
            .panel_bounds,
        dropdown_opened.root_bounds,
    );
    assertions::assert_artifact_output_contract(&dropdown_opened)?;
    assert!(
        dropdown_opened
            .floating
            .as_ref()
            .and_then(|value| value.artifact.as_ref())
            .is_some()
    );
    assert_eq!(
        dropdown_opened.artifact_order(),
        assertions::expected_artifact_order(&dropdown_opened)
    );
    let (_, dropdown_repeat) =
        harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    assert_eq!(
        assertions::composite_hash(&dropdown_opened)?,
        assertions::composite_hash(&dropdown_repeat)?
    );
    assertions::assert_accesskit(
        &dropdown_full,
        dropdown_opened.root_bounds,
        &[
            "TextSurface story",
            "太字",
            "検索",
            "置換",
            "選択ツール",
            "選択コード",
        ],
        &[],
    );

    let outside = assertions::overlay_outside_point(&dropdown_opened);
    let (outside_press_full, outside_press_output) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(outside, true)],
    )?;
    let _ = outside_press_full;
    assert!(
        outside_press_output
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_none()
    );
    assert!(
        outside_press_output
            .floating
            .as_ref()
            .and_then(|value| value.artifact.as_ref())
            .is_none()
    );
    assert!(outside_press_output.floating.as_ref().map_or_else(Vec::new, |value| value.events.clone()).iter().any(|event| {
        matches!(
            event,
            katana_ui_core::molecule::command_chrome::FloatingCommandToolbarEvent::Closed { .. }
        )
    }));
    let (outside_full, outside_closed) = harness::run_frame(
        &context,
        &mut adapter,
        &mut surface,
        &style,
        vec![harness::button(outside, false)],
    )?;
    assert!(
        outside_closed
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .is_none()
    );
    assert!(
        outside_closed
            .floating
            .as_ref()
            .and_then(|value| value.artifact.as_ref())
            .is_none()
    );
    assert_eq!(outside_closed.artifact_order(), initial.artifact_order());
    assert_eq!(
        outside_closed.artifact_order(),
        assertions::expected_artifact_order(&outside_closed)
    );
    assertions::assert_accesskit(
        &outside_full,
        outside_closed.root_bounds,
        &["TextSurface story", "太字", "検索", "置換"],
        &[support::FORBIDDEN_LABEL],
    );
    assert!(surface.text().state().text_area.focused);

    let hash_once = assertions::composite_hash(&outside_closed)?;
    let (_, repeat_output) =
        harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
    assert_eq!(hash_once, assertions::composite_hash(&repeat_output)?);

    assert_ne!(initial_text, surface.text().state().text_area.value);
    Ok(())
}
