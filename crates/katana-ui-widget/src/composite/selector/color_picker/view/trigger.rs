use crate::floem_view::FloemColor;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::views::{
    Decorators, TooltipExt, button, container, empty, h_stack, h_stack_from_iter, v_stack_from_iter,
};
use floem::{IntoView, View};

use super::super::types::ColorPickerTriggerSize;

const BUTTON_BORDER_RADIUS: f32 = 4.0;
const LABEL_PADDING_VERT: f32 = 4.0;
const LABEL_PADDING_HORIZ: f32 = 6.0;
const BORDER_SIZE: f32 = 1.0;
const TRIGGER_BASE_HEIGHT: f32 = 24.0;
const TRIGGER_RGB_BASE_WIDTH: f32 = 42.0;
const TRIGGER_RGBA_BASE_WIDTH: f32 = 76.0;
const TRIGGER_SCALE_XS: f32 = 0.67;
const TRIGGER_SCALE_SM: f32 = 0.84;
const TRIGGER_SCALE_MID: f32 = 1.0;
const TRIGGER_SCALE_LARGE: f32 = 1.34;
const TRIGGER_SCALE_XLARGE: f32 = 1.67;
const CHECKER_ROWS: usize = 2;
const CHECKER_COLUMNS: usize = 4;
const CHECKER_DARK: Color = Color {
    r: 174,
    g: 174,
    b: 174,
    a: u8::MAX,
};
const CHECKER_BRIGHT: Color = Color {
    r: 202,
    g: 202,
    b: 202,
    a: u8::MAX,
};

#[derive(Clone, Copy)]
struct TriggerMetrics {
    height: f32,
    rgb_width: f32,
    rgba_width: f32,
}

pub(super) fn trigger_button(
    state: RwSignal<super::super::types::ColorPickerValue>,
    open: RwSignal<bool>,
    resolved: super::ResolvedInlineColorPicker,
    theme: Theme,
    locked: bool,
) -> impl View {
    let tooltip = resolved.a11y_label.clone();
    let allows_alpha = resolved.alpha.allows_alpha();
    let border_color = FloemColor::from_token(theme.color.border);
    let accent_color = FloemColor::from_token(theme.color.accent);
    let surface_color = FloemColor::from_token(theme.color.surface);
    let metrics = trigger_metrics(resolved.trigger_size);
    let trigger_size = resolved.trigger_size;
    let trigger_border = resolved.trigger_border;
    let preview_width = if allows_alpha {
        metrics.rgba_width
    } else {
        metrics.rgb_width
    };

    button(
        trigger_preview(state, allows_alpha, metrics)
            .style(move |style| style.width(preview_width).height(metrics.height)),
    )
    .action(move || {
        if locked {
            return;
        }
        open.set(!open.get());
    })
    .tooltip(move || tooltip.clone())
    .style(move |style| {
        let border_color = if open.get() {
            accent_color
        } else {
            border_color
        };
        style
            .border(if trigger_border { BORDER_SIZE } else { 0.0 })
            .border_color(border_color)
            .background(surface_color)
            .padding_vert(LABEL_PADDING_VERT * trigger_scale(trigger_size))
            .padding_horiz(LABEL_PADDING_HORIZ * trigger_scale(trigger_size))
            .border_radius(BUTTON_BORDER_RADIUS)
    })
}

fn trigger_metrics(size: ColorPickerTriggerSize) -> TriggerMetrics {
    let scale = trigger_scale(size);
    TriggerMetrics {
        height: TRIGGER_BASE_HEIGHT * scale,
        rgb_width: TRIGGER_RGB_BASE_WIDTH * scale,
        rgba_width: TRIGGER_RGBA_BASE_WIDTH * scale,
    }
}

fn trigger_scale(size: ColorPickerTriggerSize) -> f32 {
    match size {
        ColorPickerTriggerSize::Xs => TRIGGER_SCALE_XS,
        ColorPickerTriggerSize::Sm => TRIGGER_SCALE_SM,
        ColorPickerTriggerSize::Mid => TRIGGER_SCALE_MID,
        ColorPickerTriggerSize::Large => TRIGGER_SCALE_LARGE,
        ColorPickerTriggerSize::Xlarge => TRIGGER_SCALE_XLARGE,
    }
}

fn opaque(color: Color) -> Color {
    Color {
        a: u8::MAX,
        ..color
    }
}

fn trigger_preview(
    state: RwSignal<super::super::types::ColorPickerValue>,
    allows_alpha: bool,
    metrics: TriggerMetrics,
) -> impl IntoView {
    let alpha_width = (metrics.rgba_width / 2.0).floor();
    let solid_width = metrics.rgba_width - alpha_width;

    if allows_alpha {
        h_stack((
            alpha_preview(state, alpha_width, metrics),
            solid_preview(state, solid_width, metrics.height),
        ))
        .style(move |style| style.height(metrics.height))
        .into_any()
    } else {
        solid_preview(state, metrics.rgb_width, metrics.height).into_any()
    }
}

fn alpha_preview(
    state: RwSignal<super::super::types::ColorPickerValue>,
    width: f32,
    metrics: TriggerMetrics,
) -> impl IntoView {
    let tile_width = width / CHECKER_COLUMNS as f32;
    let tile_height = metrics.height / CHECKER_ROWS as f32;
    container(v_stack_from_iter((0..CHECKER_ROWS).map(move |row| {
        h_stack_from_iter((0..CHECKER_COLUMNS).map(move |column| {
            let backdrop = checker_color(row, column);
            container(empty()).style(move |style| {
                style
                    .width(tile_width)
                    .height(tile_height)
                    .background(FloemColor::from_token(composite_over(
                        state.get().color,
                        backdrop,
                    )))
            })
        }))
    })))
    .style(move |style| style.width(width).height(metrics.height))
}

fn solid_preview(
    state: RwSignal<super::super::types::ColorPickerValue>,
    width: f32,
    height: f32,
) -> impl IntoView {
    container(empty()).style(move |style| {
        style
            .width(width)
            .height(height)
            .background(FloemColor::from_token(opaque(state.get().color)))
    })
}

fn checker_color(row: usize, column: usize) -> Color {
    if (row + column).is_multiple_of(2) {
        CHECKER_DARK
    } else {
        CHECKER_BRIGHT
    }
}

fn composite_over(color: Color, backdrop: Color) -> Color {
    let alpha = u16::from(color.a);
    let inverse_alpha = u16::from(u8::MAX) - alpha;
    Color {
        r: composite_channel(color.r, backdrop.r, alpha, inverse_alpha),
        g: composite_channel(color.g, backdrop.g, alpha, inverse_alpha),
        b: composite_channel(color.b, backdrop.b, alpha, inverse_alpha),
        a: u8::MAX,
    }
}

fn composite_channel(source: u8, backdrop: u8, alpha: u16, inverse_alpha: u16) -> u8 {
    let blended = u16::from(source) * alpha + u16::from(backdrop) * inverse_alpha;
    (blended / u16::from(u8::MAX)) as u8
}
