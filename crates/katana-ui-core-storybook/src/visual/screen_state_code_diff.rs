use super::screen_state::StorybookScreenState;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{
    CodeDiff, CodeDiffDirection, CodeDiffLine, CodeDiffLineKind, CodeDiffMode, CodeDiffSource,
    CollapsedBlock, HighlightRange,
};
use katana_ui_core::render_model::UiStateId;

const COLLAPSED_BLOCK_START_LINE: usize = 3;
const COLLAPSED_BLOCK_LINE_COUNT: usize = 4;

impl StorybookScreenState {
    pub(in crate::visual) fn register_code_diff_mode_switch(&mut self) {
        let result = code_diff_action(|target| UiAction::code_diff_mode(target, "Split"));
        assert!(result.handled, "the code diff mode action must be handled");
        self.action_count += 1;
        self.last_action = "diff_mode_switch";
        self.last_event = "diff_mode_changed";
        self.last_setting = "interaction.value";
        self.last_setting_value = "Split";
        self.state_label = "mode=split";
    }

    pub(in crate::visual) fn register_code_diff_hover(&mut self) {
        let result = code_diff_action(|target| UiAction::hover(target, true));
        assert!(
            result.handled && result.after.hovered,
            "the code diff hover action must update hover state"
        );
        self.action_count += 1;
        self.last_action = "code_diff_hover";
        self.last_event = "code_diff_hovered";
        self.last_setting = "interaction.hovered";
        self.last_setting_value = "true";
        self.state_label = "hover=true";
    }

    pub(in crate::visual) fn register_code_diff_focus(&mut self) {
        let result = code_diff_action(UiAction::focus);
        assert!(
            result.handled && result.after.focused,
            "the code diff focus action must update focus state"
        );
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "code_diff_focus";
        self.last_event = "code_diff_focused";
        self.last_setting = "interaction.focused";
        self.last_setting_value = "true";
        self.state_label = "focus=true";
    }

    pub(in crate::visual) fn register_code_diff_keyboard_expand(&mut self) {
        if !self.button_focused {
            self.last_action = "code_diff_keyboard_without_focus";
            self.last_event = "code_diff_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        let result = code_diff_action(UiAction::code_diff_expand);
        assert!(
            result.handled,
            "the code diff expand action must be handled"
        );
        self.action_count += 1;
        self.last_action = "code_diff_expand";
        self.last_event = "code_diff_block_expanded";
        self.last_setting = "code_diff.collapsed_block";
        self.last_setting_value = "expanded";
        self.state_label = "collapsed=false";
    }

    pub(in crate::visual) fn register_code_diff_scroll_sync(&mut self) {
        let result = code_diff_action(UiAction::code_diff_scroll_sync);
        assert!(
            result.handled && result.after.active,
            "the code diff scroll sync action must activate sync"
        );
        self.action_count += 1;
        self.last_action = "code_diff_scroll_sync";
        self.last_event = "code_diff_scroll_sync_changed";
        self.last_setting = "code_diff.scroll_sync";
        self.last_setting_value = "true";
        self.state_label = "scroll_sync=true";
    }
}

fn code_diff_action(
    action_builder: impl FnOnce(UiStateId) -> UiAction,
) -> katana_ui_core::interaction::UiActionResult {
    let mut diff = code_diff_contract_model();
    let target = diff.state_id().clone();
    diff.apply_action(&action_builder(target))
}

fn code_diff_contract_model() -> CodeDiff {
    CodeDiff::new("Code diff")
        .source(CodeDiffSource::Unified {
            text: "- old\n+ new\n  日本語 diff".to_string(),
        })
        .mode(CodeDiffMode::Inline)
        .direction(CodeDiffDirection::Horizontal)
        .language("rust")
        .line(CodeDiffLine {
            old_number: Some(1),
            new_number: None,
            kind: CodeDiffLineKind::Removed,
            text: "old line".to_string(),
        })
        .line(CodeDiffLine {
            old_number: None,
            new_number: Some(1),
            kind: CodeDiffLineKind::Added,
            text: "new line".to_string(),
        })
        .highlight(HighlightRange {
            start_line: 1,
            end_line: 2,
        })
        .collapsed_block(CollapsedBlock {
            start_line: COLLAPSED_BLOCK_START_LINE,
            line_count: COLLAPSED_BLOCK_LINE_COUNT,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_diff_keyboard_requires_focus_before_expand() {
        let mut state = StorybookScreenState::default();
        state.register_code_diff_keyboard_expand();
        assert_eq!("code_diff_keyboard_ignored", state.last_event);
        state.register_code_diff_focus();
        state.register_code_diff_keyboard_expand();
        assert_eq!("code_diff_block_expanded", state.last_event);
    }
}
