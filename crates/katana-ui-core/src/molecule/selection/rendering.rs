use super::choice::{Breadcrumb, ComboBox, MenuButton, SelectBox, SelectionList, SideMenu, Tabs};
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::molecule::virtualization;
use crate::render_model::{UiNode, UiNodeKind};

macro_rules! selection_rendering {
    ($name:ident, $kind:expr, $close_on_select:expr) => {
        impl ComponentAction for $name {
            fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
                if selection_is_disabled(action, &self.items) {
                    let before = self.state.interaction();
                    return UiActionResult::ignored(self.state.state_id.clone(), before);
                }
                let result = self.state.apply_action(action, $close_on_select);
                if result.handled {
                    self.sync_selected_option(action);
                    return UiActionResult::handled(
                        self.state.state_id.clone(),
                        action,
                        result.before,
                        self.state.interaction(),
                    );
                }
                result
            }
        }

        impl From<$name> for UiNode {
            fn from(value: $name) -> Self {
                let range = virtualization::range(&value.model.virtualization, value.items.len());
                let mut node =
                    value
                        .state
                        .node($kind, value.label)
                        .interaction(virtualization::interaction(
                            value.state.interaction(),
                            range.as_ref(),
                        ));
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

macro_rules! selected_option_sync {
    ($name:ident) => {
        impl $name {
            fn sync_selected_option(&mut self, action: &UiAction) {
                let UiAction::SetSelectedIndex { selected_index, .. } = action else {
                    return;
                };
                let Some(item) = self.items.get(*selected_index).cloned() else {
                    return;
                };
                self.model.selected_option = Some(item.clone());
                self.state.value = item.value;
            }
        }
    };
}

selected_option_sync!(Breadcrumb);
selected_option_sync!(ComboBox);
selected_option_sync!(MenuButton);
selected_option_sync!(SelectBox);
selected_option_sync!(SelectionList);
selected_option_sync!(SideMenu);
selected_option_sync!(Tabs);

fn selection_is_disabled(action: &UiAction, items: &[super::types::ChoiceItem]) -> bool {
    let UiAction::SetSelectedIndex { selected_index, .. } = action else {
        return false;
    };
    items
        .get(*selected_index)
        .map(|item| item.disabled)
        .unwrap_or(false)
}
