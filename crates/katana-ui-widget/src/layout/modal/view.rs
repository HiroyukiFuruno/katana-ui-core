use super::OverlayDialog;
use super::types::{ModalProps, ModalSize};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, button, container, dyn_container, empty, label, v_stack};
use floem::{IntoView, View};
use std::rc::Rc;

/// View-ready overlay description used by layout consumers.
#[derive(Debug, Clone, Copy)]
pub(super) struct ModalOverlayView {
    pub background: Color,
}

/// View-ready dialog description used by layout consumers.
#[derive(Debug, Clone, Copy)]
pub(super) struct ModalDialogView {
    pub background: Color,
    pub border_color: Color,
    pub width: f32,
    pub corner_radius: f32,
    pub padding: f32,
    pub title_font_size: f32,
    pub content_gap: f32,
    pub footer_gap: f32,
}

const OVERLAY_ALPHA: u8 = 160;
const CORNER_RADIUS: f32 = 8.0;
const WIDTH_SM: f32 = 320.0;
const WIDTH_MD: f32 = 480.0;
const WIDTH_LG: f32 = 640.0;
const PADDING: f32 = 24.0;
const TITLE_FONT_SIZE: f32 = 16.0;
const CONTENT_GAP: f32 = 12.0;
const FOOTER_MARGIN_TOP: f32 = 16.0;
const MODAL_EMPTY_SIZE: f32 = crate::floem_view::EMPTY_SIZE;

fn modal_overlay_color(theme: &Theme) -> Color {
    Color {
        r: theme.color.bg.r,
        g: theme.color.bg.g,
        b: theme.color.bg.b,
        a: OVERLAY_ALPHA,
    }
}

fn dialog_bg(theme: &Theme) -> Color {
    theme.color.surface
}

fn dialog_border(theme: &Theme) -> Color {
    theme.color.border
}

fn dialog_width(size: &ModalSize) -> f32 {
    match size {
        ModalSize::Sm => WIDTH_SM,
        ModalSize::Md => WIDTH_MD,
        ModalSize::Lg => WIDTH_LG,
        ModalSize::Custom(w) => *w,
    }
}

fn dialog_corner_radius() -> f32 {
    CORNER_RADIUS
}

fn dialog_padding() -> f32 {
    PADDING
}

fn dialog_title_font_size() -> f32 {
    TITLE_FONT_SIZE
}

fn dialog_content_gap() -> f32 {
    CONTENT_GAP
}

fn dialog_footer_gap() -> f32 {
    FOOTER_MARGIN_TOP
}

/// Returns a view-ready backdrop style for modal overlay.
#[must_use]
pub(super) fn overlay_view(theme: &Theme) -> ModalOverlayView {
    ModalOverlayView {
        background: modal_overlay_color(theme),
    }
}

/// Returns a view-ready dialog style for a given modal size and theme.
#[must_use]
pub(super) fn dialog_view(theme: &Theme, size: &ModalSize) -> ModalDialogView {
    ModalDialogView {
        background: dialog_bg(theme),
        border_color: dialog_border(theme),
        width: dialog_width(size),
        corner_radius: dialog_corner_radius(),
        padding: dialog_padding(),
        title_font_size: dialog_title_font_size(),
        content_gap: dialog_content_gap(),
        footer_gap: dialog_footer_gap(),
    }
}

/// Returns resolved title color for modal dialog title text.
#[must_use]
pub(super) fn title_color(theme: &Theme) -> Color {
    theme.color.text
}

impl OverlayDialog {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let open = create_rw_signal(self.props.open);
        let on_close = Rc::clone(&self.props.on_close);
        let on_focus_return = Rc::clone(&self.props.on_focus_return);
        let title = self.props.title.clone().unwrap_or_default();
        let children = self.props.children.clone().unwrap_or_default();
        let footer = self.props.footer.clone().unwrap_or_default();
        let dismiss_on_backdrop = self.props.dismiss_on_backdrop;
        let size = self.props.size.clone();
        let dismiss_on_esc = self.props.dismiss_on_esc;

        dyn_container(
            move || open.try_get().unwrap_or(false),
            move |is_open| {
                if !is_open {
                    return container(empty())
                        .style(|style| style.width(MODAL_EMPTY_SIZE).height(MODAL_EMPTY_SIZE))
                        .into_any();
                }
                let title_text = title.clone();
                let children_text = children.clone();
                let footer_text = footer.clone();
                let resolved = OverlayDialog {
                    props: ModalProps {
                        open: true,
                        title: Some(title_text.clone()),
                        size: size.clone(),
                        window_placement: self.props.window_placement,
                        parent_interaction: self.props.parent_interaction.clone(),
                        dismiss_on_backdrop,
                        dismiss_on_esc,
                        children: Some(children_text.clone()),
                        footer: Some(footer_text.clone()),
                        on_open: Rc::clone(&self.props.on_open),
                        on_close: Rc::clone(&on_close),
                        on_focus_return: Rc::clone(&on_focus_return),
                    },
                }
                .resolve(&theme);
                let overlay = crate::floem_view::FloemColor::from_token(resolved.overlay_color);
                let dialog_bg = crate::floem_view::FloemColor::from_token(resolved.dialog_bg);
                let border = crate::floem_view::FloemColor::from_token(resolved.dialog_border);
                let title_color = crate::floem_view::FloemColor::from_token(resolved.title_color);
                let dialog = v_stack((
                    label(move || title_text.clone()).style(move |style| {
                        style.font_size(resolved.title_font_size).color(title_color)
                    }),
                    label(move || children_text.clone()),
                    label(move || footer_text.clone()),
                    button(label(|| "Close")).action({
                        let on_close = Rc::clone(&on_close);
                        let on_focus_return = Rc::clone(&on_focus_return);
                        move || {
                            let _ = open.try_update(|value| *value = false);
                            on_close();
                            on_focus_return();
                        }
                    }),
                ))
                .style(move |style| {
                    style
                        .background(dialog_bg)
                        .border(1.0)
                        .border_color(border)
                        .width(resolved.dialog_width)
                        .padding(resolved.padding)
                        .gap(resolved.content_gap)
                        .border_radius(resolved.corner_radius)
                        .outline_color(overlay)
                })
                .on_event_stop(EventListener::PointerDown, |_| {});
                let dialog_id = dialog.id();
                dialog_id.request_focus();
                container(dialog)
                    .keyboard_navigable()
                    .on_event_stop(EventListener::KeyDown, {
                        let on_close = Rc::clone(&on_close);
                        let on_focus_return = Rc::clone(&on_focus_return);
                        move |event| {
                            if let Event::KeyDown(event) = event {
                                if event.key.logical_key == Key::Named(NamedKey::Tab) {
                                    dialog_id.request_focus();
                                    return;
                                }
                                if dismiss_on_esc
                                    && event.key.logical_key == Key::Named(NamedKey::Escape)
                                {
                                    let _ = open.try_update(|value| *value = false);
                                    on_close();
                                    on_focus_return();
                                }
                            }
                        }
                    })
                    .on_event_stop(EventListener::PointerDown, {
                        let on_close = Rc::clone(&on_close);
                        let on_focus_return = Rc::clone(&on_focus_return);
                        move |_| {
                            if dismiss_on_backdrop {
                                let _ = open.try_update(|value| *value = false);
                                on_close();
                                on_focus_return();
                            }
                        }
                    })
                    .style(move |style| {
                        style
                            .background(overlay)
                            .padding(resolved.padding)
                            .width_full()
                    })
                    .into_any()
            },
        )
    }
}
