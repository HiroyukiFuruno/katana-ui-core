mod ops;
mod render;
mod types;
mod view;

pub use types::{
    CallbackCommandPaletteProvider, CommandPalette, CommandPaletteItem, CommandPaletteProvider,
};

use crate::theme::Theme;
use floem::IntoView;
use std::rc::Rc;
use types::{CommandPaletteDefaults, CommandPaletteProps};

impl<P: Clone + 'static> CommandPalette<P> {
    #[must_use]
    pub fn new(provider: impl CommandPaletteProvider<P> + 'static) -> Self {
        Self {
            props: CommandPaletteProps {
                provider: Rc::new(provider),
                on_execute: Rc::new(CommandPaletteDefaults::noop_execute::<P>),
                on_selection_change: Rc::new(CommandPaletteDefaults::noop_selection),
                on_query: Rc::new(CommandPaletteDefaults::noop_query),
                on_close: Rc::new(CommandPaletteDefaults::noop_close),
                placeholder: "Search".to_string(),
                disabled: false,
            },
        }
    }

    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.props.placeholder = placeholder.into();
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn on_execute(mut self, on_execute: impl Fn(String, usize, P) + 'static) -> Self {
        self.props.on_execute = Rc::new(on_execute);
        self
    }

    #[must_use]
    pub fn on_selection_change(
        mut self,
        on_selection_change: impl Fn(String, usize) + 'static,
    ) -> Self {
        self.props.on_selection_change = Rc::new(on_selection_change);
        self
    }

    #[must_use]
    pub fn on_query(mut self, on_query: impl Fn(String) + 'static) -> Self {
        self.props.on_query = Rc::new(on_query);
        self
    }

    #[must_use]
    pub fn on_close(mut self, on_close: impl Fn() + 'static) -> Self {
        self.props.on_close = Rc::new(on_close);
        self
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        self.build_view(theme)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::icon::IconSource;
    use std::cell::RefCell;

    #[test]
    fn provider_returns_payload_icon_shortcut_and_sorted_score() {
        let provider = CallbackCommandPaletteProvider::new(|_| {
            vec![
                CommandPaletteItem::new("B", "b")
                    .score(1)
                    .shortcut("Cmd+B")
                    .icon(IconSource::SvgString("<svg />".to_string())),
                CommandPaletteItem::new("A", "a").score(10),
            ]
        });
        let mut items = provider.query("");

        crate::composite::command_palette::ops::sort_by_score(&mut items);

        assert_eq!(items[0].label, "A");
        assert_eq!(items[1].payload, "b");
        assert_eq!(items[1].shortcut.as_deref(), Some("Cmd+B"));
        assert!(items[1].icon.is_some());
    }

    #[test]
    fn query_selection_execute_and_close_callbacks_are_separate() {
        let query = Rc::new(RefCell::new(None));
        let selection = Rc::new(RefCell::new(None));
        let execute = Rc::new(RefCell::new(None));
        let close = Rc::new(RefCell::new(false));
        let query_ref = Rc::clone(&query);
        let selection_ref = Rc::clone(&selection);
        let execute_ref = Rc::clone(&execute);
        let close_ref = Rc::clone(&close);
        let palette = CommandPalette::new(CallbackCommandPaletteProvider::new(|_| {
            vec![CommandPaletteItem::new("Open", "payload")]
        }))
        .on_query(move |value| {
            *query_ref.borrow_mut() = Some(value);
        })
        .on_selection_change(move |value, index| {
            *selection_ref.borrow_mut() = Some((value, index));
        })
        .on_execute(move |value, index, payload| {
            *execute_ref.borrow_mut() = Some((value, index, payload));
        })
        .on_close(move || {
            *close_ref.borrow_mut() = true;
        });

        (palette.props.on_query)("open".to_string());
        (palette.props.on_selection_change)("open".to_string(), 0);
        (palette.props.on_execute)("open".to_string(), 0, "payload");
        (palette.props.on_close)();

        assert_eq!(*query.borrow(), Some("open".to_string()));
        assert_eq!(*selection.borrow(), Some(("open".to_string(), 0)));
        assert_eq!(*execute.borrow(), Some(("open".to_string(), 0, "payload")));
        assert!(*close.borrow());
    }
}
