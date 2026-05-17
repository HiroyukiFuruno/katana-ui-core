mod layout;
mod types;
mod view;

pub use types::{DynamicArrayEditorItem, DynamicArrayItemRenderer};

use crate::theme::Theme;
use floem::IntoView;
use std::rc::Rc;

use types::{DynamicArrayEditorItem as Item, DynamicArrayEditorProps as Props};

const DEFAULT_EMPTY_STATE: &str = "items がありません";

fn noop_change<T>(_: Vec<Item<T>>) {}
fn noop_index(_: usize) {}
fn noop_move(_: usize, _: usize) {}

/// Builder for the DynamicArrayEditor composite widget.
pub struct DynamicArrayEditor<T> {
    props: Props<T>,
}

impl<T: Clone + 'static> DynamicArrayEditor<T> {
    #[must_use]
    pub fn new<V>(
        items: Vec<Item<T>>,
        create_item: impl Fn() -> Item<T> + 'static,
        render: impl Fn(&Item<T>, usize) -> V + 'static,
    ) -> Self
    where
        V: floem::IntoView + 'static,
    {
        Self {
            props: Props {
                items,
                max_items: None,
                disabled: false,
                empty_state: DEFAULT_EMPTY_STATE.to_string(),
                item_renderer: Rc::new(move |item, index| render(item, index).into_any()),
                create_item: Rc::new(create_item),
                on_change: Rc::new(noop_change::<T>),
                on_add: Rc::new(noop_index),
                on_edit: Rc::new(noop_index),
                on_delete: Rc::new(noop_index),
                on_move: Rc::new(noop_move),
            },
        }
    }

    #[must_use]
    pub fn max_items(mut self, max_items: usize) -> Self {
        self.props.max_items = Some(max_items);
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn empty_state(mut self, empty_state: impl Into<String>) -> Self {
        self.props.empty_state = empty_state.into();
        self
    }

    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Vec<Item<T>>) + 'static) -> Self {
        self.props.on_change = Rc::new(on_change);
        self
    }

    #[must_use]
    pub fn on_add(mut self, on_add: impl Fn(usize) + 'static) -> Self {
        self.props.on_add = Rc::new(on_add);
        self
    }

    #[must_use]
    pub fn on_edit(mut self, on_edit: impl Fn(usize) + 'static) -> Self {
        self.props.on_edit = Rc::new(on_edit);
        self
    }

    #[must_use]
    pub fn on_delete(mut self, on_delete: impl Fn(usize) + 'static) -> Self {
        self.props.on_delete = Rc::new(on_delete);
        self
    }

    #[must_use]
    pub fn on_move(mut self, on_move: impl Fn(usize, usize) + 'static) -> Self {
        self.props.on_move = Rc::new(on_move);
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
    use floem::views::label;
    use std::cell::RefCell;

    #[test]
    fn builder_keeps_requirement_options() {
        let editor = DynamicArrayEditor::new(
            vec![Item::new("A".to_string())],
            || Item::new("B".to_string()),
            |item, _| {
                let value = item.value.clone();
                label(move || value.clone())
            },
        )
        .max_items(4)
        .empty_state("空です")
        .disabled(true);

        assert_eq!(editor.props.items.len(), 1);
        assert_eq!(editor.props.max_items, Some(4));
        assert_eq!(editor.props.empty_state, "空です");
        assert!(editor.props.disabled);
    }

    #[test]
    fn callbacks_are_individually_callable() {
        let add_index = Rc::new(RefCell::new(None));
        let delete_index = Rc::new(RefCell::new(None));
        let move_pair = Rc::new(RefCell::new(None));
        let add_ref = Rc::clone(&add_index);
        let delete_ref = Rc::clone(&delete_index);
        let move_ref = Rc::clone(&move_pair);
        let editor = DynamicArrayEditor::new(
            Vec::<Item<String>>::new(),
            || Item::new("A".to_string()),
            |item, _| {
                let value = item.value.clone();
                label(move || value.clone())
            },
        )
        .on_add(move |index| {
            *add_ref.borrow_mut() = Some(index);
        })
        .on_delete(move |index| {
            *delete_ref.borrow_mut() = Some(index);
        })
        .on_move(move |from, to| {
            *move_ref.borrow_mut() = Some((from, to));
        });

        (editor.props.on_add)(2);
        (editor.props.on_delete)(1);
        (editor.props.on_move)(1, 0);

        assert_eq!(*add_index.borrow(), Some(2));
        assert_eq!(*delete_index.borrow(), Some(1));
        assert_eq!(*move_pair.borrow(), Some((1, 0)));
    }
}
