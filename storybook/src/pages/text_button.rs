use floem::peniko::Color as PenikoColor;
use floem::views::{h_stack, label, scroll, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::button::text::{Size, TextButton, Tone, Variant};
use katana_ui_widget::theme::Theme;

fn btn_cell(lbl: &'static str, font_sz: f32, r: u8, g: u8, b: u8, a: u8) -> impl IntoView {
    let color = PenikoColor::rgba8(r, g, b, a);
    label(move || lbl).style(move |s| s.font_size(font_sz).color(color).padding(4.0))
}

fn resolve(theme: &Theme, variant: Variant, tone: Tone, size: Size) -> (f32, u8, u8, u8, u8) {
    let r = TextButton::new("Button")
        .variant(variant)
        .tone(tone)
        .size(size)
        .resolve(theme);
    (r.font_size, r.text_color.r, r.text_color.g, r.text_color.b, r.text_alpha)
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let (fs0, r0, g0, b0, a0) = resolve(theme, Variant::Primary, Tone::Accent, Size::Md);
    let (fs1, r1, g1, b1, a1) = resolve(theme, Variant::Primary, Tone::Danger, Size::Md);
    let (fs2, r2, g2, b2, a2) = resolve(theme, Variant::Secondary, Tone::Accent, Size::Md);
    let (fs3, r3, g3, b3, a3) = resolve(theme, Variant::Ghost, Tone::Neutral, Size::Sm);
    let (fs4, r4, g4, b4, a4) = resolve(theme, Variant::Link, Tone::Accent, Size::Sm);
    let (fs5, r5, g5, b5, a5) = resolve(theme, Variant::Primary, Tone::Neutral, Size::Lg);
    let (fs6, r6, g6, b6, a6) = resolve(theme, Variant::Secondary, Tone::Success, Size::Md);

    let disabled_r = TextButton::new("Disabled").disabled(true).resolve(theme);
    let loading_r = TextButton::new("Loading...").loading(true).resolve(theme);

    let (dr, dg, db, da, d_size) = (
        disabled_r.text_color.r,
        disabled_r.text_color.g,
        disabled_r.text_color.b,
        disabled_r.text_alpha,
        disabled_r.font_size,
    );
    let (lr, lg, lb, la, l_size) = (
        loading_r.text_color.r,
        loading_r.text_color.g,
        loading_r.text_color.b,
        loading_r.text_alpha,
        loading_r.font_size,
    );

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "TextButton Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            TextButton::new("Clickable")
                .variant(Variant::Primary)
                .tone(Tone::Accent)
                .view(theme.clone(), || {}),
            h_stack((
                btn_cell("Primary/Accent/Md", fs0, r0, g0, b0, a0),
                btn_cell("Primary/Danger/Md", fs1, r1, g1, b1, a1),
                btn_cell("Secondary/Accent/Md", fs2, r2, g2, b2, a2),
            ))
            .style(|s| s.gap(8.0)),
            h_stack((
                btn_cell("Ghost/Neutral/Sm", fs3, r3, g3, b3, a3),
                btn_cell("Link/Accent/Sm", fs4, r4, g4, b4, a4),
                btn_cell("Primary/Neutral/Lg", fs5, r5, g5, b5, a5),
                btn_cell("Secondary/Success/Md", fs6, r6, g6, b6, a6),
            ))
            .style(|s| s.gap(8.0)),
            label(|| "States").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            h_stack((
                btn_cell("Disabled", d_size, dr, dg, db, da),
                btn_cell("Loading (semi-transparent)", l_size, lr, lg, lb, la),
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

pub fn text_button_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
