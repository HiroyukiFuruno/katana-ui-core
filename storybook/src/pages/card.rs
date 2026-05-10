use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::layout::card::{Card, CardPadding, CardVariant};
use katana_ui_widget::theme::Theme;

fn card_row(
    heading: &'static str,
    bg_r: u8,
    bg_g: u8,
    bg_b: u8,
    has_border: bool,
    has_shadow: bool,
    pad: f32,
    radius: f32,
    interactive: bool,
) -> impl IntoView {
    let bg = PenikoColor::rgb8(bg_r, bg_g, bg_b);
    let border_w = if has_border { 1.0_f32 } else { 0.0_f32 };
    let shadow_tag = if has_shadow { " [shadow]" } else { "" };
    let interactive_tag = if interactive { " [interactive]" } else { "" };
    let desc: &'static str = Box::leak(
        format!("pad={pad} radius={radius}{shadow_tag}{interactive_tag}").into_boxed_str(),
    );
    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        label(move || desc).style(move |s| {
            s.background(bg)
                .border(border_w)
                .border_radius(radius)
                .padding(pad)
                .min_width(200.0)
        }),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let r_plain = Card::new().variant(CardVariant::Plain).resolve(theme);
    let r_elevated = Card::new().variant(CardVariant::Elevated).resolve(theme);
    let r_outlined = Card::new().variant(CardVariant::Outlined).resolve(theme);
    let r_no_pad = Card::new().variant(CardVariant::Outlined).padding(CardPadding::None).resolve(theme);
    let r_lg_pad = Card::new().variant(CardVariant::Outlined).padding(CardPadding::Lg).resolve(theme);
    let r_interactive = Card::new().variant(CardVariant::Elevated).interactive(true).resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "Card Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            card_row("Plain", r_plain.bg_color.r, r_plain.bg_color.g, r_plain.bg_color.b, r_plain.border_color.is_some(), r_plain.has_shadow, r_plain.padding, r_plain.corner_radius, r_plain.interactive),
            card_row("Elevated (shadow)", r_elevated.bg_color.r, r_elevated.bg_color.g, r_elevated.bg_color.b, r_elevated.border_color.is_some(), r_elevated.has_shadow, r_elevated.padding, r_elevated.corner_radius, r_elevated.interactive),
            card_row("Outlined (border)", r_outlined.bg_color.r, r_outlined.bg_color.g, r_outlined.bg_color.b, r_outlined.border_color.is_some(), r_outlined.has_shadow, r_outlined.padding, r_outlined.corner_radius, r_outlined.interactive),
            card_row("Padding None", r_no_pad.bg_color.r, r_no_pad.bg_color.g, r_no_pad.bg_color.b, r_no_pad.border_color.is_some(), r_no_pad.has_shadow, r_no_pad.padding, r_no_pad.corner_radius, r_no_pad.interactive),
            card_row("Padding Lg", r_lg_pad.bg_color.r, r_lg_pad.bg_color.g, r_lg_pad.bg_color.b, r_lg_pad.border_color.is_some(), r_lg_pad.has_shadow, r_lg_pad.padding, r_lg_pad.corner_radius, r_lg_pad.interactive),
            label(|| "Interactive").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            card_row("Elevated + interactive", r_interactive.bg_color.r, r_interactive.bg_color.g, r_interactive.bg_color.b, r_interactive.border_color.is_some(), r_interactive.has_shadow, r_interactive.padding, r_interactive.corner_radius, r_interactive.interactive),
        ))
        .style(move |s| {
            s.gap(8.0)
                .padding(16.0)
                .background(bg)
                .color(text_col)
                .min_width_full()
        }),
    )
}

pub fn card_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "Card").style(|s| s.font_size(20.0)),
            label(move || if is_dark.get() { "Dark" } else { "Light" }),
            toggle_button(move || is_dark.get()).on_toggle(move |v| is_dark.set(v)),
        ))
        .style(|s| s.gap(12.0).items_center().padding(12.0)),
        dyn_container(
            move || is_dark.get(),
            move |dark| {
                let theme = if dark {
                    Theme::default_dark()
                } else {
                    Theme::default_light()
                };
                page_content(&theme)
            },
        ),
    ))
}
