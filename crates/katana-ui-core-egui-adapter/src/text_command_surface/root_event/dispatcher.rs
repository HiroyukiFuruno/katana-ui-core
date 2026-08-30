use super::super::super::source_address_projection_lease::SourceAddressSubmissionPortHandle;
use super::super::super::types::EguiTextCommandSurfaceOutput;
use super::{EguiTextCommandSurfaceRootEventBatch, RootEventPayload};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::{DiagnosticsListEvent, StatusBarEvent};
use katana_ui_core::text_surface::TextSurfaceEvent;
use serde::Serialize;
use sha2::{Digest, Sha256};

impl RootEventPayload {
    #[cfg(test)]
    pub(super) fn empty() -> Self {
        Self {
            text: Vec::new(),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
            status_bar: None,
            diagnostics_list: None,
            source_address_submissions: Vec::new(),
        }
    }

    fn from_output(output: &EguiTextCommandSurfaceOutput) -> Self {
        Self {
            text: output.text.events.clone(),
            toolbar: output.toolbar.as_ref().map(|value| value.events.clone()),
            floating: output.floating.as_ref().map(|value| value.events.clone()),
            search: output.search.as_ref().map(|value| value.events.clone()),
            context_menu: output
                .context_menu
                .as_ref()
                .map(|value| value.events.clone()),
            status_bar: output
                .status_bar
                .as_ref()
                .map(|value| value.events().to_vec()),
            diagnostics_list: output
                .diagnostics_list
                .as_ref()
                .map(|value| value.events().to_vec()),
            source_address_submissions: Vec::new(),
        }
    }

    pub(super) fn event_cardinality(&self) -> usize {
        self.text.len()
            + self.toolbar.as_ref().map_or(0, Vec::len)
            + self.floating.as_ref().map_or(0, Vec::len)
            + self.search.as_ref().map_or(0, Vec::len)
            + self.context_menu.as_ref().map_or(0, Vec::len)
            + self.status_bar.as_ref().map_or(0, Vec::len)
            + self.diagnostics_list.as_ref().map_or(0, Vec::len)
            + self.source_address_submissions.len()
    }
}

#[derive(Serialize)]
pub(super) struct RootEventEnvelope<'a> {
    pub(super) text: &'a [TextSurfaceEvent],
    pub(super) toolbar: Option<&'a [CommandChromeToolbarEvent]>,
    pub(super) floating: Option<&'a [FloatingCommandToolbarEvent]>,
    pub(super) search: Option<&'a [CommandChromeSearchEvent]>,
    pub(super) context_menu: Option<&'a [katana_ui_core::molecule::selection::ContextMenuEvent]>,
    pub(super) status_bar: Option<&'a [StatusBarEvent]>,
    pub(super) diagnostics_list: Option<&'a [DiagnosticsListEvent]>,
}

pub(super) struct RootEventDispatcher;

impl RootEventDispatcher {
    pub(super) fn build_event_batch(
        output: &mut EguiTextCommandSurfaceOutput,
        source_address_submission_port: Option<SourceAddressSubmissionPortHandle>,
    ) -> Result<EguiTextCommandSurfaceRootEventBatch, String> {
        let mut payload = RootEventPayload::from_output(output);
        if let Some(source_address) = output.source_address.as_mut() {
            payload.source_address_submissions = source_address.output.take_submissions();
        }
        let envelope = RootEventEnvelope {
            text: &payload.text,
            toolbar: payload.toolbar.as_deref(),
            floating: payload.floating.as_deref(),
            search: payload.search.as_deref(),
            context_menu: payload.context_menu.as_deref(),
            status_bar: payload.status_bar.as_deref(),
            diagnostics_list: payload.diagnostics_list.as_deref(),
        };
        let bytes = serialize_value(&envelope)?;
        let mut fingerprint_bytes = bytes;
        fingerprint_bytes.extend_from_slice(b"|kuc-source-address-count|");
        fingerprint_bytes.extend_from_slice(
            payload
                .source_address_submissions
                .len()
                .to_string()
                .as_bytes(),
        );
        Ok(
            EguiTextCommandSurfaceRootEventBatch::with_source_address_port(
                payload,
                hex::encode(Sha256::digest(fingerprint_bytes)),
                source_address_submission_port,
            ),
        )
    }
}

pub(super) fn serialize_value(value: &impl Serialize) -> Result<Vec<u8>, String> {
    serde_json::to_vec(value).map_err(|error| error.to_string())
}

pub(super) struct RootEventFingerprint;

impl RootEventFingerprint {
    pub(super) fn correlation_fingerprint(
        root_identity: &str,
        state_revision: u64,
        event_batch_fingerprint: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"kuc.root-event-correlation/v1\0");
        hasher.update(root_identity.as_bytes());
        hasher.update([0]);
        hasher.update(state_revision.to_le_bytes());
        hasher.update([0]);
        hasher.update(event_batch_fingerprint.as_bytes());
        hex::encode(hasher.finalize())
    }
}

impl RootEventFingerprint {
    pub(super) fn fingerprint_payload(
        payload: &RootEventPayload,
    ) -> Result<String, serde_json::Error> {
        let envelope = RootEventEnvelope {
            text: &payload.text,
            toolbar: payload.toolbar.as_deref(),
            floating: payload.floating.as_deref(),
            search: payload.search.as_deref(),
            context_menu: payload.context_menu.as_deref(),
            status_bar: payload.status_bar.as_deref(),
            diagnostics_list: payload.diagnostics_list.as_deref(),
        };
        serde_json::to_vec(&envelope).map(|bytes| hex::encode(Sha256::digest(bytes)))
    }
}
