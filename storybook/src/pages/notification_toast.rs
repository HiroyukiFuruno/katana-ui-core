use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, label, scroll, v_stack};
use katana_ui_widget::composite::notification_toast::{
    NotificationToast, NotificationToastPosition, NotificationToastSeverity, NotificationToastStack,
};
use katana_ui_widget::theme::Theme;

fn severity_samples(theme: Theme) -> impl IntoView {
    v_stack((
        label(|| "Severity").style(|s| s.font_size(16.0)),
        NotificationToastStack::new(vec![
            NotificationToast::new("Info メッセージ").severity(NotificationToastSeverity::Info),
            NotificationToast::new("Success メッセージ")
                .severity(NotificationToastSeverity::Success),
            NotificationToast::new("Warning メッセージ")
                .severity(NotificationToastSeverity::Warning),
            NotificationToast::new("Error メッセージ").severity(NotificationToastSeverity::Error),
        ])
        .view(theme.clone()),
    ))
}

fn auto_dismiss_section(theme: Theme) -> impl IntoView {
    let dismiss_count = create_rw_signal(0_u32);

    let stack = NotificationToastStack::new(vec![
        NotificationToast::new("2秒後に消えます")
            .duration(2_000)
            .severity(NotificationToastSeverity::Success)
            .on_dismiss({
                let dismiss_count = dismiss_count.clone();
                move || {
                    let _ = dismiss_count.try_update(|value| *value += 1);
                }
            }),
        NotificationToast::new("4秒後に消えます")
            .duration(4_000)
            .severity(NotificationToastSeverity::Info)
            .on_dismiss({
                let dismiss_count = dismiss_count.clone();
                move || {
                    let _ = dismiss_count.try_update(|value| *value += 1);
                }
            }),
    ]);

    v_stack((
        label(|| "自動消去（2秒 / 4秒）").style(|s| s.font_size(16.0)),
        stack.view(theme.clone()),
        label(move || format!("auto-dismiss callback: {} 回", dismiss_count.get()))
            .style(|s| s.font_size(12.0)),
    ))
}

fn manual_and_action_section(theme: Theme) -> impl IntoView {
    let action_count = create_rw_signal(0_u32);

    let stack = NotificationToastStack::new(vec![
        NotificationToast::new("× ボタンで手動消去、アクションボタンを実行できます")
            .severity(NotificationToastSeverity::Warning)
            .action("元に戻す", {
                let action_count = action_count.clone();
                move || {
                    let _ = action_count.try_update(|value| *value += 1);
                }
            }),
    ]);

    v_stack((
        label(|| "手動 dismiss + アクション").style(|s| s.font_size(16.0)),
        stack.view(theme.clone()),
        label(move || format!("action callback: {} 回", action_count.get()))
            .style(|s| s.font_size(12.0)),
    ))
}

fn stack_section(theme: Theme) -> impl IntoView {
    v_stack((
        label(|| "複数通知 + stack + position").style(|s| s.font_size(16.0)),
        NotificationToastStack::new(vec![
            NotificationToast::new("TopLeft: 1").severity(NotificationToastSeverity::Info),
            NotificationToast::new("TopLeft: 2").severity(NotificationToastSeverity::Success),
            NotificationToast::new("TopLeft: 3").severity(NotificationToastSeverity::Error),
        ])
        .position(NotificationToastPosition::TopLeft)
        .max_visible(2)
        .view(theme.clone()),
        NotificationToastStack::new(vec![
            NotificationToast::new("TopRight: 1").severity(NotificationToastSeverity::Success),
            NotificationToast::new("TopRight: 2").severity(NotificationToastSeverity::Warning),
            NotificationToast::new("TopRight: 3").severity(NotificationToastSeverity::Error),
        ])
        .position(NotificationToastPosition::TopRight)
        .max_visible(2)
        .view(theme.clone()),
        NotificationToastStack::new(vec![
            NotificationToast::new("BottomLeft: 1").severity(NotificationToastSeverity::Info),
            NotificationToast::new("BottomLeft: 2").severity(NotificationToastSeverity::Warning),
            NotificationToast::new("BottomLeft: 3").severity(NotificationToastSeverity::Success),
        ])
        .position(NotificationToastPosition::BottomLeft)
        .max_visible(2)
        .view(theme.clone()),
        NotificationToastStack::new(vec![
            NotificationToast::new("BottomRight: 1").severity(NotificationToastSeverity::Warning),
            NotificationToast::new("BottomRight: 2").severity(NotificationToastSeverity::Info),
            NotificationToast::new("BottomRight: 3").severity(NotificationToastSeverity::Error),
        ])
        .position(NotificationToastPosition::BottomRight)
        .max_visible(2)
        .view(theme.clone()),
    ))
    .style(|style| style.gap(20.0))
}

fn page_content(theme: Theme) -> impl IntoView {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    v_stack((
        label(|| "NotificationToast（ライブ UI）").style(|s| s.font_size(20.0)),
        severity_samples(theme.clone()),
        auto_dismiss_section(theme.clone()),
        manual_and_action_section(theme.clone()),
        stack_section(theme),
    ))
    .style(move |style| {
        style
            .gap(16.0)
            .padding(16.0)
            .background(bg)
            .color(text)
            .min_width_full()
    })
}

pub fn notification_toast_page(theme: Theme) -> impl IntoView {
    scroll(page_content(theme))
}
