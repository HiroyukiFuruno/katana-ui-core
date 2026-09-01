use super::text_surface_artifact::{RGBA_CHANNELS, render_artifact_frame};
use super::text_surface_fixture::{
    SURFACE_HEIGHT, SURFACE_WIDTH, paint_style, raster_style, text_surface_fixture,
};
use super::text_surface_script_steps::{EXPECTED_SCRIPT_NAMES, scripted_steps};
use super::text_surface_script_types::{
    ScriptedEguiFrame, TextSurfaceArtifactError, TextSurfaceArtifactStep, TextSurfaceScriptResult,
    has_colored_star_texture,
};
use katana_ui_core::egui::text_surface::{EguiTextSurfaceAdapter, EguiTextSurfaceError};
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceClipboardOperation, TextSurfaceEvent, TextSurfaceHistoryOperation,
};

pub(super) fn run_scripted_sequence() -> Result<TextSurfaceScriptResult, TextSurfaceArtifactError> {
    let context = egui::Context::default();
    let mut adapter = EguiTextSurfaceAdapter::default();
    let mut surface = text_surface_fixture();
    let mut steps = Vec::new();
    for (index, step) in scripted_steps().into_iter().enumerate() {
        let frame = run_scripted_frame(&context, &mut adapter, &mut surface, step.events)?;
        let pixels = render_artifact_frame(&frame.output.artifact, actual_root_canvas())
            .map_err(TextSurfaceArtifactError::Contract)?;
        steps.push(TextSurfaceArtifactStep {
            index,
            name: step.name,
            artifact: frame.output.artifact,
            pixels,
            events: frame.output.events,
            surface_focused: surface.state().text_area.focused,
            raw_events: frame.raw_events,
        });
    }
    Ok(TextSurfaceScriptResult { steps })
}

fn actual_root_canvas() -> UiRect {
    UiRect::new(0, 0, SURFACE_WIDTH as u32, SURFACE_HEIGHT as u32)
}

pub(super) fn assert_sequence_contract(
    sequence: &TextSurfaceScriptResult,
) -> Result<(), TextSurfaceArtifactError> {
    let Some(first) = sequence.steps.first() else {
        return Err(TextSurfaceArtifactError::Contract(
            "the scripted sequence produced no frames".to_string(),
        ));
    };
    let names = sequence
        .steps
        .iter()
        .map(|step| step.name)
        .collect::<Vec<_>>();
    if names != EXPECTED_SCRIPT_NAMES {
        return Err(TextSurfaceArtifactError::Contract(
            "the scripted actual-egui steps changed".to_string(),
        ));
    }
    for step in &sequence.steps {
        verify_stable_frame(first, step)?;
    }
    require_event(sequence, "focus-release", is_focus_release_event)?;
    require_event(sequence, "select-all", is_select_all_event)?;
    verify_scrolled(sequence)?;
    verify_ime(sequence)?;
    require_event(sequence, "copy", is_copy_event)?;
    require_event(sequence, "history-undo", is_undo_event)?;
    require_event(sequence, "context-target", is_context_target_event)
}

fn is_focus_release_event(event: &TextSurfaceEvent) -> bool {
    *event == TextSurfaceEvent::FocusChanged(true)
}

fn is_select_all_event(event: &TextSurfaceEvent) -> bool {
    matches!(event, TextSurfaceEvent::SelectionChanged { selection_start: 0, selection_end } if *selection_end > 0)
}

fn is_copy_event(event: &TextSurfaceEvent) -> bool {
    matches!(
        event,
        TextSurfaceEvent::ClipboardRequested {
            operation: TextSurfaceClipboardOperation::Copy,
            ..
        }
    )
}

fn is_undo_event(event: &TextSurfaceEvent) -> bool {
    *event == TextSurfaceEvent::HistoryRequested(TextSurfaceHistoryOperation::Undo)
}

fn is_context_target_event(event: &TextSurfaceEvent) -> bool {
    matches!(event, TextSurfaceEvent::ContextTargetRequested { .. })
}

fn run_scripted_frame(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
) -> Result<ScriptedEguiFrame, EguiTextSurfaceError> {
    let mut output = None;
    let mut full_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SURFACE_WIDTH, SURFACE_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            let raw_events = ui.input(|input| input.events.clone());
            output = Some(
                adapter
                    .show(ui, surface, &raster_style(), &paint_style())
                    .map(|output| ScriptedEguiFrame { output, raw_events }),
            );
        },
    );
    full_output.textures_delta.clear();
    output.ok_or(EguiTextSurfaceError::FrameNotProduced)?
}

fn verify_stable_frame(
    first: &TextSurfaceArtifactStep,
    step: &TextSurfaceArtifactStep,
) -> Result<(), TextSurfaceArtifactError> {
    let frame = &step.artifact.record.frame;
    if frame.surface_bounds != first.artifact.record.frame.surface_bounds
        || frame.viewport_bounds != first.artifact.record.frame.viewport_bounds
    {
        return Err(TextSurfaceArtifactError::Contract(format!(
            "{} changed stable surface or viewport bounds",
            step.name
        )));
    }
    let pixel_length = usize::try_from(step.pixels.width)
        .unwrap_or_default()
        .saturating_mul(usize::try_from(step.pixels.height).unwrap_or_default())
        .saturating_mul(RGBA_CHANNELS);
    if step.artifact.paint_plan_hash != step.pixels.paint_plan_hash
        || step.pixels.rgba.len() != pixel_length
    {
        return Err(TextSurfaceArtifactError::Contract(format!(
            "{} does not preserve the adapter paint plan pixels",
            step.name
        )));
    }
    Ok(())
}

