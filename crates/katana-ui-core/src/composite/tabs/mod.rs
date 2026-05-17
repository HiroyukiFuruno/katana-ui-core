mod types;
mod view;

pub use types::TabItem;

use crate::theme::Theme;
use floem::IntoView;
use types::TabsProps;

/// Builder for the `Tabs` composite widget.
pub struct Tabs {
    pub(crate) props: TabsProps,
}

impl Tabs {
    #[must_use]
    pub fn new(items: Vec<TabItem>) -> Self {
        Self {
            props: TabsProps {
                items,
                overflow: false,
            },
        }
    }

    #[must_use]
    pub fn overflow(mut self, overflow: bool) -> Self {
        self.props.overflow = overflow;
        self
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        view::build_view(self, theme)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn select_callback_executes_when_enabled() {
        let called = Rc::new(RefCell::new(false));
        let called_ref = Rc::clone(&called);

        let item = TabItem::new("A").on_select(move || {
            *called_ref.borrow_mut() = true;
        });

        assert_eq!(item.select(), Some(()));
        assert!(*called.borrow());
    }

    #[test]
    fn select_callback_skips_when_disabled() {
        let called = Rc::new(RefCell::new(false));
        let called_ref = Rc::clone(&called);

        let item = TabItem::new("A").disabled(true).on_select(move || {
            *called_ref.borrow_mut() = true;
        });

        assert_eq!(item.select(), None);
        assert!(!*called.borrow());
    }

    #[test]
    fn overflow_setting_is_retained() {
        let tabs = Tabs::new(vec![TabItem::new("A")]).overflow(true);
        assert!(tabs.props.overflow);
    }
}
