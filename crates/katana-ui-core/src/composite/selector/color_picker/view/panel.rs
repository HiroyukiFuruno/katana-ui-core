use crate::floem_view::FloemColor;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::reactive::{RwSignal, SignalUpdate};
use floem::views::{Decorators, button, container, dyn_container, empty, h_stack, label, v_stack};
use std::rc::Rc;

use super::super::types::ColorPickerValue;
use super::ResolvedInlineColorPicker;
use super::controls::ColorPickerControls;
use super::plane::color_plane;
use super::preview::color_preview;

const PANEL_BASE_WIDTH: f32 = 560.0;
const PANEL_BASE_ESTIMATED_HEIGHT: f32 = 650.0;
const PANEL_RADIUS: f32 = 8.0;
const CLOSE_SIZE: f32 = 18.0;
const PANEL_PADDING: f32 = 12.0;
const PANEL_GAP: f32 = 10.0;
const HEADER_FONT_SIZE: f32 = 12.0;
const STACK_GAP: f32 = 4.0;
const BORDER_WIDTH: f32 = 1.0;

pub(super) struct PanelViewArgs {
    pub(super) state: RwSignal<ColorPickerValue>,
    pub(super) open: RwSignal<bool>,
    pub(super) resolved: ResolvedInlineColorPicker,
    pub(super) on_change: Rc<dyn Fn(Color)>,
    pub(super) locked: bool,
    pub(super) allows_alpha: bool,
    pub(super) panel_scale: f32,
    pub(super) theme: Theme,
}

pub(super) fn panel_width(scale: f32) -> f32 {
    PANEL_BASE_WIDTH * scale
}

pub(super) fn panel_estimated_height(scale: f32) -> f32 {
    PANEL_BASE_ESTIMATED_HEIGHT * scale
}

pub(super) fn panel_view(args: PanelViewArgs) -> impl IntoView {
    let PanelViewArgs {
        state,
        open,
        resolved,
        on_change,
        locked,
        allows_alpha,
        panel_scale,
        theme,
    } = args;
    let header_text = resolved.title.clone();
    let text_color = FloemColor::from_token(theme.color.text);
    let text_muted = FloemColor::from_token(theme.color.text_muted);
    let surface_color = FloemColor::from_token(theme.color.surface);
    let border_color = FloemColor::from_token(theme.color.border);

    v_stack((
        h_stack((
            dyn_container(
                move || header_text.clone(),
                move |title| {
                    if let Some(title) = title {
                        label(move || title.clone())
                            .style(move |style| style.font_size(HEADER_FONT_SIZE).color(text_color))
                            .into_any()
                    } else {
                        container(empty()).into_any()
                    }
                },
            ),
            container(empty()).style(|style| style.flex_grow(1.0)),
            button(label(|| "×").style(|style| {
                style
                    .width_full()
                    .height_full()
                    .items_center()
                    .justify_center()
            }))
            .disabled(move || locked)
            .action(move || {
                if !locked {
                    open.set(false);
                }
            })
            .style(move |style| {
                style
                    .width(CLOSE_SIZE)
                    .height(CLOSE_SIZE)
                    .items_center()
                    .justify_center()
                    .font_size(CLOSE_SIZE)
                    .color(text_muted)
            }),
        ))
        .style(|style| style.items_center().gap(STACK_GAP).justify_between()),
        ColorPickerControls::channel_row(
            state,
            Rc::clone(&on_change),
            locked,
            allows_alpha,
            theme.clone(),
        ),
        color_preview(state, theme.clone(), panel_scale),
        ColorPickerControls::blending_row(state, Rc::clone(&on_change), locked, theme.clone()),
        color_plane(
            state,
            Rc::clone(&on_change),
            locked,
            panel_scale,
            theme.clone(),
            allows_alpha,
        ),
        ColorPickerControls::hue_slider(
            state,
            Rc::clone(&on_change),
            locked,
            theme.clone(),
            allows_alpha,
            panel_scale,
        ),
        dyn_container(move || allows_alpha, {
            let alpha_change = Rc::clone(&on_change);
            let alpha_theme = theme.clone();
            move |visible| {
                if visible {
                    ColorPickerControls::alpha_slider(
                        state,
                        Rc::clone(&alpha_change),
                        locked,
                        alpha_theme.clone(),
                        allows_alpha,
                        panel_scale,
                    )
                    .into_any()
                } else {
                    container(empty()).into_any()
                }
            }
        }),
    ))
    .style(move |style| {
        style
            .width(panel_width(panel_scale))
            .padding(PANEL_PADDING * panel_scale)
            .background(surface_color)
            .border(BORDER_WIDTH)
            .border_color(border_color)
            .border_radius(PANEL_RADIUS)
            .gap(PANEL_GAP * panel_scale)
    })
}
