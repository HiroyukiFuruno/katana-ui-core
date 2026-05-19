use super::SplitPane;
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult, UiActionSource};
use crate::layout::split_pane_ratio::parse_ratio_percent;

impl ComponentAction for SplitPane {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.interaction.clone();
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        if !self.apply_resize_action(action) {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        UiActionResult::handled(
            self.state_id.clone(),
            action,
            before,
            self.interaction.clone(),
        )
    }
}

impl SplitPane {
    fn apply_resize_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::SetValue { value, source, .. }
                if *source == UiActionSource::SplitPane
                    || *source == UiActionSource::SplitPaneKeyboard =>
            {
                self.apply_ratio_value(value)
            }
            UiAction::SetValue { source, .. } if *source == UiActionSource::SplitPaneReset => {
                self.reset_ratio()
            }
            UiAction::SetHover { hovered, .. } => {
                self.interaction.hovered = *hovered;
                true
            }
            UiAction::SetDragging { dragging, .. } => {
                self.interaction.dragging = *dragging;
                true
            }
            _ => false,
        }
    }

    fn apply_ratio_value(&mut self, value: &str) -> bool {
        let Some(percent) = parse_ratio_percent(value) else {
            return false;
        };
        self.set_ratio_percent(percent);
        true
    }

    fn reset_ratio(&mut self) -> bool {
        self.set_ratio_percent(self.reset_percent);
        true
    }
}
