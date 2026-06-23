use katana_ui_core::molecule::{
    SearchControlStrip, SearchControlStripAction, SearchControlStripEvent,
    SearchNavigationDirection, SearchOptionKind,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct SearchControlScreenState {
    pub(super) query_changed: bool,
    pub(super) regex_enabled: bool,
    pub(super) focused: bool,
    pub(super) hovered: bool,
    pub(super) navigated_next: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SearchControlScreenAction {
    Query,
    ToggleRegex,
    Focus,
    Hover,
    KeyboardNext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SearchControlScreenUpdate {
    pub(super) action: &'static str,
    pub(super) event: &'static str,
    pub(super) state: &'static str,
}

impl SearchControlScreenState {
    pub(in crate::visual) fn apply(
        &mut self,
        action: SearchControlScreenAction,
    ) -> SearchControlScreenUpdate {
        match action {
            SearchControlScreenAction::Query => self.query(),
            SearchControlScreenAction::ToggleRegex => self.toggle_regex(),
            SearchControlScreenAction::Focus => {
                self.focused = true;
                SearchControlScreenUpdate::new("search_control_focus", "focus", "focus=true")
            }
            SearchControlScreenAction::Hover => {
                self.hovered = true;
                SearchControlScreenUpdate::new("search_control_hover", "hover_start", "hover=true")
            }
            SearchControlScreenAction::KeyboardNext => self.keyboard_next(),
        }
    }

    fn query(&mut self) -> SearchControlScreenUpdate {
        let mut strip = self.core_strip();
        let events = strip.apply_action(SearchControlStripAction::SetSearchQuery(
            "heading".to_string(),
        ));
        self.query_changed = strip.query_model() == "heading";
        SearchControlScreenUpdate::new("search_query_changed", event_name(&events), "query=heading")
    }

    fn toggle_regex(&mut self) -> SearchControlScreenUpdate {
        let mut strip = self.core_strip();
        let events = strip.apply_action(SearchControlStripAction::ToggleSearchOption(
            SearchOptionKind::UseRegex,
        ));
        self.regex_enabled = strip.options_model().use_regex;
        SearchControlScreenUpdate::new("search_regex_toggle", event_name(&events), "regex=true")
    }

    fn keyboard_next(&mut self) -> SearchControlScreenUpdate {
        let mut strip = self.core_strip();
        let events = strip.apply_action(SearchControlStripAction::Navigate(
            SearchNavigationDirection::Next,
        ));
        self.navigated_next = matches!(
            events.first(),
            Some(SearchControlStripEvent::SearchNavigationRequested {
                direction: SearchNavigationDirection::Next
            })
        );
        SearchControlScreenUpdate::new(
            "search_control_keyboard_next",
            event_name(&events),
            "navigation=next",
        )
    }

    fn core_strip(self) -> SearchControlStrip {
        SearchControlStrip::new("Storybook search controls")
    }
}

fn event_name(events: &[SearchControlStripEvent]) -> &'static str {
    match events.first() {
        Some(SearchControlStripEvent::SearchQueryChanged(_)) => "search_query_changed",
        Some(SearchControlStripEvent::SearchOptionChanged { .. }) => "search_option_changed",
        Some(SearchControlStripEvent::SearchNavigationRequested { .. }) => {
            "search_navigation_requested"
        }
        Some(SearchControlStripEvent::ReplaceModeChanged(_)) => "replace_mode_changed",
        Some(SearchControlStripEvent::ReplaceValueChanged(_)) => "replace_value_changed",
        Some(SearchControlStripEvent::ReplaceRequested { .. }) => "replace_requested",
        Some(SearchControlStripEvent::SearchResultPositionChanged { .. }) => {
            "search_result_position_changed"
        }
        None => "none",
    }
}

impl SearchControlScreenUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}
