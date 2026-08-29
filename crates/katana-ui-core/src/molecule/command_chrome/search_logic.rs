use super::{
    CommandChromeSearchAction, CommandChromeSearchEvent, CommandChromeSearchStrip,
    SearchControlCapabilities,
};
use crate::molecule::structured::{SearchControlStripAction, SearchOptionKind};

impl CommandChromeSearchStrip {
    #[must_use]
    pub fn apply_action(
        &mut self,
        action: CommandChromeSearchAction,
    ) -> Vec<CommandChromeSearchEvent> {
        match action {
            CommandChromeSearchAction::RequestClose => self
                .capabilities
                .close
                .is_available()
                .then_some(CommandChromeSearchEvent::CloseRequested)
                .into_iter()
                .collect(),
            CommandChromeSearchAction::Strip { action } => {
                if !allows_action(&self.capabilities, &action) {
                    return Vec::new();
                }
                self.strip
                    .apply_action(action)
                    .into_iter()
                    .map(|event| CommandChromeSearchEvent::Strip { event })
                    .collect()
            }
        }
    }
}

fn allows_action(
    capabilities: &SearchControlCapabilities,
    action: &SearchControlStripAction,
) -> bool {
    match action {
        SearchControlStripAction::ToggleSearchOption(SearchOptionKind::UseRegex) => {
            capabilities.regex.is_available()
        }
        SearchControlStripAction::Navigate(_) => capabilities.navigation.is_available(),
        SearchControlStripAction::SetReplaceValue(_) | SearchControlStripAction::Replace(_) => {
            capabilities.replace.is_available()
        }
        SearchControlStripAction::SetSearchQuery(_)
        | SearchControlStripAction::ToggleSearchOption(_)
        | SearchControlStripAction::SetReplaceMode(_)
        | SearchControlStripAction::SetResultPosition { .. } => true,
    }
}
