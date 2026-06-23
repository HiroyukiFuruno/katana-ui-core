use super::{TextAreaKey, TextAreaKeyChord};
use crate::facade::DEFAULT_FONT_ROLE;
use crate::render_model::{UiClearActionSpec, UiSlotSpec, UiTextAreaProps, UiTextEntryProps};
use serde::{Deserialize, Serialize};

const DEFAULT_MIN_ROWS: u16 = 2;
const DEFAULT_MAX_ROWS: u16 = 6;

pub use crate::render_model::{
    UiTextAreaNewlineKey as TextAreaNewlineKey, UiTextAreaSubmitKey as TextAreaSubmitKey,
    UiTextAreaTabBehavior as TextAreaTabBehavior, UiTextAreaWrapPolicy as TextAreaWrapPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAreaValidationError {
    ConflictingKeyBindings,
    MinRowsMustBePositive,
    MaxRowsBelowMinRows,
    VerticalScrollbarRequiresVerticalScroll,
    HorizontalScrollbarRequiresHorizontalScroll,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAreaOptions {
    pub value: String,
    pub placeholder: String,
    pub font_role: String,
    pub disabled: bool,
    pub readonly: bool,
    pub invalid: bool,
    pub min_rows: u16,
    pub max_rows: u16,
    pub auto_grow: bool,
    pub wrap_policy: TextAreaWrapPolicy,
    pub submit_key: TextAreaSubmitKey,
    pub newline_key: TextAreaNewlineKey,
    pub tab_behavior: TextAreaTabBehavior,
    pub ime_enabled: bool,
    pub resize_enabled: bool,
    pub vertical_scroll_enabled: bool,
    pub horizontal_scroll_enabled: bool,
    pub vertical_scrollbar_visible: bool,
    pub horizontal_scrollbar_visible: bool,
    pub leading_slot: Option<UiSlotSpec>,
    pub trailing_slot: Option<UiSlotSpec>,
    pub trailing_icon_buttons: Vec<UiSlotSpec>,
    pub clear_action: Option<UiClearActionSpec>,
}

impl Default for TextAreaOptions {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            font_role: DEFAULT_FONT_ROLE.to_string(),
            disabled: false,
            readonly: false,
            invalid: false,
            min_rows: DEFAULT_MIN_ROWS,
            max_rows: DEFAULT_MAX_ROWS,
            auto_grow: true,
            wrap_policy: TextAreaWrapPolicy::Soft,
            submit_key: TextAreaSubmitKey::Enter,
            newline_key: TextAreaNewlineKey::ShiftEnter,
            tab_behavior: TextAreaTabBehavior::MoveFocus,
            ime_enabled: true,
            resize_enabled: false,
            vertical_scroll_enabled: false,
            horizontal_scroll_enabled: false,
            vertical_scrollbar_visible: false,
            horizontal_scrollbar_visible: false,
            leading_slot: None,
            trailing_slot: None,
            trailing_icon_buttons: Vec::new(),
            clear_action: None,
        }
    }
}

impl TextAreaOptions {
    pub fn validate(&self) -> Result<(), TextAreaValidationError> {
        if self.min_rows == 0 {
            return Err(TextAreaValidationError::MinRowsMustBePositive);
        }
        if self.max_rows < self.min_rows {
            return Err(TextAreaValidationError::MaxRowsBelowMinRows);
        }
        if self.vertical_scrollbar_visible && !self.vertical_scroll_enabled {
            return Err(TextAreaValidationError::VerticalScrollbarRequiresVerticalScroll);
        }
        if self.horizontal_scrollbar_visible && !self.horizontal_scroll_enabled {
            return Err(TextAreaValidationError::HorizontalScrollbarRequiresHorizontalScroll);
        }
        if submit_chord(self.submit_key).is_some_and(|submit| {
            newline_chord(self.newline_key).is_some_and(|newline| submit == newline)
        }) {
            return Err(TextAreaValidationError::ConflictingKeyBindings);
        }
        Ok(())
    }

    pub(super) fn text_area_props(
        &self,
        rows: u16,
        internal_scroll: bool,
        resize_width_delta: u16,
        resize_height_delta: u16,
    ) -> UiTextAreaProps {
        UiTextAreaProps {
            min_rows: self.min_rows,
            max_rows: self.max_rows,
            auto_grow: self.auto_grow,
            wrap_policy: self.wrap_policy,
            submit_key: self.submit_key,
            newline_key: self.newline_key,
            tab_behavior: self.tab_behavior,
            ime_enabled: self.ime_enabled,
            resize_enabled: self.resize_enabled,
            vertical_scroll_enabled: self.vertical_scroll_enabled,
            horizontal_scroll_enabled: self.horizontal_scroll_enabled,
            vertical_scrollbar_visible: self.vertical_scrollbar_visible,
            horizontal_scrollbar_visible: self.horizontal_scrollbar_visible,
            measured_rows: rows,
            internal_scroll,
            resize_width_delta,
            resize_height_delta,
        }
    }

    pub(super) fn text_entry_props(&self) -> UiTextEntryProps {
        UiTextEntryProps {
            leading_slot: self.leading_slot.clone(),
            trailing_slot: self.trailing_slot.clone(),
            trailing_icon_buttons: self.trailing_icon_buttons.clone(),
            clear_action: self.clear_action.clone(),
            ime_enabled: self.ime_enabled,
            emoji_enabled: true,
            ..UiTextEntryProps::default()
        }
    }
}

pub(super) fn submit_chord(value: TextAreaSubmitKey) -> Option<TextAreaKeyChord> {
    match value {
        TextAreaSubmitKey::Enter => Some(TextAreaKeyChord::enter()),
        TextAreaSubmitKey::ModEnter => Some(TextAreaKeyChord::mod_enter()),
        TextAreaSubmitKey::Disabled => None,
    }
}

pub(super) fn newline_chord(value: TextAreaNewlineKey) -> Option<TextAreaKeyChord> {
    match value {
        TextAreaNewlineKey::Enter => Some(TextAreaKeyChord::enter()),
        TextAreaNewlineKey::ShiftEnter => Some(TextAreaKeyChord::shift_enter()),
        TextAreaNewlineKey::Disabled => None,
    }
}

impl TextAreaKeyChord {
    #[must_use]
    pub const fn enter() -> Self {
        Self::new(TextAreaKey::Enter, false, false)
    }

    #[must_use]
    pub const fn shift_enter() -> Self {
        Self::new(TextAreaKey::Enter, true, false)
    }

    #[must_use]
    pub const fn mod_enter() -> Self {
        Self::new(TextAreaKey::Enter, false, true)
    }

    #[must_use]
    pub const fn tab() -> Self {
        Self::new(TextAreaKey::Tab, false, false)
    }

    const fn new(key: TextAreaKey, shift: bool, primary_modifier: bool) -> Self {
        Self {
            key,
            shift,
            primary_modifier,
        }
    }
}
