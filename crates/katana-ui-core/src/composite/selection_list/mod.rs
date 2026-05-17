mod ops;
mod row;
mod types;
mod view;

pub use types::{SelectionListItem, SelectionListSection, SelectionListShowMore};

use crate::theme::Theme;
use floem::IntoView;
use types::SelectionListProps;

/// Builder for sectioned list with selectable items.
pub struct SelectionList {
    pub(crate) props: SelectionListProps,
}

impl SelectionList {
    #[must_use]
    pub fn new(sections: Vec<SelectionListSection>) -> Self {
        Self {
            props: SelectionListProps {
                sections,
                show_more: None,
            },
        }
    }

    #[must_use]
    pub fn show_more(mut self, label: impl Into<String>, on_select: impl Fn() + 'static) -> Self {
        self.props.show_more = Some(SelectionListShowMore::new(label, on_select));
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
    use crate::theme::Theme;

    #[test]
    fn item_selection_callback_executes_once_when_enabled() {
        let called = std::rc::Rc::new(std::cell::RefCell::new(false));
        let called_ref = std::rc::Rc::clone(&called);

        let theme = Theme::default_light();
        let item = SelectionListItem::new("A", theme.color.accent).on_select(move || {
            *called_ref.borrow_mut() = true;
        });

        item.select();

        assert!(*called.borrow());
    }

    #[test]
    fn item_selection_callback_skips_when_disabled() {
        let called = std::rc::Rc::new(std::cell::RefCell::new(false));
        let called_ref = std::rc::Rc::clone(&called);

        let theme = Theme::default_light();
        let item = SelectionListItem::new("A", theme.color.accent)
            .disabled(true)
            .on_select(move || {
                *called_ref.borrow_mut() = true;
            });

        assert_eq!(item.select(), None);
        assert!(!*called.borrow());
    }

    #[test]
    fn show_more_has_label() {
        let theme = Theme::default_light();
        let list = SelectionList::new(vec![SelectionListSection::new(
            "Section",
            vec![SelectionListItem::new("One", theme.color.accent)],
        )])
        .show_more("もっと表示", || {});

        assert!(list.props.show_more.is_some());
    }

    #[test]
    fn selected_item_path_uses_initial_selected_item() {
        let theme = Theme::default_light();
        let sections = vec![SelectionListSection::new(
            "Section",
            vec![
                SelectionListItem::new("One", theme.color.accent),
                SelectionListItem::new("Two", theme.color.warning).selected(true),
            ],
        )];

        let path = ops::SelectionListOps::initial_selected_path(&sections);

        assert_eq!(path, Some(ops::SelectionListItemPath::new(0, 1)));
    }

    #[test]
    fn hidden_items_are_not_visible_until_revealed() {
        assert!(!ops::SelectionListOps::item_visible(true, false));
        assert!(ops::SelectionListOps::item_visible(true, true));
        assert!(ops::SelectionListOps::item_visible(false, false));
    }

    #[test]
    fn show_more_hides_after_hidden_items_are_revealed() {
        assert!(ops::SelectionListOps::show_more_visible(true, false));
        assert!(!ops::SelectionListOps::show_more_visible(true, true));
        assert!(ops::SelectionListOps::show_more_visible(false, true));
    }
}
