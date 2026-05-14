use std::rc::Rc;

use floem::views::{container, label};
use floem::{IntoView, View};

use crate::layout::popover::Placement;
use crate::primitive::icon::IconSource;

const DEFAULT_LABEL: &str = "Menu";

/// Callback fired when the menu opens.
pub type MenuButtonOpenCallback = Rc<dyn Fn()>;

/// Callback fired when the menu closes.
pub type MenuButtonCloseCallback = Rc<dyn Fn()>;

/// Factory for arbitrary menu content.
pub type MenuButtonContentFactory = Rc<dyn Fn(MenuButtonCloseCallback) -> Box<dyn View>>;

/// Factory for node-only trigger.
pub type MenuButtonTriggerFactory = dyn Fn() -> Box<dyn View>;

/// Appearance variant for trigger rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuButtonVariant {
    #[default]
    Framed,
    Unframed,
}

/// Trigger shape for `MenuButton`.
pub enum MenuButtonTrigger {
    Label(String),
    Icon(IconSource),
    Node(Box<MenuButtonTriggerFactory>),
}

impl Default for MenuButtonTrigger {
    fn default() -> Self {
        Self::Label(DEFAULT_LABEL.to_string())
    }
}

/// Properties for `MenuButton`.
pub struct MenuButtonProps {
    pub variant: MenuButtonVariant,
    pub trigger: MenuButtonTrigger,
    pub content: MenuButtonContentFactory,
    pub on_open: MenuButtonOpenCallback,
    pub on_close: MenuButtonCloseCallback,
    pub placement: Placement,
    pub open: bool,
}

fn noop() {}

pub(crate) struct MenuButtonDefaults;

impl MenuButtonDefaults {
    pub(crate) fn default_content() -> MenuButtonContentFactory {
        Rc::new(|_| -> Box<dyn View> { container(label(|| "")).into_any() })
    }

    pub(crate) fn default_trigger() -> MenuButtonTrigger {
        MenuButtonTrigger::Label(DEFAULT_LABEL.to_string())
    }

    pub(crate) fn noop_open_callback() -> MenuButtonOpenCallback {
        Rc::new(noop)
    }

    pub(crate) fn noop_close_callback() -> MenuButtonCloseCallback {
        Rc::new(noop)
    }
}
