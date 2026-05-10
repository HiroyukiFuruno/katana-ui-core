mod types;
mod view;

pub use types::{
    ColorSwatch, ColorSwatchProps, ResolvedColorSwatch, ResolvedSwatchCell, SwatchShape, SwatchSize,
};

use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, button, h_stack_from_iter, label};
use std::rc::Rc;
use view::{border_radius, cell_size, ring_width};

const SWATCH_GAP: f32 = crate::floem_view::GAP_XS;

impl ColorSwatch {
    #[must_use]
    pub fn new(value: Color, palette: Vec<Color>, a11y_label: impl Into<String>) -> Self {
        Self {
            props: ColorSwatchProps {
                value,
                palette,
                size: SwatchSize::default(),
                shape: SwatchShape::default(),
                disabled: false,
                a11y_label: a11y_label.into(),
                on_change: Rc::new(|_| {}),
            },
        }
    }

    #[must_use]
    pub fn size(mut self, size: SwatchSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn shape(mut self, shape: SwatchShape) -> Self {
        self.props.shape = shape;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Color) + 'static) -> Self {
        self.props.on_change = Rc::new(on_change);
        self
    }

    /// Select a color and notify the caller when enabled.
    pub fn select(&self, color: Color) -> Option<Color> {
        if self.props.disabled || !self.props.palette.contains(&color) {
            return None;
        }

        (self.props.on_change)(color);
        Some(color)
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedColorSwatch {
        let cs = cell_size(self.props.size);
        let rw = ring_width(self.props.size);
        let radius = border_radius(self.props.size, self.props.shape);
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
                border_radius: radius,
            })
            .collect();

        ResolvedColorSwatch {
            cells,
            disabled: self.props.disabled,
            a11y_label: self.props.a11y_label.clone(),
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let selected = create_rw_signal(self.props.value);
        let palette = self.props.palette.clone();
        let disabled = self.props.disabled;
        let size = self.props.size;
        let shape = self.props.shape;
        let on_change = Rc::clone(&self.props.on_change);

        floem::views::dyn_container(
            move || selected.try_get().unwrap_or(self.props.value),
            move |current| {
                let on_change_for_cells = Rc::clone(&on_change);
                let cells = palette.clone().into_iter().map({
                    let theme = theme.clone();
                    move |color| {
                        let selected_now = color == current;
                        let fill = crate::floem_view::FloemColor::from_token(color);
                        let border = if disabled {
                            theme.color.border
                        } else {
                            theme.color.text
                        };
                        let border_color = crate::floem_view::FloemColor::from_token(border);
                        let selected = selected;
                        let on_change = Rc::clone(&on_change_for_cells);
                        button(label(|| ""))
                            .action(move || {
                                if !disabled {
                                    selected.set(color);
                                    on_change(color);
                                }
                            })
                            .style(move |style| {
                                style
                                    .width(cell_size(size))
                                    .height(cell_size(size))
                                    .background(fill)
                                    .border(if selected_now { ring_width(size) } else { 1.0 })
                                    .border_color(border_color)
                                    .border_radius(border_radius(size, shape))
                            })
                    }
                });
                h_stack_from_iter(cells).style(|style| {
                    style
                        .gap(SWATCH_GAP)
                        .flex_wrap(floem::style::FlexWrap::Wrap)
                })
            },
        )
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

    #[test]
    fn circle_shape_uses_half_cell_radius() {
        let theme = Theme::default_light();
        let r = ColorSwatch::new(red(), palette(), "Colors")
            .shape(SwatchShape::Circle)
            .resolve(&theme);
        assert_eq!(r.cells[0].border_radius, r.cells[0].cell_size / 2.0);
    }

    #[test]
    fn select_calls_on_change() {
        let called = std::rc::Rc::new(std::cell::RefCell::new(None));
        let called_ref = std::rc::Rc::clone(&called);
        let swatch = ColorSwatch::new(red(), palette(), "Colors").on_change(move |color| {
            *called_ref.borrow_mut() = Some(color);
        });

        assert_eq!(swatch.select(blue()), Some(blue()));
        assert_eq!(*called.borrow(), Some(blue()));
    }

    #[test]
    fn disabled_select_does_not_call_on_change() {
        let called = std::rc::Rc::new(std::cell::RefCell::new(false));
        let called_ref = std::rc::Rc::clone(&called);
        let swatch = ColorSwatch::new(red(), palette(), "Colors")
            .disabled(true)
            .on_change(move |_| {
                *called_ref.borrow_mut() = true;
            });

        assert_eq!(swatch.select(blue()), None);
        assert!(!*called.borrow());
    }
}
