use super::types::{
    ColorPickerAlpha, InlineColorPicker, LabeledColorPicker, ResolvedInlineColorPicker,
    ResolvedLabeledColorPicker, RgbaChannel,
};
use super::{ColorPickerRgba, ops};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, button, container, dyn_container, empty, h_stack, label, v_stack};
use std::rc::Rc;

const STEP: i16 = 1;
const FAST_STEP: i16 = 16;
const BUTTON_SIZE: f32 = 24.0;
const BUTTON_RADIUS: f32 = 4.0;
const BUTTON_BORDER: f32 = 1.0;
const PANEL_WIDTH: f32 = 244.0;
const PANEL_PADDING: f32 = 8.0;
const PANEL_RADIUS: f32 = 6.0;
const PREVIEW_W: f32 = 64.0;
const PREVIEW_H: f32 = 40.0;
const ROW_GAP: f32 = crate::floem_view::GAP_SM;
const CONTROL_GAP: f32 = crate::floem_view::GAP_XS;
const VALUE_FONT_SIZE: f32 = 12.0;
const CHANNEL_LABEL_WIDTH: f32 = 16.0;
const CHANNEL_VALUE_WIDTH: f32 = 36.0;
const COLOR_LABEL_MARGIN: f32 = 8.0;
const COLOR_ROW_HEIGHT: f32 = 24.0;
const EMPTY_SIZE: f32 = 0.0;

impl InlineColorPicker {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        inline_picker_view(resolved, theme)
    }
}

impl LabeledColorPicker {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        labeled_picker_view(resolved, theme)
    }
}

impl ColorPickerRgba {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        inline_picker_view(resolved, theme)
    }
}

fn labeled_picker_view(resolved: ResolvedLabeledColorPicker, theme: Theme) -> impl IntoView {
    let label_text = resolved.label.clone();
    h_stack((
        container(empty()).style(|style| style.width(COLOR_LABEL_MARGIN)),
        label(move || label_text.clone()).style(move |style| {
            style
                .width(resolved.label_width)
                .height(COLOR_ROW_HEIGHT)
                .items_center()
        }),
        container(empty()).style(move |style| style.width(resolved.spacing)),
        container(inline_picker_view(resolved.picker, theme))
            .style(move |style| style.margin_top(resolved.offset_y)),
    ))
    .style(|style| style.height(COLOR_ROW_HEIGHT).items_center())
}

fn inline_picker_view(resolved: ResolvedInlineColorPicker, theme: Theme) -> impl IntoView {
    let value = create_rw_signal(resolved.value);
    let open = create_rw_signal(false);
    let fallback = resolved.value;
    let resolved_for_view = resolved.clone();

    dyn_container(
        move || {
            (
                value.try_get().unwrap_or(fallback),
                open.try_get().unwrap_or(false),
            )
        },
        move |(color, is_open)| {
            picker_shell(
                color,
                is_open,
                value,
                open,
                resolved_for_view.clone(),
                theme.clone(),
            )
        },
    )
}

fn picker_shell(
    color: Color,
    is_open: bool,
    value: RwSignal<Color>,
    open: RwSignal<bool>,
    resolved: ResolvedInlineColorPicker,
    theme: Theme,
) -> impl IntoView {
    v_stack((
        trigger_button(color, is_open, open, resolved.clone(), theme.clone()),
        panel_slot(color, is_open, value, open, resolved, theme),
    ))
    .style(|style| style.gap(CONTROL_GAP))
}

fn trigger_button(
    color: Color,
    is_open: bool,
    open: RwSignal<bool>,
    resolved: ResolvedInlineColorPicker,
    theme: Theme,
) -> impl IntoView {
    let locked = resolved.disabled || resolved.readonly;
    let border_color = if is_open {
        theme.color.accent
    } else {
        theme.color.border
    };
    let border = crate::floem_view::FloemColor::from_token(border_color);

    button(swatch(color))
        .disabled(move || locked)
        .action(move || {
            if !locked {
                open.set(!is_open);
            }
        })
        .style(move |style| {
            style
                .width(BUTTON_SIZE)
                .height(BUTTON_SIZE)
                .padding(EMPTY_SIZE)
                .border(BUTTON_BORDER)
                .border_color(border)
                .border_radius(BUTTON_RADIUS)
        })
}

