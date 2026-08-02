use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::SearchBox;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct SearchBoxScreenState {
    pub(super) typed: bool,
    pub(super) cleared: bool,
    pub(super) submitted: bool,
    pub(super) case_sensitive: bool,
    pub(super) regex: bool,
    pub(super) focused: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SearchBoxScreenAction {
    StateRead,
    TypeQuery,
    Submit,
    Clear,
    Focus,
    KeyboardSubmit,
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
            SearchBoxScreenAction::TypeQuery => self.type_query(),
            SearchBoxScreenAction::Submit => self.submit(),
            SearchBoxScreenAction::Clear => self.clear(),
            SearchBoxScreenAction::Focus => self.focus(),
            SearchBoxScreenAction::KeyboardSubmit => self.keyboard_submit(),
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

    fn type_query(&mut self) -> SearchBoxScreenUpdate {
        let mut search = self.core_search_box();
        let action = UiAction::input_value(search.state_id().clone(), "typed query");
        let result = search.apply_action(&action);
        self.typed = result.handled && result.after.value == "typed query";
        self.cleared = false;
        SearchBoxScreenUpdate::new("search_type_query", action.name(), self.state_summary())
    }

    fn submit(&mut self) -> SearchBoxScreenUpdate {
        let mut search = self.core_search_box();
        let action = UiAction::search_submitted(search.state_id().clone());
        let result = search.apply_action(&action);
        self.submitted = result
            .callback_log
            .iter()
            .any(|log| log.action == "search_submitted");
        let state = if self.typed {
            self.state_summary()
        } else {
            "submitted=true"
        };
        SearchBoxScreenUpdate::new("search_submit", action.name(), state)
    }

    fn focus(&mut self) -> SearchBoxScreenUpdate {
        let mut search = self.core_search_box();
        let action = UiAction::focus(search.state_id().clone());
        let result = search.apply_action(&action);
        self.focused = result.handled && result.after.focused;
        SearchBoxScreenUpdate::new("search_focus", action.name(), "focus=true")
    }

    fn keyboard_submit(&mut self) -> SearchBoxScreenUpdate {
        let mut search = self.core_search_box();
        let action = UiAction::search_submitted(search.state_id().clone());
        let result = search.apply_action(&action);
        self.submitted = result
            .callback_log
            .iter()
            .any(|log| log.action == "search_submitted");
        SearchBoxScreenUpdate::new(
            "search_keyboard_submit",
            action.name(),
            "value=query submitted=true",
        )
    }

    fn clear(&mut self) -> SearchBoxScreenUpdate {
        let mut search = self.core_search_box();
        let action = UiAction::clear_value(search.state_id().clone());
        let result = search.apply_action(&action);
        self.cleared = result.handled && result.after.value.is_empty();
        self.typed = false;
        self.submitted = false;
        SearchBoxScreenUpdate::new("search_clear", action.name(), self.state_summary())
    }

    fn core_search_box(self) -> SearchBox {
        SearchBox::new("Storybook search")
            .value(self.query_value())
            .submit_on_enter(true)
            .case_sensitive(self.case_sensitive)
    }

    fn query_value(self) -> &'static str {
        if self.cleared {
            ""
        } else if self.typed {
            "typed query"
        } else {
            "query"
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
            (false, false, true, false, false) => "value=query submitted=true",
            (false, true, false, false, false) => "value=empty case=false regex=false",
            (false, true, false, true, false) => "value=empty case=true regex=false",
            (false, true, false, true, true) => "value=empty case=true regex=true",
            (true, false, false, true, false) => "value=typed query case=true regex=false",
            (true, false, false, true, true) => "value=typed query case=true regex=true",
            (true, false, false, false, true) => "value=typed query case=false regex=true",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_summary_covers_submit_case_and_regex_combinations() {
        let summary = |typed, submitted, case_sensitive, regex| {
            SearchBoxScreenState {
                typed,
                submitted,
                case_sensitive,
                regex,
                ..Default::default()
            }
            .state_summary()
        };

        assert_eq!(
            "value=query submitted=true",
            summary(false, true, false, false)
        );
        assert_eq!(
            "value=typed query case=true regex=false",
            summary(true, false, true, false)
        );
        assert_eq!(
            "value=typed query case=true regex=true",
            summary(true, false, true, true)
        );
        assert_eq!(
            "value=typed query case=false regex=true",
            summary(true, false, false, true)
        );
    }
}
