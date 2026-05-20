use super::super::{StoryCatalog, StoryExample};
#[path = "molecule_app_primitives/settings_story.rs"]
mod settings_story;
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
        banner_story(),
        toast_stack_manager_story(),
        status_bar_story(),
        shortcut_combo_story(),
        shortcut_cheatsheet_story(),
        settings_list_story(),
    ]
}

fn banner_story() -> StoryExample {
    let mut banner = molecule::Banner::new("Formatter changed 3 files.")
        .severity(BannerSeverity::Warning)
        .title("Format result")
        .leading_icon("alert-triangle")
        .dismissible(true)
        .expanded_details("src/lib.rs, src/panel.rs, tests/storybook.rs")
        .density(BannerDensity::Compact)
        .placement_hint(BannerPlacementHint::Sticky)
        .action(BannerAction::new(
            "open-diff",
            "Open diff",
            BannerActionKind::Primary,
        ))
        .action(BannerAction::new(
            "dismiss",
            "Dismiss",
            BannerActionKind::Secondary,
        ));
    let target = banner.state_id().clone();
    let opened = banner.apply_action(BannerCommand::ToggleDetails);
    let actioned = banner.apply_action(BannerCommand::PressAction("open-diff".to_string()));
    let dismissed = banner.apply_action(BannerCommand::Dismiss);
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "banner_toggle_details",
            "details_open=false",
            format!("events={opened:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "banner_primary_action",
            "action=none",
            format!("events={actioned:?}"),
        ),
        UiCallbackLog::new(
            target,
            "banner_dismiss",
            "visible=true",
            format!("events={dismissed:?}"),
        ),
    ];
    StoryCatalog::interactive_story("banner", banner, logs)
}

fn toast_stack_manager_story() -> StoryExample {
    let mut stack = molecule::ToastStackManager::new().options(ToastStackOptions {
        position: ToastPosition::BottomEnd,
        max_visible: 2,
        dedup_strategy: ToastDedupStrategy::ById,
        default_duration_ms: TOAST_DURATION_MS,
        pause_on_hover: true,
        stack_gap: STORY_TOAST_STACK_GAP,
        enter_direction: ToastStackDirection::Up,
        exit_direction: ToastStackDirection::Down,
        replace_resets_duration: false,
        max_queued: 1,
    });
    let shown = stack.apply_action(ToastStackAction::Enqueue(toast_payload("save", "Saved")));
    let queued = stack.apply_action(ToastStackAction::Enqueue(toast_payload(
        "lint",
        "Lint warning",
    )));
    let overflow = stack.apply_action(ToastStackAction::Enqueue(toast_payload(
        "build",
        "Build failed",
    )));
    let paused = stack.apply_action(ToastStackAction::PauseHover(true));
    let tick = stack.apply_action(ToastStackAction::Tick(TOAST_TICK_MS));
    let dismissed = stack.apply_action(ToastStackAction::ActivateToastAction {
        toast_id: "save".to_string(),
        action_id: "undo".to_string(),
    });
    let target = UiStateId::new("state:ToastStackManager:storybook");
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "toast_enqueue_visible",
            "visible=0 queued=0",
            format!("events={shown:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "toast_queue_and_overflow",
            "visible=1 queued=0",
            format!("queued={queued:?} overflow={overflow:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "toast_pause_hover",
            "paused=false",
            format!("events={paused:?} tick_while_paused={tick:?}"),
        ),
        UiCallbackLog::new(
            target,
            "toast_action_dismiss",
            "visible=2 queued=1",
            format!("events={dismissed:?}"),
        ),
    ];
    StoryCatalog::interactive_story("toast-stack-manager", stack, logs)
}

fn toast_payload(id: &str, message: &str) -> ToastPayload {
    ToastPayload::new(id, message)
        .severity(UiTone::Warning)
        .duration_ms(TOAST_DURATION_MS)
        .action(ToastAction::new("undo", "Undo", ToastActionKind::Primary))
}

