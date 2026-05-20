use super::{SplitPane, SplitPaneResizeMode};
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
            UiAction::SetValue { value, source, .. } if self.resize_source_allowed(*source) => {
                self.apply_ratio_value(value)
            }
            UiAction::SetValue { source, .. } if Self::is_resize_source(*source) => false,
            UiAction::SetValue { source, .. }
                if *source == UiActionSource::SplitPaneReset && self.reset_allowed() =>
            {
                self.reset_ratio()
            }
            UiAction::SetValue { source, .. } if *source == UiActionSource::SplitPaneReset => false,
            UiAction::SetHover { hovered, .. } => {
                self.interaction.hovered = *hovered;
                true
            }
            UiAction::SetDragging { dragging, .. }
                if self.resize_mode_value() != SplitPaneResizeMode::Disabled =>
            {
                self.interaction.dragging = *dragging;
                true
            }
            UiAction::SetDragging { .. } => false,
            _ => false,
        }
    }

    fn apply_ratio_value(&mut self, value: &str) -> bool {
        let Some(percent) = parse_ratio_percent(value) else {
            return false;
        };
        let clamped = self.clamped(percent);
        self.set_ratio_percent(percent);
        self.interaction.dismiss_reason = if clamped == percent {
            String::new()
        } else {
            format!("clamped:{percent}->{clamped}")
        };
        true
    }

    fn reset_ratio(&mut self) -> bool {
        self.set_ratio_percent(self.reset_percent);
        self.interaction.dismiss_reason.clear();
        true
    }

    const fn is_resize_source(source: UiActionSource) -> bool {
        matches!(
            source,
            UiActionSource::SplitPane | UiActionSource::SplitPaneKeyboard
        )
    }

    const fn resize_source_allowed(&self, source: UiActionSource) -> bool {
        match self.resize_mode_value() {
            SplitPaneResizeMode::PointerOnly => matches!(source, UiActionSource::SplitPane),
            SplitPaneResizeMode::KeyboardOnly => {
                matches!(source, UiActionSource::SplitPaneKeyboard)
            }
            SplitPaneResizeMode::PointerAndKeyboard => Self::is_resize_source(source),
            SplitPaneResizeMode::Disabled => false,
        }
    }

    const fn reset_allowed(&self) -> bool {
        !matches!(self.resize_mode_value(), SplitPaneResizeMode::Disabled)
    }
}
