use crate::molecule::structured::{ReplaceMode, SearchControlStrip, SearchOptions};
use crate::render_model::UiIconProps;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromeText {
    pub visible: String,
    pub tooltip: String,
    pub accessibility_label: String,
}

impl CommandChromeText {
    #[must_use]
    pub fn new(
        visible: impl Into<String>,
        tooltip: impl Into<String>,
        accessibility_label: impl Into<String>,
    ) -> Self {
        Self {
            visible: visible.into(),
            tooltip: tooltip.into(),
            accessibility_label: accessibility_label.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchControlStrings {
    pub strip: CommandChromeText,
    pub query: CommandChromeText,
    pub replace: CommandChromeText,
    pub match_case: CommandChromeText,
    pub whole_word: CommandChromeText,
    pub use_regex: CommandChromeText,
    pub previous: CommandChromeText,
    pub next: CommandChromeText,
    pub replace_one: CommandChromeText,
    pub replace_all: CommandChromeText,
    pub close: CommandChromeText,
    pub result_summary: SearchResultSummaryTemplate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchControlIconSlot {
    MatchCase,
    WholeWord,
    UseRegex,
    Previous,
    Next,
    ReplaceOne,
    ReplaceAll,
    Close,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchControlIcons {
    entries: Vec<(SearchControlIconSlot, UiIconProps)>,
}

impl SearchControlIcons {
    #[must_use]
    pub fn icon(mut self, slot: SearchControlIconSlot, icon: UiIconProps) -> Self {
        self.entries.retain(|(current, _)| *current != slot);
        self.entries.push((slot, icon));
        self
    }

    #[must_use]
    pub fn icon_for(&self, slot: SearchControlIconSlot) -> Option<&UiIconProps> {
        self.entries
            .iter()
            .find(|(current, _)| *current == slot)
            .map(|(_, icon)| icon)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultSummaryTemplate {
    pub empty: String,
    pub zero_results: String,
    pub single_result: String,
    pub indexed_result: String,
    pub count_results: String,
}

impl SearchResultSummaryTemplate {
    #[must_use]
    pub fn format(&self, parameters: SearchResultSummaryParameters) -> String {
        let template = match (parameters.result_count, parameters.active_index) {
            (None, _) => &self.empty,
            (Some(0), _) => &self.zero_results,
            (Some(1), _) => &self.single_result,
            (Some(_), Some(_)) => &self.indexed_result,
            (Some(_), None) => &self.count_results,
        };
        replace_parameters(template, parameters)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultSummaryParameters {
    pub result_count: Option<usize>,
    pub active_index: Option<usize>,
}

impl SearchResultSummaryParameters {
    #[must_use]
    pub const fn new(result_count: Option<usize>, active_index: Option<usize>) -> Self {
        Self {
            result_count,
            active_index,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandChromeCapability {
    Available,
    Unavailable(CommandChromeUnavailableCapability),
}

impl CommandChromeCapability {
    #[must_use]
    pub const fn available() -> Self {
        Self::Available
    }

    #[must_use]
    pub fn unavailable(disabled_reason: impl Into<String>) -> Self {
        Self::Unavailable(CommandChromeUnavailableCapability {
            disabled_reason: disabled_reason.into(),
        })
    }

    #[must_use]
    pub const fn is_available(&self) -> bool {
        matches!(self, Self::Available)
    }

    #[must_use]
    pub fn disabled_reason(&self) -> Option<&str> {
        match self {
            Self::Available => None,
            Self::Unavailable(value) => Some(&value.disabled_reason),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromeUnavailableCapability {
    pub disabled_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchControlCapabilities {
    pub regex: CommandChromeCapability,
    pub replace: CommandChromeCapability,
    pub navigation: CommandChromeCapability,
    pub close: CommandChromeCapability,
}

impl SearchControlCapabilities {
    #[must_use]
    pub const fn all_available() -> Self {
        Self {
            regex: CommandChromeCapability::Available,
            replace: CommandChromeCapability::Available,
            navigation: CommandChromeCapability::Available,
            close: CommandChromeCapability::Available,
        }
    }
}

impl Default for SearchControlCapabilities {
    fn default() -> Self {
        Self::all_available()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandChromeSearchStrip {
    pub(super) strip: SearchControlStrip,
    pub(super) strings: SearchControlStrings,
    pub(super) capabilities: SearchControlCapabilities,
    pub(super) icons: SearchControlIcons,
}

/// Controlled search/replace presentation. Updates are not user interaction events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandChromeSearchPresentation {
    pub query: String,
    pub options: SearchOptions,
    pub result_count: Option<usize>,
    pub active_index: Option<usize>,
    pub replace_mode: ReplaceMode,
    pub replace_value: String,
    pub strings: SearchControlStrings,
    pub capabilities: SearchControlCapabilities,
    pub icons: SearchControlIcons,
}

impl CommandChromeSearchStrip {
    #[must_use]
    pub fn new(strip: SearchControlStrip, strings: SearchControlStrings) -> Self {
        Self {
            strip,
            strings,
            capabilities: SearchControlCapabilities::default(),
            icons: SearchControlIcons::default(),
        }
    }

    #[must_use]
    pub fn capabilities(mut self, value: SearchControlCapabilities) -> Self {
        self.capabilities = value;
        self
    }

    #[must_use]
    pub fn icons(mut self, value: SearchControlIcons) -> Self {
        self.icons = value;
        self
    }

    #[must_use]
    pub const fn strings_model(&self) -> &SearchControlStrings {
        &self.strings
    }

    #[must_use]
    pub const fn capabilities_model(&self) -> &SearchControlCapabilities {
        &self.capabilities
    }

    #[must_use]
    pub const fn icons_model(&self) -> &SearchControlIcons {
        &self.icons
    }

    #[must_use]
    pub fn query_model(&self) -> &str {
        self.strip.query_model()
    }

    #[must_use]
    pub fn state_id_model(&self) -> &crate::render_model::UiStateId {
        self.strip.state_id()
    }

    #[must_use]
    pub const fn options_model(&self) -> &SearchOptions {
        self.strip.options_model()
    }

    #[must_use]
    pub const fn replace_mode_model(&self) -> ReplaceMode {
        self.strip.replace_mode_model()
    }

    #[must_use]
    pub const fn result_count_model(&self) -> Option<usize> {
        self.strip.result_count_model()
    }

    #[must_use]
    pub const fn active_index_model(&self) -> Option<usize> {
        self.strip.active_index_model()
    }

    #[must_use]
    pub fn replace_value_model(&self) -> &str {
        self.strip.replace_value_model()
    }

    #[must_use]
    pub fn result_summary_model(&self) -> String {
        self.strings
            .result_summary
            .format(SearchResultSummaryParameters::new(
                self.strip.result_count_model(),
                self.strip.active_index_model(),
            ))
    }
}

fn replace_parameters(template: &str, parameters: SearchResultSummaryParameters) -> String {
    let count = parameters.result_count.unwrap_or_default();
    let active = parameters
        .active_index
        .map_or(0, |index| index.saturating_add(1).min(count));
    template
        .replace("{active}", &active.to_string())
        .replace("{count}", &count.to_string())
}
