use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, h_stack, label, scroll, v_stack};
use katana_ui_core::layout::side_menu::{SideMenu, SideMenuItem, SideMenuPopMode, SideMenuSide};
use katana_ui_core::primitive::icon::IconSource;
use katana_ui_core::theme::Theme;

const ICON_PLUS: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2.4' stroke-linecap='round'><path d='M12 5v14M5 12h14'/></svg>";
const ICON_FOLDER: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2.1' stroke-linejoin='round'><path d='M3 7h7l2 3h9v9H3z'/></svg>";
const ICON_FILES: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2.1' stroke-linejoin='round'><path d='M8 3h10v14H8z'/><path d='M5 7h10v14H5z'/></svg>";
const ICON_SEARCH: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2.3' stroke-linecap='round'><circle cx='10.5' cy='10.5' r='6.5'/><path d='M16 16l5 5'/></svg>";
const ICON_HELP: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2.1'><circle cx='12' cy='12' r='9'/><path d='M9.5 9a2.7 2.7 0 1 1 4.5 2c-1 .7-1.5 1.2-1.5 2.4'/><path d='M12 17.2h.01'/></svg>";
const ICON_SETTINGS: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2'><circle cx='12' cy='12' r='3'/><path d='M12 2v3M12 19v3M4.9 4.9l2.1 2.1M17 17l2.1 2.1M2 12h3M19 12h3M4.9 19.1 7 17M17 7l2.1-2.1'/></svg>";
const ICON_LIST: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2.2'><path d='M9 6h11M9 12h11M9 18h11'/><path d='M4 6h.01M4 12h.01M4 18h.01'/></svg>";
const ICON_REFRESH: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2.1' stroke-linecap='round'><path d='M20 6v6h-6'/><path d='M4 18v-6h6'/><path d='M18 9a7 7 0 0 0-12-2M6 15a7 7 0 0 0 12 2'/></svg>";
const ICON_EXPORT: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2.1' stroke-linecap='round'><path d='M12 3v12M7 8l5-5 5 5'/><path d='M5 15v5h14v-5'/></svg>";
const ICON_EYE: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2.1'><path d='M2 12s4-7 10-7 10 7 10 7-4 7-10 7S2 12 2 12z'/><circle cx='12' cy='12' r='3'/></svg>";

fn panel(theme: Theme, title: &'static str, body: &'static str) -> Box<dyn floem::View> {
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    v_stack((
        label(move || title).style(move |style| style.font_size(15.0).color(text)),
        label(move || body).style(move |style| style.font_size(12.0).line_height(1.5).color(text)),
    ))
    .style(|style| style.gap(10.0).padding(10.0))
    .into_any()
}

fn action_item(
    icon: &'static [u8],
    log: floem::reactive::RwSignal<String>,
    message: &'static str,
) -> SideMenuItem {
    SideMenuItem::new(IconSource::SvgBytes(icon), move || {
        log.set(message.to_string())
    })
}

fn panel_item(
    theme: Theme,
    icon: &'static [u8],
    log: floem::reactive::RwSignal<String>,
    message: &'static str,
    title: &'static str,
    body: &'static str,
) -> SideMenuItem {
    SideMenuItem::new(IconSource::SvgBytes(icon), move || {
        log.set(message.to_string())
    })
    .with_expand_pop(move || panel(theme.clone(), title, body))
}

fn left_items(theme: Theme, log: floem::reactive::RwSignal<String>) -> Vec<SideMenuItem> {
    vec![
        action_item(ICON_PLUS, log, "左: 新規"),
        panel_item(
            theme.clone(),
            ICON_FOLDER,
            log,
            "左: ワークスペース",
            "Workspace",
            "ホバー（hover）で一時表示、クリック（click）で固定表示",
        ),
        panel_item(
            theme,
            ICON_FILES,
            log,
            "左: Explorer",
            "Explorer",
            "選択中のファイルツリーをここに表示",
        )
        .selected(true),
        action_item(ICON_SEARCH, log, "左: 検索"),
        action_item(ICON_HELP, log, "左: ヘルプ").bottom(),
        action_item(ICON_SETTINGS, log, "左: 設定").bottom(),
    ]
}

