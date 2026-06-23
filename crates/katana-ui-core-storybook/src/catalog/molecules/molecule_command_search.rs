use super::super::{StoryCatalog, StoryExample};
use super::molecule_virtualization;
use katana_ui_core::atom;
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
    let highlight =
        palette.apply_launcher_action(CommandLauncherAction::Keyboard(CommandKeyboardInput::Home));
    let execute =
        palette.apply_launcher_action(CommandLauncherAction::Keyboard(CommandKeyboardInput::Enter));
    let closed = palette.apply_launcher_action(CommandLauncherAction::Close);
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "command_query_changed",
            "query=open highlighted=0",
            format!("events={query:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "command_highlight_moved",
            "highlighted=theme",
            format!("events={highlight:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "command_execute",
            "highlighted=theme",
            format!("events={execute:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "command_close",
            "open=true",
            format!("events={closed:?}"),
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
        .replace_value("title")
        .child(atom::Text::new(
            "settings: query match_case whole_word regex replace_mode result_count active_index",
        ))
        .child(atom::Text::new(
            "state: query=heading match_case=true whole_word=true regex=true replace=title result=3 / 12",
        ))
        .child(atom::Text::new(
            "event: SearchQueryChanged SearchOptionChanged SearchNavigationRequested ReplaceRequested",
        ))
        .child(atom::Text::new(
            "action: query option navigate replace result-position",
        ))
        .child(atom::Text::new(
            "preset: workspace search editor find editor replace viewer search history search",
        ))
        .child(atom::Text::new(
            "quality: typed options state_id result_count event_contract",
        ));
    let target = strip.state_id().clone();
    let query = strip.apply_action(SearchControlStripAction::SetSearchQuery(
        "heading".to_string(),
    ));
    let match_case = strip.apply_action(SearchControlStripAction::ToggleSearchOption(
        SearchOptionKind::MatchCase,
    ));
    let whole_word = strip.apply_action(SearchControlStripAction::ToggleSearchOption(
        SearchOptionKind::WholeWord,
    ));
    let regex = strip.apply_action(SearchControlStripAction::ToggleSearchOption(
        SearchOptionKind::UseRegex,
    ));
    let navigate = strip.apply_action(SearchControlStripAction::Navigate(
        SearchNavigationDirection::Next,
    ));
    let replace = strip.apply_action(SearchControlStripAction::Replace(SearchReplaceScope::All));
    let result_position = strip.apply_action(SearchControlStripAction::SetResultPosition {
        result_count: SEARCH_RESULT_COUNT,
        active_index: Some(SEARCH_ACTIVE_INDEX),
    });
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
            "match_case=false",
            format!("events={match_case:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "search_option_changed",
            "whole_word=false",
            format!("events={whole_word:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "search_option_changed",
            "regex=false",
            format!("events={regex:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "search_navigation_requested",
            "active=2",
            format!("events={navigate:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "search_replace_requested",
            "replace=title",
            format!("events={replace:?}"),
        ),
        UiCallbackLog::new(
            target,
            "search_result_position_changed",
            "result=3 / 12",
            format!("events={result_position:?}"),
        ),
    ];
    StoryCatalog::interactive_story("search-control-strip", strip, logs)
}
