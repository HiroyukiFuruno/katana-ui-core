use floem::peniko::Color as PenikoColor;
use floem::views::{h_stack, label, scroll, svg, v_stack, Decorators,
};
use floem::IntoView;
use katana_ui_widget::composite::button::svg::{SvgButton, Tone, Variant};
use katana_ui_widget::primitive::icon::{IconSize, IconSource};
use katana_ui_widget::theme::Theme;

const ICON_CHECK: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16' fill='currentColor'><path d='M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.75.75 0 1 1 1.06-1.06L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0z'/></svg>";

fn button_cell(desc: String, svg_str: String, size_px: f32, r: u8, g: u8, b: u8) -> impl IntoView {
    v_stack((
        svg(svg_str).style(move |s| {
            s.width(size_px)
                .height(size_px)
                .color(PenikoColor::rgb8(r, g, b))
        }),
        label(move || desc.clone()).style(|s| s.font_size(9.0)),
    ))
    .style(|s| s.gap(4.0).items_center().padding(8.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let variants = [
        ("Plain", Variant::Plain),
        ("Subtle", Variant::Subtle),
        ("Filled", Variant::Filled),
    ];
    let tones = [
        ("Neutral", Tone::Neutral),
        ("Accent", Tone::Accent),
        ("Danger", Tone::Danger),
    ];

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let mut rows: Vec<[(String, String, f32, u8, u8, u8); 3]> = Vec::new();
    for (vname, variant) in variants {
        let mut row = [
            (vname.to_string(), String::new(), 0.0f32, 0u8, 0u8, 0u8),
            (vname.to_string(), String::new(), 0.0f32, 0u8, 0u8, 0u8),
            (vname.to_string(), String::new(), 0.0f32, 0u8, 0u8, 0u8),
        ];
        for (i, (tname, tone)) in tones.iter().enumerate() {
            let r = SvgButton::new(IconSource::SvgBytes(ICON_CHECK), "check")
                .variant(variant)
                .tone(*tone)
                .resolve(theme);
            let icon_svg = katana_ui_widget::primitive::icon::Icon::new(r.icon_source)
                .size(IconSize::Lg)
                .resolve(theme);
            let desc = format!("{vname}/{tname}");
            row[i] = (desc, icon_svg.svg_content, r.size_px, r.icon_color.r, r.icon_color.g, r.icon_color.b);
        }
        rows.push(row);
    }

    let disabled_r = SvgButton::new(IconSource::SvgBytes(ICON_CHECK), "check disabled")
        .disabled(true)
        .resolve(theme);
    let disabled_icon = katana_ui_widget::primitive::icon::Icon::new(disabled_r.icon_source)
        .size(IconSize::Lg)
        .resolve(theme);

    let (dr, dg, db) = (disabled_r.icon_color.r, disabled_r.icon_color.g, disabled_r.icon_color.b);
    let dis_svg = disabled_icon.svg_content;
    let dis_size = disabled_r.size_px;

    let [r0, r1, r2] = [rows[0].clone(), rows[1].clone(), rows[2].clone()];

    scroll(
        v_stack((
            label(|| "SvgButton — Variant × Tone grid").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            SvgButton::new(IconSource::SvgBytes(ICON_CHECK), "live check")
                .variant(Variant::Filled)
                .tone(Tone::Accent)
                .view(theme.clone(), || {}),
            h_stack((
                button_cell(r0[0].0.clone(), r0[0].1.clone(), r0[0].2, r0[0].3, r0[0].4, r0[0].5),
                button_cell(r0[1].0.clone(), r0[1].1.clone(), r0[1].2, r0[1].3, r0[1].4, r0[1].5),
                button_cell(r0[2].0.clone(), r0[2].1.clone(), r0[2].2, r0[2].3, r0[2].4, r0[2].5),
            )).style(|s| s.gap(4.0)),
            h_stack((
                button_cell(r1[0].0.clone(), r1[0].1.clone(), r1[0].2, r1[0].3, r1[0].4, r1[0].5),
                button_cell(r1[1].0.clone(), r1[1].1.clone(), r1[1].2, r1[1].3, r1[1].4, r1[1].5),
                button_cell(r1[2].0.clone(), r1[2].1.clone(), r1[2].2, r1[2].3, r1[2].4, r1[2].5),
            )).style(|s| s.gap(4.0)),
            h_stack((
                button_cell(r2[0].0.clone(), r2[0].1.clone(), r2[0].2, r2[0].3, r2[0].4, r2[0].5),
                button_cell(r2[1].0.clone(), r2[1].1.clone(), r2[1].2, r2[1].3, r2[1].4, r2[1].5),
                button_cell(r2[2].0.clone(), r2[2].1.clone(), r2[2].2, r2[2].3, r2[2].4, r2[2].5),
            )).style(|s| s.gap(4.0)),
            label(|| "Disabled state").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            button_cell("Disabled".to_string(), dis_svg, dis_size, dr, dg, db),
        ))
        .style(move |s| {
            s.gap(4.0)
                .padding(16.0)
                .background(bg)
                .color(text_col)
                .min_width_full()
        }),
    )
}

pub fn svg_button_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
