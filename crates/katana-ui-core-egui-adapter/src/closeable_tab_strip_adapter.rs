use katana_ui_core::molecule::structured::{CloseableTabStrip, CloseableTabStripEvent};
use sha2::{Digest, Sha256};
use std::fmt;

const CLOSEABLE_TAB_STRIP_HASH_LENGTH: usize = 64;
const CLOSEABLE_TAB_STRIP_REVISION_BYTES: usize = 8;

#[cfg(test)]
#[path = "closeable_tab_strip_adapter_tests.rs"]
mod closeable_tab_strip_adapter_tests;
#[path = "closeable_tab_strip_data.rs"]
mod closeable_tab_strip_data;
#[path = "closeable_tab_strip_render.rs"]
mod closeable_tab_strip_render;

pub(crate) struct CloseableTabStripAdapter;

pub(crate) struct CloseableTabStripRender {
    closed_frame: CloseableTabStripClosedFrame,
    widget_rect: egui::Rect,
    tab_rects: Vec<(String, egui::Rect)>,
    #[cfg(test)]
    close_rects: Vec<(String, egui::Rect)>,
    group_rects: Vec<(String, egui::Rect)>,
    events: Vec<CloseableTabStripEvent>,
}

impl CloseableTabStripRender {
    pub(crate) const fn closed_frame(&self) -> &CloseableTabStripClosedFrame {
        &self.closed_frame
    }

    pub(crate) const fn widget_rect(&self) -> egui::Rect {
        self.widget_rect
    }

    pub(crate) fn tab_rects(&self) -> &[(String, egui::Rect)] {
        &self.tab_rects
    }

    #[cfg(test)]
    pub(crate) fn close_rects(&self) -> &[(String, egui::Rect)] {
        &self.close_rects
    }

    pub(crate) fn group_rects(&self) -> &[(String, egui::Rect)] {
        &self.group_rects
    }

    pub(crate) fn events(&self) -> &[CloseableTabStripEvent] {
        &self.events
    }
}

impl CloseableTabStripClosedFrame {
    pub(crate) fn has_closed_fact(&self) -> bool {
        self.stable_hash.len() == CLOSEABLE_TAB_STRIP_HASH_LENGTH
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct CloseableTabStripCapabilities {
    can_activate: bool,
    can_close: bool,
    has_active_tab: bool,
    has_dirty_tab: bool,
    has_pinned_tab: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct CloseableTabStripClosedFrame {
    stable_hash: String,
    revision: u64,
    capabilities: CloseableTabStripCapabilities,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CloseableTabStripAdapterError {
    RevisionConflict,
}

impl fmt::Display for CloseableTabStripAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::RevisionConflict => "closeable tab strip frame revision conflicts",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for CloseableTabStripAdapterError {}

impl CloseableTabStripAdapter {
    pub(crate) fn show(
        &self,
        ui: &mut egui::Ui,
        strip: &mut CloseableTabStrip,
    ) -> Result<CloseableTabStripRender, CloseableTabStripAdapterError> {
        let items = closeable_tab_strip_data::TabStripItems::from_strip(strip);
        let mut events = Vec::new();

        let layout = closeable_tab_strip_render::render_tab_strip(ui, &items, strip, &mut events);

        let frame = self.close(strip)?;
        Ok(CloseableTabStripRender {
            closed_frame: frame,
            widget_rect: layout.widget_rect,
            tab_rects: layout.tab_rects,
            #[cfg(test)]
            close_rects: layout.close_rects,
            group_rects: layout.group_rects,
            events,
        })
    }

    pub(crate) fn close(
        &self,
        strip: &CloseableTabStrip,
    ) -> Result<CloseableTabStripClosedFrame, CloseableTabStripAdapterError> {
        let serialized = serde_json::to_vec(strip)
            .map_err(|_| CloseableTabStripAdapterError::RevisionConflict)?;
        let digest = Sha256::digest(&serialized);
        let stable_hash = hex::encode(digest);
        let revision = u64::from_be_bytes(
            digest[..CLOSEABLE_TAB_STRIP_REVISION_BYTES]
                .try_into()
                .map_err(|_| CloseableTabStripAdapterError::RevisionConflict)?,
        );
        let tabs = &strip.options().tabs;
        let has_active_tab = strip
            .state()
            .active_tab_id
            .as_ref()
            .is_some_and(|active| tabs.iter().any(|tab| tab.id == *active));

        Ok(CloseableTabStripClosedFrame {
            stable_hash,
            revision,
            capabilities: CloseableTabStripCapabilities {
                can_activate: !tabs.is_empty(),
                can_close: tabs.iter().any(|tab| tab.closeable && !tab.pinned),
                has_active_tab,
                has_dirty_tab: tabs.iter().any(|tab| tab.dirty),
                has_pinned_tab: tabs.iter().any(|tab| tab.pinned),
            },
        })
    }
}

#[cfg(test)]
mod error_tests {
    use super::CloseableTabStripAdapterError;

    #[test]
    fn revision_conflict_has_a_stable_error_message() {
        assert_eq!(
            CloseableTabStripAdapterError::RevisionConflict.to_string(),
            "closeable tab strip frame revision conflicts"
        );
    }
}
