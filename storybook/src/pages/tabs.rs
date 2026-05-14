use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, label, scroll, v_stack};
use katana_ui_widget::composite::tabs::{TabItem, Tabs};
use katana_ui_widget::primitive::icon::IconSource;
use katana_ui_widget::theme::Theme;

const ICON_CLOCK: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><circle cx='8' cy='8' r='6.5' fill='none' stroke='currentColor' stroke-width='1.2'/><path d='M8 4.5v3l2 1' fill='none' stroke='currentColor' stroke-width='1.2' stroke-linecap='round'/></svg>";
const ICON_DOC: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><path d='M4 2h6l2 2v10H4z' fill='none' stroke='currentColor' stroke-width='1.2'/><path d='M9 2v3h3' fill='none' stroke='currentColor' stroke-width='1.2'/></svg>";

fn section_title(text: &'static str) -> impl IntoView {
    label(move || text).style(|style| style.font_size(16.0).margin_bottom(4.0))
}

fn content_tabs(theme: Theme) -> impl IntoView {
    Tabs::new(vec![
        TabItem::new("概要")
            .selected(true)
            .icon(IconSource::SvgBytes(ICON_CLOCK))
            .content(|| label(|| "選択中: 概要タブの内容").style(|style| style.font_size(12.0))),
        TabItem::new("ドキュメント")
            .icon(IconSource::SvgBytes(ICON_DOC))
            .content(|| {
                label(|| "選択中: ドキュメントタブの内容").style(|style| style.font_size(12.0))
            }),
    ])
    .view(theme.clone())
}

fn callback_tabs(theme: Theme) -> impl IntoView {
    let selected = create_rw_signal("概要".to_string());

    crate::interaction::replay("select-tab", "tabs", "selected-settings", {
        let selected = selected;
        move || {
            selected.set("設定".to_string());
        }
    });

    let tabs = Tabs::new(vec![
        TabItem::new("概要").selected(true).on_select({
            let selected = selected;
            move || {
                selected.set("概要".to_string());
            }
        }),
        TabItem::new("設定").on_select({
            let selected = selected;
            move || {
                selected.set("設定".to_string());
            }
        }),
        TabItem::new("履歴").disabled(true).on_select({
            let selected = selected;
            move || {
                selected.set("履歴".to_string());
            }
        }),
    ])
    .view(theme.clone());

    v_stack((
        tabs,
        label(move || format!("外部UIの表示: {}", selected.get())),
    ))
}

fn closeable_tabs(theme: Theme) -> impl IntoView {
    let close_count = create_rw_signal(0_u32);

    crate::interaction::replay("close-tab", "tabs", "close-count-1", {
        let close_count = close_count;
        move || {
            close_count.set(1);
        }
    });

    let tabs = Tabs::new(vec![
        TabItem::new("保存").selected(true).on_close({
            let close_count = close_count;
            move || {
                close_count.update(|count| *count += 1);
            }
        }),
        TabItem::new("下書き").on_close({
            let close_count = close_count;
            move || {
                close_count.update(|count| *count += 1);
            }
        }),
        TabItem::new("禁止").disabled(true).on_close({
            let close_count = close_count;
            move || {
                close_count.update(|count| *count += 1);
            }
        }),
    ])
    .view(theme.clone());

    v_stack((
        tabs,
        label(move || format!("閉じる操作回数: {}", close_count.get())),
    ))
}

fn overflow_tabs(theme: Theme) -> impl IntoView {
    let mut items = Vec::new();
    for index in 0..12 {
        items.push(
            TabItem::new(format!("タブ{}", index + 1))
                .selected(index == 0)
                .on_select(|| {}),
        );
    }

    Tabs::new(items).overflow(true).view(theme.clone())
}

fn page_content(theme: Theme) -> impl IntoView {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            section_title("contentあり"),
            content_tabs(theme.clone()),
            section_title("contentなし（外部UI連携）"),
            callback_tabs(theme.clone()),
            section_title("閉じられるタブ"),
            closeable_tabs(theme.clone()),
            section_title("overflow"),
            overflow_tabs(theme),
        ))
        .style(move |style| {
            style
                .gap(16.0)
                .padding(16.0)
                .background(bg)
                .color(text_col)
                .min_width_full()
        }),
    )
}

pub fn tabs_page(theme: Theme) -> impl IntoView {
    page_content(theme)
}
