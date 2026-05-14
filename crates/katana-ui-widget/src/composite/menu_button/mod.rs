mod overlay;
mod style;
mod trigger;
mod types;
mod view;

pub use types::{
    MenuButtonCloseCallback, MenuButtonContentFactory, MenuButtonOpenCallback, MenuButtonTrigger,
    MenuButtonTriggerFactory, MenuButtonVariant,
};

pub use crate::layout::popover::{FreePlacement, Placement as MenuButtonPlacement};
use crate::theme::Theme;
use floem::IntoView;
use std::rc::Rc;
use types::{MenuButtonDefaults, MenuButtonProps};

/// Widget builder for menu-style button with popover content.
pub struct MenuButton {
    props: MenuButtonProps,
}

impl MenuButton {
    #[must_use]
    pub fn new() -> Self {
        Self {
            props: MenuButtonProps {
                variant: MenuButtonVariant::Framed,
                trigger: MenuButtonDefaults::default_trigger(),
                content: MenuButtonDefaults::default_content(),
                on_open: MenuButtonDefaults::noop_open_callback(),
                on_close: MenuButtonDefaults::noop_close_callback(),
                placement: MenuButtonPlacement::BottomStart,
                open: false,
            },
        }
    }

    #[must_use]
    pub fn variant(mut self, variant: MenuButtonVariant) -> Self {
        self.props.variant = variant;
        self
    }

    #[must_use]
    pub fn framed(mut self) -> Self {
        self.props.variant = MenuButtonVariant::Framed;
        self
    }

    #[must_use]
    pub fn unframed(mut self) -> Self {
        self.props.variant = MenuButtonVariant::Unframed;
        self
    }

    #[must_use]
    pub fn trigger_label(mut self, label: impl Into<String>) -> Self {
        self.props.trigger = MenuButtonTrigger::Label(label.into());
        self
    }

    #[must_use]
    pub fn trigger_icon(mut self, icon: crate::primitive::icon::IconSource) -> Self {
        self.props.trigger = MenuButtonTrigger::Icon(icon);
        self
    }

    #[must_use]
    pub fn trigger_node<V>(mut self, trigger: impl Fn() -> V + 'static) -> Self
    where
        V: floem::IntoView + 'static,
    {
        self.props.trigger = MenuButtonTrigger::Node(Box::new(move || trigger().into_any()));
        self
    }

    #[must_use]
    pub fn content<V>(mut self, content: impl Fn(MenuButtonCloseCallback) -> V + 'static) -> Self
    where
        V: floem::IntoView + 'static,
    {
        self.props.content = Rc::new(move |close| content(close).into_any());
        self
    }

    #[must_use]
    pub fn on_open(mut self, on_open: impl Fn() + 'static) -> Self {
        self.props.on_open = Rc::new(on_open);
        self
    }

    #[must_use]
    pub fn on_close(mut self, on_close: impl Fn() + 'static) -> Self {
        self.props.on_close = Rc::new(on_close);
        self
    }

    #[must_use]
    pub fn placement(mut self, placement: MenuButtonPlacement) -> Self {
        self.props.placement = placement;
        self
    }

    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.props.open = open;
        self
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        view::build_view(self.props, theme)
    }
}

impl Default for MenuButton {
    fn default() -> Self {
        Self::new()
    }
}
