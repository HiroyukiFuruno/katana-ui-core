mod types;
mod view;

pub use types::{ColorSwatchProps, SwatchSize};

use crate::theme::Theme;
use crate::theme::color::Color;
use view::{cell_size, ring_width};

/// Resolved visual properties for a single swatch cell.
#[derive(Debug, Clone)]
pub struct ResolvedSwatchCell {
    pub color: Color,
    pub cell_size: f32,
    pub selected: bool,
    pub ring_width: f32,
    pub ring_color: Color,
}

/// Resolved visual properties for `ColorSwatch`.
#[derive(Debug, Clone)]
pub struct ResolvedColorSwatch {
    pub cells: Vec<ResolvedSwatchCell>,
    pub disabled: bool,
    pub a11y_label: String,
}

/// Builder for the ColorSwatch composite widget.
#[derive(Debug, Clone)]
pub struct ColorSwatch {
    props: ColorSwatchProps,
}

impl ColorSwatch {
    #[must_use]
    pub fn new(value: Color, palette: Vec<Color>, a11y_label: impl Into<String>) -> Self {
        Self {
            props: ColorSwatchProps {
                value,
                palette,
                size: SwatchSize::default(),
                disabled: false,
                a11y_label: a11y_label.into(),
            },
        }
    }

    #[must_use]
    pub fn size(mut self, size: SwatchSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedColorSwatch {
        let cs = cell_size(self.props.size);
        let rw = ring_width(self.props.size);
        let ring_color = if self.props.disabled {
            theme.color.border
        } else {
            theme.color.text
        };

        let cells = self
            .props
            .palette
            .iter()
            .map(|c| ResolvedSwatchCell {
                color: *c,
                cell_size: cs,
                selected: *c == self.props.value,
                ring_width: rw,
                ring_color,
            })
            .collect();

        ResolvedColorSwatch {
            cells,
            disabled: self.props.disabled,
            a11y_label: self.props.a11y_label.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    fn red() -> Color {
        Color {
            r: 220,
            g: 50,
            b: 50,
            a: 255,
        }
    }

    fn blue() -> Color {
        Color {
            r: 50,
            g: 100,
            b: 220,
            a: 255,
        }
    }

    fn palette() -> Vec<Color> {
        vec![red(), blue()]
    }

    #[test]
    fn selected_cell_marked() {
        let theme = Theme::default_light();
        let r = ColorSwatch::new(blue(), palette(), "Pick color").resolve(&theme);
        let blue_cells: Vec<_> = r.cells.iter().filter(|c| c.color == blue()).collect();
        assert_eq!(blue_cells.len(), 1);
        assert!(blue_cells[0].selected);

        let red_cells: Vec<_> = r.cells.iter().filter(|c| c.color == red()).collect();
        assert_eq!(red_cells.len(), 1);
        assert!(!red_cells[0].selected);
    }

    #[test]
    fn disabled_ring_color_is_border() {
        let theme = Theme::default_light();
        let r = ColorSwatch::new(red(), palette(), "Pick color")
            .disabled(true)
            .resolve(&theme);
        for cell in &r.cells {
            assert_eq!(cell.ring_color, theme.color.border);
        }
    }

    #[test]
    fn cell_count_matches_palette() {
        let theme = Theme::default_light();
        let r = ColorSwatch::new(red(), palette(), "Colors").resolve(&theme);
        assert_eq!(r.cells.len(), 2);
    }
}
