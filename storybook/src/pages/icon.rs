use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{
    dyn_container, h_stack, label, scroll, svg, toggle_button, v_stack, Decorators,
};
use floem::IntoView;
use katana_ui_widget::primitive::icon::{Icon, IconSize, IconSource};
use katana_ui_widget::theme::color::Color;
use katana_ui_widget::theme::Theme;

const ICON_CHECK: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16' fill='currentColor'><path d='M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.75.75 0 1 1 1.06-1.06L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0z'/></svg>";
const ICON_ARROW: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16' fill='currentColor'><path d='M8 1a.75.75 0 0 1 .75.75v10.19l3.47-3.47a.75.75 0 1 1 1.06 1.06l-4.75 4.75a.75.75 0 0 1-1.06 0L2.72 9.53a.75.75 0 0 1 1.06-1.06L7.25 11.94V1.75A.75.75 0 0 1 8 1z'/></svg>";
const ICON_X: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16' fill='currentColor'><path d='M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.75.75 0 1 1 1.06 1.06L9.06 8l3.22 3.22a.75.75 0 1 1-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 0 1-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06z'/></svg>";

fn icon_row(name: &'static str, svg_content: String, size_px: f32, r: u8, g: u8, b: u8) -> impl IntoView {
    let color = PenikoColor::rgb8(r, g, b);
    h_stack((
        label(move || name).style(|s| s.width(120.0).font_size(11.0)),
        svg(svg_content).style(move |s| {
            s.width(size_px)
                .height(size_px)
                .color(color)
        }),
        label(move || format!("{size_px}px")).style(|s| s.font_size(11.0)),
    ))
    .style(|s| s.gap(8.0).items_center())
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let sources = [
        ("check", ICON_CHECK),
        ("arrow-down", ICON_ARROW),
        ("x-close", ICON_X),
    ];
    let sizes = [IconSize::Sm, IconSize::Md, IconSize::Lg, IconSize::Xl];
    let size_names = ["Sm", "Md", "Lg", "Xl"];

    let mut check_rows: Vec<_> = Vec::new();
    for (size, size_name) in sizes.iter().zip(size_names.iter()) {
        let resolved = Icon::new(IconSource::SvgBytes(ICON_CHECK))
            .size(*size)
            .resolve(theme);
        check_rows.push((
            format!("check / {size_name}"),
            resolved.svg_content,
            resolved.size_px,
            resolved.color_r,
            resolved.color_g,
            resolved.color_b,
        ));
    }

    let accent_resolved = Icon::new(IconSource::SvgBytes(ICON_CHECK))
        .size(IconSize::Lg)
        .color_override(Color {
            r: theme.color.accent.r,
            g: theme.color.accent.g,
            b: theme.color.accent.b,
            a: 255,
        })
        .resolve(theme);

    let danger_resolved = Icon::new(IconSource::SvgBytes(ICON_X))
        .size(IconSize::Lg)
        .color_override(Color {
            r: theme.color.danger.r,
            g: theme.color.danger.g,
            b: theme.color.danger.b,
            a: 255,
        })
        .resolve(theme);

    let _ = sources;

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let (ar, ag, ab) = (accent_resolved.color_r, accent_resolved.color_g, accent_resolved.color_b);
    let (dr, dg, db) = (danger_resolved.color_r, danger_resolved.color_g, danger_resolved.color_b);
    let acc_svg = accent_resolved.svg_content;
    let acc_size = accent_resolved.size_px;
    let dng_svg = danger_resolved.svg_content;
    let dng_size = danger_resolved.size_px;

    let (r0, g0, b0, svg0, sz0) = (check_rows[0].3, check_rows[0].4, check_rows[0].5, check_rows[0].1.clone(), check_rows[0].2);
    let (r1, g1, b1, svg1, sz1) = (check_rows[1].3, check_rows[1].4, check_rows[1].5, check_rows[1].1.clone(), check_rows[1].2);
    let (r2, g2, b2, svg2, sz2) = (check_rows[2].3, check_rows[2].4, check_rows[2].5, check_rows[2].1.clone(), check_rows[2].2);
    let (r3, g3, b3, svg3, sz3) = (check_rows[3].3, check_rows[3].4, check_rows[3].5, check_rows[3].1.clone(), check_rows[3].2);

    scroll(
        v_stack((
            label(|| "Icon Sizes (check icon)").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            icon_row("check / Sm", svg0, sz0, r0, g0, b0),
            icon_row("check / Md", svg1, sz1, r1, g1, b1),
            icon_row("check / Lg", svg2, sz2, r2, g2, b2),
            icon_row("check / Xl", svg3, sz3, r3, g3, b3),
            label(|| "Color overrides").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            icon_row("check / accent", acc_svg, acc_size, ar, ag, ab),
            icon_row("x-close / danger", dng_svg, dng_size, dr, dg, db),
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

pub fn icon_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "Icon Primitive").style(|s| s.font_size(20.0)),
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
