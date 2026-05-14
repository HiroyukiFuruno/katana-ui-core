use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, label, scroll, v_stack};
use katana_ui_widget::composite::selector::toggle::{Toggle, ToggleSize};
use katana_ui_widget::theme::Theme;

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let live_value = create_rw_signal(false);
    let log = create_rw_signal("on_change: なし".to_string());

    crate::interaction::replay("toggle-value", "toggle", "value-true", {
        let live_value = live_value;
        let log = log;
        move || {
            live_value.set(true);
            log.set("on_change: true".to_string());
        }
    });

    scroll(
        v_stack((
            label(|| "Toggle Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            Toggle::new("Live toggle")
                .value(live_value.get())
                .on_change({
                    let live_value = live_value;
                    let log = log;
                    move |value| {
                        live_value.set(value);
                        log.set(format!("on_change: {value}"));
                    }
                })
                .view(theme.clone()),
            label(move || format!("callback log: {}", log.get())).style(|s| s.font_size(12.0)),
            label(|| "Readonly display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            Toggle::new("Off").disabled(true).view(theme.clone()),
            Toggle::new("On")
                .value(true)
                .disabled(true)
                .view(theme.clone()),
            label(|| "Disabled display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            Toggle::new("Disabled off")
                .disabled(true)
                .view(theme.clone()),
            Toggle::new("Disabled on")
                .disabled(true)
                .value(true)
                .view(theme.clone()),
            label(|| "Size display")
                .style(|s| s.font_size(16.0).margin_top(12.0).margin_bottom(8.0)),
            Toggle::new("Small")
                .size(ToggleSize::Sm)
                .disabled(true)
                .view(theme.clone()),
            Toggle::new("Large")
                .size(ToggleSize::Lg)
                .disabled(true)
                .value(true)
                .view(theme.clone()),
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

pub fn toggle_page(theme: Theme) -> impl IntoView {
    page_content(&theme)
}
