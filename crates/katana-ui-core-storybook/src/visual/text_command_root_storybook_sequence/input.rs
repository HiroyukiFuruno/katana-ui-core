use super::super::{
    CODE_DROPDOWN_POINT, HEADING_POINT, SEARCH_CONTROL_POINT, SEARCH_INPUT_POINT,
    SEARCH_REPLACE_CONTROL_POINT, TEXT_INPUT_POINT, key, pointer_button,
};
use super::scenario::Scenario;
use eframe::egui;

pub(super) fn initial_root() -> Scenario {
    scenario("initial-root", [], [])
}

pub(super) fn focus_and_multiline_input() -> Scenario {
    scenario(
        "focus-and-multiline-input",
        [
            "pointer-primary",
            "key-enter",
            "text-japanese-vs16",
            "key-enter",
            "text-zwj",
        ],
        [
            pointer_button(TEXT_INPUT_POINT, egui::PointerButton::Primary, true),
            pointer_button(TEXT_INPUT_POINT, egui::PointerButton::Primary, false),
            key(egui::Key::Enter, egui::Modifiers::NONE),
            egui::Event::Text("日本語 ⭐️".to_string()),
            key(egui::Key::Enter, egui::Modifiers::NONE),
            egui::Event::Text("👩‍💻".to_string()),
        ],
    )
}

pub(super) fn ime_preedit() -> Scenario {
    scenario(
        "ime-preedit",
        ["ime-preedit-japanese-vs16"],
        [egui::Event::Ime(egui::ImeEvent::Preedit(
            "かな⭐️".to_string(),
        ))],
    )
}

pub(super) fn ime_commit() -> Scenario {
    scenario(
        "ime-commit",
        ["ime-commit-japanese-vs16-zwj"],
        [egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語 ⭐️👩‍💻".to_string(),
        ))],
    )
}

pub(super) fn selection_anchored_floating_toolbar() -> Scenario {
    scenario(
        "selection-anchored-floating-toolbar",
        ["command-a-selection"],
        [key(
            egui::Key::A,
            egui::Modifiers {
                command: true,
                ..egui::Modifiers::NONE
            },
        )],
    )
}

pub(super) fn heading_and_dropdown() -> Scenario {
    scenario(
        "heading-and-dropdown",
        [
            "toolbar-heading",
            "toolbar-code-dropdown",
            "dropdown-next",
            "dropdown-enter",
        ],
        [
            pointer_button(HEADING_POINT, egui::PointerButton::Primary, true),
            pointer_button(HEADING_POINT, egui::PointerButton::Primary, false),
            pointer_button(CODE_DROPDOWN_POINT, egui::PointerButton::Primary, true),
            pointer_button(CODE_DROPDOWN_POINT, egui::PointerButton::Primary, false),
            key(egui::Key::ArrowDown, egui::Modifiers::NONE),
            key(egui::Key::Enter, egui::Modifiers::NONE),
        ],
    )
}

pub(super) fn context_menu_open_and_dismiss() -> Scenario {
    scenario(
        "context-menu-open-and-dismiss",
        [
            "context-secondary-click",
            "context-arrow-down",
            "context-enter",
            "context-escape",
        ],
        [
            pointer_button(TEXT_INPUT_POINT, egui::PointerButton::Secondary, true),
            pointer_button(TEXT_INPUT_POINT, egui::PointerButton::Secondary, false),
            key(egui::Key::ArrowDown, egui::Modifiers::NONE),
            key(egui::Key::Enter, egui::Modifiers::NONE),
            key(egui::Key::Escape, egui::Modifiers::NONE),
        ],
    )
}

pub(super) fn search_query_next_previous() -> Scenario {
    scenario(
        "search-query-next-previous",
        [
            "search-focus",
            "search-query",
            "search-next",
            "search-previous",
        ],
        [
            pointer_button(SEARCH_INPUT_POINT, egui::PointerButton::Primary, true),
            pointer_button(SEARCH_INPUT_POINT, egui::PointerButton::Primary, false),
            key(
                egui::Key::A,
                egui::Modifiers {
                    command: true,
                    ..egui::Modifiers::NONE
                },
            ),
            egui::Event::Text("⭐️".to_string()),
            key(egui::Key::Enter, egui::Modifiers::NONE),
            key(
                egui::Key::Enter,
                egui::Modifiers {
                    shift: true,
                    ..egui::Modifiers::NONE
                },
            ),
        ],
    )
}

pub(super) fn replace_and_replace_all() -> Scenario {
    scenario(
        "replace-and-replace-all",
        ["search-replace", "search-replace-all"],
        [
            pointer_button(SEARCH_CONTROL_POINT, egui::PointerButton::Primary, true),
            pointer_button(SEARCH_CONTROL_POINT, egui::PointerButton::Primary, false),
            pointer_button(
                SEARCH_REPLACE_CONTROL_POINT,
                egui::PointerButton::Primary,
                true,
            ),
            pointer_button(
                SEARCH_REPLACE_CONTROL_POINT,
                egui::PointerButton::Primary,
                false,
            ),
        ],
    )
}

fn scenario<const INPUTS: usize, const EVENTS: usize>(
    name: &'static str,
    input: [&'static str; INPUTS],
    events: [egui::Event; EVENTS],
) -> Scenario {
    Scenario {
        name,
        input: input.into(),
        events: events.into(),
    }
}
