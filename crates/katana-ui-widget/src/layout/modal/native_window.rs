use super::Modal;
use super::ops;
use super::placement;
use super::types::{ModalOpenError, ModalParentInteraction, ModalProps, ModalSize};
use crate::theme::Theme;
use floem::action::exec_after;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::views::{Decorators, button, container, empty, h_stack, label, v_stack};
use floem::window::{WindowConfig, WindowId, WindowLevel};
use floem::{IntoView, View, ViewId, close_window, new_window};
use std::rc::Rc;
use std::time::Duration;

const WINDOW_HEIGHT: f64 = 280.0;
const WINDOW_MIN_WIDTH: f64 = 360.0;
const WIDTH_SM: f32 = 320.0;
const WIDTH_MD: f32 = 480.0;
const WIDTH_LG: f32 = 640.0;
const WINDOW_PADDING: f32 = 16.0;
const WINDOW_GAP: f32 = 10.0;
const BUTTON_GAP: f32 = 8.0;
const BODY_FONT_SIZE: f32 = 12.0;
const FOOTER_FONT_SIZE: f32 = 11.0;
const DEFER_NATIVE_WINDOW_MS: u64 = 1;
const ERROR_TEXT_RED: u8 = 200;
const ERROR_TEXT_GREEN: u8 = 40;
const ERROR_TEXT_BLUE: u8 = 40;

impl Modal {
    pub fn open_window(self, theme: Theme) -> Result<bool, ModalOpenError> {
        if !self.props.open {
            return Ok(false);
        }

        let props = self.props;
        let width = window_width(&props.size);
        let window_level = window_level_for_parent_interaction(&props.parent_interaction);
        let mut config = WindowConfig::default()
            .title(window_title(&props))
            .size((width, WINDOW_HEIGHT))
            .resizable(false)
            .window_level(window_level);

        if let Some(position) =
            placement::window_position(props.window_placement, width, WINDOW_HEIGHT)?
        {
            config = config.position(position);
        }

        exec_after(Duration::from_millis(DEFER_NATIVE_WINDOW_MS), move |_| {
            let on_open = Rc::clone(&props.on_open);
            new_window(
                move |window_id| {
                    on_open();
                    modal_window_view(window_id, props, theme)
                },
                Some(config),
            );
        });
        Ok(true)
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> Box<dyn View> {
        match self.open_window(theme) {
            Ok(_) => empty().into_any(),
            Err(error) => modal_open_error_view(error).into_any(),
        }
    }
}

fn modal_open_error_view(error: ModalOpenError) -> impl IntoView {
    label(move || format!("Modal native window open failed: {error}")).style(|style| {
        style
            .color(floem::peniko::Color::rgb8(
                ERROR_TEXT_RED,
                ERROR_TEXT_GREEN,
                ERROR_TEXT_BLUE,
            ))
            .padding(WINDOW_PADDING)
    })
}

fn window_title(props: &ModalProps) -> String {
    props.title.clone().unwrap_or_else(|| "Modal".to_string())
}

fn window_width(size: &ModalSize) -> f64 {
    let width = match size {
        ModalSize::Sm => WIDTH_SM,
        ModalSize::Md => WIDTH_MD,
        ModalSize::Lg => WIDTH_LG,
        ModalSize::Custom(width) => *width,
    };
    f64::from(width.max(WINDOW_MIN_WIDTH as f32))
}

pub(super) fn window_level_for_parent_interaction(
    parent_interaction: &ModalParentInteraction,
) -> WindowLevel {
    match parent_interaction {
        ModalParentInteraction::Block => WindowLevel::AlwaysOnTop,
        ModalParentInteraction::Allow => WindowLevel::Normal,
    }
}

fn modal_window_view(window_id: WindowId, props: ModalProps, theme: Theme) -> impl IntoView {
    let title = window_title(&props);
    let body = props.children.clone().unwrap_or_default();
    let footer = props.footer.clone().unwrap_or_default();
    let on_close = Rc::clone(&props.on_close);
    let on_focus_return = Rc::clone(&props.on_focus_return);
    let dismiss_on_esc = props.dismiss_on_esc;
    let trap_focus = ops::should_trap_tab_navigation(&props);
    let resolved = Modal { props }.resolve(&theme);
    let dialog_bg = crate::floem_view::FloemColor::from_token(resolved.dialog_bg);
    let dialog_border = crate::floem_view::FloemColor::from_token(resolved.dialog_border);
    let text_color = crate::floem_view::FloemColor::from_token(resolved.title_color);
    let close_button = close_button(window_id, Rc::clone(&on_close), Rc::clone(&on_focus_return));

    let dialog = v_stack((
        label(move || title.clone())
            .style(move |style| style.font_size(resolved.title_font_size).color(text_color)),
        label(move || body.clone()).style(|style| style.font_size(BODY_FONT_SIZE)),
        label(move || footer.clone()).style(move |style| {
            style
                .font_size(FOOTER_FONT_SIZE)
                .margin_top(resolved.footer_gap)
        }),
        h_stack((close_button,)).style(|style| style.gap(BUTTON_GAP)),
    ))
    .style(move |style| {
        style
            .background(dialog_bg)
            .border(1.0)
            .border_color(dialog_border)
            .width(resolved.dialog_width)
            .padding(resolved.padding)
            .gap(resolved.content_gap)
            .border_radius(resolved.corner_radius)
            .color(text_color)
    });
    let dialog_id = dialog.id();
    raise_modal_window(dialog_id);

    container(dialog)
        .keyboard_navigable()
        .on_event_stop(EventListener::KeyDown, move |event| {
            if trap_focus_on_tab(event, dialog_id, trap_focus) {
                return;
            }
            close_by_escape(
                event,
                dismiss_on_esc,
                window_id,
                Rc::clone(&on_close),
                Rc::clone(&on_focus_return),
            );
        })
        .style(|style| style.padding(WINDOW_PADDING).gap(WINDOW_GAP))
}

fn raise_modal_window(dialog_id: ViewId) {
    /* WHY: macOS では window level だけでは前面化が保証されないため、生成後に明示的に表示し直す。 */
    exec_after(Duration::from_millis(DEFER_NATIVE_WINDOW_MS), move |_| {
        dialog_id.window_visible(true);
        dialog_id.request_focus();
    });
}

fn trap_focus_on_tab(event: &Event, dialog_id: ViewId, trap_focus: bool) -> bool {
    match event {
        Event::KeyDown(event)
            if trap_focus && event.key.logical_key == Key::Named(NamedKey::Tab) =>
        {
            dialog_id.request_focus();
            true
        }
        _ => false,
    }
}

fn close_button(
    window_id: WindowId,
    on_close: Rc<dyn Fn()>,
    on_focus_return: Rc<dyn Fn()>,
) -> impl IntoView {
    button(label(|| "Close")).action(move || {
        on_close();
        on_focus_return();
        close_window(window_id);
    })
}

fn close_by_escape(
    event: &Event,
    dismiss_on_esc: bool,
    window_id: WindowId,
    on_close: Rc<dyn Fn()>,
    on_focus_return: Rc<dyn Fn()>,
) {
    match event {
        Event::KeyDown(event)
            if dismiss_on_esc && event.key.logical_key == Key::Named(NamedKey::Escape) =>
        {
            on_close();
            on_focus_return();
            close_window(window_id);
        }
        _ => (),
    }
}
