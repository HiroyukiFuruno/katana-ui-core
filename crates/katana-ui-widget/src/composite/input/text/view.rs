use super::TextInput;
use super::types::{IconSlotMode, InputSize, TrailingSlot};
use crate::primitive::icon::Icon;
use crate::primitive::spinner::{Spinner, SpinnerSize};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_effect, create_rw_signal};
use floem::views::{
    Decorators, button, container, empty, h_stack, label, text_input as floem_text_input,
};
use std::cell::Cell;
use std::rc::Rc;

const FONT_SM: f32 = 11.0;
const FONT_MD: f32 = 13.0;
const FONT_LG: f32 = 15.0;
const PAD_V_SM: f32 = 4.0;
const PAD_V_MD: f32 = 6.0;
const PAD_V_LG: f32 = 8.0;
const PAD_H_SM: f32 = 8.0;
const PAD_H_MD: f32 = 10.0;
const PAD_H_LG: f32 = 12.0;
const INPUT_GAP: f32 = crate::floem_view::GAP_XS;
const ICON_RESERVED_SIZE: f32 = 16.0;

pub(super) fn font_size(size: InputSize) -> f32 {
    match size {
        InputSize::Sm => FONT_SM,
        InputSize::Md => FONT_MD,
        InputSize::Lg => FONT_LG,
    }
}

pub(super) fn padding(size: InputSize) -> (f32, f32) {
    match size {
        InputSize::Sm => (PAD_V_SM, PAD_H_SM),
        InputSize::Md => (PAD_V_MD, PAD_H_MD),
        InputSize::Lg => (PAD_V_LG, PAD_H_LG),
    }
}

pub(super) fn bg_color(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.surface
    } else {
        theme.color.bg
    }
}

pub(super) fn text_color(disabled: bool, has_value: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.text_disabled
    } else if has_value {
        theme.color.text
    } else {
        theme.color.text_muted
    }
}

pub(super) fn border_color(invalid: bool, disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.border
    } else if invalid {
        theme.color.danger
    } else {
        theme.color.border
    }
}

pub(super) fn focus_ring_color(invalid: bool, theme: &Theme) -> Color {
    if invalid {
        theme.color.danger
    } else {
        theme.color.accent
    }
}

impl TextInput {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let leading_icon = self.props.leading_icon.clone();
        let value = create_rw_signal(resolved.value.clone());
        let on_change = Rc::clone(&resolved.on_change);
        let disabled = resolved.disabled || resolved.readonly;
        let did_mount = Rc::new(Cell::new(false));

        create_effect({
            let did_mount = Rc::clone(&did_mount);
            move |_| {
                let next = value.try_get().unwrap_or_default();
                if did_mount.replace(true) && !disabled {
                    on_change(next.clone());
                }
                next
            }
        });

        let bg = crate::floem_view::FloemColor::from_token(resolved.bg_color);
        let text = crate::floem_view::FloemColor::from_token(resolved.text_color);
        let border = crate::floem_view::FloemColor::from_token(resolved.border_color);
        let placeholder = resolved.placeholder.clone().unwrap_or_default();
        let leading = match (
            resolved.has_leading_icon,
            self.props.leading_icon_mode,
            leading_icon,
        ) {
            (true, IconSlotMode::Visible, Some(icon)) => {
                Icon::new(icon).view(theme.clone()).into_any()
            }
            (_, IconSlotMode::Reserved, _) => empty_icon_space(),
            _ => empty_slot(),
        };
        let trailing = match resolved.trailing.clone() {
            TrailingSlot::None => empty_slot(),
            TrailingSlot::Reserved => empty_icon_space(),
            TrailingSlot::ClearButton => button(label(|| "×"))
                .action(move || {
                    if !disabled {
                        value.set(String::new());
                    }
                })
                .into_any(),
            TrailingSlot::Custom(icon) => Icon::new(icon).view(theme.clone()).into_any(),
            TrailingSlot::Spinner => Spinner::new()
                .size(SpinnerSize::Pt(resolved.font_size))
                .view(theme.clone())
                .into_any(),
        };

        h_stack((
            leading,
            floem_text_input(value)
                .placeholder(placeholder)
                .disabled(move || disabled)
                .style(move |style| style.font_size(resolved.font_size).color(text)),
            trailing,
        ))
        .style(move |style| {
            style
                .gap(INPUT_GAP)
                .items_center()
                .background(bg)
                .border(1.0)
                .border_color(border)
                .padding_vert(resolved.pad_v)
                .padding_horiz(resolved.pad_h)
        })
    }
}

fn empty_slot() -> Box<dyn floem::View> {
    container(empty())
        .style(|style| style.width(0.0))
        .into_any()
}

fn empty_icon_space() -> Box<dyn floem::View> {
    container(empty())
        .style(|style| style.width(ICON_RESERVED_SIZE).height(ICON_RESERVED_SIZE))
        .into_any()
}
