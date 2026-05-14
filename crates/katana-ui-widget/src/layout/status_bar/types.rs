use floem::View;
use std::rc::Rc;

/// Severity levels for status messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusSeverity {
    Error,
    Warning,
    Success,
    #[default]
    Info,
}

/// Raw properties for `StatusBar`.
pub(crate) struct StatusBarProps {
    pub message: String,
    pub severity: StatusSeverity,
    pub trailing: Option<Box<dyn View>>,
    pub action_label: Option<String>,
    pub on_action: Rc<dyn Fn()>,
    pub height: Option<f32>,
    pub padding: Option<f32>,
    pub gap: Option<f32>,
}
