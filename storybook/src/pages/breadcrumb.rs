use floem::IntoView;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, h_stack, label, scroll, v_stack};
use katana_ui_core::composite::breadcrumb::{Breadcrumb, BreadcrumbCrumb};
use katana_ui_core::primitive::icon::IconSource;
use katana_ui_core::theme::Theme;

const ICON_FOLDER: &[u8] =
    b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><path d='M2 4h5l2 2h5v7H2z' fill='none' stroke='currentColor' stroke-width='1.2'/></svg>";
const ICON_FILE: &[u8] =
    b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><path d='M3 2h6l3 3v9H3z' fill='none' stroke='currentColor' stroke-width='1.2'/><path d='M9 2v3h3' fill='none' stroke='currentColor' stroke-width='1.2'/></svg>";

fn log_view(signal: RwSignal<String>, title: &'static str) -> impl IntoView {
    v_stack((
        label(move || title).style(|style| style.font_size(14.0)),
        label(move || signal.get()),
    ))
}

fn breadcrumb_title(theme: Theme) -> impl IntoView {
    label(|| "Breadcrumb").style(move |style| {
        style.font_size(18.0).color(floem::peniko::Color::rgb8(
            theme.color.text.r,
            theme.color.text.g,
            theme.color.text.b,
        ))
    })
}

fn with_label(signal: RwSignal<String>, label: &'static str) -> impl Fn() + 'static {
    move || {
        signal.set(label.to_string());
    }
}

pub fn breadcrumb_page(theme: Theme) -> impl IntoView {
    let bg = floem::peniko::Color::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text =
        floem::peniko::Color::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let file_log = create_rw_signal("未選択".to_string());
    let settings_log = create_rw_signal("未選択".to_string());
    let long_log = create_rw_signal("未選択".to_string());

    crate::interaction::replay("click-crumb", "breadcrumb", "clicked-font", {
        let settings_log = settings_log;
        move || {
            settings_log.set("font".to_string());
        }
    });

    let file_path = Breadcrumb::new(vec![
        BreadcrumbCrumb::new("ホーム").icon(IconSource::SvgBytes(ICON_FOLDER)),
        BreadcrumbCrumb::new("src")
            .icon(IconSource::SvgBytes(ICON_FOLDER))
            .children(vec![
                BreadcrumbCrumb::new("composite").icon(IconSource::SvgBytes(ICON_FOLDER)),
                BreadcrumbCrumb::new("layout").icon(IconSource::SvgBytes(ICON_FOLDER)),
                BreadcrumbCrumb::new("theme").icon(IconSource::SvgBytes(ICON_FOLDER)),
            ]),
        BreadcrumbCrumb::new("composite")
            .icon(IconSource::SvgBytes(ICON_FOLDER))
            .children(vec![
                BreadcrumbCrumb::new("breadcrumb.rs").icon(IconSource::SvgBytes(ICON_FILE)),
                BreadcrumbCrumb::new("tabs.rs").icon(IconSource::SvgBytes(ICON_FILE)),
            ]),
        BreadcrumbCrumb::new("breadcrumb.rs").icon(IconSource::SvgBytes(ICON_FILE)),
    ])
    .separator(" / ")
    .allow_last_click(false)
    .view(theme.clone());

    let settings_path = Breadcrumb::new(vec![
        BreadcrumbCrumb::new("home")
            .on_click(with_label(settings_log, "home"))
            .icon(IconSource::SvgBytes(ICON_FOLDER)),
        BreadcrumbCrumb::new("settings")
            .on_click(with_label(settings_log, "settings"))
            .icon(IconSource::SvgBytes(ICON_FOLDER)),
        BreadcrumbCrumb::new("appearance").on_click(with_label(settings_log, "appearance")),
        BreadcrumbCrumb::new("font")
            .on_click(with_label(settings_log, "font"))
            .disabled(false),
    ])
    .separator(" > ")
    .allow_last_click(true)
    .background(true)
    .border(true)
    .view(theme.clone());

    let long_path = Breadcrumb::new(vec![
        BreadcrumbCrumb::new("workspace").on_click(with_label(long_log, "workspace")),
        BreadcrumbCrumb::new("apps").on_click(with_label(long_log, "apps")),
        BreadcrumbCrumb::new("katana-ui-core").on_click(with_label(long_log, "katana-ui-core")),
        BreadcrumbCrumb::new("crates").on_click(with_label(long_log, "crates")),
        BreadcrumbCrumb::new("katana-ui-core").on_click(with_label(long_log, "crate")),
        BreadcrumbCrumb::new("src").on_click(with_label(long_log, "src")),
        BreadcrumbCrumb::new("composite").on_click(with_label(long_log, "composite")),
        BreadcrumbCrumb::new("navigation").disabled(true),
        BreadcrumbCrumb::new("breadcrumbs").on_click(with_label(long_log, "breadcrumbs")),
    ])
    .separator(" / ")
    .max_visible_crumbs(5)
    .view(theme.clone());

    scroll(
        v_stack((
            breadcrumb_title(theme.clone()),
            h_stack((
                label(|| "・ファイル階層（ホバー/クリックで子階層）"),
                log_view(file_log, "クリック結果"),
            )),
            file_path,
            h_stack((
                label(|| "・設定パス"),
                log_view(settings_log, "クリック結果"),
            )),
            settings_path,
            h_stack((label(|| "・長いパス"), log_view(long_log, "クリック結果"))),
            long_path,
        ))
        .style(|style| style.gap(12.0).padding(16.0).width_full()),
    )
    .style(move |style| style.background(bg).color(text))
}
