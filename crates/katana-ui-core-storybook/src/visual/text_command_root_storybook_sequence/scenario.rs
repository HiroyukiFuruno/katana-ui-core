use super::input;
use eframe::egui;

pub(super) struct Scenario {
    pub(super) name: &'static str,
    pub(super) input: Vec<&'static str>,
    pub(super) events: Vec<egui::Event>,
}

pub(super) fn scripted_steps() -> Vec<Scenario> {
    vec![
        input::initial_root(),
        input::focus_and_multiline_input(),
        input::ime_preedit(),
        input::ime_commit(),
        input::selection_anchored_floating_toolbar(),
        input::heading_and_dropdown(),
        input::context_menu_open_and_dismiss(),
        input::search_query_next_previous(),
        input::replace_and_replace_all(),
    ]
}

pub(super) fn required_step_names() -> &'static [&'static str] {
    &[
        "focus-and-multiline-input",
        "ime-preedit",
        "ime-commit",
        "selection-anchored-floating-toolbar",
        "heading-and-dropdown",
        "context-menu-open-and-dismiss",
        "search-query-next-previous",
        "replace-and-replace-all",
    ]
}
