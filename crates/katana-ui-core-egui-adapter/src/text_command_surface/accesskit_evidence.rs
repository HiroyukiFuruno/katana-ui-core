use katana_ui_core::render_model::UiRect;

use super::root::KucRootEventBatchContext;

const LEDGER_ID: &str = "kuc.text-command.accesskit-evidence";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AccessKitTargetClass {
    TextSurfaceContextTarget,
    Toolbar,
    FloatingToolbar,
    DropdownTrigger,
    DropdownItem,
    SearchControl,
    ContextMenuItem,
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

pub(crate) struct AccessKitEvidenceLedger;

impl AccessKitEvidenceLedger {
    pub(crate) fn begin_frame(ctx: &egui::Context) {
        ctx.data_mut(|data| data.insert_temp(ledger_id(), Ledger::default()));
    }

    pub(crate) fn record(ctx: &egui::Context, evidence: AccessKitEvidence) {
        ctx.data_mut(|data| {
            let mut ledger = data.get_temp::<Ledger>(ledger_id()).unwrap_or_default();
            ledger.0.push(evidence);
            data.insert_temp(ledger_id(), ledger);
        });
    }

    pub(crate) fn finish_frame(ctx: &egui::Context) -> Vec<AccessKitEvidence> {
        ctx.data_mut(|data| data.get_temp::<Ledger>(ledger_id()).unwrap_or_default().0)
    }

    pub(crate) fn bind_frame(
        entries: Vec<AccessKitEvidence>,
        root_identity: &str,
        context: &KucRootEventBatchContext,
    ) -> BoundAccessKitEvidence {
        BoundAccessKitEvidence {
            root_identity: root_identity.to_owned(),
            state_revision: context.state_revision(),
            correlation_fingerprint: context.correlation_fingerprint().to_owned(),
            entries,
        }
    }
}
