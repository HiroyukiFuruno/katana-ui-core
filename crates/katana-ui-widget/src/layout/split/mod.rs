mod ops;
mod types;
mod view;

pub use types::{Direction, SplitPaneProps};

use crate::theme::Theme;
use crate::theme::color::Color;
use view::{handle_color, handle_hover_color, handle_thickness};

/// Resolved visual properties for `SplitPane`.
#[derive(Debug, Clone)]
pub struct ResolvedSplitPane {
    pub direction: Direction,
    pub ratio: f32,
    pub min_ratio: f32,
    pub max_ratio: f32,
    pub handle_thickness: f32,
    pub handle_color: Color,
    pub handle_hover_color: Color,
}

/// Builder for the SplitPane layout widget.
#[derive(Debug, Clone)]
pub struct SplitPane {
    props: SplitPaneProps,
}

const DEFAULT_RATIO: f32 = 0.5;
const DEFAULT_MIN_RATIO: f32 = 0.1;
const DEFAULT_MAX_RATIO: f32 = 0.9;

impl SplitPane {
    #[must_use]
    pub fn new() -> Self {
        Self {
            props: SplitPaneProps {
                direction: Direction::default(),
                ratio: DEFAULT_RATIO,
                min_ratio: DEFAULT_MIN_RATIO,
                max_ratio: DEFAULT_MAX_RATIO,
            },
        }
    }

    #[must_use]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.props.direction = direction;
        self
    }

    #[must_use]
    pub fn ratio(mut self, ratio: f32) -> Self {
        self.props.ratio = ratio;
        self
    }

    #[must_use]
    pub fn min_ratio(mut self, min: f32) -> Self {
        self.props.min_ratio = min;
        self
    }

    #[must_use]
    pub fn max_ratio(mut self, max: f32) -> Self {
        self.props.max_ratio = max;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedSplitPane {
        let ratio = ops::clamp_ratio(self.props.ratio, self.props.min_ratio, self.props.max_ratio);
        ResolvedSplitPane {
            direction: self.props.direction,
            ratio,
            min_ratio: self.props.min_ratio,
            max_ratio: self.props.max_ratio,
            handle_thickness: handle_thickness(),
            handle_color: handle_color(theme),
            handle_hover_color: handle_hover_color(theme),
        }
    }
}

impl Default for SplitPane {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn ratio_clamped_to_min() {
        let theme = Theme::default_light();
        let r = SplitPane::new().ratio(0.05).min_ratio(0.1).resolve(&theme);
        assert!((r.ratio - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn ratio_clamped_to_max() {
        let theme = Theme::default_light();
        let r = SplitPane::new().ratio(0.95).max_ratio(0.9).resolve(&theme);
        assert!((r.ratio - 0.9).abs() < f32::EPSILON);
    }

    #[test]
    fn drag_ratio_computes_correctly() {
        let new_ratio = ops::drag_ratio(0.5, 50.0, 500.0);
        assert!((new_ratio - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn drag_zero_total_returns_start() {
        let r = ops::drag_ratio(0.5, 100.0, 0.0);
        assert!((r - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn double_click_resets_to_half() {
        assert!((ops::reset_ratio() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn horizontal_and_vertical_both_resolve() {
        let theme = Theme::default_light();
        let _h = SplitPane::new()
            .direction(Direction::Horizontal)
            .resolve(&theme);
        let _v = SplitPane::new()
            .direction(Direction::Vertical)
            .resolve(&theme);
    }
}