fn verify_scrolled(sequence: &TextSurfaceScriptResult) -> Result<(), TextSurfaceArtifactError> {
    let wheel = step(sequence, "wheel-scroll")?;
    if wheel.artifact.record.frame.viewport.scroll_y <= 0
        || !wheel
            .events
            .iter()
            .any(|event| matches!(event, TextSurfaceEvent::Scrolled { .. }))
    {
        return Err(TextSurfaceArtifactError::Contract(
            "wheel input did not produce a scrolled actual-egui frame".to_string(),
        ));
    }
    let ime_caret = step(sequence, "ime-caret-release")?;
    if ime_caret.artifact.record.frame.selection_start
        != ime_caret.artifact.record.frame.selection_end
    {
        return Err(TextSurfaceArtifactError::Contract(
            "the actual pointer did not collapse the selection before IME".to_string(),
        ));
    }
    Ok(())
}

fn verify_ime(sequence: &TextSurfaceScriptResult) -> Result<(), TextSurfaceArtifactError> {
    let preedit = step(sequence, "ime-preedit-star")?;
    if preedit
        .artifact
        .record
        .frame
        .preedit
        .as_ref()
        .map(|value| value.text.as_str())
        != Some("⭐️")
    {
        return Err(TextSurfaceArtifactError::Contract(
            "IME preedit did not retain the star variation selector".to_string(),
        ));
    }
    let committed = step(sequence, "ime-commit-star")?;
    if !committed.artifact.record.raster_identity.contains("二行目")
        || !committed.artifact.record.raster_identity.contains("⭐️")
        || !has_colored_star_texture(&committed.artifact)
    {
        return Err(TextSurfaceArtifactError::Contract(
            "IME commit did not reach the platform-raster paint plan".to_string(),
        ));
    }
    Ok(())
}

fn step<'a>(
    sequence: &'a TextSurfaceScriptResult,
    name: &str,
) -> Result<&'a TextSurfaceArtifactStep, TextSurfaceArtifactError> {
    match sequence.steps.iter().find(|step| step.name == name) {
        Some(step) => Ok(step),
        None => Err(TextSurfaceArtifactError::Contract(format!(
            "missing {name} scripted frame"
        ))),
    }
}

