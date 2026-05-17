use super::types::SelectSize;
use super::{SelectBox, SelectBoxProps};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{
    Decorators, button, container, dyn_container, empty, label, v_stack, v_stack_from_iter,
};
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
const SELECT_GAP: f32 = 2.0;
const SELECT_EMPTY_SIZE: f32 = crate::floem_view::EMPTY_SIZE;

pub(super) fn font_size(size: SelectSize) -> f32 {
    match size {
        SelectSize::Sm => FONT_SM,
        SelectSize::Md => FONT_MD,
        SelectSize::Lg => FONT_LG,
    }
}

pub(super) fn padding(size: SelectSize) -> (f32, f32) {
    match size {
        SelectSize::Sm => (PAD_V_SM, PAD_H_SM),
        SelectSize::Md => (PAD_V_MD, PAD_H_MD),
        SelectSize::Lg => (PAD_V_LG, PAD_H_LG),
    }
}

pub(super) fn trigger_bg(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.surface
    } else {
        theme.color.bg
    }
}

pub(super) fn trigger_text(disabled: bool, has_value: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.text_disabled
    } else if has_value {
        theme.color.text
    } else {
        theme.color.text_muted
    }
}

pub(super) fn border_color(is_open: bool, disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.border
    } else if is_open {
        theme.color.accent
    } else {
        theme.color.border
    }
}

pub(super) fn option_bg(selected: bool, theme: &Theme) -> Color {
    if selected {
        theme.color.accent_muted
    } else {
        theme.color.bg
    }
}

pub(super) fn option_text(selected: bool, theme: &Theme) -> Color {
    if selected {
        theme.color.accent
    } else {
        theme.color.text
    }
}

impl<K: PartialEq + Clone> SelectBox<K> {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView
    where
        K: 'static,
    {
        let value = create_rw_signal(self.props.value.clone());
        let open = create_rw_signal(self.props.is_open);
        let initial_value = self.props.value.clone();
        let initial_open = self.props.is_open;
        let options = self.props.options.clone();
        let disabled = self.props.disabled;
        let placeholder = self.props.placeholder.clone();
        let a11y_label = self.props.a11y_label.clone();
        let size = self.props.size;
        let on_change = Rc::clone(&self.props.on_change);

        dyn_container(
            move || {
                (
                    value.try_get().unwrap_or_else(|| initial_value.clone()),
                    open.try_get().unwrap_or(initial_open),
                )
            },
            move |(current_value, is_open)| {
                let trigger_label = current_value
                    .as_ref()
                    .and_then(|current| {
                        options
                            .iter()
                            .find(|(key, _)| key == current)
                            .map(|(_, label)| label.clone())
                    })
                    .unwrap_or_else(|| placeholder.clone());
                let probe = SelectBox {
                    props: SelectBoxProps {
                        value: current_value.clone(),
                        options: options.clone(),
                        placeholder: placeholder.clone(),
                        size,
                        disabled,
                        is_open,
                        a11y_label: a11y_label.clone(),
                        on_change: Rc::clone(&on_change),
                    },
                };
                let resolved = probe.resolve(&theme);
                let trigger_bg_color =
                    crate::floem_view::FloemColor::from_token(resolved.trigger_bg);
                let trigger_text_color =
                    crate::floem_view::FloemColor::from_token(resolved.trigger_text);
                let border = crate::floem_view::FloemColor::from_token(resolved.border_color);
                let option_views = options.clone().into_iter().map({
                    let value = value;
                    let open = open;
                    let on_change = Rc::clone(&on_change);
                    let theme = theme.clone();
                    move |(key, option_label)| {
                        let selected = current_value.as_ref() == Some(&key);
                        let bg =
                            crate::floem_view::FloemColor::from_token(option_bg(selected, &theme));
                        let text = crate::floem_view::FloemColor::from_token(option_text(
                            selected, &theme,
                        ));
                        let on_change_for_action = Rc::clone(&on_change);
                        button(
                            label(move || option_label.clone())
                                .style(move |style| style.color(text).font_size(font_size(size))),
                        )
                        .action(move || {
                            if !disabled {
                                value.set(Some(key.clone()));
                                open.set(false);
                                on_change_for_action(key.clone());
                            }
                        })
                        .style(move |style| {
                            let (pad_v, pad_h) = padding(size);
                            style
                                .background(bg)
                                .padding_vert(pad_v)
                                .padding_horiz(pad_h)
                        })
                    }
                });
                let panel = if is_open {
                    v_stack_from_iter(option_views).into_any()
                } else {
                    container(empty())
                        .style(|style| style.width(SELECT_EMPTY_SIZE).height(SELECT_EMPTY_SIZE))
                        .into_any()
                };

                v_stack((
                    button(label(move || trigger_label.clone()).style(move |style| {
                        style
                            .color(trigger_text_color)
                            .font_size(resolved.font_size)
                    }))
                    .action(move || {
                        if !disabled {
                            open.update(|is_open| *is_open = !*is_open);
                        }
                    })
                    .style(move |style| {
                        style
                            .background(trigger_bg_color)
                            .border(1.0)
                            .border_color(border)
                            .padding_vert(resolved.pad_v)
                            .padding_horiz(resolved.pad_h)
                    }),
                    panel,
                ))
                .style(|style| style.gap(SELECT_GAP))
            },
        )
    }
}