fn panel_slot(
    color: Color,
    is_open: bool,
    value: RwSignal<Color>,
    open: RwSignal<bool>,
    resolved: ResolvedInlineColorPicker,
    theme: Theme,
) -> impl IntoView {
    if !is_open {
        return container(empty())
            .style(|style| style.width(EMPTY_SIZE).height(EMPTY_SIZE))
            .into_any();
    }

    let bg = crate::floem_view::FloemColor::from_token(theme.color.surface);
    let border = crate::floem_view::FloemColor::from_token(theme.color.border);
    container(panel_content(color, value, open, resolved))
        .style(move |style| {
            style
                .width(PANEL_WIDTH)
                .padding(PANEL_PADDING)
                .background(bg)
                .border(BUTTON_BORDER)
                .border_color(border)
                .border_radius(PANEL_RADIUS)
        })
        .into_any()
}

fn panel_content(
    color: Color,
    value: RwSignal<Color>,
    open: RwSignal<bool>,
    resolved: ResolvedInlineColorPicker,
) -> impl IntoView {
    let rows = if resolved.alpha.allows_alpha() {
        v_stack((
            channel_row("R", RgbaChannel::Red, color.r, value, resolved.clone()),
            channel_row("G", RgbaChannel::Green, color.g, value, resolved.clone()),
            channel_row("B", RgbaChannel::Blue, color.b, value, resolved.clone()),
            channel_row("A", RgbaChannel::Alpha, color.a, value, resolved.clone()),
        ))
        .style(|style| style.gap(CONTROL_GAP))
        .into_any()
    } else {
        v_stack((
            channel_row("R", RgbaChannel::Red, color.r, value, resolved.clone()),
            channel_row("G", RgbaChannel::Green, color.g, value, resolved.clone()),
            channel_row("B", RgbaChannel::Blue, color.b, value, resolved.clone()),
        ))
        .style(|style| style.gap(CONTROL_GAP))
        .into_any()
    };

    v_stack((
        preview(color, resolved.alpha),
        rows,
        button(label(|| "Close")).action(move || open.set(false)),
    ))
    .style(|style| style.gap(ROW_GAP))
}

fn preview(color: Color, alpha: ColorPickerAlpha) -> impl IntoView {
    h_stack((
        swatch(color).style(|style| style.width(PREVIEW_W).height(PREVIEW_H)),
        v_stack((
            label(move || ops::color_text(color, alpha))
                .style(|style| style.font_size(VALUE_FONT_SIZE)),
            label(move || ops::hex_text(color, alpha))
                .style(|style| style.font_size(VALUE_FONT_SIZE)),
        ))
        .style(|style| style.gap(CONTROL_GAP)),
    ))
    .style(|style| style.gap(CONTROL_GAP).items_center())
}

fn swatch(color: Color) -> impl IntoView {
    let fill = PenikoColor::rgba8(color.r, color.g, color.b, color.a);
    container(empty()).style(move |style| {
        style
            .width(BUTTON_SIZE)
            .height(BUTTON_SIZE)
            .background(fill)
            .border_radius(BUTTON_RADIUS)
    })
}

fn channel_row(
    label_text: &'static str,
    channel: RgbaChannel,
    current: u8,
    value: RwSignal<Color>,
    resolved: ResolvedInlineColorPicker,
) -> impl IntoView {
    h_stack((
        label(move || label_text).style(|style| style.width(CHANNEL_LABEL_WIDTH)),
        action_button("-16", channel, -FAST_STEP, value, resolved.clone()),
        action_button("-1", channel, -STEP, value, resolved.clone()),
        label(move || current.to_string()).style(|style| style.width(CHANNEL_VALUE_WIDTH)),
        action_button("+1", channel, STEP, value, resolved.clone()),
        action_button("+16", channel, FAST_STEP, value, resolved),
    ))
    .style(|style| style.gap(CONTROL_GAP).items_center())
}

fn action_button(
    text: &'static str,
    channel: RgbaChannel,
    delta: i16,
    value: RwSignal<Color>,
    resolved: ResolvedInlineColorPicker,
) -> impl IntoView {
    let locked = resolved.disabled || resolved.readonly;
    let on_change = Rc::clone(&resolved.on_change);
    let alpha = resolved.alpha;
    button(label(move || text))
        .disabled(move || locked)
        .action(move || {
            if locked {
                return;
            }

            if let Some(next) = value.try_update(|color| {
                *color = ops::adjust_channel(*color, channel, delta, alpha);
                *color
            }) {
                on_change(next);
            }
        })
}
