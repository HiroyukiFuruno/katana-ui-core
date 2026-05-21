use crate::render_model::{UiDimension, UiNodeKind, UiSize, UiStateId, UiTone};
use serde::{Deserialize, Serialize};

pub(super) const DEFAULT_RADIUS_PX: u16 = 4;
pub(super) const DEFAULT_TEXT_LINE_RATIO: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SkeletonShape {
    Rect,
    Circle,
    Line { thickness: f32 },
    Text { lines: usize, last_line_ratio: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkeletonAnimation {
    None,
    Pulse,
    Wave,
    Shimmer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkeletonSize {
    Auto,
    Fill,
    Fixed {
        width: UiDimension,
        height: UiDimension,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Skeleton {
    pub(super) label: String,
    pub(super) state_id: UiStateId,
    pub(super) shape: SkeletonShape,
    pub(super) size: SkeletonSize,
    pub(super) visual_size: UiSize,
    pub(super) animation: SkeletonAnimation,
    pub(super) radius_px: u16,
    pub(super) tone: UiTone,
    pub(super) accessibility_label: String,
    pub(super) aspect_ratio: Option<SkeletonAspectRatio>,
    pub(super) reduced_motion: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkeletonAspectRatio {
    pub width: u16,
    pub height: u16,
}

impl Skeleton {
    #[must_use]
    pub fn new(label: impl Into<String>, shape: SkeletonShape) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::Skeleton),
            shape,
            size: SkeletonSize::Auto,
            visual_size: UiSize::Medium,
            animation: SkeletonAnimation::Pulse,
            radius_px: DEFAULT_RADIUS_PX,
            tone: UiTone::Neutral,
            accessibility_label: String::new(),
            aspect_ratio: None,
            reduced_motion: false,
        }
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn size(mut self, size: SkeletonSize) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn animation(mut self, animation: SkeletonAnimation) -> Self {
        self.animation = animation;
        self
    }

    #[must_use]
    pub fn radius_px(mut self, radius_px: u16) -> Self {
        self.radius_px = radius_px;
        self
    }

    #[must_use]
    pub fn tone(mut self, tone: UiTone) -> Self {
        self.tone = tone;
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, label: impl Into<String>) -> Self {
        self.accessibility_label = label.into();
        self
    }

    #[must_use]
    pub fn aspect_ratio(mut self, width: u16, height: u16) -> Self {
        self.aspect_ratio = Some(SkeletonAspectRatio { width, height });
        self
    }

    #[must_use]
    pub fn reduced_motion(mut self, reduced_motion: bool) -> Self {
        self.reduced_motion = reduced_motion;
        self
    }

    #[must_use]
    pub fn effective_animation(&self) -> SkeletonAnimation {
        if self.reduced_motion {
            SkeletonAnimation::None
        } else {
            self.animation
        }
    }
}

impl Default for SkeletonShape {
    fn default() -> Self {
        Self::Text {
            lines: 1,
            last_line_ratio: DEFAULT_TEXT_LINE_RATIO,
        }
    }
}
