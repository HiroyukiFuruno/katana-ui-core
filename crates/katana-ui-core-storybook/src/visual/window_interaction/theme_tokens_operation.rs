use katana_ui_core::theme::ThemeSnapshot;

use super::StorybookWindowState;
use crate::visual::preview_detail;

const PAGE: &str = "theme-tokens";
const RESIZE_HIT_SIZE: usize = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum ThemeTokensStoryAction {
    Hover,
    Focus,
    Keyboard,
    Resize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::visual) struct ThemeTokensStoryState {
    hovered: bool,
    focused: bool,
    keyboard_selected_light: bool,
    resized: bool,
    callback: &'static str,
}

impl ThemeTokensStoryState {
    pub(in crate::visual) fn apply_action(
        &mut self,
        action: ThemeTokensStoryAction,
    ) -> ThemeTokensStoryUpdate {
        match action {
            ThemeTokensStoryAction::Hover => {
                assert!(
                    ThemeSnapshot::dark().color("accent").is_some(),
                    "core dark theme must expose an accent color token"
                );
                self.hovered = true;
                self.callback = "callback=theme-color";
                ThemeTokensStoryUpdate::new("theme_token_hover", "hover_start", "hover=accent")
            }
            ThemeTokensStoryAction::Focus => {
                assert!(
                    ThemeSnapshot::dark().font("body").is_some(),
                    "core dark theme must expose a body font token"
                );
                self.focused = true;
                self.callback = "callback=theme-focus";
                ThemeTokensStoryUpdate::new("theme_token_focus", "focus", "focus=swatch")
            }
            ThemeTokensStoryAction::Keyboard => {
                let diff = ThemeSnapshot::light().diff(&ThemeSnapshot::dark());
                assert!(
                    diff.changed_sections().iter().any(|it| it == "colors"),
                    "light/dark theme keyboard switch must expose a color diff"
                );
                self.keyboard_selected_light = true;
                self.callback = "callback=theme-keyboard";
                ThemeTokensStoryUpdate::new(
                    "theme_token_keyboard_light",
                    "theme_changed",
                    "keyboard=light",
                )
            }
            ThemeTokensStoryAction::Resize => {
                assert!(
                    !ThemeSnapshot::dark().spacing.is_empty(),
                    "core dark theme must expose spacing tokens"
                );
                self.resized = true;
                self.callback = "callback=theme-spacing";
                ThemeTokensStoryUpdate::new(
                    "theme_token_resize_spacing",
                    "theme_spacing_changed",
                    "resize=spacing",
                )
            }
        }
    }

    pub(in crate::visual) const fn hovered(&self) -> bool {
        self.hovered
    }

    pub(in crate::visual) const fn focused(&self) -> bool {
        self.focused
    }

    pub(in crate::visual) const fn keyboard_selected_light(&self) -> bool {
        self.keyboard_selected_light
    }

    pub(in crate::visual) const fn resized(&self) -> bool {
        self.resized
    }

    pub(in crate::visual) const fn callback(&self) -> &'static str {
        self.callback
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct ThemeTokensStoryUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
}

impl ThemeTokensStoryUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

pub(super) fn operation_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<ThemeTokensStoryAction> {
    if state.selected_page != PAGE {
        return None;
    }
    let component = preview_detail::component_action_hit_rect(PAGE);
    if component.contains(x, y)
        && x + RESIZE_HIT_SIZE >= component.right()
        && y + RESIZE_HIT_SIZE >= component.bottom()
    {
        return Some(ThemeTokensStoryAction::Resize);
    }
    None
}
