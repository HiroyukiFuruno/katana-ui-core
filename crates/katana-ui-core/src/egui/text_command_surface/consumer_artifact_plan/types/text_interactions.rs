use super::execution::{interaction_error, show_frame};
use super::support::raw_input;
use super::{ConsumerArtifactPlanError, EguiTextCommandSurfaceHostRoot, GenericInteractionClass};

pub(super) fn apply_text_input(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    frame: crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    class: GenericInteractionClass,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    let focused = complete_text_focus(root, context, frame)?;
    let before = focused.record().record_hash().to_owned();
    let applied = show_frame(root, context, raw_input(class))?;
    ensure_record_changed(&before, applied.record().record_hash()).map(|()| applied)
}

pub(super) fn apply_scroll(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    frame: crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    let focused = complete_text_focus(root, context, frame)?;
    let before = focused.record().record_hash().to_owned();
    let scrolled = show_frame(root, context, raw_input(GenericInteractionClass::Scroll))?;
    ensure_record_changed(&before, scrolled.record().record_hash()).map(|()| scrolled)
}

pub(super) fn ensure_record_changed(
    before: &str,
    after: &str,
) -> Result<(), ConsumerArtifactPlanError> {
    if before == after {
        return Err(interaction_error(
            "text interaction did not change the retained text target",
        ));
    }
    Ok(())
}

pub(super) fn complete_search_trace(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    frame: crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    let mut continuation = frame
        .interaction_locator()
        .begin_search_trace()
        .map_err(interaction_error)?;
    loop {
        let mut input = raw_input(GenericInteractionClass::Search);
        continuation
            .apply_to_raw_input_once(&mut input)
            .map_err(interaction_error)?;
        let next = show_frame(root, context, input)?;
        match continuation
            .advance(next.interaction_locator())
            .map_err(interaction_error)?
        {
            Some(next_continuation) => continuation = next_continuation,
            None => return Ok(next),
        }
    }
}

pub(super) fn complete_selection(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    frame: crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    let continuation = frame
        .interaction_locator()
        .begin_text_selection()
        .map_err(interaction_error)?;
    advance_text_interaction(root, context, continuation)
}

fn complete_text_focus(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    frame: crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    let continuation = frame
        .interaction_locator()
        .begin_text_focus()
        .map_err(interaction_error)?;
    advance_text_interaction(root, context, continuation)
}

fn advance_text_interaction(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    mut continuation: crate::egui::text_command_surface::KucOpaqueTextSelectionContinuation,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    loop {
        let mut input = raw_input(GenericInteractionClass::Selection);
        continuation
            .apply_to_raw_input_once(&mut input)
            .map_err(interaction_error)?;
        let next = show_frame(root, context, input)?;
        match continuation
            .advance(next.interaction_locator())
            .map_err(interaction_error)?
        {
            Some(next_continuation) => continuation = next_continuation,
            None => return Ok(next),
        }
    }
}