fn right_items(theme: Theme, log: floem::reactive::RwSignal<String>) -> Vec<SideMenuItem> {
    vec![
        panel_item(
            theme.clone(),
            ICON_LIST,
            log,
            "右: 目次",
            "TOC",
            "右配置ではパネル（panel）が左へ伸びる",
        ),
        action_item(ICON_REFRESH, log, "右: 更新"),
        panel_item(
            theme.clone(),
            ICON_SEARCH,
            log,
            "右: 検索",
            "Search",
            "プレビュー（preview）内検索の内容",
        ),
        panel_item(
            theme.clone(),
            ICON_EXPORT,
            log,
            "右: Export",
            "Export",
            "HTML / PDF / image export",
        ),
        panel_item(
            theme,
            ICON_EYE,
            log,
            "右: Preview",
            "Preview",
            "表示モードの切り替え",
        ),
    ]
}

fn mixed_pop_items(theme: Theme, log: floem::reactive::RwSignal<String>) -> Vec<SideMenuItem> {
    let modal_theme = theme.clone();
    let popover_theme = theme.clone();
    let expand_theme = theme;
    vec![
        SideMenuItem::new(IconSource::SvgBytes(ICON_HELP), move || {
            log.set("hover: modal pop".to_string())
        })
        .with_modal_pop(move || panel(modal_theme.clone(), "Modal Pop", "ダイアログ表示の確認")),
        SideMenuItem::new(IconSource::SvgBytes(ICON_SEARCH), move || {
            log.set("hover: popover pop".to_string())
        })
        .with_popover_pop(move || {
            panel(
                popover_theme.clone(),
                "Popover Pop",
                "ポップオーバー表示の確認",
            )
        }),
        SideMenuItem::new(IconSource::SvgBytes(ICON_FILES), move || {
            log.set("hover: expand pop".to_string())
        })
        .with_expand_pop(move || panel(expand_theme.clone(), "Expand Pop", "展開パネル表示の確認")),
    ]
}

fn shell(
    theme: Theme,
    left_log: floem::reactive::RwSignal<String>,
    right_log: floem::reactive::RwSignal<String>,
) -> impl IntoView {
    let workspace = PenikoColor::rgb8(25, 25, 25);
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let left = SideMenu::new(left_items(theme.clone(), left_log))
        .side(SideMenuSide::Left)
        .view(theme.clone());
    let right = SideMenu::new(right_items(theme.clone(), right_log))
        .side(SideMenuSide::Right)
        .view(theme);
    let center = v_stack((
        label(|| "KatanA SideMenu").style(move |style| style.font_size(14.0).color(text)),
        label(move || format!("左ログ: {}", left_log.get()))
            .style(move |style| style.font_size(12.0).color(text)),
        label(move || format!("右ログ: {}", right_log.get()))
            .style(move |style| style.font_size(12.0).color(text)),
    ))
    .style(move |style| {
        style
            .flex_grow(1.0)
            .height_full()
            .padding(18.0)
            .gap(8.0)
            .background(workspace)
    });

    h_stack((left, center, right)).style(|style| style.width_full().height(520.0))
}

pub fn side_menu_page(theme: Theme) -> impl IntoView {
    let left_log = create_rw_signal("未操作".to_string());
    let right_log = create_rw_signal("未操作".to_string());
    let hover_log = create_rw_signal("未操作".to_string());
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let hover_side_menu = SideMenu::new(mixed_pop_items(Theme::default_dark(), hover_log))
        .side(SideMenuSide::Left)
        .width(68.0)
        .hover_expand(20.0)
        .view(Theme::default_dark());
    let mut fixed_side_menu = SideMenu::new(left_items(Theme::default_dark(), hover_log))
        .side(SideMenuSide::Left)
        .width(68.0)
        .fixed();
    if crate::interaction::open_requested("side-menu", "initial-popover-open") {
        fixed_side_menu = fixed_side_menu.initial_pop(1, SideMenuPopMode::Popover);
    }
    let fixed_side_menu = fixed_side_menu.view(Theme::default_dark());

    scroll(
        v_stack((
            label(|| "SideMenu").style(move |style| style.font_size(16.0).color(text)),
            shell(Theme::default_dark(), left_log, right_log),
            label(|| "hover expand / fixed / pop mode").style(move |style| style.color(text)),
            h_stack((hover_side_menu, fixed_side_menu))
                .style(|style| style.gap(12.0).height(260.0).items_start()),
            label(move || format!("追加ログ: {}", hover_log.get()))
                .style(move |style| style.font_size(12.0).color(text)),
        ))
        .style(move |style| {
            style
                .gap(12.0)
                .padding(16.0)
                .background(bg)
                .min_width_full()
        }),
    )
    .style(|style| style.width_full().height_full())
}
