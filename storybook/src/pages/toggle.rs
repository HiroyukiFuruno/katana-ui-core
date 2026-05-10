use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::composite::selector::toggle::{Toggle, ToggleSize};
use katana_ui_widget::theme::Theme;

fn toggle_row(
    lbl: &'static str,
    track_w: f32,
    track_h: f32,
    tr: u8,
    tg: u8,
    tb: u8,
    value: bool,
) -> impl IntoView {
    let track_color = PenikoColor::rgb8(tr, tg, tb);
    let state = if value { "ON" } else { "OFF" };
    h_stack((
        label(move || lbl).style(|s| s.min_width(200.0)),
        label(move || state).style(move |s| {
            s.width(track_w)
                .height(track_h)
                .background(track_color)
                .padding(2.0)
                .font_size(10.0)
        }),
    ))
    .style(|s| s.gap(8.0).items_center())
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let r_off = Toggle::new("Off toggle").value(false).resolve(theme);
    let r_on = Toggle::new("On toggle").value(true).resolve(theme);
    let r_sm = Toggle::new("Small toggle").size(ToggleSize::Sm).value(true).resolve(theme);
    let r_lg = Toggle::new("Large toggle").size(ToggleSize::Lg).value(true).resolve(theme);
    let r_dis = Toggle::new("Disabled toggle").disabled(true).resolve(theme);
    let r_dis_on = Toggle::new("Disabled on").disabled(true).value(true).resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "Toggle Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            toggle_row(
                "Off (default)",
                r_off.track_width,
                r_off.track_height,
                r_off.track_color.r,
                r_off.track_color.g,
                r_off.track_color.b,
                r_off.value,
            ),
            toggle_row(
                "On",
                r_on.track_width,
                r_on.track_height,
                r_on.track_color.r,
                r_on.track_color.g,
                r_on.track_color.b,
                r_on.value,
            ),
            toggle_row(
                "Small / On",
                r_sm.track_width,
                r_sm.track_height,
                r_sm.track_color.r,
                r_sm.track_color.g,
                r_sm.track_color.b,
                r_sm.value,
            ),
            toggle_row(
                "Large / On",
                r_lg.track_width,
                r_lg.track_height,
                r_lg.track_color.r,
                r_lg.track_color.g,
                r_lg.track_color.b,
                r_lg.value,
            ),
            label(|| "States").style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            toggle_row(
                "Disabled / Off",
                r_dis.track_width,
                r_dis.track_height,
                r_dis.track_color.r,
                r_dis.track_color.g,
                r_dis.track_color.b,
                r_dis.value,
            ),
            toggle_row(
                "Disabled / On",
                r_dis_on.track_width,
                r_dis_on.track_height,
                r_dis_on.track_color.r,
                r_dis_on.track_color.g,
                r_dis_on.track_color.b,
                r_dis_on.value,
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

pub fn toggle_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "Toggle").style(|s| s.font_size(20.0)),
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
