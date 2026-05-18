mod basic;
mod color;
mod diff;
mod disclosure;
mod disclosure_foundation;
mod selection;
mod state;
mod structured;

pub use basic::{Card, FormField, List, Menu, MoleculeEventRouting, StatusBar, Toolbar};
pub use color::{ColorBlendingMode, ColorPicker, RgbaColor};
pub use diff::{
    CodeDiff, CodeDiffDirection, CodeDiffLine, CodeDiffLineKind, CodeDiffMode, CodeDiffSource,
    CodeDiffWhitespace, CollapsedBlock, HighlightRange,
};
pub use disclosure::{
    Accordion, Modal, ModalOverlay, NotificationToast, Popover, SearchBox, SegmentedToggle,
    SlideControl, Tooltip,
};
pub use disclosure_foundation::DisclosureTriggerArea;
pub use selection::{
    Breadcrumb, ChoiceItem, ComboBox, MenuButton, SelectBox, SelectionList, SideMenu, Tabs,
};
pub use structured::{
    ArrayEditorItem, CommandItem, CommandPalette, DynamicArrayEditor, TreeLineStyle, TreeNode,
    TreeNodeKind, TreeView,
};

#[cfg(test)]
mod tests {
    use super::MoleculeEventRouting;
    use super::{Card, Toolbar};
    use crate::atom::Button;
    use crate::render_model::{UiNodeId, UiNodeKind, UiTree};

    #[test]
    fn molecule_snapshot_keeps_children() {
        let tree = UiTree::new(Toolbar::new("main").child(Button::new("Save")));
        assert_eq!(1, tree.root().children().len());
    }

    #[test]
    fn card_uses_molecule_kind() {
        let tree = UiTree::new(Card::new("summary"));
        assert_eq!(UiNodeKind::Card, tree.root().kind());
    }

    #[test]
    fn molecule_event_routing_visits_nested_target_then_parents() {
        let route = MoleculeEventRouting::bubble_nested(
            UiNodeId::new("button"),
            UiNodeId::new("toolbar"),
            UiNodeId::new("root"),
            false,
        );
        let actual: Vec<&str> = route.order().iter().map(UiNodeId::as_str).collect();
        assert_eq!(["button", "toolbar", "root"], actual.as_slice());
    }
}
