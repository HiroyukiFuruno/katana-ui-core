use crate::primitive::icon::IconSource;
use std::rc::Rc;

fn noop_select() {}

/// Properties for a single tab item.
pub struct TabItem {
    pub label: String,
    pub icon: Option<IconSource>,
    pub selected: bool,
    pub disabled: bool,
    pub on_select: Rc<dyn Fn()>,
    pub on_close: Option<Rc<dyn Fn()>>,
    pub content: Option<Rc<dyn Fn() -> Box<dyn floem::View>>>,
}

impl TabItem {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            selected: false,
            disabled: false,
            on_select: Rc::new(noop_select),
            on_close: None,
            content: None,
        }
    }

    #[must_use]
    pub fn icon(mut self, icon: IconSource) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn() + 'static) -> Self {
        self.on_select = Rc::new(on_select);
        self
    }

    #[must_use]
    pub fn on_close(mut self, on_close: impl Fn() + 'static) -> Self {
        self.on_close = Some(Rc::new(on_close));
        self
    }

    #[must_use]
    pub fn content<V, F>(mut self, content: F) -> Self
    where
        V: floem::IntoView + 'static,
        F: Fn() -> V + 'static,
    {
        self.content = Some(Rc::new(move || content().into_any()));
        self
    }

    /// Select this tab and notify callback unless disabled.
    pub fn select(&self) -> Option<()> {
        if self.disabled {
            return None;
        }

        (self.on_select)();
        Some(())
    }
}

#[derive(Default)]
pub(crate) struct TabsProps {
    pub items: Vec<TabItem>,
    pub overflow: bool,
}
