use super::{
    UiIconProps, UiInteractionState, UiLoadingProps, UiNodeId, UiNodeKind, UiProps, UiSize,
    UiStateId, UiStatusProps, UiTextEntryProps, UiTone, UiVariant, UiVisualRole,
};
use crate::theme::ThemeSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiNode {
    id: UiNodeId,
    kind: UiNodeKind,
    props: UiProps,
    children: Vec<UiNode>,
}

impl UiNode {
    #[must_use]
    pub fn new(kind: UiNodeKind, label: impl Into<String>) -> Self {
        Self::from_state(kind, label, UiStateId::next_for(kind))
    }

    pub(crate) fn from_state(
        kind: UiNodeKind,
        label: impl Into<String>,
        state_id: UiStateId,
    ) -> Self {
        Self {
            id: UiNodeId::next_for(kind),
            kind,
            props: UiProps::new(label, state_id),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn disabled(mut self, value: bool) -> Self {
        self.props.disabled = value;
        self
    }

    #[must_use]
    pub fn focusable(mut self, value: bool) -> Self {
        self.props.focusable = value;
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.props.accessibility_label = value.into();
        self
    }

    #[must_use]
    pub fn interaction(mut self, value: UiInteractionState) -> Self {
        self.props.interaction = value;
        self
    }

    #[must_use]
    pub fn theme(mut self, value: &ThemeSnapshot) -> Self {
        self.props.theme_id = value.id.as_str().to_string();
        self
    }

    #[must_use]
    pub fn font_role(mut self, value: impl Into<String>) -> Self {
        self.props.font_role = value.into();
        self
    }

    #[must_use]
    pub fn visual_role(mut self, value: UiVisualRole) -> Self {
        self.props.visual_role = value;
        self
    }

    #[must_use]
    pub fn variant(mut self, value: UiVariant) -> Self {
        self.props.variant = value;
        self
    }

    #[must_use]
    pub fn tone(mut self, value: UiTone) -> Self {
        self.props.tone = value;
        self
    }

    #[must_use]
    pub fn size(mut self, value: UiSize) -> Self {
        self.props.size = value;
        self
    }

    #[must_use]
    pub fn loading(mut self, value: bool) -> Self {
        self.props.loading = value;
        self
    }

    #[must_use]
    pub fn readonly(mut self, value: bool) -> Self {
        self.props.readonly = value;
        self
    }

    #[must_use]
    pub fn invalid(mut self, value: bool) -> Self {
        self.props.invalid = value;
        self
    }

    #[must_use]
    pub fn placeholder(mut self, value: impl Into<String>) -> Self {
        self.props.placeholder = value.into();
        self
    }

    #[must_use]
    pub fn checked(mut self, value: bool) -> Self {
        self.props.checked = value;
        self
    }

    #[must_use]
    pub fn progress(mut self, determinate: bool, percent: u8) -> Self {
        self.props.determinate = determinate;
        self.props.progress_percent = percent;
        self
    }

    #[must_use]
    pub fn severity(mut self, value: UiTone) -> Self {
        self.props.severity = value;
        self
    }

    #[must_use]
    pub fn text_entry(mut self, value: UiTextEntryProps) -> Self {
        self.props.text_entry = value;
        self
    }

    #[must_use]
    pub fn status(mut self, value: UiStatusProps) -> Self {
        self.props.status = value;
        self
    }

    #[must_use]
    pub fn loading_indicator(mut self, value: UiLoadingProps) -> Self {
        self.props.loading_indicator = value;
        self
    }

    #[must_use]
    pub fn icon(mut self, value: UiIconProps) -> Self {
        self.props.icon = value;
        self
    }

    #[must_use]
    pub fn style_class(mut self, value: impl Into<String>) -> Self {
        self.props.style_classes.push(value.into());
        self
    }

    #[must_use]
    pub fn style_classes(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.props.style_classes.extend(values);
        self
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn kind(&self) -> UiNodeKind {
        self.kind
    }

    #[must_use]
    pub fn children(&self) -> &[UiNode] {
        &self.children
    }

    #[must_use]
    pub fn id(&self) -> &UiNodeId {
        &self.id
    }

    #[must_use]
    pub fn props(&self) -> &UiProps {
        &self.props
    }
}
