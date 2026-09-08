use super::execution::{interaction_error, show_frame};
use super::support::raw_input;
use super::text_interactions::{
    apply_scroll, apply_text_input, complete_search_trace, complete_selection,
};
use super::{ConsumerArtifactPlanError, EguiTextCommandSurfaceHostRoot, GenericInteractionClass};
use crate::egui::text_command_surface::{KucInteractionActionClass, KucInteractionSelector};

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
        | GenericInteractionClass::Search => GenericInteractionClass::Selection,
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
        _ => Ok(frame),
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
    show_frame(root, context, input)
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
    show_frame(root, context, input)
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
    show_frame(root, context, input)
}
