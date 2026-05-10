use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::indicator::tooltip::{Placement, Tooltip};
use katana_ui_widget::theme::Theme;

fn tooltip_row(
    heading: &'static str,
    lbl: &'static str,
    placement_tag: &'static str,
    bg_r: u8,
    bg_g: u8,
    bg_b: u8,
    text_r: u8,
    text_g: u8,
    text_b: u8,
    font_sz: f32,
) -> impl IntoView {
    let bg = PenikoColor::rgb8(bg_r, bg_g, bg_b);
    let text_color = PenikoColor::rgb8(text_r, text_g, text_b);
    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        h_stack((
            label(move || placement_tag).style(|s| s.font_size(10.0).margin_right(4.0)),
            label(move || lbl)
                .style(move |s| s.background(bg).color(text_color).font_size(font_sz).padding(4.0)),
        ))
        .style(|s| s.items_center()),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let r_top = Tooltip::new("Top tooltip").placement(Placement::Top).resolve(theme);
    let r_bottom = Tooltip::new("Bottom tooltip").placement(Placement::Bottom).resolve(theme);
    let r_start = Tooltip::new("Start (left) tooltip").placement(Placement::Start).resolve(theme);
    let r_end = Tooltip::new("End (right) tooltip").placement(Placement::End).resolve(theme);
    let r_long = Tooltip::new("This is a longer tooltip text that may need wrapping at max width").resolve(theme);
    let r_fast = Tooltip::new("Fast tooltip").delay_ms(0).resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let placement_str = |p: Placement| match p {
        Placement::Top => "[▲ Top]",
        Placement::Bottom => "[▼ Bottom]",
        Placement::Start => "[◀ Start]",
        Placement::End => "[▶ End]",
    };

    let to_row = |heading: &'static str, r: katana_ui_widget::composite::indicator::tooltip::ResolvedTooltip| {
        let lbl: &'static str = Box::leak(r.label.clone().into_boxed_str());
        let ptag = placement_str(r.placement);
        tooltip_row(heading, lbl, ptag, r.bg_color.r, r.bg_color.g, r.bg_color.b, r.text_color.r, r.text_color.g, r.text_color.b, r.font_size)
    };

    scroll(
        v_stack((
            label(|| "Tooltip Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            to_row("Top placement", r_top),
            to_row("Bottom placement", r_bottom),
            to_row("Start placement", r_start),
            to_row("End placement", r_end),
            to_row("Long text (max_width wrap)", r_long),
            to_row("Fast (delay=0ms)", r_fast),
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

pub fn tooltip_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "Tooltip").style(|s| s.font_size(20.0)),
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
