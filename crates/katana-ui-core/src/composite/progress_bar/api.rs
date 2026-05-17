use crate::composite::progress_bar::types::ProgressBarProps;
use crate::composite::progress_bar::view::{
    DEFAULT_ANIMATION_SPEED_MS, DEFAULT_BAR_SIZE, DEFAULT_TRACK_WIDTH, MIN_BAR_SIZE,
    MIN_TRACK_WIDTH, label_text, normalize_progress, resolve_fill_color, resolve_radius,
    resolve_size, resolve_track_color, resolve_track_width,
};
use crate::theme::Theme;
use crate::theme::color::Color;

const INDETERMINATE_PROGRESS: f32 = 0.0;
const MIN_SCALE: f32 = 4.0;
const STOP_ANIMATION_MS: u64 = 0;

#[derive(Debug, Clone)]
pub(super) struct ResolvedProgressBar {
    pub(super) progress: f32,
    pub(super) indeterminate: bool,
    pub(super) track_width: f32,
    pub(super) size: f32,
    pub(super) radius: f32,
    pub(super) track_color: Color,
    pub(super) fill_color: Color,
    pub(super) label_text: String,
    pub(super) show_label: bool,
    pub(super) animation_speed_ms: u64,
}

/// Builder for `ProgressBar`.
#[derive(Debug, Clone)]
pub struct ProgressBar {
    props: ProgressBarProps,
}

impl ProgressBar {
    #[must_use]
    pub fn new() -> Self {
        Self {
            props: ProgressBarProps::default(),
        }
    }

    #[must_use]
    pub fn value(mut self, value: f32) -> Self {
        self.props.value = value;
        self
    }

    #[must_use]
    pub fn min(mut self, min: f32) -> Self {
        self.props.min = min;
        self
    }

    #[must_use]
    pub fn max(mut self, max: f32) -> Self {
        self.props.max = max;
        self
    }

    #[must_use]
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.props.indeterminate = indeterminate;
        self
    }

    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn radius(mut self, radius: f32) -> Self {
        self.props.radius = radius;
        self
    }

    #[must_use]
    pub fn track_width(mut self, width: f32) -> Self {
        self.props.track_width = width;
        self
    }

    #[must_use]
    pub fn track_color(mut self, color: Color) -> Self {
        self.props.track_color = Some(color);
        self
    }

    #[must_use]
    pub fn fill_color(mut self, color: Color) -> Self {
        self.props.fill_color = Some(color);
        self
    }

    #[must_use]
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.props.label = Some(text.into());
        self.props.show_label = true;
        self
    }

    #[must_use]
    pub fn show_label(mut self, show: bool) -> Self {
        self.props.show_label = show;
        self
    }

    #[must_use]
    pub fn animation_speed_ms(mut self, speed_ms: u64) -> Self {
        self.props.animation_speed_ms = speed_ms;
        self
    }

    /// Resolve visual and behavior properties from current props and theme.
    #[must_use]
    pub(super) fn resolve(&self, theme: &Theme) -> ResolvedProgressBar {
        let indeterminate = self.props.indeterminate;
        let progress = if indeterminate {
            INDETERMINATE_PROGRESS
        } else {
            normalize_progress(&self.props)
        };
        let size = resolve_size(self.props.size).clamp(MIN_BAR_SIZE, DEFAULT_BAR_SIZE * MIN_SCALE);
        let track_width = resolve_track_width(self.props.track_width)
            .clamp(MIN_TRACK_WIDTH, DEFAULT_TRACK_WIDTH * MIN_SCALE);
        let radius = resolve_radius(self.props.radius, size);
        let label = label_text(progress, indeterminate, &self.props.label);
        let animation_speed_ms =
            if indeterminate && self.props.animation_speed_ms == STOP_ANIMATION_MS {
                DEFAULT_ANIMATION_SPEED_MS
            } else if indeterminate {
                self.props.animation_speed_ms
            } else {
                STOP_ANIMATION_MS
            };

        ResolvedProgressBar {
            progress,
            indeterminate,
            track_width,
            size,
            radius,
            track_color: resolve_track_color(self.props.track_color, theme),
            fill_color: resolve_fill_color(self.props.fill_color, theme),
            label_text: label,
            show_label: self.props.show_label,
            animation_speed_ms,
        }
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}
