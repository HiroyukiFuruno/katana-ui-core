use super::actions::WorkspaceTabBarAction;
use super::events::WorkspaceTabBarEvent;
use super::identifiers::WorkspaceTabId;
use super::keyboard::{WorkspaceTabKeyboardController, WorkspaceTabKeyboardInput};
use super::options::{WorkspaceTab, WorkspaceTabBarOptions, WorkspaceTabGroup, WorkspaceTabTone};
use super::ordering::ordered_tabs;
use super::state::WorkspaceTabBarState;
use crate::render_model::{UiCommonProps, UiDimension, UiNode, UiNodeKind, UiVisualRole};
use serde::{Deserialize, Serialize};

const WORKSPACE_TAB_BAR_HEIGHT_PX: u16 = 40;
const WORKSPACE_TAB_HEIGHT_PX: u16 = 36;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabBar {
    pub(super) label: String,
    pub(super) options: WorkspaceTabBarOptions,
    pub(super) state: WorkspaceTabBarState,
    pub(super) event_log: Vec<WorkspaceTabBarEvent>,
}

impl WorkspaceTabBar {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        let options = WorkspaceTabBarOptions::default();
        Self {
            label: label.into(),
            state: WorkspaceTabBarState::new(&options.tabs),
            options,
            event_log: Vec::new(),
        }
    }

    #[must_use]
    pub fn tab(mut self, tab: WorkspaceTab) -> Self {
        self.options.tabs.push(tab);
        self.normalize_tabs();
        self
    }

    #[must_use]
    pub fn group(mut self, group: WorkspaceTabGroup) -> Self {
        self.options.groups.push(group);
        self
    }

    #[must_use]
    pub fn active_tab_id(mut self, value: impl Into<WorkspaceTabId>) -> Self {
        self.state.active_tab_id = Some(value.into());
        self
    }

    #[must_use]
    pub fn options(&self) -> &WorkspaceTabBarOptions {
        &self.options
    }

    #[must_use]
    pub fn state(&self) -> &WorkspaceTabBarState {
        &self.state
    }

    #[must_use]
    pub fn event_log(&self) -> &[WorkspaceTabBarEvent] {
        &self.event_log
    }

    #[must_use]
    pub fn visual_tabs(&self) -> Vec<&WorkspaceTab> {
        ordered_tabs(&self.options.tabs)
    }

    pub fn apply_action(&mut self, action: WorkspaceTabBarAction) -> Vec<WorkspaceTabBarEvent> {
        let events = match action {
            WorkspaceTabBarAction::SelectTab { tab_id } => self.select_tab(tab_id),
            WorkspaceTabBarAction::CloseTab { tab_id } => self.close_tab(tab_id),
            WorkspaceTabBarAction::PinTab { tab_id } => self.set_pinned(tab_id, true),
            WorkspaceTabBarAction::UnpinTab { tab_id } => self.set_pinned(tab_id, false),
            WorkspaceTabBarAction::MoveTab {
                tab_id,
                to_visual_index,
            } => self.move_tab(tab_id, to_visual_index),
            WorkspaceTabBarAction::MoveToGroup { tab_id, target } => {
                self.move_to_group(tab_id, target)
            }
            WorkspaceTabBarAction::StartDrag { tab_id } => self.start_drag(tab_id),
            WorkspaceTabBarAction::EndDrag { committed } => self.end_drag(committed),
            WorkspaceTabBarAction::CancelDrag => self.end_drag(false),
            WorkspaceTabBarAction::HoverCollapsedGroupForDrop {
                group_id,
                elapsed_ms,
            } => self.hover_collapsed_group_for_drop(group_id, elapsed_ms),
            WorkspaceTabBarAction::ToggleGroupCollapse { group_id } => {
                self.toggle_group_collapse(group_id)
            }
            WorkspaceTabBarAction::OpenOverflow { hidden_tab_ids } => {
                self.open_overflow(hidden_tab_ids)
            }
            WorkspaceTabBarAction::ConfirmClose { tab_id } => self.confirm_close(tab_id),
        };
        self.event_log.extend(events.clone());
        events
    }

    pub fn apply_keyboard_input(
        &mut self,
        input: WorkspaceTabKeyboardInput,
        visible_tab_ids: &[WorkspaceTabId],
    ) -> Vec<WorkspaceTabBarEvent> {
        if input == WorkspaceTabKeyboardInput::CancelDrag {
            return self.apply_action(WorkspaceTabBarAction::CancelDrag);
        }
        let action = WorkspaceTabKeyboardController::action_for_input(
            &input,
            self.state.active_tab_id.as_ref(),
            visible_tab_ids,
        );
        action.map_or_else(Vec::new, |it| self.apply_action(it))
    }
}

impl From<WorkspaceTabBar> for UiNode {
    fn from(value: WorkspaceTabBar) -> Self {
        let common = UiCommonProps::default()
            .width(UiDimension::Fill)
            .height(UiDimension::Px(WORKSPACE_TAB_BAR_HEIGHT_PX))
            .selectable(false)
            .accessibility_label(value.label.clone());
        let interaction = value.state.interaction(value.options.tabs.len());
        let state_id = value.state.state_id.clone();
        let label = value.label.clone();
        let mut node = UiNode::from_state(UiNodeKind::CloseableTabStrip, label, state_id)
            .common(common)
            .interaction(interaction)
            .visual_role(UiVisualRole::Control)
            .style_class("closeable-tab-strip");
        for tab in value.visual_tabs() {
            node = node.child(workspace_tab_node(tab, value.state.child_state_id(&tab.id)));
        }
        node
    }
}

fn workspace_tab_node(
    tab: &WorkspaceTab,
    state_id: Option<&crate::render_model::UiStateId>,
) -> UiNode {
    let mut node = state_id.map_or_else(
        || UiNode::new(UiNodeKind::CloseableTab, tab.title.clone()),
        |it| UiNode::from_state(UiNodeKind::CloseableTab, tab.title.clone(), it.clone()),
    );
    node = node
        .width(UiDimension::FitContent)
        .height(UiDimension::Px(WORKSPACE_TAB_HEIGHT_PX))
        .focusable(true)
        .selectable(true)
        .tone(tab_tone(tab.tone))
        .accessibility_label(tab.accessibility_text())
        .style_class("closeable-tab");
    if tab.pinned {
        node = node.style_class("closeable-tab-pinned");
    }
    if tab.closeable && !tab.pinned {
        node = node.style_class("closeable-tab-closeable");
    }
    if tab.dirty {
        node = node.style_class("closeable-tab-dirty");
    }
    node
}

fn tab_tone(tone: WorkspaceTabTone) -> crate::render_model::UiTone {
    match tone {
        WorkspaceTabTone::Default | WorkspaceTabTone::Muted => crate::render_model::UiTone::Neutral,
        WorkspaceTabTone::Accent => crate::render_model::UiTone::Accent,
        WorkspaceTabTone::Warning => crate::render_model::UiTone::Warning,
        WorkspaceTabTone::Danger => crate::render_model::UiTone::Danger,
    }
}
