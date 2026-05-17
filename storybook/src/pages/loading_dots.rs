use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{
    Decorators, dyn_container, h_stack, h_stack_from_iter, label, scroll, toggle_button, v_stack,
};
use katana_ui_core::primitive::loading_dots::LoadingDots;
use katana_ui_core::theme::Theme;
use katana_ui_core::theme::color::Color;

fn loading_row(desc: &'static str, dots: impl IntoView + 'static) -> impl IntoView {
    h_stack((
        label(move || desc).style(|s| s.width(120.0).font_size(12.0)),
        dots,
    ))
    .style(|s| s.gap(8.0).items_center())
}

fn dots(
    theme: Theme,
    dot_size: f32,
    active: bool,
    label_text: Option<&'static str>,
) -> impl IntoView {
    dots_full(theme, 3, dot_size, 6.0, 260, None, active, label_text)
}

fn dots_full(
    theme: Theme,
    dot_count: usize,
    dot_size: f32,
    dot_gap: f32,
    speed_ms: u64,
    color: Option<Color>,
    active: bool,
    label_text: Option<&'static str>,
) -> impl IntoView {
    let mut loading = LoadingDots::new()
        .dot_count(dot_count)
        .dot_size(dot_size)
        .dot_gap(dot_gap)
        .animation_speed_ms(speed_ms)
        .active(active);
    if let Some(color) = color {
        loading = loading.color(color);
    }

    if let Some(text) = label_text {
        loading = loading.label(text);
    }

    loading.view(theme)
}

pub fn loading_dots_page(theme: Theme) -> impl IntoView {
    let running = create_rw_signal(true);
    let live_theme = theme.clone();
    let readonly_theme = theme.clone();
    let size_theme = theme.clone();
    let active_off_theme = theme.clone();
    let count_theme = theme.clone();
    let gap_theme = theme.clone();
    let speed_theme = theme.clone();
    let color_theme = theme.clone();
    let danger = theme.color.danger;
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_color = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "LoadingDots")
                .style(|s| s.font_size(20.0).margin_left(4.0).margin_bottom(12.0)),
            h_stack((
                toggle_button(move || running.get())
                    .on_toggle(move |v| running.set(v))
                    .style(|s| s.margin_right(8.0)),
                label(|| "Live animation"),
            ))
            .style(|s| s.gap(8.0).items_center()),
            dyn_container(
                move || running.try_get().unwrap_or(true),
                move |is_running| {
                    loading_row(
                        if is_running { "live" } else { "off" },
                        dots(live_theme.clone(), 8.0, is_running, Some("thinking")),
                    )
                },
            )
            .into_any(),
            loading_row(
                "readonly",
                dots(readonly_theme.clone(), 6.0, false, Some("waiting")),
            ),
            v_stack((
                label(|| "size").style(|s| s.margin_top(8.0)),
                h_stack_from_iter([
                    dots(size_theme.clone(), 4.0, true, None),
                    dots(size_theme.clone(), 6.0, true, None),
                    dots(size_theme, 9.0, true, None),
                ])
                .style(|s| s.gap(16.0).items_center()),
            ))
            .style(|s| s.gap(4.0)),
            label_row("active / off", active_off_theme),
            label(|| "dot_count").style(|s| s.margin_top(8.0)),
            h_stack_from_iter([
                dots_full(count_theme.clone(), 2, 6.0, 6.0, 260, None, true, Some("2")),
                dots_full(count_theme.clone(), 3, 6.0, 6.0, 260, None, true, Some("3")),
                dots_full(count_theme, 5, 6.0, 6.0, 260, None, true, Some("5")),
            ])
            .style(|s| s.gap(16.0).items_center()),
            label(|| "dot_gap").style(|s| s.margin_top(8.0)),
            h_stack_from_iter([
                dots_full(
                    gap_theme.clone(),
                    3,
                    6.0,
                    2.0,
                    260,
                    None,
                    true,
                    Some("gap 2"),
                ),
                dots_full(gap_theme, 3, 6.0, 12.0, 260, None, true, Some("gap 12")),
            ])
            .style(|s| s.gap(16.0).items_center()),
            label(|| "animation_speed_ms").style(|s| s.margin_top(8.0)),
            h_stack_from_iter([
                dots_full(
                    speed_theme.clone(),
                    3,
                    6.0,
                    6.0,
                    120,
                    None,
                    true,
                    Some("120ms"),
                ),
                dots_full(speed_theme, 3, 6.0, 6.0, 600, None, true, Some("600ms")),
            ])
            .style(|s| s.gap(16.0).items_center()),
            loading_row(
                "color override",
                dots_full(
                    color_theme,
                    3,
                    6.0,
                    6.0,
                    260,
                    Some(danger),
                    true,
                    Some("danger"),
                ),
            ),
        ))
        .style(move |s| {
            s.gap(12.0)
                .padding(16.0)
                .background(bg)
                .color(text_color)
                .min_width_full()
        }),
    )
}

fn label_row(desc: &'static str, theme: Theme) -> impl IntoView {
    loading_row(
        desc,
        h_stack_from_iter([
            dots(theme.clone(), 6.0, true, Some("active")),
            dots(theme, 6.0, false, Some("off")),
        ])
        .style(|s| s.gap(8.0)),
    )
}
