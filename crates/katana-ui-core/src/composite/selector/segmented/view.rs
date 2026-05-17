use super::SegmentedToggle;
use super::types::{Segment, SegmentedSize};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, button, h_stack_from_iter, label};
use std::rc::Rc;

const FONT_SM: f32 = 11.0;
const FONT_MD: f32 = 13.0;
const FONT_LG: f32 = 15.0;
const PAD_V_SM: f32 = 4.0;
const PAD_V_MD: f32 = 6.0;
const PAD_V_LG: f32 = 8.0;
const PAD_H_SM: f32 = 8.0;
const PAD_H_MD: f32 = 12.0;
const PAD_H_LG: f32 = 16.0;
const SEGMENT_GAP: f32 = 2.0;

pub(super) fn font_size(size: SegmentedSize) -> f32 {
    match size {
        SegmentedSize::Sm => FONT_SM,
        SegmentedSize::Md => FONT_MD,
        SegmentedSize::Lg => FONT_LG,
    }
}

pub(super) fn padding(size: SegmentedSize) -> (f32, f32) {
    match size {
        SegmentedSize::Sm => (PAD_V_SM, PAD_H_SM),
        SegmentedSize::Md => (PAD_V_MD, PAD_H_MD),
        SegmentedSize::Lg => (PAD_V_LG, PAD_H_LG),
    }
}

pub(super) fn selected_bg(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.border
    } else {
        theme.color.accent
    }
}

pub(super) fn unselected_bg(theme: &Theme) -> Color {
    theme.color.surface
}

pub(super) fn selected_text(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.text_disabled
    } else {
        theme.color.bg
    }
}

pub(super) fn unselected_text(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.text_disabled
    } else {
        theme.color.text_muted
    }
}

impl<K: PartialEq + Clone> SegmentedToggle<K> {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView
    where
        K: 'static,
    {
        let selected = create_rw_signal(self.props.value.clone());
        let options = self.props.options.clone();
        let disabled = self.props.disabled;
        let size = self.props.size;
        let on_change = Rc::clone(&self.props.on_change);

        floem::views::dyn_container(
            move || {
                selected
                    .try_get()
                    .unwrap_or_else(|| self.props.value.clone())
            },
            move |current| {
                let on_change_for_cells = Rc::clone(&on_change);
                let cells =
                    options.clone().into_iter().map({
                        let theme = theme.clone();
                        move |(key, segment)| {
                            let label_text = match segment {
                                Segment::Label(value) | Segment::Icon(_, value) => value,
                            };
                            let is_selected = key == current;
                            let bg = if is_selected {
                                selected_bg(disabled, &theme)
                            } else {
                                unselected_bg(&theme)
                            };
                            let text = if is_selected {
                                selected_text(disabled, &theme)
                            } else {
                                unselected_text(disabled, &theme)
                            };
                            let text_color = crate::floem_view::FloemColor::from_token(text);
                            let bg_color = crate::floem_view::FloemColor::from_token(bg);
                            let on_change = Rc::clone(&on_change_for_cells);
                            let selected = selected;
                            button(label(move || label_text.clone()).style(move |style| {
                                style.font_size(font_size(size)).color(text_color)
                            }))
                            .action(move || {
                                if !disabled {
                                    selected.set(key.clone());
                                    on_change(key.clone());
                                }
                            })
                            .style(move |style| {
                                let (pad_v, pad_h) = padding(size);
                                style
                                    .background(bg_color)
                                    .padding_vert(pad_v)
                                    .padding_horiz(pad_h)
                            })
                        }
                    });
                h_stack_from_iter(cells).style(|style| style.gap(SEGMENT_GAP))
            },
        )
    }
}
