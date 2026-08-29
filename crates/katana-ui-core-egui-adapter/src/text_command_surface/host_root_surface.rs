use super::super::types::{EguiTextCommandSurface, EguiTextCommandSurfacePresentation};
use super::EguiTextCommandSurfaceCommandFamilyProjection;
use katana_ui_core::atom::TextArea;
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceGutter, TextSurfaceProps, TextSurfaceViewport,
};

pub(super) fn surface_from_presentation(
    identity: &str,
    presentation: &EguiTextCommandSurfacePresentation,
    command_families: &EguiTextCommandSurfaceCommandFamilyProjection,
) -> EguiTextCommandSurface {
    let text = text_surface_from_presentation(identity, presentation);
    let mut surface = EguiTextCommandSurface::new(text);
    let _ = surface.synchronize_presentation(presentation.clone());
    apply_command_families(&mut surface, command_families);
    surface
}

fn apply_command_families(
    surface: &mut EguiTextCommandSurface,
    command_families: &EguiTextCommandSurfaceCommandFamilyProjection,
) {
    if let Some(toolbar) = surface.toolbar.take() {
        surface.toolbar =
            Some(toolbar.command_family(command_families.primary().cloned().unwrap_or_default()));
    }
    if let Some(toolbar) = surface.deferred_floating_toolbar.take() {
        surface.deferred_floating_toolbar =
            Some(toolbar.command_family(command_families.floating().cloned().unwrap_or_default()));
    }
    if let Some(floating) = surface.floating.take() {
        surface.floating =
            Some(floating.command_family(command_families.floating().cloned().unwrap_or_default()));
    }
    surface.synchronize_command_families(
        command_families.primary().cloned(),
        command_families.floating().cloned(),
    );
}

fn text_surface_from_presentation(
    identity: &str,
    presentation: &EguiTextCommandSurfacePresentation,
) -> TextSurface {
    let text_presentation = &presentation.text;
    let state_id = presentation
        .text_state_id
        .clone()
        .unwrap_or_else(|| UiStateId::new(format!("{identity}/text")));
    let label = if text_presentation.accessibility_label.is_empty() {
        format!("{identity}/text")
    } else {
        text_presentation.accessibility_label.clone()
    };
    let text_area = TextArea::new(label)
        .stable_state_id(state_id)
        .value(text_presentation.value.clone())
        .readonly(text_presentation.readonly)
        .disabled(text_presentation.disabled)
        .ime_enabled(text_presentation.ime_enabled);
    let mut props = TextSurfaceProps::new(
        text_area,
        text_presentation.spans.clone(),
        TextSurfaceViewport::new(0, 0, 1, 1),
    )
    .adapter_measured_viewport();
    props.annotations = text_presentation.annotations.clone();
    props.gutter = text_presentation
        .automatic_gutter
        .as_ref()
        .map(|_| TextSurfaceGutter::new(0).automatic_numbered());
    props.accessibility_label = text_presentation.accessibility_label.clone();
    props.accessibility_actions = text_presentation.accessibility_actions.clone();
    props.context_target_label = text_presentation.context_target_label.clone();
    props.disabled_reason = text_presentation.disabled_reason.clone();
    props.scroll_request = text_presentation.scroll_request.clone();
    props.focus_request = text_presentation.focus_request.clone();
    let mut surface = TextSurface::new(props);
    let _ = surface.synchronize_presentation(text_presentation.clone());
    surface
}
