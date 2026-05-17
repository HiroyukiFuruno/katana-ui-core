use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, h_stack, label, scroll, v_stack};
use katana_ui_core::composite::button::text::TextButton;
use katana_ui_core::composite::indicator::badge::{Badge, BadgeTone, BadgeVariant};
use katana_ui_core::composite::input::text::TextInput;
use katana_ui_core::layout::accordion::Accordion;
use katana_ui_core::layout::card::{Card, CardPadding, CardVariant};
use katana_ui_core::theme::Theme;

fn card_row(
    heading: &'static str,
    color: (u8, u8, u8),
    has_border: bool,
    has_shadow: bool,
    pad: f32,
    radius: f32,
    interactive: bool,
    with_slots: bool,
) -> impl IntoView {
    let bg = PenikoColor::rgb8(color.0, color.1, color.2);
    let desc = [
        format!("pad={pad} radius={radius}"),
        if has_shadow {
            " [shadow]".to_string()
        } else {
            String::new()
        },
        if interactive {
            " [interactive]".to_string()
        } else {
            String::new()
        },
        if with_slots {
            " [slots]".to_string()
        } else {
            String::new()
        },
    ]
    .join("");
    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        label(move || desc.clone()).style(move |s| {
            s.background(bg)
                .border(if has_border { 1.0_f32 } else { 0.0_f32 })
                .border_radius(radius)
                .padding(pad)
        }),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: Theme) -> impl IntoView + use<> {
    let page_background = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let page_text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let samples = [
        Card::new().variant(CardVariant::Plain).resolve(&theme),
        Card::new().variant(CardVariant::Elevated).resolve(&theme),
        Card::new().variant(CardVariant::Outlined).resolve(&theme),
        Card::new()
            .variant(CardVariant::Outlined)
            .padding(CardPadding::None)
            .resolve(&theme),
        Card::new()
            .variant(CardVariant::Outlined)
            .padding(CardPadding::Lg)
            .resolve(&theme),
        Card::new()
            .variant(CardVariant::Elevated)
            .interactive(true)
            .resolve(&theme),
    ];

    let card_clicks = create_rw_signal(0_i32);
    let button_clicks = create_rw_signal(0_i32);
    let text_input_value = create_rw_signal(String::new());
    let accordion_open = create_rw_signal(false);

    let complex_card = {
        Card::new()
            .variant(CardVariant::Outlined)
            .padding(CardPadding::Lg)
            .interactive(true)
            .on_click({
                let card_clicks = card_clicks;
                move || card_clicks.update(|value| *value += 1)
            })
            .header(h_stack((
                label(|| "User Profile").style(|s| s.font_size(14.0)),
                Badge::new("Interactive")
                    .tone(BadgeTone::Info)
                    .variant(BadgeVariant::Solid)
                    .view(theme.clone()),
            )))
            .body(v_stack((
                label(|| "Body slot with form controls"),
                TextInput::new("Name")
                    .on_change({
                        let text_input_value = text_input_value;
                        move |value| text_input_value.set(value)
                    })
                    .view(theme.clone()),
                label(|| "Inputs and buttons are placed in Card slots."),
            )))
            .content(
                Accordion::new("Details")
                    .expanded(false)
                    .on_toggle({
                        let accordion_open = accordion_open;
                        move |open| accordion_open.set(open)
                    })
                    .view(theme.clone(), || {
                        label(|| "Content slot: place nested layout freely.")
                            .style(|s| s.font_size(12.0).line_height(1.4))
                    }),
            )
            .actions(h_stack((
                TextButton::new("Save").view(theme.clone(), {
                    let button_clicks = button_clicks;
                    move || button_clicks.update(|value| *value += 1)
                }),
                TextButton::new("Clear").view(theme.clone(), || {
                    let _ = 1;
                }),
            )))
            .footer(label(move || {
                let open = if accordion_open.get() {
                    "open"
                } else {
                    "closed"
                };
                format!(
                    "card click: {} / button click: {} / input: {} / accordion: {}",
                    card_clicks.get(),
                    button_clicks.get(),
                    text_input_value.get(),
                    open,
                )
            }))
            .view(theme.clone(), label(|| ""))
    };

    let rows = v_stack((
        card_row(
            "Plain",
            (
                samples[0].bg_color.r,
                samples[0].bg_color.g,
                samples[0].bg_color.b,
            ),
            samples[0].border_color.is_some(),
            samples[0].has_shadow,
            samples[0].padding,
            samples[0].corner_radius,
            samples[0].interactive,
            false,
        ),
        card_row(
            "Elevated",
            (
                samples[1].bg_color.r,
                samples[1].bg_color.g,
                samples[1].bg_color.b,
            ),
            samples[1].border_color.is_some(),
            samples[1].has_shadow,
            samples[1].padding,
            samples[1].corner_radius,
            samples[1].interactive,
            false,
        ),
        card_row(
            "Outlined",
            (
                samples[2].bg_color.r,
                samples[2].bg_color.g,
                samples[2].bg_color.b,
            ),
            samples[2].border_color.is_some(),
            samples[2].has_shadow,
            samples[2].padding,
            samples[2].corner_radius,
            samples[2].interactive,
            false,
        ),
        card_row(
            "Padding None",
            (
                samples[3].bg_color.r,
                samples[3].bg_color.g,
                samples[3].bg_color.b,
            ),
            samples[3].border_color.is_some(),
            samples[3].has_shadow,
            samples[3].padding,
            samples[3].corner_radius,
            samples[3].interactive,
            false,
        ),
        card_row(
            "Padding Lg",
            (
                samples[4].bg_color.r,
                samples[4].bg_color.g,
                samples[4].bg_color.b,
            ),
            samples[4].border_color.is_some(),
            samples[4].has_shadow,
            samples[4].padding,
            samples[4].corner_radius,
            samples[4].interactive,
            false,
        ),
        card_row(
            "Interactive",
            (
                samples[5].bg_color.r,
                samples[5].bg_color.g,
                samples[5].bg_color.b,
            ),
            samples[5].border_color.is_some(),
            samples[5].has_shadow,
            samples[5].padding,
            samples[5].corner_radius,
            samples[5].interactive,
            true,
        ),
    ));

    scroll(
        v_stack((
            label(|| "Card Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            v_stack((label(|| "Live widget"), complex_card)).style(|s| s.gap(8.0)),
            rows,
        ))
        .style(move |s| {
            s.gap(12.0)
                .padding(16.0)
                .background(page_background)
                .color(page_text)
                .min_width_full()
        }),
    )
    .style(|style| style.width_full().height_full().flex_grow(1.0))
}

pub fn card_page(theme: Theme) -> impl IntoView {
    page_content(theme)
}
