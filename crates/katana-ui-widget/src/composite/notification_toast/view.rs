use crate::floem_view::FloemColor;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::action::exec_after;
use floem::reactive::{SignalUpdate, create_rw_signal};
use floem::views::{Decorators, button, container, h_stack, label};
use floem::{IntoView, View};
use std::rc::Rc;
use std::time::Duration;

use super::types::{NotificationToast, NotificationToastAction, NotificationToastSeverity};

const TOAST_RADIUS: f32 = crate::floem_view::CORNER_RADIUS_SM;
const TOAST_PADDING: f32 = crate::floem_view::GAP_SM;
const TOAST_ICON_SIZE: f32 = 16.0;
const TOAST_MIN_WIDTH: f32 = 260.0;
const TOAST_GAP_SMALL: f32 = 6.0;
const TOAST_GAP_NORMAL: f32 = 8.0;
const TOAST_BORDER_WIDTH: f32 = 1.0;
const BUTTON_PADDING_VERT: f32 = 2.0;
const BUTTON_PADDING_HORIZ: f32 = 8.0;
const ACTION_LABEL_SIZE: f32 = 11.0;
const CLOSE_LABEL_SIZE: f32 = 14.0;
const INFO_BACKGROUND_ALPHA: u8 = 24;
const SEVERITY_BACKGROUND_ALPHA: u8 = 30;

#[derive(Clone)]
struct ResolvedNotificationToast {
    message: String,
    icon: &'static str,
    icon_color: Color,
    bg_color: Color,
    border_color: Color,
    action: Option<NotificationToastAction>,
    duration_ms: Option<u64>,
}

impl NotificationToast {
    fn resolve(&self, theme: &Theme) -> ResolvedNotificationToast {
        let (icon_color, bg_color) = match self.props.severity {
            NotificationToastSeverity::Info => (
                theme.color.accent,
                Color {
                    a: INFO_BACKGROUND_ALPHA,
                    ..theme.color.accent
                },
            ),
            NotificationToastSeverity::Success => (
                theme.color.success,
                Color {
                    a: SEVERITY_BACKGROUND_ALPHA,
                    ..theme.color.success
                },
            ),
            NotificationToastSeverity::Warning => (
                theme.color.warning,
                Color {
                    a: SEVERITY_BACKGROUND_ALPHA,
                    ..theme.color.warning
                },
            ),
            NotificationToastSeverity::Error => (
                theme.color.danger,
                Color {
                    a: SEVERITY_BACKGROUND_ALPHA,
                    ..theme.color.danger
                },
            ),
        };

        let icon = match self.props.severity {
            NotificationToastSeverity::Info => "ⓘ",
            NotificationToastSeverity::Success => "✓",
            NotificationToastSeverity::Warning => "⭑",
            NotificationToastSeverity::Error => "⚠",
        };

        ResolvedNotificationToast {
            message: self.props.message.clone(),
            icon,
            icon_color,
            bg_color,
            border_color: icon_color,
            action: self.props.action.clone(),
            duration_ms: self.props.duration,
        }
    }
}

fn dismiss_handler(dismiss: Rc<dyn Fn()>) -> Rc<dyn Fn()> {
    let dismissed = create_rw_signal(false);
    Rc::new(move || {
        let first_time = dismissed
            .try_update(|state| {
                if *state {
                    false
                } else {
                    *state = true;
                    true
                }
            })
            .unwrap_or(false);
        if first_time {
            dismiss();
        }
    })
}

fn action_button(action: NotificationToastAction) -> Box<dyn View> {
    let action_label = action.label;
    let on_action = Rc::clone(&action.on_action);
    button(
        container(label(move || action_label.clone()))
            .style(|style| style.font_size(ACTION_LABEL_SIZE)),
    )
    .action(move || {
        (on_action)();
    })
    .style(move |style| {
        style
            .padding_vert(BUTTON_PADDING_VERT)
            .padding_horiz(BUTTON_PADDING_HORIZ)
            .border(TOAST_BORDER_WIDTH)
            .border_radius(TOAST_RADIUS)
    })
    .into_any()
}

pub(super) fn render_toast(
    theme: Theme,
    toast: NotificationToast,
    on_dismiss: Rc<dyn Fn()>,
) -> Box<dyn View> {
    let resolved = toast.resolve(&theme);
    let dismiss = dismiss_handler(on_dismiss);

    if let Some(duration_ms) = resolved.duration_ms {
        let duration = Duration::from_millis(duration_ms);
        let dismiss_on_timeout = Rc::clone(&dismiss);
        exec_after(duration, move |_| {
            dismiss_on_timeout();
        });
    }

    let bg_color = FloemColor::from_token(resolved.bg_color);
    let icon_color = FloemColor::from_token(resolved.icon_color);
    let text_color = FloemColor::from_token(theme.color.text);
    let border_color = FloemColor::from_token(resolved.border_color);

    let action_slot = resolved.action.map(action_button);

    let close_slot =
        button(container(label(|| "×")).style(|style| style.font_size(CLOSE_LABEL_SIZE)))
            .action(move || {
                dismiss();
            })
            .style(move |style| {
                style
                    .padding_vert(BUTTON_PADDING_VERT)
                    .padding_horiz(BUTTON_PADDING_HORIZ)
                    .border(TOAST_BORDER_WIDTH)
                    .border_radius(TOAST_RADIUS)
                    .border_color(border_color)
            })
            .into_any();

    let trailing = match action_slot {
        Some(action_slot) => h_stack((action_slot, close_slot))
            .style(|style| style.gap(TOAST_GAP_SMALL).items_center())
            .into_any(),
        None => close_slot,
    };

    h_stack((
        h_stack((
            container(label(move || resolved.icon.to_string()))
                .style(move |style| style.font_size(TOAST_ICON_SIZE).color(icon_color)),
            container(label(move || resolved.message.clone())).style(move |style| {
                style
                    .font_size(theme.typography.body.font_size)
                    .color(text_color)
            }),
        ))
        .style(|style| style.gap(TOAST_GAP_NORMAL).items_center())
        .into_any(),
        trailing,
    ))
    .style(move |style| {
        style
            .min_width(TOAST_MIN_WIDTH)
            .padding(TOAST_PADDING)
            .gap(TOAST_GAP_NORMAL)
            .items_center()
            .justify_between()
            .border(TOAST_BORDER_WIDTH)
            .border_color(border_color)
            .border_radius(TOAST_RADIUS)
            .background(bg_color)
    })
    .into_any()
}
