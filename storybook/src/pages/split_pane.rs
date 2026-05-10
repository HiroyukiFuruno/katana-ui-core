use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::layout::split::{Direction, SplitPane};
use katana_ui_widget::theme::Theme;

fn split_row(
    heading: &'static str,
    direction_tag: &'static str,
    ratio: f32,
    handle_r: u8,
    handle_g: u8,
    handle_b: u8,
    thickness: f32,
) -> impl IntoView {
    let handle_color = PenikoColor::rgb8(handle_r, handle_g, handle_b);
    let ratio_pct: &'static str = Box::leak(format!("{:.0}%", ratio * 100.0).into_boxed_str());
    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        h_stack((
            label(move || direction_tag).style(|s| s.font_size(10.0).margin_right(4.0)),
            label(move || "[Pane A]").style(move |s| s.padding(4.0).border(0.5).min_width(60.0)),
            label(move || "").style(move |s| s.width(thickness).height(20.0).background(handle_color)),
            label(move || "[Pane B]").style(move |s| s.padding(4.0).border(0.5).min_width(60.0)),
            label(move || ratio_pct).style(|s| s.font_size(10.0).margin_left(4.0)),
        ))
        .style(|s| s.items_center().gap(2.0)),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let r_h = SplitPane::new().direction(Direction::Horizontal).resolve(theme);
    let r_v = SplitPane::new().direction(Direction::Vertical).resolve(theme);
    let r_60 = SplitPane::new().ratio(0.6).resolve(theme);
    let r_min = SplitPane::new().ratio(0.05).min_ratio(0.15).resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "SplitPane Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            split_row("Horizontal 50/50", "[H]", r_h.ratio, r_h.handle_color.r, r_h.handle_color.g, r_h.handle_color.b, r_h.handle_thickness),
            split_row("Vertical 50/50", "[V]", r_v.ratio, r_v.handle_color.r, r_v.handle_color.g, r_v.handle_color.b, r_v.handle_thickness),
            split_row("60/40", "[H]", r_60.ratio, r_60.handle_color.r, r_60.handle_color.g, r_60.handle_color.b, r_60.handle_thickness),
            split_row("Min constraint (5% clamped to 15%)", "[H]", r_min.ratio, r_min.handle_color.r, r_min.handle_color.g, r_min.handle_color.b, r_min.handle_thickness),
            label(|| "Double-click handle resets to 50/50").style(|s| s.font_size(11.0).margin_top(8.0)),
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

pub fn split_pane_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "SplitPane").style(|s| s.font_size(20.0)),
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
