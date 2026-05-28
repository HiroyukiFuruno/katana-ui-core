use super::{UiNode, UiNodeKind, UiRect, UiTree};
use serde::{Deserialize, Serialize};
use std::fmt;

const RGBA_BYTES_PER_PIXEL: usize = 4;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiImageSurfaceFit {
    Original,
    #[default]
    Contain,
    Cover,
    Stretch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiImageSurfaceHighlight {
    pub rect: UiRect,
    pub current: bool,
    pub label: String,
}

impl UiImageSurfaceHighlight {
    #[must_use]
    pub fn search_hit(rect: UiRect, label: impl Into<String>) -> Self {
        Self {
            rect,
            current: false,
            label: label.into(),
        }
    }

    #[must_use]
    pub fn current_search_hit(rect: UiRect, label: impl Into<String>) -> Self {
        Self {
            rect,
            current: true,
            label: label.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiImageSurfaceValidationError {
    EmptyFingerprint,
    ZeroExtent,
    RgbaLengthOverflow { width: u32, height: u32 },
    RgbaLengthMismatch { expected: usize, actual: usize },
}

impl fmt::Display for UiImageSurfaceValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFingerprint => write!(formatter, "image surface fingerprint is empty"),
            Self::ZeroExtent => write!(formatter, "image surface extent must be non-zero"),
            Self::RgbaLengthOverflow { width, height } => write!(
                formatter,
                "rgba length overflow for image surface extent {width}x{height}"
            ),
            Self::RgbaLengthMismatch { expected, actual } => write!(
                formatter,
                "rgba length mismatch: expected {expected} bytes, got {actual} bytes"
            ),
        }
    }
}

impl std::error::Error for UiImageSurfaceValidationError {}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiImageSurfaceProps {
    pub fingerprint: String,
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub content_scale: u32,
    pub fit: UiImageSurfaceFit,
    pub accessibility_label: String,
    pub highlight_rects: Vec<UiImageSurfaceHighlight>,
}

impl UiImageSurfaceProps {
    pub fn new(
        fingerprint: impl Into<String>,
        width: u32,
        height: u32,
        rgba: Vec<u8>,
    ) -> Result<Self, UiImageSurfaceValidationError> {
        let fingerprint = fingerprint.into();
        Self::validate(&fingerprint, width, height, rgba.len())?;
        Ok(Self {
            fingerprint,
            width,
            height,
            rgba,
            content_scale: 100,
            fit: UiImageSurfaceFit::Contain,
            accessibility_label: String::new(),
            highlight_rects: Vec::new(),
        })
    }

    #[must_use]
    pub fn content_scale(mut self, value: u32) -> Self {
        self.content_scale = value;
        self
    }

    #[must_use]
    pub fn fit(mut self, value: UiImageSurfaceFit) -> Self {
        self.fit = value;
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = value.into();
        self
    }

    #[must_use]
    pub fn highlight_rect(mut self, value: UiImageSurfaceHighlight) -> Self {
        self.highlight_rects.push(value);
        self
    }

    fn validate(
        fingerprint: &str,
        width: u32,
        height: u32,
        actual_len: usize,
    ) -> Result<(), UiImageSurfaceValidationError> {
        if fingerprint.is_empty() {
            return Err(UiImageSurfaceValidationError::EmptyFingerprint);
        }
        if width == 0 || height == 0 {
            return Err(UiImageSurfaceValidationError::ZeroExtent);
        }
        let expected = expected_rgba_len(width, height)?;
        if actual_len != expected {
            return Err(UiImageSurfaceValidationError::RgbaLengthMismatch {
                expected,
                actual: actual_len,
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiImageSurfaceRenderPlan {
    pub fingerprint: String,
    pub width: u32,
    pub height: u32,
    pub rgba_byte_len: usize,
    pub content_scale: u32,
    pub fit: UiImageSurfaceFit,
    pub accessibility_label: String,
    pub highlight_rects: Vec<UiImageSurfaceHighlight>,
}

impl UiImageSurfaceRenderPlan {
    #[must_use]
    pub fn collect_from_tree(tree: &UiTree) -> Vec<Self> {
        let mut plans = Vec::new();
        Self::collect_from_node(tree.root(), &mut plans);
        plans
    }

    fn collect_from_node(node: &UiNode, plans: &mut Vec<Self>) {
        if node.kind() == UiNodeKind::ImageSurface {
            plans.push(Self::from_props(&node.props().image_surface));
        }
        for child in node.children() {
            Self::collect_from_node(child, plans);
        }
    }

    fn from_props(props: &UiImageSurfaceProps) -> Self {
        Self {
            fingerprint: props.fingerprint.clone(),
            width: props.width,
            height: props.height,
            rgba_byte_len: props.rgba.len(),
            content_scale: props.content_scale,
            fit: props.fit,
            accessibility_label: props.accessibility_label.clone(),
            highlight_rects: props.highlight_rects.clone(),
        }
    }
}

fn expected_rgba_len(width: u32, height: u32) -> Result<usize, UiImageSurfaceValidationError> {
    let pixels = (width as usize)
        .checked_mul(height as usize)
        .ok_or(UiImageSurfaceValidationError::RgbaLengthOverflow { width, height })?;
    pixels
        .checked_mul(RGBA_BYTES_PER_PIXEL)
        .ok_or(UiImageSurfaceValidationError::RgbaLengthOverflow { width, height })
}
