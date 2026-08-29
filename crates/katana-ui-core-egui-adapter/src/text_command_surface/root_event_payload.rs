use crate::text_command_surface::types::EguiTextCommandSurfaceOutput;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::text_surface::TextSurfaceEvent;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize)]
pub(crate) struct RootEventEnvelope<'a> {
    text: &'a [TextSurfaceEvent],
    toolbar: Option<&'a [CommandChromeToolbarEvent]>,
    floating: Option<&'a [FloatingCommandToolbarEvent]>,
    search: Option<&'a [CommandChromeSearchEvent]>,
    context_menu: Option<&'a [katana_ui_core::molecule::selection::ContextMenuEvent]>,
}

pub(crate) struct RootEventPayload {
    pub(crate) text: Vec<TextSurfaceEvent>,
    pub(crate) toolbar: Option<Vec<CommandChromeToolbarEvent>>,
    pub(crate) floating: Option<Vec<FloatingCommandToolbarEvent>>,
    pub(crate) search: Option<Vec<CommandChromeSearchEvent>>,
    pub(crate) context_menu: Option<Vec<katana_ui_core::molecule::selection::ContextMenuEvent>>,
}

impl RootEventPayload {
    pub(crate) fn from_output(output: &EguiTextCommandSurfaceOutput) -> Self {
        Self {
            text: output.text.events.clone(),
            toolbar: output.toolbar.as_ref().map(|value| value.events.clone()),
            floating: output.floating.as_ref().map(|value| value.events.clone()),
            search: output.search.as_ref().map(|value| value.events.clone()),
            context_menu: output
                .context_menu
                .as_ref()
                .map(|value| value.events.clone()),
        }
    }

    pub(crate) fn event_cardinality(&self) -> usize {
        self.text.len()
            + self.toolbar.as_ref().map_or(0, Vec::len)
            + self.floating.as_ref().map_or(0, Vec::len)
            + self.search.as_ref().map_or(0, Vec::len)
            + self.context_menu.as_ref().map_or(0, Vec::len)
    }

    pub(crate) fn fingerprint(&self) -> Result<String, serde_json::Error> {
        let envelope = RootEventEnvelope {
            text: &self.text,
            toolbar: self.toolbar.as_deref(),
            floating: self.floating.as_deref(),
            search: self.search.as_deref(),
            context_menu: self.context_menu.as_deref(),
        };
        serde_json::to_vec(&envelope).map(|bytes| hex::encode(Sha256::digest(bytes)))
    }
}
