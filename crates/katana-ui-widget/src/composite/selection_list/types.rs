use std::rc::Rc;

use floem::{IntoView, View};

use crate::theme::color::Color;

fn noop_select() {}

/// Single selectable row item.
pub struct SelectionListItem {
    pub label: String,
    pub marker_color: Color,
    pub selected: bool,
    pub disabled: bool,
    pub hidden: bool,
    pub on_select: Rc<dyn Fn()>,
    pub content: Option<Box<dyn View>>,
}

impl SelectionListItem {
    #[must_use]
    pub fn new(label: impl Into<String>, marker_color: Color) -> Self {
        Self {
            label: label.into(),
            marker_color,
            selected: false,
            disabled: false,
            hidden: false,
            on_select: Rc::new(noop_select),
            content: None,
        }
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
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn() + 'static) -> Self {
        self.on_select = Rc::new(on_select);
        self
    }

    #[must_use]
    pub fn content<V>(mut self, content: V) -> Self
    where
        V: IntoView + 'static,
    {
        self.content = Some(content.into_any());
        self
    }

    /// Select this item and notify callback unless disabled.
    pub fn select(&self) -> Option<()> {
        if self.disabled {
            return None;
        }

        (self.on_select)();
        Some(())
    }
}

/// Optional callback rendered at the list tail.
pub struct SelectionListShowMore {
    pub label: String,
    pub on_select: Rc<dyn Fn()>,
}

impl SelectionListShowMore {
    #[must_use]
    pub fn new(label: impl Into<String>, on_select: impl Fn() + 'static) -> Self {
        Self {
            label: label.into(),
            on_select: Rc::new(on_select),
        }
    }

    pub fn trigger(&self) {
        (self.on_select)();
    }
}

/// Section container for grouped rows.
pub struct SelectionListSection {
    pub label: String,
    pub items: Vec<SelectionListItem>,
}

impl SelectionListSection {
    #[must_use]
    pub fn new(label: impl Into<String>, items: Vec<SelectionListItem>) -> Self {
        Self {
            label: label.into(),
            items,
        }
    }
}

pub(crate) struct SelectionListProps {
    pub sections: Vec<SelectionListSection>,
    pub show_more: Option<SelectionListShowMore>,
}
