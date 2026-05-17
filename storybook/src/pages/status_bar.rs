use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, container, label, scroll, v_stack};
use katana_ui_core::layout::status_bar::{StatusBar, StatusSeverity};
use katana_ui_core::primitive::spinner::{Spinner, SpinnerSize};
use katana_ui_core::theme::Theme;

fn sample_statuses(theme: Theme) -> impl IntoView {
    v_stack((
        StatusBar::new("エラー: 保存処理を開始できませんでした")
            .severity(StatusSeverity::Error)
            .view(theme.clone()),
        StatusBar::new("警告: 設定値が未保存のままです")
            .severity(StatusSeverity::Warning)
            .view(theme.clone()),
        StatusBar::new("保存が完了しました")
            .severity(StatusSeverity::Success)
            .view(theme.clone()),
        StatusBar::new("情報: 次の同期を待っています")
            .severity(StatusSeverity::Info)
            .view(theme.clone()),
    ))
    .style(|style| style.gap(8.0))
}

fn action_area_sample(theme: Theme) -> impl IntoView {
    let count = create_rw_signal(0usize);
    let click_count = count;

    let action_target = StatusBar::new("操作に失敗しました")
        .severity(StatusSeverity::Error)
        .action_label("再試行")
        .on_action(move || {
            click_count.try_update(|value| *value += 1);
        })
        .view(theme.clone());

    v_stack((
        action_target,
        label(move || format!("アクション実行回数: {} 回", count.get()))
            .style(|style| style.font_size(11.0)),
    ))
    .style(|style| style.gap(8.0))
}

fn spinner_sample(theme: Theme) -> impl IntoView {
    let spinner = Spinner::new().size(SpinnerSize::Sm).view(theme.clone());
    StatusBar::new("バックグラウンドで同期しています")
        .severity(StatusSeverity::Info)
        .trailing(spinner)
        .view(theme.clone())
}

fn spacing_sample(theme: Theme) -> impl IntoView {
    v_stack((
        StatusBar::new("compact: height=30 padding=4 gap=4")
            .severity(StatusSeverity::Info)
            .height(30.0)
            .padding(4.0)
            .gap(4.0)
            .view(theme.clone()),
        StatusBar::new("spacious: height=52 padding=12 gap=12")
            .severity(StatusSeverity::Success)
            .height(52.0)
            .padding(12.0)
            .gap(12.0)
            .view(theme.clone()),
    ))
    .style(|style| style.gap(8.0))
}

pub fn status_bar_page(theme: Theme) -> impl IntoView {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    scroll(
        v_stack((
            label(|| "StatusBar").style(|style| style.font_size(16.0).margin_bottom(8.0)),
            container(sample_statuses(theme.clone())).style(|style| style.width_full()),
            label(|| "Action button")
                .style(|style| style.font_size(14.0).margin_top(12.0).margin_bottom(6.0)),
            container(action_area_sample(theme.clone())).style(|style| style.width_full()),
            label(|| "Spinner")
                .style(|style| style.font_size(14.0).margin_top(12.0).margin_bottom(6.0)),
            container(spinner_sample(theme.clone())).style(|style| style.width_full()),
            label(|| "Height / padding / gap")
                .style(|style| style.font_size(14.0).margin_top(12.0).margin_bottom(6.0)),
            container(spacing_sample(theme.clone())).style(|style| style.width_full()),
        ))
        .style(move |s| {
            s.gap(8.0)
                .padding(16.0)
                .background(bg)
                .color(text)
                .width_full()
        }),
    )
}
