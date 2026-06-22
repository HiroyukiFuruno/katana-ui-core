use crate::interaction::UiAction;
use crate::render_model::UiStateId;

impl UiAction {
    #[must_use]
    pub fn tab_select(target: UiStateId, tab_id: impl Into<String>) -> Self {
        Self::TabSelect {
            target,
            tab_id: tab_id.into(),
        }
    }

    #[must_use]
    pub fn tab_add(
        target: UiStateId,
        tab_id: impl Into<String>,
        label: impl Into<String>,
        activate: bool,
    ) -> Self {
        Self::TabAdd {
            target,
            tab_id: tab_id.into(),
            label: label.into(),
            activate,
        }
    }

    #[must_use]
    pub fn tab_close(target: UiStateId, tab_id: impl Into<String>) -> Self {
        Self::TabClose {
            target,
            tab_id: tab_id.into(),
        }
    }

    #[must_use]
    pub fn tab_close_others(target: UiStateId, tab_id: impl Into<String>) -> Self {
        Self::TabCloseOthers {
            target,
            tab_id: tab_id.into(),
        }
    }

    #[must_use]
    pub fn tab_close_to_right(target: UiStateId, tab_id: impl Into<String>) -> Self {
        Self::TabCloseToRight {
            target,
            tab_id: tab_id.into(),
        }
    }

    #[must_use]
    pub fn tab_close_to_left(target: UiStateId, tab_id: impl Into<String>) -> Self {
        Self::TabCloseToLeft {
            target,
            tab_id: tab_id.into(),
        }
    }

    #[must_use]
    pub fn tab_close_all(target: UiStateId) -> Self {
        Self::TabCloseAll { target }
    }

    #[must_use]
    pub fn tab_restore_closed(target: UiStateId) -> Self {
        Self::TabRestoreClosed { target }
    }

    #[must_use]
    pub fn tab_pin(target: UiStateId, tab_id: impl Into<String>, pinned: bool) -> Self {
        Self::TabPin {
            target,
            tab_id: tab_id.into(),
            pinned,
        }
    }

    #[must_use]
    pub fn tab_move(target: UiStateId, tab_id: impl Into<String>, to_visual_index: usize) -> Self {
        Self::TabMove {
            target,
            tab_id: tab_id.into(),
            to_visual_index,
        }
    }

    #[must_use]
    pub fn tab_move_to_group(
        target: UiStateId,
        tab_id: impl Into<String>,
        group_id: impl Into<String>,
    ) -> Self {
        Self::TabMoveToGroup {
            target,
            tab_id: tab_id.into(),
            group_id: Some(group_id.into()),
        }
    }

    #[must_use]
    pub fn tab_move_to_ungrouped(target: UiStateId, tab_id: impl Into<String>) -> Self {
        Self::TabMoveToGroup {
            target,
            tab_id: tab_id.into(),
            group_id: None,
        }
    }

    #[must_use]
    pub fn tab_move_to_new_group(
        target: UiStateId,
        tab_id: impl Into<String>,
        group_id: impl Into<String>,
        group_label: impl Into<String>,
    ) -> Self {
        Self::TabMoveToNewGroup {
            target,
            tab_id: tab_id.into(),
            group_id: group_id.into(),
            group_label: group_label.into(),
        }
    }

    #[must_use]
    pub fn tab_move_group(target: UiStateId, group_id: impl Into<String>, to_index: usize) -> Self {
        Self::TabMoveGroup {
            target,
            group_id: group_id.into(),
            to_index,
        }
    }

    #[must_use]
    pub fn tab_rename_group(
        target: UiStateId,
        group_id: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self::TabRenameGroup {
            target,
            group_id: group_id.into(),
            label: label.into(),
        }
    }

    #[must_use]
    pub fn tab_set_group_color(
        target: UiStateId,
        group_id: impl Into<String>,
        color: impl Into<String>,
    ) -> Self {
        Self::TabSetGroupColor {
            target,
            group_id: group_id.into(),
            color: color.into(),
        }
    }

    #[must_use]
    pub fn tab_ungroup(target: UiStateId, group_id: impl Into<String>) -> Self {
        Self::TabUngroup {
            target,
            group_id: group_id.into(),
        }
    }

    #[must_use]
    pub fn tab_close_group(target: UiStateId, group_id: impl Into<String>) -> Self {
        Self::TabCloseGroup {
            target,
            group_id: group_id.into(),
        }
    }

    #[must_use]
    pub fn tab_toggle_group_collapse(target: UiStateId, group_id: impl Into<String>) -> Self {
        Self::TabToggleGroupCollapse {
            target,
            group_id: group_id.into(),
        }
    }
}
