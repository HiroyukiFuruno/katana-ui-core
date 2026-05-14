use floem::IntoView;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::Decorators;
use floem::views::{button, container, h_stack, label, scroll, v_stack};
use katana_ui_widget::composite::menu_button::{
    MenuButton, MenuButtonCloseCallback, MenuButtonPlacement, MenuButtonVariant,
};
use katana_ui_widget::primitive::icon::IconSource;
use katana_ui_widget::theme::Theme;
use std::rc::Rc;

const ICON_DOTS: &[u8] =
    b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='5' cy='12' r='1.5' fill='currentColor'/><circle cx='12' cy='12' r='1.5' fill='currentColor'/><circle cx='19' cy='12' r='1.5' fill='currentColor'/></svg>";

fn should_start_open() -> bool {
    let should_open = crate::interaction::requested("open");
    if should_open {
        crate::interaction::mark_supported("menu-button", "open");
        crate::interaction::mark_exercised("menu-button", "open", "initial-open");
    }
    should_open
}

fn menu_item(
    text: &'static str,
    namespace: &'static str,
    log: RwSignal<String>,
    close: MenuButtonCloseCallback,
) -> Box<dyn floem::View> {
    let action = text.to_string();
    let namespace = namespace.to_string();
    let log_for_action = log;
    let close_for_action = Rc::clone(&close);
    let label_text = action.clone();
    button(label(move || label_text.clone()))
        .action(move || {
            log_for_action.set(format!("{namespace}: {action}"));
            close_for_action();
        })
        .style(|s| s.padding_vert(6.0).padding_horiz(10.0))
        .into_any()
}

fn sample_menu(
    namespace: &'static str,
    log: RwSignal<String>,
) -> impl Fn(MenuButtonCloseCallback) -> Box<dyn floem::View> + 'static {
    move |close| {
        Box::new(
            v_stack((
                menu_item("開く", namespace, log, Rc::clone(&close)),
                menu_item("編集", namespace, log, Rc::clone(&close)),
                menu_item("削除", namespace, log, Rc::clone(&close)),
            ))
            .style(|s| s.gap(4.0))
            .into_any(),
        )
    }
}

fn status_row(namespace: &'static str, log: RwSignal<String>) -> impl IntoView {
    h_stack((
        label(move || namespace.to_string()),
        label(move || log.get()),
    ))
    .style(|s| s.gap(6.0))
}

fn counter(label_text: &'static str, value: RwSignal<u32>) -> impl IntoView {
    label(move || format!("{label_text}: {}", value.get())).style(|s| s.font_size(11.0))
}

fn menu_status(
    label_text: &'static str,
    open: RwSignal<u32>,
    close: RwSignal<u32>,
    log: RwSignal<String>,
) -> impl IntoView {
    h_stack((
        counter("open", open),
        counter("close", close),
        status_row(label_text, log),
    ))
    .style(|s| s.gap(12.0))
}

fn placement_sample(
    theme: Theme,
    title: &'static str,
    placement: MenuButtonPlacement,
    log: RwSignal<String>,
) -> impl IntoView {
    v_stack((
        label(move || title).style(|s| s.font_size(12.0)),
        MenuButton::new()
            .trigger_label("Placement menu")
            .placement(placement)
            .content(sample_menu("placement", log))
            .view(theme),
    ))
    .style(|s| s.gap(4.0))
}

pub fn menu_button_page(theme: Theme) -> impl IntoView {
    let bg = floem::peniko::Color::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text =
        floem::peniko::Color::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    let framed_log = create_rw_signal("未選択".to_string());
    let framed_open = create_rw_signal(0_u32);
    let framed_close = create_rw_signal(0_u32);
    let framed = MenuButton::new()
        .trigger_label("Action menu")
        .open(should_start_open())
        .on_open({
            let framed_open = framed_open;
            move || {
                let _ = framed_open.try_update(|value| *value += 1);
            }
        })
        .on_close({
            let framed_close = framed_close;
            move || {
                let _ = framed_close.try_update(|value| *value += 1);
            }
        })
        .content(sample_menu("framed", framed_log));

    let unframed_log = create_rw_signal("未選択".to_string());
    let unframed_open = create_rw_signal(0_u32);
    let unframed_close = create_rw_signal(0_u32);
    let unframed = MenuButton::new()
        .variant(MenuButtonVariant::Unframed)
        .trigger_label("Text menu")
        .on_open({
            let unframed_open = unframed_open;
            move || {
                let _ = unframed_open.try_update(|value| *value += 1);
            }
        })
        .on_close({
            let unframed_close = unframed_close;
            move || {
                let _ = unframed_close.try_update(|value| *value += 1);
            }
        })
        .content(sample_menu("unframed", unframed_log));

    let icon_log = create_rw_signal("未選択".to_string());
    let icon_open = create_rw_signal(0_u32);
    let icon_close = create_rw_signal(0_u32);
    let icon = MenuButton::new()
        .trigger_icon(IconSource::SvgBytes(ICON_DOTS))
        .on_open({
            let icon_open = icon_open;
            move || {
                let _ = icon_open.try_update(|value| *value += 1);
            }
        })
        .on_close({
            let icon_close = icon_close;
            move || {
                let _ = icon_close.try_update(|value| *value += 1);
            }
        })
        .content(sample_menu("icon", icon_log));
    let placement_log = create_rw_signal("未選択".to_string());

    scroll(
        v_stack((
            label(|| "MenuButton").style(|s| s.font_size(20.0)),
            label(|| "framed button menu").style(|s| s.font_size(14.0)),
            container(framed.view(theme.clone()))
                .style(|s| s.padding(2.0))
                .into_any(),
            menu_status("framed", framed_open, framed_close, framed_log),
            label(|| "unframed text menu").style(|s| s.font_size(14.0)),
            container(unframed.view(theme.clone()))
                .style(|s| s.padding(2.0))
                .into_any(),
            menu_status("unframed", unframed_open, unframed_close, unframed_log),
            label(|| "icon menu").style(|s| s.font_size(14.0)),
            container(icon.view(theme.clone()))
                .style(|s| s.padding(2.0))
                .into_any(),
            menu_status("icon", icon_open, icon_close, icon_log),
            label(|| "placement").style(|s| s.font_size(14.0)),
            h_stack((
                placement_sample(
                    theme.clone(),
                    "BottomStart",
                    MenuButtonPlacement::BottomStart,
                    placement_log,
                ),
                placement_sample(
                    theme.clone(),
                    "TopEnd",
                    MenuButtonPlacement::TopEnd,
                    placement_log,
                ),
                placement_sample(
                    theme.clone(),
                    "End",
                    MenuButtonPlacement::End,
                    placement_log,
                ),
            ))
            .style(|s| s.gap(12.0)),
            status_row("placement", placement_log),
        ))
        .style(move |s| s.gap(14.0).padding(16.0).background(bg).color(text)),
    )
}
