use std::time::Duration;

use floem::IntoView;
use floem::action::exec_after;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, dyn_container, h_stack, label, scroll, v_stack};
use katana_ui_widget::composite::progress_bar::ProgressBar;
use katana_ui_widget::theme::Theme;

const LIVE_FRAME_MS: u64 = 120;
const LIVE_STEP: f32 = 0.04;

fn schedule_progress_tick(value: RwSignal<f32>, mounted: RwSignal<bool>) {
    exec_after(Duration::from_millis(LIVE_FRAME_MS), move |_| {
        if !mounted.try_get_untracked().unwrap_or(false) {
            return;
        }

        if value
            .try_update(|current| {
                let next = *current + LIVE_STEP;
                *current = if next > 1.0 { 0.0 } else { next };
            })
            .is_none()
        {
            return;
        }

        schedule_progress_tick(value, mounted);
    });
}

fn item_row(label_text: &'static str, item: impl IntoView + 'static) -> impl IntoView {
    h_stack((
        label(move || label_text).style(|s| s.width(160.0).font_size(12.0)),
        item,
    ))
    .style(|s| s.gap(10.0).items_center())
}

fn live_example(theme: Theme) -> impl IntoView {
    let value = create_rw_signal(0.0);
    let mounted = create_rw_signal(true);
    schedule_progress_tick(value, mounted);

    let progress_theme = theme.clone();

    dyn_container(
        move || value.try_get().unwrap_or(0.0),
        move |v| {
            ProgressBar::new()
                .value(v)
                .min(0.0)
                .max(1.0)
                .show_label(true)
                .size(10.0)
                .view(progress_theme.clone())
        },
    )
    .on_cleanup(move || mounted.set(false))
}

pub fn progress_bar_page(theme: Theme) -> impl IntoView {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_color = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "ProgressBar")
                .style(|s| s.font_size(20.0).margin_left(4.0).margin_bottom(12.0)),
            label(|| "Live widget").style(|s| s.font_size(13.0)),
            live_example(theme.clone()),
            label(|| "Value samples")
                .style(|s| s.font_size(14.0).margin_top(12.0).margin_bottom(4.0)),
            item_row(
                "0.0 .. 1.0 => 25%",
                ProgressBar::new().value(0.25).view(theme.clone()),
            ),
            item_row(
                "min=10 max=110 => 20%",
                ProgressBar::new()
                    .min(10.0)
                    .max(110.0)
                    .value(30.0)
                    .view(theme.clone()),
            ),
            item_row(
                "value clamped out of range",
                ProgressBar::new()
                    .min(0.0)
                    .max(100.0)
                    .value(250.0)
                    .show_label(false)
                    .view(theme.clone()),
            ),
            item_row(
                "No label",
                ProgressBar::new()
                    .value(0.6)
                    .show_label(false)
                    .view(theme.clone()),
            ),
            label(|| "Indeterminate")
                .style(|s| s.font_size(14.0).margin_top(12.0).margin_bottom(4.0)),
            item_row(
                "moving",
                ProgressBar::new()
                    .indeterminate(true)
                    .size(10.0)
                    .animation_speed_ms(80)
                    .view(theme.clone()),
            ),
            item_row(
                "label fixed text",
                ProgressBar::new()
                    .indeterminate(true)
                    .label("Processing")
                    .track_color(theme.color.text_muted)
                    .fill_color(theme.color.success)
                    .size(12.0)
                    .view(theme.clone()),
            ),
            label(|| "Size / color")
                .style(|s| s.font_size(14.0).margin_top(12.0).margin_bottom(4.0)),
            item_row(
                "small",
                ProgressBar::new()
                    .value(0.75)
                    .size(4.0)
                    .track_width(160.0)
                    .view(theme.clone()),
            ),
            item_row(
                "medium",
                ProgressBar::new()
                    .value(0.75)
                    .size(10.0)
                    .track_width(220.0)
                    .view(theme.clone()),
            ),
            item_row(
                "large + radius 16",
                ProgressBar::new()
                    .value(0.75)
                    .size(14.0)
                    .radius(16.0)
                    .track_width(260.0)
                    .track_color(theme.color.border)
                    .fill_color(theme.color.danger)
                    .view(theme.clone()),
            ),
        ))
        .style(move |s| {
            s.gap(8.0)
                .padding(16.0)
                .background(bg)
                .color(text_color)
                .min_width_full()
        }),
    )
}
