use crate::render_model::{
    UiAnimationState, UiLoadingProps, UiNodeKind, UiProgressMode, UiVariant, UiVisualRole,
};

pub(super) fn loading_props(kind: UiNodeKind) -> UiLoadingProps {
    let animation_state = match kind {
        UiNodeKind::LoadingDots | UiNodeKind::Spinner => UiAnimationState::Running,
        _ => UiAnimationState::Idle,
    };
    UiLoadingProps {
        mode: UiProgressMode::Indeterminate,
        label: String::new(),
        animation_state,
        ..UiLoadingProps::default()
    }
}

pub(super) fn visual_role(kind: UiNodeKind) -> UiVisualRole {
    match kind {
        UiNodeKind::Icon => UiVisualRole::Icon,
        UiNodeKind::Input => UiVisualRole::Input,
        UiNodeKind::Badge => UiVisualRole::Status,
        UiNodeKind::Divider => UiVisualRole::Separator,
        UiNodeKind::KeyCap => UiVisualRole::Shortcut,
        UiNodeKind::LoadingDots | UiNodeKind::Spinner => UiVisualRole::Loading,
        UiNodeKind::ProgressBar => UiVisualRole::Progress,
        UiNodeKind::Button
        | UiNodeKind::Checkbox
        | UiNodeKind::Radio
        | UiNodeKind::ColorSwatch
        | UiNodeKind::Toggle
        | UiNodeKind::SlideControl
        | UiNodeKind::SvgButton
        | UiNodeKind::TextButton
        | UiNodeKind::IconTextButton => UiVisualRole::Control,
        _ => UiVisualRole::Content,
    }
}

pub(super) fn variant(kind: UiNodeKind) -> UiVariant {
    match kind {
        UiNodeKind::Button => UiVariant::Filled,
        UiNodeKind::Icon | UiNodeKind::SvgButton => UiVariant::Icon,
        UiNodeKind::TextButton => UiVariant::Text,
        UiNodeKind::IconTextButton => UiVariant::IconText,
        _ => UiVariant::Plain,
    }
}
