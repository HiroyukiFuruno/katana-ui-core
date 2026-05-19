pub use crate::render_model::{
    UiContextMenuAnchor as ContextMenuAnchor, UiContextMenuItem as ContextMenuItem,
    UiContextMenuItemKind as ContextMenuItemKind, UiContextMenuPlacement as ContextMenuPlacement,
    UiContextMenuRect as ContextMenuRect,
};

use crate::molecule::selection::types::ChoiceItem;
use crate::render_model::UiContextMenuItem;

impl From<ChoiceItem> for UiContextMenuItem {
    fn from(value: ChoiceItem) -> Self {
        Self::action(value.value, value.label).disabled(value.disabled)
    }
}
