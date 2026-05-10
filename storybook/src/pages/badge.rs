use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::indicator::badge::{Badge, BadgeSize, BadgeTone, BadgeVariant};
use katana_ui_widget::theme::Theme;

fn badge_cell(
    lbl: &'static str,
    text_r: u8,
    text_g: u8,
    text_b: u8,
    bg_r: u8,
    bg_g: u8,
    bg_b: u8,
    bg_a: u8,
    font_sz: f32,
) -> impl IntoView {
    let text_color = PenikoColor::rgba8(text_r, text_g, text_b, 255);
    let bg_color = PenikoColor::rgba8(bg_r, bg_g, bg_b, bg_a);
    label(move || lbl).style(move |s| {
        s.background(bg_color)
            .color(text_color)
            .font_size(font_sz)
            .padding(4.0)
            .border_radius(4.0)
    })
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let tones = [
        (BadgeTone::Neutral, "Neutral"),
        (BadgeTone::Accent, "Accent"),
        (BadgeTone::Danger, "Danger"),
        (BadgeTone::Warning, "Warning"),
        (BadgeTone::Success, "Success"),
        (BadgeTone::Info, "Info"),
    ];

    let variants = [
        (BadgeVariant::Solid, "Solid"),
        (BadgeVariant::Subtle, "Subtle"),
        (BadgeVariant::Outline, "Outline"),
    ];

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let mut rows: Vec<Box<dyn floem::IntoView>> = vec![
        Box::new(label(|| "Badge Samples").style(|s| s.font_size(16.0).margin_bottom(8.0))),
    ];

    for (variant, vname) in variants {
        let cells: Vec<_> = tones.iter().map(|(tone, tname)| {
            let r = Badge::new(*tname).tone(*tone).variant(variant).resolve(theme);
            let lbl: &'static str = Box::leak(format!("{vname}/{tname}").into_boxed_str());
            let bg_color = r.bg_color.unwrap_or(theme.color.bg);
            badge_cell(lbl, r.text_color.r, r.text_color.g, r.text_color.b, bg_color.r, bg_color.g, bg_color.b, bg_color.a, r.font_size)
        }).collect();
        let vname_static: &'static str = Box::leak(vname.to_string().into_boxed_str());
        rows.push(Box::new(
            v_stack((
                label(move || vname_static).style(|s| s.font_size(12.0).margin_bottom(2.0)),
                h_stack(cells).style(|s| s.gap(4.0).flex_wrap(true)),
            )).style(|s| s.gap(4.0))
        ));
    }

    let r_sm = Badge::new("Sm badge").size(BadgeSize::Sm).tone(BadgeTone::Accent).resolve(theme);
    let r_md = Badge::new("Md badge").size(BadgeSize::Md).tone(BadgeTone::Accent).resolve(theme);
    let sm_bg = r_sm.bg_color.unwrap_or(theme.color.bg);
    let md_bg = r_md.bg_color.unwrap_or(theme.color.bg);

    rows.push(Box::new(
        v_stack((
            label(|| "Sizes").style(|s| s.font_size(12.0).margin_bottom(2.0)),
            h_stack((
                badge_cell("Sm", r_sm.text_color.r, r_sm.text_color.g, r_sm.text_color.b, sm_bg.r, sm_bg.g, sm_bg.b, sm_bg.a, r_sm.font_size),
                badge_cell("Md", r_md.text_color.r, r_md.text_color.g, r_md.text_color.b, md_bg.r, md_bg.g, md_bg.b, md_bg.a, r_md.font_size),
            )).style(|s| s.gap(4.0)),
        )).style(|s| s.gap(4.0))
    ));

    scroll(
        v_stack(rows).style(move |s| {
            s.gap(12.0)
                .padding(16.0)
                .background(bg)
                .color(text_col)
                .min_width_full()
        }),
    )
}

pub fn badge_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "Badge").style(|s| s.font_size(20.0)),
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
