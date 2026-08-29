use super::super::{assertions, facts, harness};
use crate::visual::{command_chrome_fixture, text_surface_fixture};
use katana_ui_core::text_surface::TextSurfaceEvent;

const FRAME_BOUNDARY_OFFSET: f32 = 8.0;

fn inside_text_content_point(text_bounds: katana_ui_core::render_model::UiRect) -> egui::Pos2 {
    egui::pos2(
        text_bounds.x as f32 + (text_bounds.width / 2).max(1) as f32,
        text_bounds.y as f32 + (text_bounds.height / 2).max(1) as f32,
    )
}

pub(crate) struct RepeatableChildrenScenario;

impl RepeatableChildrenScenario {
    pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
        let first = harness::Harness::run_frame_for_fact(Vec::new())?;
        let second = harness::Harness::run_frame_for_fact(Vec::new())?;
        assert_eq!(first, second);
        assert_eq!(
            first.root,
            katana_ui_core::render_model::UiRect::new(
                0,
                0,
                command_chrome_fixture::FRAME_WIDTH as u32,
                command_chrome_fixture::FRAME_HEIGHT as u32,
            )
        );
        assertions::Assertions::assert_inside("text", first.text, first.root);
        assertions::Assertions::assert_inside("toolbar", first.toolbar, first.root);
        assertions::Assertions::assert_inside("search", first.search, first.root);
        assert_eq!(
            first.artifact_plan_count,
            facts::FrameFacts::expected_plan_count(false)
        );
        assert!(
            first
                .labels
                .iter()
                .any(|label| label == "TextSurface story")
        );
        assert!(first.labels.iter().any(|label| label.contains('⭐')));

        let context = egui::Context::default();
        context.enable_accesskit();
        let mut adapter =
            katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceAdapter::with_text_raster_config(
                katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
            )?;
        let mut surface =
            katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurface::new(
                text_surface_fixture::text_surface_fixture(),
            )
            .with_toolbar(super::super::harness::Harness::primary_toolbar_fixture())
            .with_floating_toolbar(
                super::super::harness::Harness::floating_toolbar_fixture(),
                katana_ui_core::molecule::command_chrome::FloatingCommandToolbarVisibility::Closed,
            )
            .with_search_strip(command_chrome_fixture::search_fixture(false));
        let style = harness::Harness::style()?;
        let (_initial_full, initial_output) =
            harness::Harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
        let text_point = inside_text_content_point(initial_output.text.record.frame.content_bounds);

        let (_press_full, _press_output) = harness::Harness::run_frame(
            &context,
            &mut adapter,
            &mut surface,
            &style,
            vec![harness::Harness::button(text_point, true)],
        )?;
        let (_focus_full, focus_output) = harness::Harness::run_frame(
            &context,
            &mut adapter,
            &mut surface,
            &style,
            vec![harness::Harness::button(text_point, false)],
        )?;
        let _ = focus_output;

        let _ = harness::Harness::run_frame(
            &context,
            &mut adapter,
            &mut surface,
            &style,
            vec![egui::Event::Text("storybook storybook".into())],
        )?;
        let (open_full, open_output) = harness::Harness::run_frame(
            &context,
            &mut adapter,
            &mut surface,
            &style,
            vec![harness::Harness::key(egui::Key::ArrowRight, true)],
        )?;
        assert!(
            open_output
                .text
                .events
                .iter()
                .any(|event| matches!(event, TextSurfaceEvent::SelectionChanged { .. })),
            "shift+arrow selection should emit selection change"
        );
        assert_ne!(
            open_output.text.record.frame.selection.range.anchor,
            open_output.text.record.frame.selection.range.focus
        );
        let open_floating = open_output
            .floating
            .as_ref()
            .ok_or_else(|| std::io::Error::other("floating output was absent"))?;
        assert!(open_floating.record.is_some());

        assertions::Assertions::assert_accesskit(
            &open_full,
            open_output.root_bounds,
            &[
                "TextSurface story",
                "太字",
                "検索",
                "置換",
                "選択ツール",
                "選択コード",
            ],
            &[],
        )?;

        let floating_record = open_floating
            .record
            .as_ref()
            .ok_or_else(|| std::io::Error::other("floating record was absent"))?;
        assertions::Assertions::assert_inside(
            "floating",
            floating_record.toolbar.bounds,
            open_output.root_bounds,
        );
        assertions::Assertions::assert_inside(
            "floating panel",
            floating_record.panel_bounds,
            open_output.root_bounds,
        );
        let open_plans = open_output.artifact_paint_plans()?;
        assert_eq!(
            open_plans.len(),
            facts::FrameFacts::expected_plan_count(true)
        );
        assertions::Assertions::assert_artifact_output_contract(&open_output)?;

        let opened = facts::FrameFacts::collect(&open_full, &open_output)?;
        assert_eq!(
            opened.artifact_plan_count,
            facts::FrameFacts::expected_plan_count(true)
        );
        assert_eq!(
            opened.artifact_order,
            facts::FrameFacts::expected_artifact_order(&open_output)
        );
        assert_eq!(opened.artifact_order, open_output.artifact_order());
        assert!(
            opened
                .floating
                .is_some_and(|bounds| bounds == floating_record.toolbar.bounds)
        );
        assert!(
            opened
                .floating_panel
                .is_some_and(|bounds| bounds == floating_record.panel_bounds)
        );

        let open_repeat =
            harness::Harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
        assert_eq!(
            facts::FrameFacts::composite_hash(&open_output)?,
            facts::FrameFacts::composite_hash(&open_repeat.1)?,
        );

        let outside = egui::pos2(
            command_chrome_fixture::FRAME_WIDTH - FRAME_BOUNDARY_OFFSET,
            command_chrome_fixture::FRAME_HEIGHT - FRAME_BOUNDARY_OFFSET,
        );
        let (_outside_press_full, _outside_press_output) = harness::Harness::run_frame(
            &context,
            &mut adapter,
            &mut surface,
            &style,
            vec![harness::Harness::button(outside, true)],
        )?;
        let (outside_full, outside_closed) = harness::Harness::run_frame(
            &context,
            &mut adapter,
            &mut surface,
            &style,
            vec![harness::Harness::button(outside, false)],
        )?;
        let outside_floating = outside_closed
            .floating
            .as_ref()
            .ok_or_else(|| std::io::Error::other("floating output was absent"))?;
        assert!(outside_floating.record.is_none());
        assert_eq!(
            outside_closed.artifact_order(),
            facts::FrameFacts::expected_artifact_order(&outside_closed)
        );
        assertions::Assertions::assert_accesskit(
            &outside_full,
            outside_closed.root_bounds,
            &["TextSurface story", "太字", "検索", "置換"],
            &["floating"],
        )?;

        let hash_once = facts::FrameFacts::composite_hash(&outside_closed)?;
        let (_, repeat_output) =
            harness::Harness::run_frame(&context, &mut adapter, &mut surface, &style, Vec::new())?;
        assert_eq!(
            hash_once,
            facts::FrameFacts::composite_hash(&repeat_output)?
        );

        Ok(())
    }
}
