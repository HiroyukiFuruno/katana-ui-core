mod types;
mod view;

pub use types::{Segment, SegmentedSize, SegmentedToggleProps};

use crate::theme::Theme;
use crate::theme::color::Color;
use view::{font_size, padding, selected_bg, selected_text, unselected_bg, unselected_text};

/// Resolved visual properties for a single segment.
#[derive(Debug, Clone)]
pub struct ResolvedSegment {
    pub label: String,
    pub bg_color: Color,
    pub text_color: Color,
    pub selected: bool,
}

/// Resolved visual properties for `SegmentedToggle`.
#[derive(Debug, Clone)]
pub struct ResolvedSegmentedToggle {
    pub segments: Vec<ResolvedSegment>,
    pub font_size: f32,
    pub pad_v: f32,
    pub pad_h: f32,
    pub disabled: bool,
    pub a11y_label: String,
}

/// Builder for the SegmentedToggle composite widget.
#[derive(Debug, Clone)]
pub struct SegmentedToggle<K> {
    props: SegmentedToggleProps<K>,
}

impl<K: PartialEq + Clone> SegmentedToggle<K> {
    #[must_use]
    pub fn new(value: K, options: Vec<(K, Segment)>, a11y_label: impl Into<String>) -> Self {
        Self {
            props: SegmentedToggleProps {
                value,
                options,
                size: SegmentedSize::default(),
                disabled: false,
                a11y_label: a11y_label.into(),
            },
        }
    }

    #[must_use]
    pub fn size(mut self, size: SegmentedSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedSegmentedToggle {
        let fs = font_size(self.props.size);
        let (pv, ph) = padding(self.props.size);
        let disabled = self.props.disabled;

        let segments = if self.props.options.is_empty() {
            vec![ResolvedSegment {
                label: String::new(),
                bg_color: unselected_bg(theme),
                text_color: unselected_text(disabled, theme),
                selected: false,
            }]
        } else {
            self.props
                .options
                .iter()
                .map(|(key, seg)| {
                    let selected = *key == self.props.value;
                    let label = match seg {
                        Segment::Label(s) => s.clone(),
                        Segment::Icon(_, s) => s.clone(),
                    };
                    let bg_color = if selected {
                        selected_bg(disabled, theme)
                    } else {
                        unselected_bg(theme)
                    };
                    let text_color = if selected {
                        selected_text(disabled, theme)
                    } else {
                        unselected_text(disabled, theme)
                    };
                    ResolvedSegment {
                        label,
                        bg_color,
                        text_color,
                        selected,
                    }
                })
                .collect()
        };

        ResolvedSegmentedToggle {
            segments,
            font_size: fs,
            pad_v: pv,
            pad_h: ph,
            disabled,
            a11y_label: self.props.a11y_label.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[derive(Debug, Clone, PartialEq)]
    enum Mode {
        List,
        Grid,
        Map,
    }

    fn options() -> Vec<(Mode, Segment)> {
        vec![
            (Mode::List, Segment::Label("List".into())),
            (Mode::Grid, Segment::Label("Grid".into())),
            (Mode::Map, Segment::Label("Map".into())),
        ]
    }

    #[test]
    fn selected_segment_has_accent_bg() {
        let theme = Theme::default_light();
        let r = SegmentedToggle::new(Mode::Grid, options(), "View mode").resolve(&theme);
        let grid = r
            .segments
            .iter()
            .filter(|s| s.label == "Grid")
            .collect::<Vec<_>>();
        assert_eq!(grid.len(), 1);
        assert!(grid[0].selected);
        assert_eq!(grid[0].bg_color, theme.color.accent);
    }

    #[test]
    fn unselected_segments_have_surface_bg() {
        let theme = Theme::default_light();
        let r = SegmentedToggle::new(Mode::Grid, options(), "View mode").resolve(&theme);
        let list = r
            .segments
            .iter()
            .filter(|s| s.label == "List")
            .collect::<Vec<_>>();
        assert_eq!(list.len(), 1);
        assert!(!list[0].selected);
        assert_eq!(list[0].bg_color, theme.color.surface);
    }

    #[test]
    fn empty_options_returns_one_fallback_segment() {
        let theme = Theme::default_light();
        let r = SegmentedToggle::new(Mode::List, vec![], "Empty").resolve(&theme);
        assert_eq!(r.segments.len(), 1);
        assert!(r.segments[0].label.is_empty());
    }

    #[test]
    fn disabled_uses_border_for_selected_bg() {
        let theme = Theme::default_light();
        let r = SegmentedToggle::new(Mode::List, options(), "Mode")
            .disabled(true)
            .resolve(&theme);
        let list = r
            .segments
            .iter()
            .filter(|s| s.label == "List")
            .collect::<Vec<_>>();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].bg_color, theme.color.border);
    }
}
