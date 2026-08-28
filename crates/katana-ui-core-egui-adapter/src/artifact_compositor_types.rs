use crate::command_chrome::CommandChromePaintPlan;
use crate::context_menu::ContextMenuPaintPlan;
use crate::text_surface::TextSurfacePaintPlan;
use katana_ui_core::render_model::UiRect;
use std::fmt;

/// Bounds allocated for the actual root egui frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactCanvasBounds(UiRect);

impl ArtifactCanvasBounds {
    #[must_use]
    pub const fn new(bounds: UiRect) -> Self {
        Self(bounds)
    }

    #[must_use]
    pub const fn ui_rect(self) -> UiRect {
        self.0
    }
}

/// A borrowed adapter paint plan in its actual paint order.
#[derive(Debug, Clone, Copy)]
pub enum ArtifactPaintPlanRef<'a> {
    TextSurface(&'a TextSurfacePaintPlan),
    CommandChrome(&'a CommandChromePaintPlan),
    ContextMenu(&'a ContextMenuPaintPlan),
}

/// One actual root canvas and its ordered adapter paint plans.
#[derive(Debug, Clone)]
pub struct ArtifactCompositeRequest<'a> {
    pub canvas: ArtifactCanvasBounds,
    pub plans: &'a [ArtifactPaintPlanRef<'a>],
}

/// Deterministic RGBA output of an adapter paint plan composition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactCompositeFrame {
    pub canvas: ArtifactCanvasBounds,
    pub rgba_pixels: Vec<u8>,
    pub pixel_hash: String,
    pub paint_plan_hash: String,
    pub non_transparent_pixel_count: usize,
}

/// Typed rejection for invalid artifact composition input.
#[derive(Debug, PartialEq, Eq)]
pub enum ArtifactCompositeError {
    ZeroCanvas,
    Overflow {
        context: &'static str,
    },
    ZeroTexture {
        identity: String,
    },
    TextureByteLength {
        identity: String,
        expected: usize,
        actual: usize,
    },
    TexturePixelRange {
        identity: String,
        start: usize,
        end: usize,
        actual: usize,
    },
    Serialization(String),
}

impl fmt::Display for ArtifactCompositeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroCanvas => {
                formatter.write_str("artifact canvas must have non-zero dimensions")
            }
            Self::Overflow { context } => {
                write!(formatter, "artifact arithmetic overflow while {context}")
            }
            Self::ZeroTexture { identity } => {
                write!(
                    formatter,
                    "artifact texture `{identity}` has zero dimensions"
                )
            }
            Self::TextureByteLength {
                identity,
                expected,
                actual,
            } => write!(
                formatter,
                "artifact texture `{identity}` has {actual} RGBA bytes; expected {expected}"
            ),
            Self::TexturePixelRange {
                identity,
                start,
                end,
                actual,
            } => write!(
                formatter,
                "artifact texture `{identity}` cannot provide RGBA range {start}..{end} from {actual} bytes"
            ),
            Self::Serialization(error) => {
                write!(formatter, "artifact plan serialization failed: {error}")
            }
        }
    }
}

impl std::error::Error for ArtifactCompositeError {}

#[cfg(test)]
mod tests {
    use super::ArtifactCompositeError;

    #[test]
    fn error_display_covers_all_variants() {
        let errors = [
            ArtifactCompositeError::ZeroCanvas.to_string(),
            ArtifactCompositeError::Overflow {
                context: "test context",
            }
            .to_string(),
            ArtifactCompositeError::ZeroTexture {
                identity: "texture".to_owned(),
            }
            .to_string(),
            ArtifactCompositeError::TextureByteLength {
                identity: "texture".to_owned(),
                expected: 4,
                actual: 2,
            }
            .to_string(),
            ArtifactCompositeError::TexturePixelRange {
                identity: "texture".to_owned(),
                start: 0,
                end: 4,
                actual: 2,
            }
            .to_string(),
            ArtifactCompositeError::Serialization("oops".to_owned()).to_string(),
        ];
        assert!(errors[0].contains("non-zero"));
        assert!(errors[1].contains("arithmetic overflow while test context"));
        assert!(errors[2].contains("texture `texture` has zero"));
        assert!(errors[3].contains("RGBA bytes; expected 4"));
        assert!(errors[4].contains("cannot provide RGBA range 0..4"));
        assert!(errors[5].contains("artifact plan serialization failed"));
    }
}
