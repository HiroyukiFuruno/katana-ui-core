use super::choice::{Breadcrumb, ComboBox, MenuButton, SelectBox, SelectionList, SideMenu, Tabs};
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{UiNode, UiNodeKind};

macro_rules! selection_rendering {
    ($name:ident, $kind:expr, $close_on_select:expr) => {
        impl ComponentAction for $name {
            fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
                self.state.apply_action(action, $close_on_select)
            }
        }

        impl From<$name> for UiNode {
            fn from(value: $name) -> Self {
                let mut node = value.state.node($kind, value.label);
                for child in value.children {
                    node = node.child(child);
                }
                node
            }
        }
    };
}

selection_rendering!(Breadcrumb, UiNodeKind::Breadcrumb, false);
selection_rendering!(ComboBox, UiNodeKind::ComboBox, true);
selection_rendering!(MenuButton, UiNodeKind::MenuButton, true);
selection_rendering!(SelectBox, UiNodeKind::SelectBox, true);
selection_rendering!(SelectionList, UiNodeKind::SelectionList, false);
selection_rendering!(SideMenu, UiNodeKind::SideMenu, false);
selection_rendering!(Tabs, UiNodeKind::Tabs, false);
