use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::layout::accordion::{Accordion, IndicatorPosition};
use katana_ui_widget::theme::Theme;

fn accordion_row(
    heading: &'static str,
    header_text: &'static str,
    chevron: Option<&'static str>,
    indicator: IndicatorPosition,
    expanded: bool,
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
    let body_tag: &'static str = if expanded { "[content visible]" } else { "[collapsed]" };
    let indicator_tag: &'static str = match indicator {
        IndicatorPosition::Leading => "[leading ▶]",
        IndicatorPosition::Trailing => "[trailing ▶]",
        IndicatorPosition::None => "[no chevron]",
    };

    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        h_stack((
            label(move || chevron.unwrap_or("")).style(|s| s.min_width(16.0)),
            label(move || header_text)
                .style(move |s| s.background(bg).color(text_color).font_size(font_sz).padding(6.0).flex_grow(1.0)),
            label(move || indicator_tag).style(|s| s.font_size(10.0).margin_left(4.0)),
        ))
        .style(|s| s.items_center().gap(4.0)),
        label(move || body_tag).style(|s| s.font_size(11.0).padding_left(16.0)),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let r_collapsed = Accordion::new("Section A").resolve(theme);
    let r_expanded = Accordion::new("Section B").expanded(true).resolve(theme);
    let r_leading = Accordion::new("Leading chevron").indicator(IndicatorPosition::Leading).expanded(true).resolve(theme);
    let r_none = Accordion::new("No chevron").indicator(IndicatorPosition::None).resolve(theme);
    let r_disabled = Accordion::new("Disabled section").disabled(true).resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let row = |heading: &'static str, r: katana_ui_widget::layout::accordion::ResolvedAccordion| {
        let h: &'static str = Box::leak(r.header.clone().into_boxed_str());
        accordion_row(
            heading, h, r.chevron, r.indicator, r.expanded,
            r.header_bg.r, r.header_bg.g, r.header_bg.b,
            r.header_text.r, r.header_text.g, r.header_text.b,
            r.header_font_size,
        )
    };

    scroll(
        v_stack((
            label(|| "Accordion Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            row("Collapsed (default)", r_collapsed),
            row("Expanded", r_expanded),
            row("Leading indicator", r_leading),
            row("No indicator", r_none),
            label(|| "States").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            row("Disabled", r_disabled),
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

pub fn accordion_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "Accordion").style(|s| s.font_size(20.0)),
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
