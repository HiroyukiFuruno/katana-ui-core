use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, h_stack, label, scroll, v_stack};
use katana_ui_core::composite::button::icon_text::{IconPosition, IconTextButton};
use katana_ui_core::composite::button::text::{Size, Tone, Variant};
use katana_ui_core::primitive::icon::{IconSize, IconSource};
use katana_ui_core::theme::Theme;

const SAMPLE_SVG: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><circle cx=\"8\" cy=\"8\" r=\"6\" fill=\"currentColor\"/></svg>";

fn btn_cell(lbl: &'static str, font_sz: f32, r: u8, g: u8, b: u8, a: u8) -> impl IntoView {
    let color = PenikoColor::rgba8(r, g, b, a);
    label(move || lbl).style(move |s| s.font_size(font_sz).color(color).padding(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let icon = IconSource::SvgBytes(SAMPLE_SVG);
    let click_log = create_rw_signal("未クリック".to_string());

    let r0 = IconTextButton::new(icon.clone(), "Leading Icon")
        .icon_position(IconPosition::Leading)
        .variant(Variant::Primary)
        .tone(Tone::Accent)
        .size(Size::Md)
        .resolve(theme);

    let r1 = IconTextButton::new(icon.clone(), "Trailing Icon")
        .icon_position(IconPosition::Trailing)
        .variant(Variant::Secondary)
        .tone(Tone::Accent)
        .size(Size::Md)
        .resolve(theme);

    let r2 = IconTextButton::new(icon.clone(), "Danger")
        .variant(Variant::Primary)
        .tone(Tone::Danger)
        .size(Size::Md)
        .resolve(theme);

    let r3 = IconTextButton::new(icon.clone(), "Small")
        .variant(Variant::Ghost)
        .tone(Tone::Neutral)
        .size(Size::Sm)
        .icon_size(IconSize::Sm)
        .resolve(theme);

    let r4 = IconTextButton::new(icon.clone(), "Large")
        .variant(Variant::Primary)
        .tone(Tone::Neutral)
        .size(Size::Lg)
        .icon_size(IconSize::Lg)
        .resolve(theme);

    let r_disabled = IconTextButton::new(icon.clone(), "Disabled")
        .disabled(true)
        .resolve(theme);

    let r_loading = IconTextButton::new(icon.clone(), "Loading...")
        .loading(true)
        .resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "IconTextButton Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            IconTextButton::new(icon.clone(), "Clickable")
                .variant(Variant::Primary)
                .tone(Tone::Accent)
                .view(theme.clone(), {
                    let click_log = click_log;
                    move || click_log.set("クリック: IconText Primary/Accent".to_string())
                }),
            label(move || format!("callback log: {}", click_log.get()))
                .style(|s| s.font_size(12.0)),
            h_stack((
                btn_cell(
                    "Leading/Primary/Accent",
                    r0.font_size,
                    r0.text_color.r,
                    r0.text_color.g,
                    r0.text_color.b,
                    r0.text_alpha,
                ),
                btn_cell(
                    "Trailing/Secondary/Accent",
                    r1.font_size,
                    r1.text_color.r,
                    r1.text_color.g,
                    r1.text_color.b,
                    r1.text_alpha,
                ),
                btn_cell(
                    "Primary/Danger",
                    r2.font_size,
                    r2.text_color.r,
                    r2.text_color.g,
                    r2.text_color.b,
                    r2.text_alpha,
                ),
            ))
            .style(|s| s.gap(8.0)),
            h_stack((
                btn_cell(
                    "Ghost/Neutral/Sm",
                    r3.font_size,
                    r3.text_color.r,
                    r3.text_color.g,
                    r3.text_color.b,
                    r3.text_alpha,
                ),
                btn_cell(
                    "Primary/Neutral/Lg",
                    r4.font_size,
                    r4.text_color.r,
                    r4.text_color.g,
                    r4.text_color.b,
                    r4.text_alpha,
                ),
            ))
            .style(|s| s.gap(8.0)),
            label(|| "States").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            h_stack((
                btn_cell(
                    "Disabled",
                    r_disabled.font_size,
                    r_disabled.text_color.r,
                    r_disabled.text_color.g,
                    r_disabled.text_color.b,
                    r_disabled.text_alpha,
                ),
                btn_cell(
                    "Loading (semi-transparent)",
                    r_loading.font_size,
                    r_loading.text_color.r,
                    r_loading.text_color.g,
                    r_loading.text_color.b,
                    r_loading.text_alpha,
                ),
            ))
            .style(|s| s.gap(8.0)),
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

pub fn icon_text_button_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
