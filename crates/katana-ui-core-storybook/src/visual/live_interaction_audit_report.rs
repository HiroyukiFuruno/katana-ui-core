use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StorybookLiveInteractionAuditReport {
    pub scenarios: Vec<StorybookLiveInteractionScenario>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StorybookLiveInteractionScenario {
    pub page: &'static str,
    pub operation: &'static str,
    pub operation_kind: &'static str,
    pub clicked: bool,
    pub passed: bool,
    pub action: &'static str,
    pub event: &'static str,
    pub state: &'static str,
    pub checked: bool,
    pub selected: bool,
    pub body_pixel_diff: usize,
    pub clipboard_text_len: usize,
}
