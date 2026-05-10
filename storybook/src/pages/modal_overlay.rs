use floem::peniko::Color as PenikoColor;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::views::{dyn_container, h_stack, label, scroll, toggle_button, v_stack, Decorators};
use floem::IntoView;
use katana_ui_widget::layout::modal::{Modal, ModalSize};
use katana_ui_widget::theme::Theme;

fn modal_row(
    heading: &'static str,
    open: bool,
    dismiss_backdrop: bool,
    width: f32,
    bg_r: u8,
    bg_g: u8,
    bg_b: u8,
    bg_a: u8,
) -> impl IntoView {
    let overlay_color =
        PenikoColor::rgba8(bg_r, bg_g, bg_b, bg_a);
    v_stack((
        label(move || heading).style(|s| s.font_size(12.0).margin_bottom(2.0)),
        h_stack((
            label(move || if open { "[Open]" } else { "[Closed]" })
                .style(|s| s.font_size(10.0).margin_right(4.0)),
            label(move || "")
                .style(move |s| s.width(width.min(120.0)).height(16.0).background(overlay_color)),
            label(move || if dismiss_backdrop { "backdrop:on" } else { "backdrop:off" })
                .style(|s| s.font_size(10.0).margin_left(4.0)),
        ))
        .style(|s| s.items_center().gap(2.0)),
    ))
    .style(|s| s.gap(4.0))
}

fn page_content(theme: &Theme) -> impl IntoView + use<> {
    let r_default = Modal::new().open(true).title("Default Modal").resolve(theme);
    let r_sm = Modal::new().open(true).size(ModalSize::Sm).title("Small").resolve(theme);
    let r_lg = Modal::new().open(true).size(ModalSize::Lg).title("Large").resolve(theme);
    let r_no_backdrop = Modal::new()
        .open(true)
        .dismiss_on_backdrop(false)
        .title("No Backdrop Dismiss")
        .resolve(theme);
    let r_custom = Modal::new()
        .open(true)
        .size(ModalSize::Custom(360.0))
        .title("Custom 360pt")
        .resolve(theme);
    let r_closed = Modal::new().open(false).resolve(theme);

    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text_col = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "Modal Samples").style(|s| s.font_size(16.0).margin_bottom(8.0)),
            modal_row(
                "Default Md (open)",
                r_default.open,
                r_default.dismiss_on_backdrop,
                r_default.dialog_width,
                r_default.overlay_color.r,
                r_default.overlay_color.g,
                r_default.overlay_color.b,
                r_default.overlay_color.a,
            ),
            modal_row(
                "Sm (open)",
                r_sm.open,
                r_sm.dismiss_on_backdrop,
                r_sm.dialog_width,
                r_sm.overlay_color.r,
                r_sm.overlay_color.g,
                r_sm.overlay_color.b,
                r_sm.overlay_color.a,
            ),
            modal_row(
                "Lg (open)",
                r_lg.open,
                r_lg.dismiss_on_backdrop,
                r_lg.dialog_width,
                r_lg.overlay_color.r,
                r_lg.overlay_color.g,
                r_lg.overlay_color.b,
                r_lg.overlay_color.a,
            ),
            modal_row(
                "No backdrop dismiss",
                r_no_backdrop.open,
                r_no_backdrop.dismiss_on_backdrop,
                r_no_backdrop.dialog_width,
                r_no_backdrop.overlay_color.r,
                r_no_backdrop.overlay_color.g,
                r_no_backdrop.overlay_color.b,
                r_no_backdrop.overlay_color.a,
            ),
            modal_row(
                "Custom 360pt",
                r_custom.open,
                r_custom.dismiss_on_backdrop,
                r_custom.dialog_width,
                r_custom.overlay_color.r,
                r_custom.overlay_color.g,
                r_custom.overlay_color.b,
                r_custom.overlay_color.a,
            ),
            modal_row(
                "Closed",
                r_closed.open,
                r_closed.dismiss_on_backdrop,
                r_closed.dialog_width,
                r_closed.overlay_color.r,
                r_closed.overlay_color.g,
                r_closed.overlay_color.b,
                r_closed.overlay_color.a,
            ),
            label(|| "Esc / backdrop dismiss governed by dismiss_on_esc / dismiss_on_backdrop props")
                .style(|s| s.font_size(11.0).margin_top(8.0)),
        ))
        .style(move |s| {
            s.gap(8.0)
                .padding(16.0)
                .background(bg)
                .color(text_col)
                .min_width_full()
        }),
    )
}

pub fn modal_overlay_page() -> impl IntoView {
    let is_dark = create_rw_signal(false);

    v_stack((
        h_stack((
            label(|| "Modal Overlay").style(|s| s.font_size(20.0)),
            label(move || if is_dark.get() { "Dark" } else { "Light" }),
            toggle_button(move || is_dark.get()).on_toggle(move |v| is_dark.set(v)),
        ))
        .style(|s| s.gap(12.0).items_center().padding(12.0)),
        dyn_container(
            move || is_dark.get(),
            move |dark| {
                let theme = if dark {
                    Theme::default_dark()
                } else {
                    Theme::default_light()
                };
                page_content(&theme)
            },
        ),
    ))
}
