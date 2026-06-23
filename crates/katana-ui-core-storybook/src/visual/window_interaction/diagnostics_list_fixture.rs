use katana_ui_core::molecule::{
    DiagnosticAction, DiagnosticId, DiagnosticItem, DiagnosticLocation, DiagnosticSeverity,
    DiagnosticsGroupBy, DiagnosticsList, DiagnosticsListOptions, DiagnosticsSortBy,
};
use std::collections::BTreeSet;

pub(in crate::visual) const ERROR_ID: &str = "syntax-error";
const WARNING_ID: &str = "unused-import";
const ERROR_LINE: u32 = 12;
const WARNING_LINE: u32 = 24;
const ERROR_COLUMN: u32 = 9;
const WARNING_COLUMN: u32 = 5;

pub(in crate::visual) fn diagnostics_list() -> DiagnosticsList {
    DiagnosticsList::new("Storybook diagnostics")
        .option(default_options())
        .item(diagnostic_error())
        .item(diagnostic_warning())
}

pub(in crate::visual) fn error_id() -> DiagnosticId {
    DiagnosticId::new(ERROR_ID)
}

fn default_options() -> DiagnosticsListOptions {
    DiagnosticsListOptions {
        group_by: DiagnosticsGroupBy::Severity,
        sort_by: DiagnosticsSortBy::Severity,
        severity_filter: [DiagnosticSeverity::Error, DiagnosticSeverity::Warning]
            .into_iter()
            .collect::<BTreeSet<_>>(),
        ..DiagnosticsListOptions::default()
    }
}

fn diagnostic_error() -> DiagnosticItem {
    DiagnosticItem::new(
        ERROR_ID,
        DiagnosticSeverity::Error,
        "Missing semicolon",
        DiagnosticLocation::new("src/lib.rs", ERROR_LINE, ERROR_COLUMN),
    )
    .source("rustc")
    .quickfix(DiagnosticAction::new(
        "insert-semicolon",
        "Insert semicolon",
    ))
}

fn diagnostic_warning() -> DiagnosticItem {
    DiagnosticItem::new(
        WARNING_ID,
        DiagnosticSeverity::Warning,
        "Unused import",
        DiagnosticLocation::new("src/story.rs", WARNING_LINE, WARNING_COLUMN),
    )
    .source("clippy")
}
