use super::super::{StoryCatalog, StoryExample};
mod feedback_story;
mod settings_list_story;
#[path = "molecule_app_primitives/settings_story.rs"]
mod settings_story;
mod shortcut_story;
mod status_story;
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::{UiStateId, UiTone};
use katana_ui_core::{atom, molecule};
use molecule::{
    BannerAction, BannerActionKind, BannerCommand, BannerDensity, BannerPlacementHint,
    BannerSeverity, ProgressMeterShape, ProgressMeterSpec, SettingsListAction, SettingsValue,
    ShortcutCheatsheetAction, ShortcutCheatsheetGroup, ShortcutCheatsheetItem, StatusBarAction,
    StatusBarDensity, StatusBarMode, StatusBarPopoverSpec, StatusBarSegment,
    StatusBarSegmentAlignment, ToastAction, ToastActionKind, ToastDedupStrategy, ToastPayload,
    ToastPosition, ToastStackAction, ToastStackDirection, ToastStackOptions,
};

const TOAST_DURATION_MS: u64 = 8_000;
const TOAST_TICK_MS: u64 = 4_000;
const STORY_TOAST_STACK_GAP: u16 = 10;
const STATUS_PROGRESS_PERCENT: u8 = 72;
const UPDATED_FONT_SIZE: i64 = 16;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        feedback_story::banner_story(),
        feedback_story::toast_stack_manager_story(),
        status_story::status_bar_story(),
        shortcut_story::shortcut_combo_story(),
        shortcut_story::shortcut_cheatsheet_story(),
        settings_list_story::settings_list_story(),
    ]
}
