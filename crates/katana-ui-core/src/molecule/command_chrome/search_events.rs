use crate::molecule::structured::{SearchControlStripAction, SearchControlStripEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandChromeSearchAction {
    Strip { action: SearchControlStripAction },
    RequestClose,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandChromeSearchEvent {
    Strip { event: SearchControlStripEvent },
    CloseRequested,
}
