use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, dyn_container, label, scroll, v_stack};
use katana_ui_core::composite::slide_control::{SlideControl, SlideValueFormat};
use katana_ui_core::theme::Theme;

fn decimal_row(theme: Theme) -> impl IntoView {
    let gamma = create_rw_signal(0.0);
    let gamma_theme = theme.clone();
    let gamma_view = dyn_container(
        move || gamma.try_get_untracked().unwrap_or(0.0),
        move |value| {
            SlideControl::new("ガンマ")
                .value(value)
                .min(-1.0)
                .max(1.0)
                .step(0.05)
                .unit("γ")
                .format(SlideValueFormat::Decimal(2))
                .on_change({
                    let gamma = gamma.clone();
                    move |value| gamma.set(value)
                })
                .view(gamma_theme.clone())
        },
    );

    let gamma_label = label(move || {
        let current = gamma.try_get().unwrap_or_default();
        format!("現在の値: {:.2} γ", current)
    })
    .style(|s| s.margin_left(2.0).font_size(12.0));

    v_stack((gamma_view, gamma_label)).style(|s| s.gap(6.0))
}

fn integer_row(theme: Theme) -> impl IntoView {
    let contrast = create_rw_signal(40.0);
    let contrast_theme = theme.clone();
    let contrast_view = dyn_container(
        move || contrast.try_get_untracked().unwrap_or(40.0),
        move |value| {
            SlideControl::new("コントラスト")
                .value(value)
                .min(0.0)
                .max(100.0)
                .step(1.0)
                .unit("%")
                .integer()
                .on_change({
                    let contrast = contrast.clone();
                    move |value| contrast.set(value)
                })
                .view(contrast_theme.clone())
        },
    );

    let contrast_label = label(move || {
        let current = contrast.try_get().unwrap_or_default();
        format!("現在の値: {:.0}%", current)
    })
    .style(|s| s.margin_left(2.0).font_size(12.0));

    v_stack((contrast_view, contrast_label)).style(|s| s.gap(6.0))
}

fn custom_format_row(theme: Theme) -> impl IntoView {
    let opacity = create_rw_signal(0.42);
    let opacity_theme = theme.clone();
    let opacity_view = dyn_container(
        move || opacity.try_get_untracked().unwrap_or(0.42),
        move |value| {
            SlideControl::new("透明度")
                .value(value)
                .min(0.0)
                .max(1.0)
                .step(0.01)
                .custom_format(|value| format!("{:.0} / 255", value * 255.0))
                .on_change({
                    let opacity = opacity.clone();
                    move |value| opacity.set(value)
                })
                .view(opacity_theme.clone())
        },
    );

    let opacity_label = label(move || {
        let current = opacity.try_get().unwrap_or_default();
        format!("custom_format: {:.0} / 255", current * 255.0)
    })
    .style(|s| s.margin_left(2.0).font_size(12.0));

    v_stack((opacity_view, opacity_label)).style(|s| s.gap(6.0))
}

fn state_rows(theme: Theme) -> impl IntoView {
    let state_log = create_rw_signal("disabled / readonly は操作不可".to_string());
    v_stack((
        SlideControl::new("disabled")
            .value(30.0)
            .min(0.0)
            .max(100.0)
            .step(1.0)
            .unit("%")
            .disabled(true)
            .on_change({
                let state_log = state_log;
                move |value| state_log.set(format!("disabled changed: {value}"))
            })
            .view(theme.clone()),
        SlideControl::new("readonly")
            .value(70.0)
            .min(0.0)
            .max(100.0)
            .step(1.0)
            .unit("%")
            .readonly(true)
            .on_change({
                let state_log = state_log;
                move |value| state_log.set(format!("readonly changed: {value}"))
            })
            .view(theme),
        label(move || format!("callback log: {}", state_log.get())).style(|s| s.font_size(12.0)),
    ))
    .style(|s| s.gap(8.0))
}

pub fn slide_control_page(theme: Theme) -> impl IntoView {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_color = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let title = label(|| "SlideControl Samples").style(|s| s.font_size(16.0).margin_bottom(8.0));
    let subtitle = label(|| "Slider と数値入力を同期させ、単位・フォーマットを切り替えます。")
        .style(|s| s.font_size(13.0));

    scroll(
        v_stack((
            title,
            subtitle,
            integer_row(theme.clone()),
            decimal_row(theme.clone()),
            custom_format_row(theme.clone()),
            label(|| "disabled / readonly").style(|s| s.font_size(14.0).margin_top(8.0)),
            state_rows(theme),
        ))
        .style(move |s| {
            s.gap(14.0)
                .padding(16.0)
                .background(bg)
                .color(text_color)
                .min_width_full()
        }),
    )
}
