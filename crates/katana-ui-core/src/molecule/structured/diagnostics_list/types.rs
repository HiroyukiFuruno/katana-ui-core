use crate::molecule::CodeDiff;
use serde::{Deserialize, Serialize};

const DIAGNOSTIC_SEVERITY_COUNT: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DiagnosticId(String);

impl DiagnosticId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum DiagnosticSeverity {
    #[default]
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl DiagnosticLocation {
    #[must_use]
    pub fn new(file: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            file: file.into(),
            line,
            column,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticAction {
    pub id: String,
    pub label: String,
}

impl DiagnosticAction {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticFixPreview {
    pub diff: CodeDiff,
}

impl DiagnosticFixPreview {
    #[must_use]
    pub fn new(diff: CodeDiff) -> Self {
        Self { diff }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticItem {
    pub id: DiagnosticId,
    pub message: String,
    pub severity: DiagnosticSeverity,
    pub source: String,
    pub location: DiagnosticLocation,
    pub quickfix: Option<DiagnosticAction>,
    pub fix_preview: Option<DiagnosticFixPreview>,
}

impl DiagnosticItem {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
        location: DiagnosticLocation,
    ) -> Self {
        Self {
            id: DiagnosticId::new(id),
            message: message.into(),
            severity,
            source: String::new(),
            location,
            quickfix: None,
            fix_preview: None,
        }
    }

    #[must_use]
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = value.into();
        self
    }

    #[must_use]
    pub fn quickfix(mut self, value: DiagnosticAction) -> Self {
        self.quickfix = Some(value);
        self
    }

    #[must_use]
    pub fn fix_preview(mut self, value: DiagnosticFixPreview) -> Self {
        self.fix_preview = Some(value);
        self
    }
}

impl DiagnosticSeverity {
    #[must_use]
    pub fn all() -> [Self; DIAGNOSTIC_SEVERITY_COUNT] {
        [Self::Error, Self::Warning, Self::Info, Self::Hint]
    }
}
