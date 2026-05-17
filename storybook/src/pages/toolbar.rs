use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, container, h_stack, label, scroll, v_stack};
use katana_ui_core::composite::button::icon_text::{IconPosition, IconTextButton};
use katana_ui_core::composite::button::svg::{SvgButton, Tone, Variant};
use katana_ui_core::composite::button::text::Size;
use katana_ui_core::layout::toolbar::{Toolbar, ToolbarAlignment};
use katana_ui_core::primitive::icon::{Icon, IconSize, IconSource};
use katana_ui_core::theme::Theme;

const ICON_SAVE: &[u8] =
    b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><path d='M2 2h10l2 2v10a2 2 0 0 1-2 2H2zm0 3v9h12V6l-1-1H7V5H2'/><path d='M6 2v2h5v1H6z'/></svg>";
const ICON_SEARCH: &[u8] =
    b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><circle cx='7' cy='7' r='4.5'/><path d='M11 11l3 3'/></svg>";
const ICON_IMPORT: &[u8] =
    b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><path d='M8 2v8'/><path d='M5 7l3 3 3-3'/><path d='M3 14h10'/></svg>";

fn icon_action(
    theme: Theme,
    log: floem::reactive::RwSignal<String>,
    icon: &'static [u8],
    label_text: &'static str,
) -> impl IntoView {
    SvgButton::new(IconSource::SvgBytes(icon), label_text)
        .variant(Variant::Subtle)
        .tone(Tone::Accent)
        .size(IconSize::Lg)
        .view(theme, move || {
            log.set(format!("icon action: {label_text}"));
        })
}

fn text_plus_icon_toolbar(theme: Theme, log: floem::reactive::RwSignal<String>) -> impl IntoView {
    let lead = h_stack((
        label(|| "Text + Icon Toolbar").style(|s| s.font_size(14.0)),
        IconTextButton::new(IconSource::SvgBytes(ICON_IMPORT), "Import")
            .icon_position(IconPosition::Leading)
            .size(Size::Md)
            .view(theme.clone(), move || {
                log.set("text+icon action: Import".to_string());
            }),
        IconTextButton::new(IconSource::SvgBytes(ICON_IMPORT), "Export")
            .icon_position(IconPosition::Trailing)
            .size(Size::Md)
            .view(theme.clone(), move || {
                log.set("text+icon action: Export".to_string());
            }),
    ));

    Toolbar::new()
        .leading(lead)
        .height(62.0)
        .padding(10.0)
        .gap(10.0)
        .alignment(ToolbarAlignment::Top)
        .show_border(true)
        .background(theme.color.surface)
        .trailing(label(|| "right"))
        .view(theme)
}

fn identity_with_actions(theme: Theme, log: floem::reactive::RwSignal<String>) -> impl IntoView {
    let identity = h_stack((
        Icon::new(IconSource::SvgBytes(ICON_SAVE))
            .size(IconSize::Lg)
            .view(theme.clone()),
        label(|| "Project: design-kit")
            .style(|s| s.font_size(16.0))
            .into_any(),
    ))
    .style(|s| s.gap(8.0).items_center());

    let actions = h_stack((
        icon_action(theme.clone(), log, ICON_SAVE, "Save"),
        icon_action(theme.clone(), log, ICON_SEARCH, "Search"),
    ))
    .style(|s| s.gap(6.0));

    Toolbar::new()
        .leading(identity)
        .trailing(actions)
        .alignment(ToolbarAlignment::Bottom)
        .height(64.0)
        .padding(12.0)
        .gap(16.0)
        .show_border(true)
        .background(theme.color.surface)
        .view(theme)
}

fn icon_toolbar(theme: Theme, log: floem::reactive::RwSignal<String>) -> impl IntoView {
    let icons = h_stack((
        icon_action(theme.clone(), log, ICON_SEARCH, "search"),
        icon_action(theme.clone(), log, ICON_IMPORT, "import"),
        icon_action(theme.clone(), log, ICON_SAVE, "save"),
    ))
    .style(|s| s.gap(8.0));

    Toolbar::new()
        .leading(icons)
        .height(52.0)
        .padding(10.0)
        .gap(8.0)
        .alignment(ToolbarAlignment::Center)
        .show_border(true)
        .background(theme.color.surface)
        .view(theme)
}

pub fn toolbar_page(theme: Theme) -> impl IntoView {
    let action_log = create_rw_signal("未操作".to_string());
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    crate::interaction::replay("toolbar-action", "toolbar", "action-search", {
        let action_log = action_log;
        move || {
            action_log.set("icon action: Search".to_string());
        }
    });

    scroll(
        v_stack((
            label(|| "Toolbar Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            container(icon_toolbar(theme.clone(), action_log)).style(|s| s.width_full()),
            container(text_plus_icon_toolbar(theme.clone(), action_log))
                .style(|s| s.width_full().margin_top(12.0)),
            container(identity_with_actions(theme.clone(), action_log))
                .style(|s| s.width_full().margin_top(12.0)),
            container(label(move || format!("action log: {}", action_log.get())))
                .style(move |s| s.margin_top(12.0).font_size(12.0).color(text)),
        ))
        .style(move |s| {
            s.gap(12.0)
                .padding(16.0)
                .background(bg)
                .color(text)
                .min_width_full()
        }),
    )
}
