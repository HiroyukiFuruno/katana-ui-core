use super::{UiImageSurfaceTransform, UiRect};
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
    pub display_width: u32,
    pub display_height: u32,
    pub display_width_milli: u32,
    pub display_height_milli: u32,
    pub rgba: Vec<u8>,
    pub content_scale: u32,
    pub fit: UiImageSurfaceFit,
    pub accessibility_label: String,
    pub selection_text: String,
    pub highlight_rects: Vec<UiImageSurfaceHighlight>,
    pub transform: UiImageSurfaceTransform,
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
            display_width: 0,
            display_height: 0,
            display_width_milli: 0,
            display_height_milli: 0,
            rgba,
            content_scale: 100,
            fit: UiImageSurfaceFit::Contain,
            accessibility_label: String::new(),
            selection_text: String::new(),
            highlight_rects: Vec::new(),
            transform: UiImageSurfaceTransform::default(),
        })
    }

    #[must_use]
    pub fn content_scale(mut self, value: u32) -> Self {
        self.content_scale = value;
        self
    }

    #[must_use]
    pub fn display_size(mut self, width: u32, height: u32) -> Self {
        self.display_width = width;
        self.display_height = height;
        self.display_width_milli = width.saturating_mul(DISPLAY_SIZE_MILLI);
        self.display_height_milli = height.saturating_mul(DISPLAY_SIZE_MILLI);
        self
    }

    #[must_use]
    pub fn display_size_exact(mut self, width: f32, height: f32) -> Self {
        self.display_width = positive_finite_dimension(width).ceil() as u32;
        self.display_height = positive_finite_dimension(height).ceil() as u32;
        self.display_width_milli = display_size_milli(width);
        self.display_height_milli = display_size_milli(height);
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
    pub fn selection_text(mut self, value: impl Into<String>) -> Self {
        self.selection_text = value.into();
        self
    }

    #[must_use]
    pub fn highlight_rect(mut self, value: UiImageSurfaceHighlight) -> Self {
        self.highlight_rects.push(value);
        self
    }

    #[must_use]
    pub fn transform(mut self, value: UiImageSurfaceTransform) -> Self {
        self.transform = value;
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

const DISPLAY_SIZE_MILLI: u32 = 1000;

fn positive_finite_dimension(value: f32) -> f32 {
    if value.is_finite() && value > 0.0 {
        return value;
    }
    0.0
}

fn display_size_milli(value: f32) -> u32 {
    let value = positive_finite_dimension(value);
    (value * DISPLAY_SIZE_MILLI as f32).round() as u32
}

fn expected_rgba_len(width: u32, height: u32) -> Result<usize, UiImageSurfaceValidationError> {
    let pixels = (width as usize)
        .checked_mul(height as usize)
        .ok_or(UiImageSurfaceValidationError::RgbaLengthOverflow { width, height })?;
    pixels
        .checked_mul(RGBA_BYTES_PER_PIXEL)
        .ok_or(UiImageSurfaceValidationError::RgbaLengthOverflow { width, height })
}
