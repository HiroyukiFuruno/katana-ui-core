use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::input::search::SearchBox;
use katana_ui_widget::composite::input::text::{InputSize, TrailingSlot};
use katana_ui_widget::theme::Theme;

fn search_row(
    heading: &'static str,
    display: &'static str,
    trailing_tag: &'static str,
    r: u8,
    g: u8,
    b: u8,
    border_r: u8,
    border_g: u8,
    border_b: u8,
    font_sz: f32,
) -> impl IntoView {
    let text_color = PenikoColor::rgb8(r, g, b);
    let border_color = PenikoColor::rgb8(border_r, border_g, border_b);
    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        h_stack((
            label(move || "[🔍]").style(|s| s.font_size(12.0).margin_right(4.0)),
            label(move || display).style(move |s| {
                s.font_size(font_sz)
                    .color(text_color)
                    .border(1.0)
                    .border_color(border_color)
                    .padding(6.0)
                    .min_width(180.0)
            }),
            label(move || trailing_tag).style(|s| s.font_size(12.0).margin_left(4.0)),
        ))
        .style(|s| s.items_center()),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let r_empty = SearchBox::new("Search").resolve(theme);
    let r_query = SearchBox::new("Search").value("floem widgets").resolve(theme);
    let r_sm = SearchBox::new("Small search").size(InputSize::Sm).value("small").resolve(theme);
    let r_lg = SearchBox::new("Large search").size(InputSize::Lg).resolve(theme);
    let r_disabled = SearchBox::new("Disabled search").disabled(true).value("disabled").resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let trailing_tag = |r: &katana_ui_widget::composite::input::text::ResolvedTextInput| {
        match &r.trailing {
            TrailingSlot::ClearButton => "[×]",
            TrailingSlot::None => "",
            _ => "",
        }
    };

    let ph_or_val = |r: &katana_ui_widget::composite::input::text::ResolvedTextInput| -> &'static str {
        if r.value.is_empty() {
            Box::leak(r.placeholder.clone().unwrap_or_default().into_boxed_str())
        } else {
            Box::leak(r.value.clone().into_boxed_str())
        }
    };

    scroll(
        v_stack((
            label(|| "SearchBox Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            search_row("Empty (placeholder)", ph_or_val(&r_empty), trailing_tag(&r_empty), r_empty.text_color.r, r_empty.text_color.g, r_empty.text_color.b, r_empty.border_color.r, r_empty.border_color.g, r_empty.border_color.b, r_empty.font_size),
            search_row("With query (shows clear ×)", ph_or_val(&r_query), trailing_tag(&r_query), r_query.text_color.r, r_query.text_color.g, r_query.text_color.b, r_query.border_color.r, r_query.border_color.g, r_query.border_color.b, r_query.font_size),
            search_row("Size Sm", ph_or_val(&r_sm), trailing_tag(&r_sm), r_sm.text_color.r, r_sm.text_color.g, r_sm.text_color.b, r_sm.border_color.r, r_sm.border_color.g, r_sm.border_color.b, r_sm.font_size),
            search_row("Size Lg", ph_or_val(&r_lg), trailing_tag(&r_lg), r_lg.text_color.r, r_lg.text_color.g, r_lg.text_color.b, r_lg.border_color.r, r_lg.border_color.g, r_lg.border_color.b, r_lg.font_size),
            label(|| "States").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            search_row("Disabled", ph_or_val(&r_disabled), trailing_tag(&r_disabled), r_disabled.text_color.r, r_disabled.text_color.g, r_disabled.text_color.b, r_disabled.border_color.r, r_disabled.border_color.g, r_disabled.border_color.b, r_disabled.font_size),
            label(|| "Key behavior: Esc clears, Enter submits (resolved by caller)").style(|s| s.font_size(11.0).margin_top(8.0)),
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

pub fn search_box_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "SearchBox").style(|s| s.font_size(20.0)),
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
