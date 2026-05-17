mod types;
mod view;

pub use types::{SpinnerProps, SpinnerSize};

use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::action::exec_after;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, dyn_container, svg};
use std::time::Duration;
use view::build_svg;

const FRAME_INTERVAL_MS: u64 = 80;
const DEGREES_PER_REVOLUTION: f32 = 360.0;
const MILLIS_PER_SECOND: f32 = 1000.0;

/// Resolved spinner state at a given animation angle.
#[derive(Debug, Clone)]
pub struct ResolvedSpinner {
    pub svg_content: String,
    pub size_px: f32,
    pub speed_rps: f32,
    pub reduced_motion: bool,
}

/// Builder for the Spinner primitive.
#[derive(Debug, Clone)]
pub struct Spinner {
    props: SpinnerProps,
}

impl Spinner {
    #[must_use]
    pub fn new() -> Self {
        Self {
            props: SpinnerProps::default(),
        }
    }

    #[must_use]
    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn color_override(mut self, color: Color) -> Self {
        self.props.color_override = Some(color);
        self
    }

    #[must_use]
    pub fn speed_rps(mut self, rps: f32) -> Self {
        self.props.speed_rps = rps;
        self
    }

    #[must_use]
    pub fn reduced_motion(mut self, reduce: bool) -> Self {
        self.props.reduced_motion = reduce;
        self
    }

    /// Create an animated Floem view for this spinner.
    ///
    /// The caller controls whether the spinner is present in the view tree.
    /// Rotation timing and frame updates are owned by the widget itself.
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let angle_deg = create_rw_signal(0.0);
        let is_mounted = create_rw_signal(true);
        let step_deg = self.step_degrees();

        if !self.props.reduced_motion && step_deg > 0.0 {
            schedule_next_frame(angle_deg, is_mounted, step_deg);
        }

        dyn_container(
            move || angle_deg.try_get().unwrap_or(0.0),
            move |angle| {
                let resolved = self.resolve(&theme, angle);
                svg(resolved.svg_content)
                    .style(move |s| s.width(resolved.size_px).height(resolved.size_px))
            },
        )
        .on_cleanup(move || {
            is_mounted.set(false);
        })
    }

    /// Resolve spinner properties to an SVG frame at `angle_deg` (0.0 for default position).
    #[must_use]
    pub fn resolve(&self, theme: &Theme, angle_deg: f32) -> ResolvedSpinner {
        let size_px = self.props.size.resolve_px(&theme.spacing);
        let c = self
            .props
            .color_override
            .as_ref()
            .unwrap_or(&theme.color.accent);
        let svg_content = if self.props.reduced_motion {
            build_svg(size_px, c.r, c.g, c.b, 0.0)
        } else {
            build_svg(size_px, c.r, c.g, c.b, angle_deg)
        };
        ResolvedSpinner {
            svg_content,
            size_px,
            speed_rps: self.props.speed_rps,
            reduced_motion: self.props.reduced_motion,
        }
    }

    fn step_degrees(&self) -> f32 {
        self.props.speed_rps * DEGREES_PER_REVOLUTION * FRAME_INTERVAL_MS as f32 / MILLIS_PER_SECOND
    }
}

fn schedule_next_frame(angle_deg: RwSignal<f32>, is_mounted: RwSignal<bool>, step_deg: f32) {
    exec_after(Duration::from_millis(FRAME_INTERVAL_MS), move |_| {
        if !is_mounted.try_get_untracked().unwrap_or(false) {
            return;
        }

        if angle_deg
            .try_update(|angle| {
                *angle = (*angle + step_deg) % DEGREES_PER_REVOLUTION;
            })
            .is_none()
        {
            return;
        }

        schedule_next_frame(angle_deg, is_mounted, step_deg);
    });
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn default_props_resolve_without_panic() {
        let theme = Theme::default_light();
        let resolved = Spinner::new().resolve(&theme, 0.0);
        assert!(resolved.size_px > 0.0);
        assert!(!resolved.svg_content.is_empty());
    }

    #[test]
    fn reduced_motion_produces_same_svg_regardless_of_angle() {
        let theme = Theme::default_light();
        let s = Spinner::new().reduced_motion(true);
        let a = s.resolve(&theme, 0.0).svg_content;
        let b = s.resolve(&theme, 90.0).svg_content;
        assert_eq!(a, b);
    }
}
