use super::model::CodeDiff;
use super::types::{CodeDiffDirection, CodeDiffMode};
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult, UiActionSource};

impl ComponentAction for CodeDiff {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.state.interaction();
        if action.target() != &self.state.state_id {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        if !self.apply_diff_action(action) {
            return self.state.apply_action(action, false);
        }
        UiActionResult::handled(
            self.state.state_id.clone(),
            action,
            before,
            self.state.interaction(),
        )
    }
}

impl CodeDiff {
    fn apply_diff_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::SetValue { source, value, .. } if *source == UiActionSource::CodeDiffMode => {
                self.apply_mode(value)
            }
            UiAction::SetValue { source, value, .. }
                if *source == UiActionSource::CodeDiffDirection =>
            {
                self.apply_direction(value)
            }
            UiAction::Press {
                source: UiActionSource::CodeDiffExpand,
                ..
            } => self.expand_first_block(),
            UiAction::Press {
                source: UiActionSource::CodeDiffScrollSync,
                ..
            } => self.toggle_scroll_sync(),
            _ => false,
        }
    }

    fn apply_mode(&mut self, value: &str) -> bool {
        let Some(mode) = parse_mode(value) else {
            return false;
        };
        self.mode = mode;
        self.state.value = value.to_string();
        true
    }

    fn apply_direction(&mut self, value: &str) -> bool {
        let Some(direction) = parse_direction(value) else {
            return false;
        };
        self.direction = direction;
        self.state.value = value.to_string();
        true
    }

    fn expand_first_block(&mut self) -> bool {
        if let Some(block) = self.collapsed_blocks.first().copied() {
            self.collapsed_blocks.remove(0);
            self.expanded_blocks.push(block);
            self.state.item_count = self.lines.len();
            return true;
        }
        let Some(block) = self.expanded_blocks.pop() else {
            return false;
        };
        self.collapsed_blocks.push(block);
        self.collapsed_blocks.sort_by_key(|block| block.start_line);
        self.state.item_count = self.lines.len();
        true
    }

    fn toggle_scroll_sync(&mut self) -> bool {
        self.scroll_sync_enabled = !self.scroll_sync_enabled;
        self.state.transient.active = self.scroll_sync_enabled;
        true
    }
}

fn parse_mode(value: &str) -> Option<CodeDiffMode> {
    match value {
        "inline" | "Inline" => Some(CodeDiffMode::Inline),
        "split" | "Split" => Some(CodeDiffMode::Split),
        _ => None,
    }
}

fn parse_direction(value: &str) -> Option<CodeDiffDirection> {
    match value {
        "horizontal" | "Horizontal" => Some(CodeDiffDirection::Horizontal),
        "vertical" | "Vertical" => Some(CodeDiffDirection::Vertical),
        _ => None,
    }
}
