use super::ColorPicker;
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};

impl ComponentAction for ColorPicker {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.state.interaction();
        if action.target() != &self.state.state_id || self.state.disabled {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        if self.state.readonly && color_change_action(action) {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        if let UiAction::SetValue {
            color_drag: Some(drag),
            ..
        } = action
        {
            self.value = self.color_for_mode(drag.value.into());
            self.hue = drag.hue;
            self.alpha = self.value.alpha;
            self.preview = drag.preview;
            self.state.value = self.value.css_rgba();
            return UiActionResult::handled(
                self.state.state_id.clone(),
                action,
                before,
                self.state.interaction(),
            );
        }
        self.state.apply_action(action, false)
    }
}

fn color_change_action(action: &UiAction) -> bool {
    matches!(
        action,
        UiAction::SetValue {
            color_drag: Some(_),
            ..
        }
    )
}
