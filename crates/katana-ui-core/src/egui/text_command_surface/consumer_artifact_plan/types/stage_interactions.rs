use super::execution::{interaction_error, show_frame};
use super::support::raw_input;
use super::text_interactions::{
    apply_scroll, apply_text_input, complete_search_trace, complete_selection,
};
use super::{ConsumerArtifactPlanError, EguiTextCommandSurfaceHostRoot, GenericInteractionClass};
use crate::egui::text_command_surface::{KucInteractionActionClass, KucInteractionSelector};

const RESIZED_VIEWPORT_DIMENSIONS: (u32, u32) = (900, 520);

pub(super) fn render_stage(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    class: GenericInteractionClass,
    action_target: &str,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    let initial_class = match class {
        GenericInteractionClass::TextInput
        | GenericInteractionClass::ImeCommit
        | GenericInteractionClass::Scroll
        | GenericInteractionClass::Search
        | GenericInteractionClass::ViewportResize => GenericInteractionClass::Selection,
        _ => class,
    };
    let frame = show_frame(root, context, raw_input(initial_class))?;
    match class {
        GenericInteractionClass::TextInput | GenericInteractionClass::ImeCommit => {
            apply_text_input(root, context, frame, class)
        }
        GenericInteractionClass::Selection => complete_selection(root, context, frame),
        GenericInteractionClass::Scroll => apply_scroll(root, context, frame),
        GenericInteractionClass::ToolbarActivation => apply_bound_action(
            root,
            context,
            frame,
            action_target,
            KucInteractionActionClass::Toolbar,
        ),
        GenericInteractionClass::FloatingToolbar => {
            let selected = complete_selection(root, context, frame)?;
            apply_bound_action(
                root,
                context,
                selected,
                action_target,
                KucInteractionActionClass::FloatingToolbar,
            )
        }
        GenericInteractionClass::ContextMenu => apply_context_menu(root, context, frame, class),
        GenericInteractionClass::Search => complete_search_trace(root, context, frame),
        GenericInteractionClass::AccessibilityActivation => {
            apply_accesskit_activation(root, context, frame, action_target)
        }
        GenericInteractionClass::ViewportResize => apply_viewport_resize(root, context, frame),
    }
}

fn apply_context_menu(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    frame: crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    class: GenericInteractionClass,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    let mut request = frame
        .interaction_locator()
        .request_context_open()
        .map_err(interaction_error)?;
    let mut input = raw_input(class);
    request
        .apply_to_raw_input_once(&mut input)
        .map_err(interaction_error)?;
    let applied = show_frame(root, context, input)?;
    ensure_context_menu_opened(
        applied.contains_context_menu_opened(),
        applied.context_menu_is_visible(),
    )
    .map(|()| applied)
}

fn apply_accesskit_activation(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    frame: crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    action_target: &str,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    let mut request = frame
        .interaction_locator()
        .request_accesskit_activation(action_target, KucInteractionActionClass::Toolbar)
        .map_err(interaction_error)?;
    let mut input = raw_input(GenericInteractionClass::AccessibilityActivation);
    request
        .apply_to_raw_input_once(&mut input)
        .map_err(interaction_error)?;
    let applied = show_frame(root, context, input)?;
    ensure_bound_action_event(applied.contains_command_activation(action_target, false))
        .map(|()| applied)
}

fn apply_viewport_resize(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    frame: crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    let before = frame.record().dimensions();
    show_frame(
        root,
        context,
        raw_input(GenericInteractionClass::ViewportResize),
    )
    .and_then(|resized| {
        let after = resized.record().dimensions();
        ensure_viewport_resize(
            (before.width(), before.height()),
            (after.width(), after.height()),
        )
        .map(|()| resized)
    })
}

fn apply_bound_action(
    root: &mut EguiTextCommandSurfaceHostRoot,
    context: &egui::Context,
    frame: crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    action_target: &str,
    action_class: KucInteractionActionClass,
) -> Result<
    crate::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    ConsumerArtifactPlanError,
> {
    let mut request = frame
        .interaction_locator()
        .request(KucInteractionSelector::new(action_target, action_class))
        .map_err(interaction_error)?;
    let mut input = raw_input(GenericInteractionClass::ToolbarActivation);
    request
        .apply_to_raw_input_once(&mut input)
        .map_err(interaction_error)?;
    let applied = show_frame(root, context, input)?;
    ensure_bound_action_event(applied.contains_command_activation(
        action_target,
        action_class == KucInteractionActionClass::FloatingToolbar,
    ))
    .map(|()| applied)
}

fn ensure_bound_action_event(observed: bool) -> Result<(), ConsumerArtifactPlanError> {
    if !observed {
        return Err(interaction_error(
            "bound action did not emit an activation event",
        ));
    }
    Ok(())
}

fn ensure_context_menu_opened(
    observed_event: bool,
    visible: bool,
) -> Result<(), ConsumerArtifactPlanError> {
    if !observed_event {
        return Err(interaction_error(
            "context menu did not emit an opened event",
        ));
    }
    if !visible {
        return Err(interaction_error("context menu was not visible"));
    }
    Ok(())
}

fn ensure_viewport_resize(
    before: (u32, u32),
    after: (u32, u32),
) -> Result<(), ConsumerArtifactPlanError> {
    if before == after {
        return Err(interaction_error(
            "viewport resize did not change root dimensions",
        ));
    }
    if after != RESIZED_VIEWPORT_DIMENSIONS {
        return Err(interaction_error(
            "viewport resize did not produce the requested root dimensions",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound_action_requires_its_activation_event() {
        assert_eq!(ensure_bound_action_event(true), Ok(()));
        assert_eq!(
            ensure_bound_action_event(false),
            Err(ConsumerArtifactPlanError::Artifact(
                "KUC interaction protocol failed: bound action did not emit an activation event"
                    .to_owned(),
            ))
        );
    }

    #[test]
    fn context_menu_requires_open_event_and_visible_state() {
        assert_eq!(ensure_context_menu_opened(true, true), Ok(()));
        assert!(matches!(
            ensure_context_menu_opened(false, true),
            Err(ConsumerArtifactPlanError::Artifact(message)) if message == "KUC interaction protocol failed: context menu did not emit an opened event"
        ));
        assert!(matches!(
            ensure_context_menu_opened(true, false),
            Err(ConsumerArtifactPlanError::Artifact(message)) if message == "KUC interaction protocol failed: context menu was not visible"
        ));
    }

    #[test]
    fn viewport_resize_requires_changed_requested_dimensions() {
        assert_eq!(
            ensure_viewport_resize((1280, 720), RESIZED_VIEWPORT_DIMENSIONS),
            Ok(())
        );
        assert!(matches!(
            ensure_viewport_resize(RESIZED_VIEWPORT_DIMENSIONS, RESIZED_VIEWPORT_DIMENSIONS),
            Err(ConsumerArtifactPlanError::Artifact(message)) if message == "KUC interaction protocol failed: viewport resize did not change root dimensions"
        ));
        assert!(matches!(
            ensure_viewport_resize((1280, 720), (901, 520)),
            Err(ConsumerArtifactPlanError::Artifact(message)) if message == "KUC interaction protocol failed: viewport resize did not produce the requested root dimensions"
        ));
    }
}
