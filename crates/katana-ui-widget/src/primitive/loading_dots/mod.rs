mod animation;
mod types;
mod view;

#[cfg(test)]
mod tests;

pub use types::LoadingDotsProps;

use crate::theme::Theme;
use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, dyn_container};
use types::MIN_SIZE;

use animation::schedule_next_step;

/// Builder for `LoadingDots`.
#[derive(Debug, Clone)]
pub struct LoadingDots {
    props: LoadingDotsProps,
}

/// Resolved properties ready to render.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ResolvedLoadingDots {
    pub dot_count: usize,
    pub dot_size: f32,
    pub dot_gap: f32,
    pub color: crate::theme::color::Color,
    pub label: Option<String>,
    pub active: bool,
    pub animation_speed_ms: u64,
}

impl LoadingDots {
    #[must_use]
    pub fn new() -> Self {
        Self {
            props: LoadingDotsProps::default(),
        }
    }

    #[must_use]
    pub fn dot_count(mut self, dot_count: usize) -> Self {
        self.props.dot_count = dot_count;
        self
    }

    #[must_use]
    pub fn dot_size(mut self, dot_size: f32) -> Self {
        self.props.dot_size = dot_size;
        self
    }

    #[must_use]
    pub fn dot_gap(mut self, dot_gap: f32) -> Self {
        self.props.dot_gap = dot_gap;
        self
    }

    #[must_use]
    pub fn color(mut self, color: crate::theme::color::Color) -> Self {
        self.props.color_override = Some(color);
        self
    }

    #[must_use]
    pub fn active(mut self, active: bool) -> Self {
        self.props.active = active;
        self
    }

    #[must_use]
    pub fn animation_speed_ms(mut self, animation_speed_ms: u64) -> Self {
        self.props.animation_speed_ms = animation_speed_ms;
        self
    }

    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.props.label = Some(label.into());
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedLoadingDots {
        let dot_size = self.props.dot_size.max(MIN_SIZE);
        let dot_gap = if self.props.dot_gap.is_sign_negative() {
            0.0
        } else {
            self.props.dot_gap
        };
        let color = self.props.color_override.unwrap_or(theme.color.accent);
        let animation_speed_ms = if self.props.active {
            self.props.animation_speed_ms.max(1)
        } else {
            0
        };

        ResolvedLoadingDots {
            dot_count: self.props.dot_count,
            dot_size,
            dot_gap,
            color,
            label: self.props.label.clone(),
            active: self.props.active,
            animation_speed_ms,
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let active_step = create_rw_signal(0usize);
        let mounted = create_rw_signal(true);

        if resolved.active && resolved.dot_count > 0 && resolved.animation_speed_ms > 0 {
            schedule_next_step(
                active_step,
                mounted,
                resolved.dot_count,
                resolved.animation_speed_ms,
            );
        }

        dyn_container(
            move || active_step.try_get().unwrap_or_default(),
            move |frame| view::dots_row(resolved.clone(), frame),
        )
        .on_cleanup(move || mounted.set(false))
    }
}

impl ResolvedLoadingDots {
    fn active_dot_index(&self, frame: usize) -> Option<usize> {
        if !self.active || self.dot_count == 0 {
            return None;
        }
        Some(frame % self.dot_count)
    }
}

impl Default for LoadingDots {
    fn default() -> Self {
        Self::new()
    }
}
