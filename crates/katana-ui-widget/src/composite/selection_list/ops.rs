use super::SelectionListSection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SelectionListItemPath {
    pub section_index: usize,
    pub item_index: usize,
}

impl SelectionListItemPath {
    #[must_use]
    pub(crate) const fn new(section_index: usize, item_index: usize) -> Self {
        Self {
            section_index,
            item_index,
        }
    }
}

pub(crate) struct SelectionListOps;

impl SelectionListOps {
    #[must_use]
    pub(crate) fn initial_selected_path(
        sections: &[SelectionListSection],
    ) -> Option<SelectionListItemPath> {
        sections
            .iter()
            .enumerate()
            .find_map(|(section_index, section)| {
                section
                    .items
                    .iter()
                    .position(|item| item.selected)
                    .map(|item_index| SelectionListItemPath::new(section_index, item_index))
            })
    }

    #[must_use]
    pub(crate) fn has_hidden_items(sections: &[SelectionListSection]) -> bool {
        sections
            .iter()
            .any(|section| section.items.iter().any(|item| item.hidden))
    }

    #[must_use]
    pub(crate) const fn item_visible(hidden: bool, hidden_items_revealed: bool) -> bool {
        !hidden || hidden_items_revealed
    }

    #[must_use]
    pub(crate) const fn show_more_visible(
        has_hidden_items: bool,
        hidden_items_revealed: bool,
    ) -> bool {
        !has_hidden_items || !hidden_items_revealed
    }
}
