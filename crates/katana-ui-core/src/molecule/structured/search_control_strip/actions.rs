use super::{ReplaceMode, SearchControlStrip, SearchOptionKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchNavigationDirection {
    Previous,
    Next,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchReplaceScope {
    One,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchControlStripAction {
    SetSearchQuery(String),
    ToggleSearchOption(SearchOptionKind),
    Navigate(SearchNavigationDirection),
    SetReplaceMode(ReplaceMode),
    SetReplaceValue(String),
    Replace(SearchReplaceScope),
    SetResultPosition {
        result_count: usize,
        active_index: Option<usize>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchControlStripEvent {
    SearchQueryChanged(String),
    SearchOptionChanged {
        option: SearchOptionKind,
        enabled: bool,
    },
    SearchNavigationRequested {
        direction: SearchNavigationDirection,
    },
    ReplaceModeChanged(ReplaceMode),
    ReplaceValueChanged(String),
    ReplaceRequested {
        scope: SearchReplaceScope,
        value: String,
    },
    SearchResultPositionChanged {
        result_count: usize,
        active_index: Option<usize>,
    },
}

pub(super) fn apply(
    strip: &mut SearchControlStrip,
    action: SearchControlStripAction,
) -> Vec<SearchControlStripEvent> {
    match action {
        SearchControlStripAction::SetSearchQuery(value) => set_query(strip, value),
        SearchControlStripAction::ToggleSearchOption(value) => toggle_option(strip, value),
        SearchControlStripAction::Navigate(direction) => {
            vec![SearchControlStripEvent::SearchNavigationRequested { direction }]
        }
        SearchControlStripAction::SetReplaceMode(value) => set_replace_mode(strip, value),
        SearchControlStripAction::SetReplaceValue(value) => set_replace_value(strip, value),
        SearchControlStripAction::Replace(scope) => replace(strip, scope),
        SearchControlStripAction::SetResultPosition {
            result_count,
            active_index,
        } => set_result_position(strip, result_count, active_index),
    }
}

fn set_query(strip: &mut SearchControlStrip, value: String) -> Vec<SearchControlStripEvent> {
    strip.query = value.clone();
    vec![SearchControlStripEvent::SearchQueryChanged(value)]
}

fn toggle_option(
    strip: &mut SearchControlStrip,
    value: SearchOptionKind,
) -> Vec<SearchControlStripEvent> {
    let enabled = strip.options.toggle(value);
    vec![SearchControlStripEvent::SearchOptionChanged {
        option: value,
        enabled,
    }]
}

fn set_replace_mode(
    strip: &mut SearchControlStrip,
    value: ReplaceMode,
) -> Vec<SearchControlStripEvent> {
    strip.replace_mode = value;
    vec![SearchControlStripEvent::ReplaceModeChanged(value)]
}

fn set_replace_value(
    strip: &mut SearchControlStrip,
    value: String,
) -> Vec<SearchControlStripEvent> {
    strip.replace_value = value.clone();
    vec![SearchControlStripEvent::ReplaceValueChanged(value)]
}

fn replace(strip: &SearchControlStrip, scope: SearchReplaceScope) -> Vec<SearchControlStripEvent> {
    if strip.replace_mode != ReplaceMode::Visible {
        return Vec::new();
    }
    vec![SearchControlStripEvent::ReplaceRequested {
        scope,
        value: strip.replace_value.clone(),
    }]
}

fn set_result_position(
    strip: &mut SearchControlStrip,
    result_count: usize,
    active_index: Option<usize>,
) -> Vec<SearchControlStripEvent> {
    strip.result_count = Some(result_count);
    strip.active_index = active_index;
    vec![SearchControlStripEvent::SearchResultPositionChanged {
        result_count,
        active_index,
    }]
}
