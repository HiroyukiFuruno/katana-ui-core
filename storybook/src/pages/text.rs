use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::views::{Decorators, container, h_stack, label, scroll, v_stack};
use katana_ui_core::primitive::text::{Text, TextAlign, TextRole};
use katana_ui_core::theme::Theme;
use katana_ui_core::theme::color::Color;

fn role_row(desc: &'static str, size: f32) -> impl IntoView {
    h_stack((
        label(move || desc).style(|s| s.width(100.0).font_size(11.0)),
        label(move || "The quick brown fox").style(move |s| s.font_size(size)),
    ))
    .style(|s| s.gap(8.0).items_center())
}

fn text_demo(desc: &'static str, body: impl IntoView + 'static) -> impl IntoView {
    v_stack((
        label(move || desc).style(|s| s.font_size(11.0)),
        container(body).style(|s| s.width(360.0).border(1.0).padding(6.0).margin_bottom(2.0)),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let h1 = Text::new("")
        .role(TextRole::Heading1)
        .resolve(theme)
        .font_size;
    let h2 = Text::new("")
        .role(TextRole::Heading2)
        .resolve(theme)
        .font_size;
    let h3 = Text::new("")
        .role(TextRole::Heading3)
        .resolve(theme)
        .font_size;
    let body = Text::new("").role(TextRole::Body).resolve(theme).font_size;
    let strong = Text::new("")
        .role(TextRole::BodyStrong)
        .resolve(theme)
        .font_size;
    let caption = Text::new("")
        .role(TextRole::Caption)
        .resolve(theme)
        .font_size;
    let code = Text::new("").role(TextRole::Code).resolve(theme).font_size;

    let red = Color {
        r: 220,
        g: 38,
        b: 38,
        a: 255,
    };
    let override_size = Text::new("").color_override(red).resolve(theme).font_size;

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "Text Roles").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            Text::new("Text::view sample")
                .role(TextRole::Heading2)
                .view(theme.clone()),
            role_row("Heading1", h1),
            role_row("Heading2", h2),
            role_row("Heading3", h3),
            role_row("Body", body),
            role_row("BodyStrong", strong),
            role_row("Caption", caption),
            role_row("Code", code),
            label(|| "color_override: Danger red").style(move |s| {
                s.font_size(override_size)
                    .color(PenikoColor::rgb8(220, 38, 38))
            }),
            label(|| "max_lines / align")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            text_demo(
                "max_lines=1",
                Text::new("1行目: 表示される\n2行目: 省略される\n3行目: 省略される")
                    .max_lines(1)
                    .view(theme.clone()),
            ),
            text_demo(
                "align=start",
                Text::new("左寄せ")
                    .align(TextAlign::Start)
                    .view(theme.clone()),
            ),
            text_demo(
                "align=center",
                Text::new("中央寄せ")
                    .align(TextAlign::Center)
                    .view(theme.clone()),
            ),
            text_demo(
                "align=end",
                Text::new("右寄せ")
                    .align(TextAlign::End)
                    .view(theme.clone()),
            ),
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

pub fn text_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
