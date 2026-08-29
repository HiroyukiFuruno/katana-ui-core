#[path = "text_command_surface/assertions.rs"]
mod assertions;
#[path = "text_command_surface/fixtures.rs"]
mod fixtures;
#[path = "text_command_surface/harness.rs"]
mod harness;
#[path = "text_command_surface/scenario.rs"]
mod scenario;
#[path = "text_command_surface/support.rs"]
mod support;

mod text_command_surface_contract {
    use super::scenario;
    use super::{fixtures, harness};
    use katana_ui_core::atom::TextArea;
    use katana_ui_core::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};
    use katana_ui_core_egui_adapter::text_command_surface::{
        EguiTextCommandSurface, EguiTextCommandSurfaceAdapter,
    };

    #[test]
    fn actual_egui_text_command_surface_composes_full_interaction_sequence()
    -> Result<(), Box<dyn std::error::Error>> {
        scenario::actual_egui_text_command_surface_composes_full_interaction_sequence()
    }

    #[test]
    fn actual_egui_text_command_surface_escapes_floating_toolbar_with_raw_input()
    -> Result<(), Box<dyn std::error::Error>> {
        scenario::actual_egui_text_command_surface_escapes_floating_toolbar_with_raw_input()
    }

    #[test]
    fn actual_egui_text_command_surface_activates_last_floating_dropdown_item()
    -> Result<(), Box<dyn std::error::Error>> {
        scenario::actual_egui_text_command_surface_activates_last_floating_dropdown_item()
    }

    #[test]
    fn actual_egui_text_command_surface_owns_context_menu_from_actual_input()
    -> Result<(), Box<dyn std::error::Error>> {
        scenario::actual_egui_text_command_surface_owns_context_menu_from_actual_input()
    }

    #[test]
    fn root_overrides_fixed_provider_viewport_with_allocated_text_rect()
    -> Result<(), Box<dyn std::error::Error>> {
        let context = egui::Context::default();
        let mut adapter = EguiTextCommandSurfaceAdapter::with_text_raster_config(
            katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
        )
        .expect("text command adapter");
        let mut surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture())
            .with_toolbar(fixtures::toolbar_fixture())
            .with_search_strip(fixtures::search_fixture(false));
        let (_, output) = harness::run_frame_sized(
            &context,
            &mut adapter,
            &mut surface,
            &harness::style(),
            egui::vec2(fixtures::FRAME_WIDTH, fixtures::FRAME_HEIGHT),
            Vec::new(),
        )?;
        let toolbar = output.toolbar.as_ref().expect("toolbar output");
        let search = output.search.as_ref().expect("search output");
        let text = output.text.record.frame;

        assert_eq!(text.surface_bounds.width, output.root_bounds.width);
        assert_eq!(text.viewport_bounds.width, text.surface_bounds.width - 40);
        assert_eq!(text.surface_bounds.y, toolbar.record.bounds.height as i32);
        assert_eq!(
            text.surface_bounds.height,
            fixtures::FRAME_HEIGHT as u32
                - toolbar.record.bounds.height
                - search.record.bounds.height
        );
        assert_eq!(text.viewport_bounds.height, text.surface_bounds.height);
        assert_eq!(
            surface.text().props().viewport.width,
            text.viewport_bounds.width
        );
        assert_eq!(
            surface.text().props().viewport.height,
            text.viewport_bounds.height
        );
        Ok(())
    }

    #[test]
    fn root_measurement_preserves_scroll_and_updates_after_resize()
    -> Result<(), Box<dyn std::error::Error>> {
        let context = egui::Context::default();
        let mut adapter = EguiTextCommandSurfaceAdapter::with_text_raster_config(
            katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
        )
        .expect("text command adapter");
        let source = (0..32)
            .map(|index| format!("line {index}: 日本語 ⭐️"))
            .collect::<Vec<_>>()
            .join("\n");
        let text = TextSurface::new(TextSurfaceProps::new(
            TextArea::new("root-scroll-retention").value(&source),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 1, 1).scroll_offset(0, 48),
        ));
        let mut surface = EguiTextCommandSurface::new(text)
            .with_toolbar(fixtures::toolbar_fixture())
            .with_search_strip(fixtures::search_fixture(false));
        let (_, initial) = harness::run_frame_sized(
            &context,
            &mut adapter,
            &mut surface,
            &harness::style(),
            egui::vec2(900.0, 220.0),
            Vec::new(),
        )?;
        let initial_viewport = initial.text.record.frame.viewport_bounds;
        assert_eq!(initial.text.record.frame.viewport.scroll_y, 48);
        let (_, scrolled) = harness::run_frame_sized(
            &context,
            &mut adapter,
            &mut surface,
            &harness::style(),
            egui::vec2(900.0, 220.0),
            Vec::new(),
        )?;
        assert_eq!(surface.text().state().scroll_y, 48);
        assert_eq!(
            scrolled.text.record.frame.viewport.scroll_y,
            surface.text().state().scroll_y
        );
        assert_eq!(
            scrolled.text.record.frame.viewport_bounds.width,
            initial_viewport.width
        );

        let (_, resized) = harness::run_frame_sized(
            &context,
            &mut adapter,
            &mut surface,
            &harness::style(),
            egui::vec2(700.0, 180.0),
            Vec::new(),
        )?;
        let toolbar_height = resized
            .toolbar
            .as_ref()
            .expect("resized toolbar output")
            .record
            .bounds
            .height;
        let search_height = resized
            .search
            .as_ref()
            .expect("resized search output")
            .record
            .bounds
            .height;
        assert_eq!(resized.text.record.frame.surface_bounds.width, 700);
        assert_eq!(
            resized.text.record.frame.surface_bounds.height,
            180 - toolbar_height - search_height
        );
        assert_eq!(
            resized.text.record.frame.viewport.scroll_y,
            surface.text().state().scroll_y
        );
        Ok(())
    }

    #[test]
    fn root_without_chrome_matches_the_full_allocated_rect()
    -> Result<(), Box<dyn std::error::Error>> {
        let context = egui::Context::default();
        let mut adapter = EguiTextCommandSurfaceAdapter::with_text_raster_config(
            katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
        )
        .expect("text command adapter");
        let mut surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture());
        let (_, output) = harness::run_frame_sized(
            &context,
            &mut adapter,
            &mut surface,
            &harness::style(),
            egui::vec2(900.0, 500.0),
            Vec::new(),
        )?;
        assert_eq!(output.text.record.frame.surface_bounds, output.root_bounds);
        assert_eq!(output.text.record.frame.viewport_bounds.width, 900 - 40);
        assert_eq!(output.text.record.frame.viewport_bounds.height, 500);
        Ok(())
    }
}

#[test]
fn actual_egui_text_command_surface_scrolls_context_menu_overflow_from_actual_input()
-> Result<(), Box<dyn std::error::Error>> {
    scenario::actual_egui_text_command_surface_scrolls_context_menu_overflow_from_actual_input()
}