fn status_bar_story() -> StoryExample {
    let mut status = molecule::StatusBar::new("Status bar")
        .mode(StatusBarMode::MultiSegment)
        .density(StatusBarDensity::Compact)
        .segment(
            StatusBarSegment::new("branch", "main")
                .icon("git-branch")
                .tooltip("Current branch")
                .alignment(StatusBarSegmentAlignment::Leading)
                .popover(StatusBarPopoverSpec::new(
                    "Git branch",
                    "main is ahead by 2",
                )),
        )
        .segment(
            StatusBarSegment::new("diagnostics", "2 warnings")
                .alignment(StatusBarSegmentAlignment::Center)
                .interactive(true)
                .tooltip("Linter summary")
                .accessibility_label("Diagnostics summary"),
        )
        .segment(
            StatusBarSegment::new("index", "Indexing")
                .alignment(StatusBarSegmentAlignment::Trailing)
                .tooltip("Index progress")
                .progress(
                    ProgressMeterSpec::new(ProgressMeterShape::Linear, STATUS_PROGRESS_PERCENT)
                        .label("Indexing")
                        .tooltip("Indexing 72%")
                        .tone(UiTone::Accent),
                ),
        )
        .child(atom::Badge::new("Sync"))
        .child(atom::Text::new("Ln 12, Col 4"));
    let pressed = status.apply_action(&StatusBarAction::PressSegment {
        id: "branch".to_string(),
    });
    let closed = status.apply_action(&StatusBarAction::ClosePopover {
        id: "branch".to_string(),
    });
    let logs = vec![UiCallbackLog::new(
        UiStateId::new("state:StatusBar:storybook"),
        "status_bar_segment_popover",
        "open_popover=None",
        format!("pressed={pressed:?} closed={closed:?}"),
    )];
    StoryCatalog::interactive_story("status-bar", status, logs)
}

fn shortcut_combo_story() -> StoryExample {
    let combo = atom::ShortcutCombo::new("Open command palette", command_combo('k'))
        .platform_display(atom::ShortcutPlatform::MacOS)
        .separator(atom::ShortcutSeparator::None)
        .size(katana_ui_core::render_model::UiSize::Medium)
        .tone(UiTone::Accent)
        .accessibility_label("Open command palette shortcut");
    let logs = vec![UiCallbackLog::new(
        UiStateId::new("state:ShortcutCombo:storybook"),
        "shortcut_platform_preview",
        "platform=Auto",
        "platform=MacOS combo=Command+K",
    )];
    StoryCatalog::interactive_story("shortcut-combo", combo, logs)
}

fn shortcut_cheatsheet_story() -> StoryExample {
    let mut cheatsheet = molecule::ShortcutCheatsheet::new("Shortcut cheatsheet")
        .group_layout(molecule::ShortcutCheatsheetLayout::TwoColumn)
        .group(shortcut_group(
            "Navigation",
            "command-palette",
            "Command palette",
            'k',
        ))
        .group(shortcut_group("Editing", "format", "Format document", 'f'))
        .query("format");
    let query = cheatsheet.apply_action(ShortcutCheatsheetAction::SetQuery("format".to_string()));
    let selected = cheatsheet.apply_action(ShortcutCheatsheetAction::SelectShortcut(
        "format".to_string(),
    ));
    let logs = vec![UiCallbackLog::new(
        UiStateId::new("state:ShortcutCheatsheet:storybook"),
        "shortcut_filter_select",
        "query=none selected=false",
        format!("query={query:?} selected={selected:?}"),
    )];
    StoryCatalog::interactive_story("shortcut-cheatsheet", cheatsheet, logs)
}

fn shortcut_group(title: &str, id: &str, label: &str, key: char) -> ShortcutCheatsheetGroup {
    ShortcutCheatsheetGroup::new(title).item(ShortcutCheatsheetItem::new(
        id,
        label,
        command_combo(key),
    ))
}

const fn command_combo(key: char) -> atom::KeyCombo {
    atom::KeyCombo::new(
        atom::KeyModifiers {
            command: true,
            control: false,
            alt: false,
            shift: false,
            meta: false,
        },
        atom::KeyKind::Char(key),
    )
}

fn settings_list_story() -> StoryExample {
    let mut settings = settings_story::settings_list();
    let target = settings.state_id().clone();
    let query =
        settings.apply_settings_action(SettingsListAction::SetQuery(Some("font".to_string())));
    let updated = settings.apply_settings_action(SettingsListAction::UpdateField {
        field_id: "editor.font-size".to_string(),
        value: SettingsValue::Number(UPDATED_FONT_SIZE),
    });
    let collapsed = settings.apply_settings_action(SettingsListAction::ToggleSection {
        section_id: "appearance".to_string(),
    });
    let logs = vec![UiCallbackLog::new(
        target,
        "settings_filter_update_collapse",
        "query=None dirty=1 collapsed=0",
        format!("query={query:?} updated={updated:?} collapsed={collapsed:?}"),
    )];
    StoryCatalog::interactive_story("settings-list", settings, logs)
}
