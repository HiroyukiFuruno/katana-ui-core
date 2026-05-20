pub use crate::render_model::{
    UiContextMenuAnchor as ContextMenuAnchor, UiContextMenuDividerTone as ContextMenuDividerTone,
    UiContextMenuItem as ContextMenuItem, UiContextMenuItemKind as ContextMenuItemKind,
    UiContextMenuPlacement as ContextMenuPlacement, UiContextMenuRect as ContextMenuRect,
};

use crate::molecule::selection::types::ChoiceItem;
use crate::render_model::UiContextMenuItem;

impl From<ChoiceItem> for UiContextMenuItem {
    fn from(value: ChoiceItem) -> Self {
        Self::action(value.value, value.label).disabled(value.disabled)
    }
}

impl UiContextMenuItem {
    #[must_use]
    pub fn from_choice_item(value: ChoiceItem) -> Self {
        Self::from(value)
    }

    #[must_use]
    pub fn to_choice_item(&self) -> Option<ChoiceItem> {
        if self.kind != crate::render_model::UiContextMenuItemKind::Action {
            return None;
        }
        Some(ChoiceItem::new(self.id.clone(), self.label.clone()).disabled(self.disabled))
    }
}
