use katana_ui_core::render_model::UiRect;
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::root::KucRootEventBatchContext;

const LEDGER_ID: &str = "kuc.text-command.accesskit-evidence";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AccessKitTargetClass {
    TextSurfaceContextTarget,
    TextInput,
    Toolbar,
    FloatingToolbar,
    DropdownTrigger,
    DropdownItem,
    SearchControl,
    ContextMenuItem,
    TabStripControl,
    StatusBarSegment,
    DiagnosticsScope,
    DiagnosticsSeverityFilter,
    DiagnosticsItem,
    DiagnosticsFix,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AccessKitEvidence {
    pub(crate) response_id: egui::Id,
    pub(crate) bounds: UiRect,
    pub(crate) label: String,
    pub(crate) disabled: bool,
    pub(crate) target_identity: String,
    pub(crate) target_class: AccessKitTargetClass,
}

#[derive(Debug, Clone)]
pub(crate) struct BoundAccessKitEvidence {
    root_identity: String,
    state_revision: u64,
    correlation_fingerprint: String,
    entries: Vec<AccessKitEvidence>,
}

impl BoundAccessKitEvidence {
    pub(crate) fn entries(&self) -> &[AccessKitEvidence] {
        &self.entries
    }

    pub(crate) fn matches(&self, context: &KucRootEventBatchContext, root_identity: &str) -> bool {
        self.root_identity == root_identity
            && self.state_revision == context.state_revision()
            && self.correlation_fingerprint == context.correlation_fingerprint()
    }
}

#[derive(Debug, Clone, Default)]
struct Ledger(Vec<AccessKitEvidence>);

fn ledger_id() -> egui::Id {
    egui::Id::new(LEDGER_ID)
}

struct AccessKitEvidenceLedger;

impl AccessKitEvidenceLedger {
    fn begin_frame(ctx: &egui::Context) {
        ctx.data_mut(|data| data.insert_temp(ledger_id(), Ledger::default()));
    }

    fn record(ctx: &egui::Context, evidence: AccessKitEvidence) {
        ctx.data_mut(|data| {
            let mut ledger = data.get_temp::<Ledger>(ledger_id()).unwrap_or_default();
            ledger.0.push(evidence);
            data.insert_temp(ledger_id(), ledger);
        });
    }

    fn publish_labeled_button_accesskit(
        ui: &egui::Ui,
        id: egui::Id,
        label: &str,
        disabled: bool,
        bounds: UiRect,
        target_identity: &str,
        target_class: AccessKitTargetClass,
    ) {
        ui.ctx().accesskit_node_builder(id, |node| {
            node.set_role(egui::accesskit::Role::Button);
            node.set_label(label);
            node.set_bounds(egui::accesskit::Rect {
                x0: bounds.x.into(),
                y0: bounds.y.into(),
                x1: bounds.x.saturating_add(bounds.width as i32).into(),
                y1: bounds.y.saturating_add(bounds.height as i32).into(),
            });
            node.add_action(egui::accesskit::Action::Click);
            if disabled {
                node.set_disabled();
            }
        });
        Self::record(
            ui.ctx(),
            AccessKitEvidence {
                response_id: id,
                bounds,
                label: label.to_owned(),
                disabled,
                target_identity: target_identity.to_owned(),
                target_class,
            },
        );
    }

    fn record_custom(
        ctx: &egui::Context,
        response_id: egui::Id,
        bounds: UiRect,
        label: &str,
        disabled: bool,
        target_identity: &str,
        target_class: AccessKitTargetClass,
    ) {
        Self::record(
            ctx,
            AccessKitEvidence {
                response_id,
                bounds,
                label: label.to_owned(),
                disabled,
                target_identity: target_identity.to_owned(),
                target_class,
            },
        );
    }

    fn finish_frame(ctx: &egui::Context) -> Vec<AccessKitEvidence> {
        ctx.data_mut(|data| data.get_temp::<Ledger>(ledger_id()).unwrap_or_default().0)
    }

    fn snapshot_hash(entries: &[AccessKitEvidence]) -> Result<String, String> {
        let material: Vec<_> = entries
            .iter()
            .map(|entry| AccessKitEvidenceSnapshot {
                response_id: format!("{:?}", entry.response_id),
                bounds: AccessKitBoundsSnapshot::from(entry.bounds),
                label: &entry.label,
                disabled: entry.disabled,
                target_class: target_class_name(entry.target_class),
            })
            .collect();
        serde_json::to_vec(&material)
            .map(|bytes| hex::encode(Sha256::digest(bytes)))
            .map_err(|error| error.to_string())
    }
}

type PublishLabeledButtonAccessKit =
    fn(&egui::Ui, egui::Id, &str, bool, UiRect, &str, AccessKitTargetClass);

#[allow(non_upper_case_globals)]
pub(crate) const begin_frame: fn(&egui::Context) = AccessKitEvidenceLedger::begin_frame;
#[allow(non_upper_case_globals)]
pub(crate) const record: fn(&egui::Context, AccessKitEvidence) = AccessKitEvidenceLedger::record;
#[allow(non_upper_case_globals)]
pub(crate) const publish_labeled_button_accesskit: PublishLabeledButtonAccessKit =
    AccessKitEvidenceLedger::publish_labeled_button_accesskit;
#[allow(non_upper_case_globals)]
pub(crate) const record_custom: fn(
    &egui::Context,
    egui::Id,
    UiRect,
    &str,
    bool,
    &str,
    AccessKitTargetClass,
) = AccessKitEvidenceLedger::record_custom;
#[allow(non_upper_case_globals)]
pub(crate) const finish_frame: fn(&egui::Context) -> Vec<AccessKitEvidence> =
    AccessKitEvidenceLedger::finish_frame;
#[allow(non_upper_case_globals)]
pub(crate) const snapshot_hash: fn(&[AccessKitEvidence]) -> Result<String, String> =
    AccessKitEvidenceLedger::snapshot_hash;

#[derive(Serialize)]
struct AccessKitEvidenceSnapshot<'a> {
    response_id: String,
    bounds: AccessKitBoundsSnapshot,
    label: &'a str,
    disabled: bool,
    target_class: &'static str,
}

