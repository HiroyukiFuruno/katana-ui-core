use crate::render_model::{
    UiCommonProps, UiImageSurfaceFit, UiImageSurfaceHighlight, UiImageSurfaceProps,
    UiImageSurfaceTransform, UiImageSurfaceValidationError, UiNode, UiNodeKind, UiStateId,
    UiVisualRole,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageSurface {
    label: String,
    common: UiCommonProps,
    props: UiImageSurfaceProps,
    state_id: UiStateId,
}

impl ImageSurface {
    #[must_use]
    pub fn new(label: impl Into<String>, props: UiImageSurfaceProps) -> Self {
        Self {
            label: label.into(),
            common: UiCommonProps::default(),
            props,
            state_id: UiStateId::next_for(UiNodeKind::ImageSurface),
        }
    }

    pub fn from_rgba(
        label: impl Into<String>,
        fingerprint: impl Into<String>,
        width: u32,
        height: u32,
        rgba: Vec<u8>,
    ) -> Result<Self, UiImageSurfaceValidationError> {
        Ok(Self::new(
            label,
            UiImageSurfaceProps::new(fingerprint, width, height, rgba)?,
        ))
    }

    #[must_use]
    pub fn content_scale(mut self, value: u32) -> Self {
        self.props = self.props.content_scale(value);
        self
    }

    #[must_use]
    pub fn display_size(mut self, width: u32, height: u32) -> Self {
        self.props = self.props.display_size(width, height);
        self
    }

    #[must_use]
    pub fn display_size_exact(mut self, width: f32, height: f32) -> Self {
        self.props = self.props.display_size_exact(width, height);
        self
    }

    #[must_use]
    pub fn fit(mut self, value: UiImageSurfaceFit) -> Self {
        self.props = self.props.fit(value);
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.props = self.props.accessibility_label(value);
        self
    }

    #[must_use]
    pub fn selection_text(mut self, value: impl Into<String>) -> Self {
        self.props = self.props.selection_text(value);
        self.common = self.common.selectable(true);
        self
    }

    #[must_use]
    pub fn highlight_rect(mut self, value: UiImageSurfaceHighlight) -> Self {
        self.props = self.props.highlight_rect(value);
        self
    }

    #[must_use]
    pub fn transform(mut self, value: UiImageSurfaceTransform) -> Self {
        self.props = self.props.transform(value);
        self
    }

    #[must_use]
    pub fn common(mut self, value: UiCommonProps) -> Self {
        self.common = value;
        self
    }
}

impl From<ImageSurface> for UiNode {
    fn from(value: ImageSurface) -> Self {
        UiNode::from_state(UiNodeKind::ImageSurface, value.label, value.state_id)
            .common(value.common)
            .accessibility_label(value.props.accessibility_label.clone())
            .visual_role(UiVisualRole::Content)
            .image_surface(value.props)
    }
}
