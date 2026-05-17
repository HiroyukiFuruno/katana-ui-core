use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::views::{Decorators, container, empty, h_stack, label, scroll, v_stack};
use katana_ui_core::theme::{ColorTokens, SpacingTokens, Theme, TypographyTokens};

fn token_color(r: u8, g: u8, b: u8) -> PenikoColor {
    PenikoColor::rgb8(r, g, b)
}

fn color_swatch(name: &'static str, r: u8, g: u8, b: u8) -> impl IntoView {
    let col = token_color(r, g, b);
    h_stack((
        container(empty()).style(move |s| s.width(32.0).height(32.0).background(col)),
        label(move || name),
    ))
    .style(|s| s.gap(8.0).items_center())
}

fn color_section(c: ColorTokens) -> impl IntoView {
    v_stack((
        label(|| "Color Tokens").style(|s| s.font_size(16.0).margin_bottom(8.0)),
        color_swatch("bg", c.bg.r, c.bg.g, c.bg.b),
        color_swatch("surface", c.surface.r, c.surface.g, c.surface.b),
        color_swatch("border", c.border.r, c.border.g, c.border.b),
        color_swatch("text", c.text.r, c.text.g, c.text.b),
        color_swatch("text_muted", c.text_muted.r, c.text_muted.g, c.text_muted.b),
        color_swatch(
            "text_disabled",
            c.text_disabled.r,
            c.text_disabled.g,
            c.text_disabled.b,
        ),
        color_swatch("accent", c.accent.r, c.accent.g, c.accent.b),
        color_swatch(
            "accent_muted",
            c.accent_muted.r,
            c.accent_muted.g,
            c.accent_muted.b,
        ),
        color_swatch("danger", c.danger.r, c.danger.g, c.danger.b),
        color_swatch("warning", c.warning.r, c.warning.g, c.warning.b),
        color_swatch("success", c.success.r, c.success.g, c.success.b),
    ))
    .style(|s| s.gap(6.0))
}

fn spacing_section(s: SpacingTokens) -> impl IntoView {
    v_stack((
        label(|| "Spacing Scale").style(|s| s.font_size(16.0).margin_bottom(8.0)),
        spacing_row("xxs", s.xxs),
        spacing_row("xs", s.xs),
        spacing_row("sm", s.sm),
        spacing_row("md", s.md),
        spacing_row("lg", s.lg),
        spacing_row("xl", s.xl),
        spacing_row("xxl", s.xxl),
    ))
    .style(|s| s.gap(6.0))
}

fn spacing_row(name: &'static str, px: f32) -> impl IntoView {
    h_stack((
        label(move || name).style(|s| s.width(40.0)),
        container(empty()).style(move |s| s.width(px).height(16.0).background(PenikoColor::BLUE)),
        label(move || format!("{px}px")),
    ))
    .style(|s| s.gap(8.0).items_center())
}

fn typography_section(t: TypographyTokens) -> impl IntoView {
    v_stack((
        label(|| "Typography").style(|s| s.font_size(16.0).margin_bottom(8.0)),
        label(|| "heading_1: The quick brown fox")
            .style(move |s| s.font_size(t.heading_1.font_size)),
        label(|| "heading_2: The quick brown fox")
            .style(move |s| s.font_size(t.heading_2.font_size)),
        label(|| "heading_3: The quick brown fox")
            .style(move |s| s.font_size(t.heading_3.font_size)),
        label(|| "body: The quick brown fox").style(move |s| s.font_size(t.body.font_size)),
        label(|| "body_strong: The quick brown fox")
            .style(move |s| s.font_size(t.body_strong.font_size)),
        label(|| "caption: The quick brown fox").style(move |s| s.font_size(t.caption.font_size)),
        label(|| "code: fn main() { println!(\"hello\"); }")
            .style(move |s| s.font_size(t.code.font_size)),
    ))
    .style(|s| s.gap(8.0))
}

fn theme_content(theme: Theme) -> impl IntoView {
    let bg = token_color(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = token_color(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let color = theme.color.clone();
    scroll(
        h_stack((
            color_section(color),
            spacing_section(theme.spacing),
            typography_section(theme.typography),
        ))
        .style(move |s| {
            s.gap(32.0)
                .padding(16.0)
                .background(bg)
                .color(text)
                .min_width_full()
        }),
    )
}

pub fn theme_tokens_page(theme: Theme) -> impl IntoView {
    theme_content(theme)
}
