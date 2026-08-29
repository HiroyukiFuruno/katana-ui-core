use super::text_surface_artifact::{RGBA_CHANNELS, render_artifact_frame};
use super::text_surface_fixture::{
    SURFACE_HEIGHT, SURFACE_WIDTH, paint_style, raster_style, text_surface_fixture,
};
use super::text_surface_script_steps::{EXPECTED_SCRIPT_NAMES, scripted_steps};
use super::text_surface_script_types::{
    ScriptedEguiFrame, TextSurfaceArtifactError, TextSurfaceArtifactStep, TextSurfaceScriptResult,
    has_colored_star_texture,
};
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceClipboardOperation, TextSurfaceEvent, TextSurfaceHistoryOperation,
};
use katana_ui_core_egui_adapter::text_surface::{EguiTextSurfaceAdapter, EguiTextSurfaceError};

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
    let Some(first) = sequence.steps.first() else {
        return Err(TextSurfaceArtifactError::Contract(
            "the scripted sequence produced no frames".to_string(),
        ));
    };
    for step in &sequence.steps {
        verify_stable_frame(first, step)?;
    }
    require_event(sequence, "focus-release", |event| {
        matches!(event, TextSurfaceEvent::FocusChanged(true))
    })?;
    require_event(
        sequence,
        "select-all",
        |event| matches!(event, TextSurfaceEvent::SelectionChanged { selection_start: 0, selection_end } if *selection_end > 0),
    )?;
    verify_scrolled(sequence)?;
    verify_ime(sequence)?;
    require_event(sequence, "copy", |event| {
        matches!(
            event,
            TextSurfaceEvent::ClipboardRequested {
                operation: TextSurfaceClipboardOperation::Copy,
                ..
            }
        )
    })?;
    require_event(sequence, "history-undo", |event| {
        matches!(
            event,
            TextSurfaceEvent::HistoryRequested(TextSurfaceHistoryOperation::Undo)
        )
    })?;
    require_event(sequence, "context-target", |event| {
        matches!(event, TextSurfaceEvent::ContextTargetRequested { .. })
    })
}

fn run_scripted_frame(
    context: &egui::Context,
    adapter: &mut EguiTextSurfaceAdapter,
    surface: &mut TextSurface,
    events: Vec<egui::Event>,
) -> Result<ScriptedEguiFrame, EguiTextSurfaceError> {
    let mut output = None;
    let _ = context.run_ui(
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
    sequence
        .steps
        .iter()
        .find(|step| step.name == name)
        .ok_or_else(|| TextSurfaceArtifactError::Contract(format!("missing {name} scripted frame")))
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
