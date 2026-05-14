use crate::composite::selector::color_picker::ops;
use crate::composite::selector::color_picker::types::{ColorPickerValue, RgbaChannel};
use crate::composite::selector::color_picker::view::{apply_state, channel_to_color_name};
use crate::floem_view::FloemColor;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::reactive::{RwSignal, SignalGet};
use floem::views::{Decorators, button, container, h_stack, h_stack_from_iter, label};
use std::rc::Rc;

const ROW_GAP: f32 = 4.0;
const TYPE_LABEL_WIDTH: f32 = 26.0;
const STEP_BUTTON_WIDTH: f32 = 20.0;
const VALUE_WIDTH: f32 = 32.0;
const CHANNEL_LABEL_WIDTH: f32 = 14.0;
const FIELD_HEIGHT: f32 = 22.0;
const TEXT_SIZE: f32 = 12.0;
const CHANNEL_STEP: i16 = 1;
const FIELD_RADIUS: f32 = 2.0;
const CONTROL_GAP: f32 = 2.0;

pub(crate) struct ChannelControls;

impl ChannelControls {
    pub(crate) fn row(
        state: RwSignal<ColorPickerValue>,
        on_change: Rc<dyn Fn(Color)>,
        locked: bool,
        allows_alpha: bool,
        theme: Theme,
    ) -> impl IntoView {
        let mut fields = Vec::new();
        fields.push(channel_field(
            state,
            Rc::clone(&on_change),
            locked,
            RgbaChannel::Red,
            theme.clone(),
        ));
        fields.push(channel_field(
            state,
            Rc::clone(&on_change),
            locked,
            RgbaChannel::Green,
            theme.clone(),
        ));
        fields.push(channel_field(
            state,
            Rc::clone(&on_change),
            locked,
            RgbaChannel::Blue,
            theme.clone(),
        ));
        if allows_alpha {
            fields.push(channel_field(
                state,
                Rc::clone(&on_change),
                locked,
                RgbaChannel::Alpha,
                theme.clone(),
            ));
        }

        h_stack((
            mode_button(theme.clone()),
            h_stack_from_iter(fields).style(|style| style.items_center().gap(ROW_GAP)),
        ))
        .style(|style| style.items_center().gap(ROW_GAP))
    }
}

fn mode_button(theme: Theme) -> impl IntoView {
    let bg = FloemColor::from_token(theme.color.surface);
    let text = FloemColor::from_token(theme.color.text);
    container(label(|| "U8")).style(move |style| {
        style
            .width(TYPE_LABEL_WIDTH)
            .height(FIELD_HEIGHT)
            .items_center()
            .background(bg)
            .color(text)
    })
}

fn channel_field(
    state: RwSignal<ColorPickerValue>,
    on_change: Rc<dyn Fn(Color)>,
    locked: bool,
    channel: RgbaChannel,
    theme: Theme,
) -> impl IntoView {
    let bg = FloemColor::from_token(theme.color.bg);
    let text = FloemColor::from_token(theme.color.text);
    let border = FloemColor::from_token(theme.color.border);
    h_stack((
        label(move || channel_to_color_name(channel).to_string()).style(move |style| {
            style
                .width(CHANNEL_LABEL_WIDTH)
                .height(FIELD_HEIGHT)
                .items_center()
                .font_size(TEXT_SIZE)
                .color(text)
        }),
        step_button(
            "-",
            state,
            Rc::clone(&on_change),
            locked,
            channel,
            -CHANNEL_STEP,
            theme.clone(),
        ),
        container(label(move || {
            channel_value(state.get().color, channel).to_string()
        }))
        .style(move |style| {
            style
                .width(VALUE_WIDTH)
                .height(FIELD_HEIGHT)
                .items_center()
                .font_size(TEXT_SIZE)
                .background(bg)
                .color(text)
                .border(1.0)
                .border_color(border)
                .border_radius(FIELD_RADIUS)
        }),
        step_button(
            "+",
            state,
            Rc::clone(&on_change),
            locked,
            channel,
            CHANNEL_STEP,
            theme,
        ),
    ))
    .style(|style| style.items_center().gap(CONTROL_GAP))
}

fn step_button(
    text: &'static str,
    state: RwSignal<ColorPickerValue>,
    on_change: Rc<dyn Fn(Color)>,
    locked: bool,
    channel: RgbaChannel,
    step: i16,
    theme: Theme,
) -> impl IntoView {
    let bg = FloemColor::from_token(theme.color.surface);
    let text_color = FloemColor::from_token(theme.color.text);
    let border = FloemColor::from_token(theme.color.border);
    button(label(move || text.to_string()))
        .disabled(move || locked)
        .action(move || {
            if locked {
                return;
            }
            let next = ops::ColorPickerOps::adjust_channel_state(state.get(), channel, step);
            apply_state(&state, Rc::clone(&on_change), next);
        })
        .style(move |style| {
            style
                .width(STEP_BUTTON_WIDTH)
                .height(FIELD_HEIGHT)
                .font_size(TEXT_SIZE)
                .background(bg)
                .color(text_color)
                .border(1.0)
                .border_color(border)
                .border_radius(FIELD_RADIUS)
        })
}

fn channel_value(color: Color, channel: RgbaChannel) -> u8 {
    match channel {
        RgbaChannel::Red => color.r,
        RgbaChannel::Green => color.g,
        RgbaChannel::Blue => color.b,
        RgbaChannel::Alpha => color.a,
    }
}
