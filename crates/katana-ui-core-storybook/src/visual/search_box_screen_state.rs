#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct SearchBoxScreenState {
    pub(super) typed: bool,
    pub(super) cleared: bool,
    pub(super) submitted: bool,
    pub(super) case_sensitive: bool,
    pub(super) regex: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SearchBoxScreenAction {
    StateRead,
    TypeQuery,
    Submit,
    Clear,
    ToggleCase,
    ToggleRegex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SearchBoxScreenUpdate {
    pub(super) action: &'static str,
    pub(super) event: &'static str,
    pub(super) state: &'static str,
}

impl SearchBoxScreenState {
    pub(super) fn apply(&mut self, action: SearchBoxScreenAction) -> SearchBoxScreenUpdate {
        match action {
            SearchBoxScreenAction::StateRead => SearchBoxScreenUpdate::new(
                "search_state_read",
                "search_value_read",
                self.state_summary(),
            ),
            SearchBoxScreenAction::TypeQuery => {
                self.typed = true;
                self.cleared = false;
                SearchBoxScreenUpdate::new("search_type_query", "input_value", self.state_summary())
            }
            SearchBoxScreenAction::Submit => {
                self.submitted = true;
                SearchBoxScreenUpdate::new(
                    "search_submit",
                    "search_submitted",
                    self.state_summary(),
                )
            }
            SearchBoxScreenAction::Clear => {
                self.cleared = true;
                self.typed = false;
                self.submitted = false;
                SearchBoxScreenUpdate::new("search_clear", "clear_value", self.state_summary())
            }
            SearchBoxScreenAction::ToggleCase => {
                self.case_sensitive = !self.case_sensitive;
                SearchBoxScreenUpdate::new(
                    "search_case_toggle",
                    "search_option_changed",
                    self.state_summary(),
                )
            }
            SearchBoxScreenAction::ToggleRegex => {
                self.regex = !self.regex;
                SearchBoxScreenUpdate::new(
                    "search_regex_toggle",
                    "search_option_changed",
                    self.state_summary(),
                )
            }
        }
    }

    pub(super) fn state_summary(self) -> &'static str {
        match (
            self.typed,
            self.cleared,
            self.submitted,
            self.case_sensitive,
            self.regex,
        ) {
            (false, false, false, false, false) => "value=query case=false regex=false",
            (true, false, false, false, false) => "value=typed query case=false regex=false",
            (true, false, true, false, false) => "value=typed query submitted=true",
            (false, true, false, false, false) => "value=empty case=false regex=false",
            (false, true, false, true, false) => "value=empty case=true regex=false",
            (false, true, false, true, true) => "value=empty case=true regex=true",
            (true, false, false, true, false) => "value=typed query case=true regex=false",
            (true, false, false, true, true) => "value=typed query case=true regex=true",
            _ => "value=query case=false regex=false",
        }
    }
}

impl SearchBoxScreenUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}
