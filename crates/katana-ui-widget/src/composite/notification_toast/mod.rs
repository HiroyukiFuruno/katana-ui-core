mod layout;
#[cfg(test)]
mod tests;
mod types;
mod view;

pub use types::{
    NotificationToast, NotificationToastPosition, NotificationToastSeverity, NotificationToastStack,
};

use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, container, dyn_container, empty, v_stack_from_iter};
use std::rc::Rc;

use crate::theme::Theme;
use layout::position_toast_stack;
use types::{ActiveToast, NotificationToastAction, NotificationToastProps};

const DEFAULT_MAX_VISIBLE: usize = 3;
const DEFAULT_STACK_GAP: f32 = 8.0;
const CONTAINER_PADDING_LARGE: f32 = 12.0;
const CONTAINER_PADDING_SMALL: f32 = 8.0;
const TOAST_ID_START: u64 = 1;
const TOAST_ID_FALLBACK: u64 = 0;

fn noop() {}

impl NotificationToast {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            props: NotificationToastProps {
                message: message.into(),
                severity: NotificationToastSeverity::Info,
                action: None,
                duration: None,
                on_dismiss: Rc::new(noop),
            },
        }
    }

    #[must_use]
    pub fn severity(mut self, severity: NotificationToastSeverity) -> Self {
        self.props.severity = severity;
        self
    }

    #[must_use]
    pub fn action(mut self, label: impl Into<String>, on_action: impl Fn() + 'static) -> Self {
        self.props.action = Some(NotificationToastAction {
            label: label.into(),
            on_action: Rc::new(on_action),
        });
        self
    }

    #[must_use]
    pub fn duration(mut self, duration_ms: u64) -> Self {
        self.props.duration = Some(duration_ms);
        self
    }

    #[must_use]
    pub fn on_dismiss(mut self, on_dismiss: impl Fn() + 'static) -> Self {
        self.props.on_dismiss = Rc::new(on_dismiss);
        self
    }
}

impl Default for NotificationToast {
    fn default() -> Self {
        Self::new("")
    }
}

impl NotificationToastStack {
    #[must_use]
    pub fn new(toasts: Vec<NotificationToast>) -> Self {
        Self {
            props: toasts,
            position: NotificationToastPosition::default(),
            max_visible: DEFAULT_MAX_VISIBLE,
            gap: DEFAULT_STACK_GAP,
        }
    }

    #[must_use]
    pub fn position(mut self, position: NotificationToastPosition) -> Self {
        self.position = position;
        self
    }

    #[must_use]
    pub fn max_visible(mut self, max_visible: usize) -> Self {
        self.max_visible = max_visible;
        self
    }

    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let max_visible = self.max_visible;
        let position = self.position;
        let gap = self.gap;
        let initial = self
            .props
            .into_iter()
            .enumerate()
            .map(|(index, toast)| ActiveToast {
                id: u64::try_from(index).unwrap_or(TOAST_ID_FALLBACK) + TOAST_ID_START,
                toast,
            })
            .collect::<Vec<_>>();
        let active = create_rw_signal(initial);

        let remove = {
            Rc::new(move |id: u64| {
                let callback = active
                    .try_update(|current| {
                        let target = current.iter().position(|item| item.id == id)?;
                        Some(current.remove(target).toast.props.on_dismiss)
                    })
                    .flatten();

                if let Some(callback) = callback {
                    (callback)();
                }
            })
        };

        let list = dyn_container(
            move || active.get().len(),
            move |_| {
                let rows = active
                    .get()
                    .into_iter()
                    .take(max_visible)
                    .map(|item| {
                        let dismiss = {
                            let remove = Rc::clone(&remove);
                            Rc::new(move || {
                                remove(item.id);
                            })
                        };
                        view::render_toast(theme.clone(), item.toast.clone(), dismiss)
                    })
                    .collect::<Vec<_>>();

                let stack = if rows.is_empty() {
                    container(empty()).into_any()
                } else {
                    v_stack_from_iter(rows)
                        .style(move |style| style.gap(gap))
                        .into_any()
                };

                position_toast_stack(stack, position)
            },
        )
        .style(|style| {
            style
                .width_full()
                .height_full()
                .padding(CONTAINER_PADDING_LARGE)
        });

        container(list).style(|style| {
            style
                .width_full()
                .height_full()
                .padding(CONTAINER_PADDING_SMALL)
        })
    }
}

impl Default for NotificationToastStack {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
