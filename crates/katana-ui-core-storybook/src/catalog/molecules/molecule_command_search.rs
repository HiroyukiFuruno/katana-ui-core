use super::super::{StoryCatalog, StoryExample};
use super::molecule_virtualization;
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::molecule;
use katana_ui_core::molecule::{
    CommandKeyboardInput, CommandLauncherAction, CommandResultRow, ReplaceMode,
    SearchControlStripAction, SearchNavigationDirection, SearchOptionKind, SearchReplaceScope,
};
use katana_ui_core::widget::atoms::{KeyCombo, KeyKind, KeyModifiers, NamedKey, ShortcutCombo};

const SEARCH_RESULT_COUNT: usize = 12;
const SEARCH_ACTIVE_INDEX: usize = 2;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![command_palette_story(), search_control_strip_story()]
}

fn command_palette_story() -> StoryExample {
    let virtualization = molecule_virtualization::estimated_config(
        molecule_virtualization::COMMAND_TOTAL_COUNT,
        Some(0),
    );
    let mut palette = molecule::CommandPalette::new("Command palette")
        .open(true)
        .query("open")
        .result_row(command_row("open-file", "Open File", "workspace"))
        .result_row(command_row("format", "Format Document", "editor"))
        .result_row(CommandResultRow::new("locked", "Locked command").disabled("readonly"))
        .result_row(command_row("theme", "Switch Theme", "app"))
        .result_row(command_row("recent", "Open Recent", "workspace"))
        .highlighted_index(Some(0))
        .virtualization(virtualization.clone())
        .child(molecule::SearchBox::new("Command query").value("open"))
        .child(katana_ui_core::atom::Badge::new(
            molecule_virtualization::compact_label(&virtualization),
        ));
    let target = palette.state_id().clone();
    let query = palette.apply_launcher_action(CommandLauncherAction::SetQuery("theme".into()));
    let execute =
        palette.apply_launcher_action(CommandLauncherAction::Keyboard(CommandKeyboardInput::Enter));
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "command_query_changed",
            "query=open highlighted=0",
            format!("events={query:?}"),
        ),
        UiCallbackLog::new(
            target,
            "command_execute",
            "highlighted=theme",
            format!("events={execute:?}"),
        ),
        molecule_virtualization::log(
            palette.state_id().clone(),
            "command_palette_virtualization_range",
            &virtualization,
        ),
    ];
    StoryCatalog::interactive_story("command-palette", palette, logs)
}

fn command_row(id: &'static str, label: &'static str, provider: &'static str) -> CommandResultRow {
    CommandResultRow::new(id, label)
        .secondary_label(provider)
        .provider_id(provider)
        .group_id("commands")
        .icon("command")
        .shortcut(ShortcutCombo::new(
            label,
            KeyCombo::new(
                KeyModifiers::command_shift(),
                KeyKind::Named(NamedKey::Enter),
            ),
        ))
}

fn search_control_strip_story() -> StoryExample {
    let mut strip = molecule::SearchControlStrip::new("Search control strip")
        .query("head")
        .result_position(SEARCH_RESULT_COUNT, Some(SEARCH_ACTIVE_INDEX))
        .replace_mode(ReplaceMode::Visible)
        .replace_value("title");
    let target = strip.state_id().clone();
    let query = strip.apply_action(SearchControlStripAction::SetSearchQuery(
        "heading".to_string(),
    ));
    let option = strip.apply_action(SearchControlStripAction::ToggleSearchOption(
        SearchOptionKind::UseRegex,
    ));
    let navigate = strip.apply_action(SearchControlStripAction::Navigate(
        SearchNavigationDirection::Next,
    ));
    let replace = strip.apply_action(SearchControlStripAction::Replace(SearchReplaceScope::All));
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "search_query_changed",
            "query=heading",
            format!("events={query:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "search_option_changed",
            "regex=false",
            format!("events={option:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "search_navigation_requested",
            "active=2",
            format!("events={navigate:?}"),
        ),
        UiCallbackLog::new(
            target,
            "search_replace_requested",
            "replace=title",
            format!("events={replace:?}"),
        ),
    ];
    StoryCatalog::interactive_story("search-control-strip", strip, logs)
}
