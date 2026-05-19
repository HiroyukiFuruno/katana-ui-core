use crate::render_model::{RenderContext, UiNodeId};
use crate::style::StyleSheet;
use crate::theme::{FontFamily, FontToken, ThemeId, ThemeSnapshot};
use serde::{Deserialize, Serialize};

pub const DEFAULT_FONT_ROLE: &str = "body";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGlobalState {
    pub active_theme_id: ThemeId,
    pub focus_target: Option<UiNodeId>,
    pub active_overlay: Option<UiNodeId>,
    pub modal_stack: Vec<UiNodeId>,
}

impl UiGlobalState {
    #[must_use]
    pub fn new(active_theme_id: ThemeId) -> Self {
        Self {
            active_theme_id,
            focus_target: None,
            active_overlay: None,
            modal_stack: Vec::new(),
        }
    }

    #[must_use]
    pub fn focus_target(mut self, target: UiNodeId) -> Self {
        self.focus_target = Some(target);
        self
    }

    #[must_use]
    pub fn active_overlay(mut self, target: UiNodeId) -> Self {
        self.active_overlay = Some(target);
        self
    }

    #[must_use]
    pub fn modal(mut self, target: UiNodeId) -> Self {
        self.modal_stack.push(target);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiCoreFacade {
    theme: ThemeSnapshot,
    style_sheet: StyleSheet,
    global_state: UiGlobalState,
    default_font_role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiFacadeChangeLog {
    pub field: String,
    pub before: String,
    pub after: String,
}

impl UiCoreFacade {
    #[must_use]
    pub fn new(theme: ThemeSnapshot) -> Self {
        let active_theme_id = theme.id.clone();
        Self {
            theme,
            style_sheet: StyleSheet::new(),
            global_state: UiGlobalState::new(active_theme_id),
            default_font_role: DEFAULT_FONT_ROLE.to_string(),
        }
    }

    #[must_use]
    pub fn with_theme(mut self, theme: ThemeSnapshot) -> Self {
        self.global_state.active_theme_id = theme.id.clone();
        self.theme = theme;
        self
    }

    #[must_use]
    pub fn with_style_sheet(mut self, style_sheet: StyleSheet) -> Self {
        self.style_sheet = style_sheet;
        self
    }

    #[must_use]
    pub fn with_global_state(mut self, global_state: UiGlobalState) -> Self {
        self.global_state = global_state;
        self
    }

    #[must_use]
    pub fn with_default_font_role(mut self, role: impl Into<String>) -> Self {
        self.default_font_role = role.into();
        self
    }

    pub fn set_theme(&mut self, theme: ThemeSnapshot) -> UiFacadeChangeLog {
        let before = self.theme.id.as_str().to_string();
        let after = theme.id.as_str().to_string();
        self.global_state.active_theme_id = theme.id.clone();
        self.theme = theme;
        UiFacadeChangeLog::new("theme", before, after)
    }

    pub fn set_style_sheet(&mut self, style_sheet: StyleSheet) -> UiFacadeChangeLog {
        let before = self.style_sheet.rule_count().to_string();
        let after = style_sheet.rule_count().to_string();
        self.style_sheet = style_sheet;
        UiFacadeChangeLog::new("style_sheet", before, after)
    }

    pub fn set_global_state(&mut self, global_state: UiGlobalState) -> UiFacadeChangeLog {
        let before = self.global_state.active_theme_id.as_str().to_string();
        let after = global_state.active_theme_id.as_str().to_string();
        self.global_state = global_state;
        UiFacadeChangeLog::new("global_state", before, after)
    }

    pub fn set_default_font_role(&mut self, role: impl Into<String>) -> UiFacadeChangeLog {
        let before = self.default_font_role.clone();
        let after = role.into();
        self.default_font_role = after.clone();
        UiFacadeChangeLog::new("default_font_role", before, after)
    }

    #[must_use]
    pub fn theme(&self) -> &ThemeSnapshot {
        &self.theme
    }

    #[must_use]
    pub fn style_sheet(&self) -> &StyleSheet {
        &self.style_sheet
    }

    #[must_use]
    pub fn global_state(&self) -> &UiGlobalState {
        &self.global_state
    }

    #[must_use]
    pub fn default_font_role(&self) -> &str {
        &self.default_font_role
    }

    #[must_use]
    pub fn font(&self, role: &str) -> Option<&FontToken> {
        self.theme
            .font(role)
            .or_else(|| self.theme.font(&self.default_font_role))
    }

    #[must_use]
    pub fn font_family(&self, role: &str) -> Option<FontFamily> {
        self.font(role).map(|it| it.family)
    }

    #[must_use]
    pub fn render_context(&self, viewport_width: f32, viewport_height: f32) -> RenderContext {
        RenderContext::new(
            self.global_state.active_theme_id.clone(),
            viewport_width,
            viewport_height,
        )
    }
}

impl UiFacadeChangeLog {
    #[must_use]
    pub fn new(
        field: impl Into<String>,
        before: impl Into<String>,
        after: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            before: before.into(),
            after: after.into(),
        }
    }
}

impl Default for UiCoreFacade {
    fn default() -> Self {
        Self::new(ThemeSnapshot::dark())
    }
}