#[derive(Serialize)]
struct AccessKitBoundsSnapshot {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl From<UiRect> for AccessKitBoundsSnapshot {
    fn from(bounds: UiRect) -> Self {
        Self {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: bounds.height,
        }
    }
}

fn target_class_name(class: AccessKitTargetClass) -> &'static str {
    match class {
        AccessKitTargetClass::TextSurfaceContextTarget => "text-surface-context-target",
        AccessKitTargetClass::TextInput => "text-input",
        AccessKitTargetClass::Toolbar => "toolbar",
        AccessKitTargetClass::FloatingToolbar => "floating-toolbar",
        AccessKitTargetClass::DropdownTrigger => "dropdown-trigger",
        AccessKitTargetClass::DropdownItem => "dropdown-item",
        AccessKitTargetClass::SearchControl => "search-control",
        AccessKitTargetClass::ContextMenuItem => "context-menu-item",
        AccessKitTargetClass::TabStripControl => "tab-strip-control",
        AccessKitTargetClass::StatusBarSegment => "status-bar-segment",
        AccessKitTargetClass::DiagnosticsScope => "diagnostics-scope",
        AccessKitTargetClass::DiagnosticsSeverityFilter => "diagnostics-severity-filter",
        AccessKitTargetClass::DiagnosticsItem => "diagnostics-item",
        AccessKitTargetClass::DiagnosticsFix => "diagnostics-fix",
    }
}

#[cfg(test)]
mod tests {
    use super::AccessKitTargetClass;
    use super::target_class_name;

    #[test]
    fn status_bar_segment_and_diagnostics_fix_class_names_are_mapped() {
        assert_eq!(
            "status-bar-segment",
            target_class_name(AccessKitTargetClass::StatusBarSegment)
        );
        assert_eq!(
            "diagnostics-fix",
            target_class_name(AccessKitTargetClass::DiagnosticsFix)
        );
    }
}

impl BoundAccessKitEvidence {
    fn bind_frame(
        entries: Vec<AccessKitEvidence>,
        root_identity: &str,
        context: &KucRootEventBatchContext,
    ) -> Self {
        Self {
            root_identity: root_identity.to_owned(),
            state_revision: context.state_revision(),
            correlation_fingerprint: context.correlation_fingerprint().to_owned(),
            entries,
        }
    }
}

#[allow(non_upper_case_globals)]
pub(crate) const bind_frame: fn(
    Vec<AccessKitEvidence>,
    &str,
    &KucRootEventBatchContext,
) -> BoundAccessKitEvidence = BoundAccessKitEvidence::bind_frame;