fn require_event(
    sequence: &TextSurfaceScriptResult,
    name: &str,
    predicate: impl Fn(&TextSurfaceEvent) -> bool,
) -> Result<(), TextSurfaceArtifactError> {
    let artifact_step = step(sequence, name)?;
    if artifact_step.events.iter().any(predicate) {
        Ok(())
    } else {
        Err(TextSurfaceArtifactError::Contract(format!(
            "{name} did not produce its typed event: focused={} raw={:?} typed={:?}",
            artifact_step.surface_focused, artifact_step.raw_events, artifact_step.events
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::render_model::UiRect;

    #[test]
    fn event_predicates_reject_unrelated_events() {
        let event = TextSurfaceEvent::FocusChanged(false);
        assert!(!is_focus_release_event(&event));
        assert!(!is_select_all_event(&event));
        assert!(!is_copy_event(&event));
        assert!(!is_undo_event(&event));
        assert!(!is_context_target_event(&event));
    }

    #[test]
    fn assert_sequence_contract_rejects_reordered_steps() -> Result<(), TextSurfaceArtifactError> {
        let mut sequence = run_scripted_sequence()?;
        sequence.steps[0].name = "wrong-first";
        let error = assert_sequence_contract(&sequence);
        assert!(matches!(
            error,
            Err(TextSurfaceArtifactError::Contract(error))
                if error.contains("the scripted actual-egui steps changed")
        ));
        assert_eq!("wrong-first", sequence.steps[0].name);
        Ok(())
    }

    #[test]
    fn assert_sequence_contract_rejects_missing_focus_release_event()
    -> Result<(), TextSurfaceArtifactError> {
        let mut sequence = run_scripted_sequence()?;
        assert_eq!(
            sequence
                .steps
                .iter()
                .filter(|step| step.name == "focus-release")
                .count(),
            1
        );
        sequence
            .steps
            .iter_mut()
            .filter(|step| step.name == "focus-release")
            .for_each(|step| step.events.clear());
        let result = assert_sequence_contract(&sequence);
        assert!(matches!(
            result,
            Err(TextSurfaceArtifactError::Contract(error))
                if error.contains("focus-release did not produce its typed event")
        ));
        Ok(())
    }

    #[test]
    fn assert_sequence_contract_rejects_missing_ime_preedit_evidence()
    -> Result<(), TextSurfaceArtifactError> {
        let mut sequence = run_scripted_sequence()?;
        assert_eq!(
            sequence
                .steps
                .iter()
                .filter(|step| step.name == "ime-preedit-star")
                .count(),
            1
        );
        sequence
            .steps
            .iter_mut()
            .filter(|step| step.name == "ime-preedit-star")
            .for_each(|step| step.artifact.record.frame.preedit = None);
        let result = assert_sequence_contract(&sequence);
        assert!(matches!(
            result,
            Err(TextSurfaceArtifactError::Contract(error))
                if error.contains("IME preedit did not retain the star variation selector")
        ));
        Ok(())
    }

    #[test]
    fn assert_sequence_contract_rejects_unstable_viewport_bounds()
    -> Result<(), TextSurfaceArtifactError> {
        let mut sequence = run_scripted_sequence()?;
        let first_surface_bounds = sequence.steps[0].artifact.record.frame.surface_bounds;
        sequence.steps[0].artifact.record.frame.surface_bounds = UiRect::new(
            first_surface_bounds.x + 1,
            first_surface_bounds.y,
            first_surface_bounds.width,
            first_surface_bounds.height,
        );
        let result = assert_sequence_contract(&sequence);
        assert!(matches!(
            result,
            Err(TextSurfaceArtifactError::Contract(error))
                if error.contains("changed stable surface or viewport bounds")
        ));
        Ok(())
    }

    #[test]
    fn assert_sequence_contract_verifies_expected_raw_input_traces()
    -> Result<(), TextSurfaceArtifactError> {
        let sequence = run_scripted_sequence()?;
        assert_sequence_contract(&sequence)?;

        let copy_step = step(&sequence, "copy")?;
        assert!(
            copy_step
                .raw_events
                .iter()
                .any(|event| matches!(event, egui::Event::Copy))
        );
        assert!(copy_step.surface_focused);

        let undo_step = step(&sequence, "history-undo")?;
        assert!(undo_step.raw_events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Key {
                    key: egui::Key::Z,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers,
                } if modifiers.command
            )
        }));

        let ime_preedit = step(&sequence, "ime-preedit-star")?;
        assert!(ime_preedit.raw_events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Ime(egui::ImeEvent::Preedit { text: value, .. })
                    if value == "⭐️"
            )
        }));

        let ime_commit = step(&sequence, "ime-commit-star")?;
        assert!(ime_commit.raw_events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Ime(egui::ImeEvent::Commit(value))
                    if value == "⭐️"
            )
        }));

        let context_target = step(&sequence, "context-target")?;
        assert!(context_target.raw_events.iter().any(|event| matches!(
            event,
            egui::Event::PointerButton {
                button: egui::PointerButton::Secondary,
                ..
            }
        )));

        assert!(
            sequence
                .steps
                .iter()
                .all(|step| !step.pixels.rgba.is_empty()),
            "each step must render non-empty paint plan raster"
        );
        Ok(())
    }

    #[test]
    fn assert_sequence_contract_rejects_each_remaining_invalid_frame_fact()
    -> Result<(), TextSurfaceArtifactError> {
        let base = run_scripted_sequence()?;

        assert!(matches!(
            step(&base, "missing"),
            Err(TextSurfaceArtifactError::Contract(error)) if error.contains("missing missing")
        ));

        let empty = TextSurfaceScriptResult { steps: Vec::new() };
        assert!(matches!(
            assert_sequence_contract(&empty),
            Err(TextSurfaceArtifactError::Contract(error))
                if error.contains("produced no frames")
        ));

        let mut invalid_pixels = base.clone();
        invalid_pixels.steps[0].pixels.rgba.clear();
        assert!(matches!(
            assert_sequence_contract(&invalid_pixels),
            Err(TextSurfaceArtifactError::Contract(error))
                if error.contains("does not preserve the adapter paint plan pixels")
        ));

        let mut invalid_scroll = base.clone();
        invalid_scroll
            .steps
            .iter_mut()
            .filter(|step| step.name == "wheel-scroll")
            .for_each(|step| step.artifact.record.frame.viewport.scroll_y = 0);
        assert!(matches!(
            assert_sequence_contract(&invalid_scroll),
            Err(TextSurfaceArtifactError::Contract(error))
                if error.contains("did not produce a scrolled actual-egui frame")
        ));

        let mut invalid_caret = base.clone();
        invalid_caret
            .steps
            .iter_mut()
            .filter(|step| step.name == "ime-caret-release")
            .for_each(|step| {
                step.artifact.record.frame.selection_end =
                    step.artifact.record.frame.selection_start.saturating_add(1);
            });
        assert!(matches!(
            assert_sequence_contract(&invalid_caret),
            Err(TextSurfaceArtifactError::Contract(error))
                if error.contains("did not collapse the selection")
        ));

        let mut invalid_commit = base;
        invalid_commit
            .steps
            .iter_mut()
            .filter(|step| step.name == "ime-commit-star")
            .for_each(|step| step.artifact.record.raster_identity.clear());
        assert!(matches!(
            assert_sequence_contract(&invalid_commit),
            Err(TextSurfaceArtifactError::Contract(error))
                if error.contains("IME commit did not reach")
        ));
        Ok(())
    }
}
