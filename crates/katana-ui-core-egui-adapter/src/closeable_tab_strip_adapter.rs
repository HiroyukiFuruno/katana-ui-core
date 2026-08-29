use katana_ui_core::molecule::structured::{CloseableTabStrip, CloseableTabStripEvent};
use sha2::{Digest, Sha256};
use std::fmt;

mod render;

const CLOSED_HASH_HEX_LENGTH: usize = 64;
const REVISION_DIGEST_BYTES: usize = 8;

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
        self.stable_hash.len() == CLOSED_HASH_HEX_LENGTH
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
        let items = render::TabStripItems::from_strip(strip);
        let mut tabs = Vec::new();
        #[cfg(test)]
        let mut close_rects = Vec::new();
        let mut groups = Vec::new();
        let mut events = Vec::new();

        let widget_rect = ui
            .scope(|ui| {
                ui.horizontal_wrapped(|ui| {
                    for tab in &items.pinned_tabs {
                        render::render_tab(
                            ui,
                            tab,
                            strip,
                            &mut tabs,
                            #[cfg(test)]
                            &mut close_rects,
                            &mut events,
                        );
                    }
                    for group in &items.root_groups {
                        render::render_group(
                            ui,
                            group,
                            &items,
                            strip,
                            &mut tabs,
                            #[cfg(test)]
                            &mut close_rects,
                            &mut groups,
                            &mut events,
                        );
                    }
                    for tab in &items.unknown_group_tabs {
                        render::render_tab(
                            ui,
                            tab,
                            strip,
                            &mut tabs,
                            #[cfg(test)]
                            &mut close_rects,
                            &mut events,
                        );
                    }
                    for tab in &items.ungrouped_tabs {
                        render::render_tab(
                            ui,
                            tab,
                            strip,
                            &mut tabs,
                            #[cfg(test)]
                            &mut close_rects,
                            &mut events,
                        );
                    }
                });
            })
            .response
            .rect;

        let frame = self.close(strip)?;
        Ok(CloseableTabStripRender {
            closed_frame: frame,
            widget_rect,
            tab_rects: tabs,
            #[cfg(test)]
            close_rects,
            group_rects: groups,
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
        let stable_hash = format!("{digest:x}");
        let revision = u64::from_be_bytes(
            digest[..REVISION_DIGEST_BYTES]
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
mod tests {
    use super::{CloseableTabStripAdapter, CloseableTabStripCapabilities};
    use katana_ui_core::molecule::structured::{
        CloseableTab, CloseableTabGroup, CloseableTabStrip, CloseableTabStripEvent,
    };

    const SCREEN_SIZE: egui::Vec2 = egui::vec2(600.0, 100.0);

    #[test]
    fn closed_frame_keeps_only_opaque_revision_and_capability_facts() {
        let strip = CloseableTabStrip::new("tabs")
            .tab(
                katana_ui_core::molecule::structured::CloseableTab::new("editor", "editor.md")
                    .dirty(true)
                    .pinned(true)
                    .closeable(false),
            )
            .tab(katana_ui_core::molecule::structured::CloseableTab::new(
                "notes", "notes.md",
            ))
            .active_tab_id("editor")
            .stable_state_id("editor-tabs");

        let frame = CloseableTabStripAdapter
            .close(&strip)
            .expect("generic closeable tab strip is serializable");

        assert_eq!(
            frame.revision,
            u64::from_be_bytes([0x78, 0x21, 0x2c, 0xb6, 0xd6, 0x1a, 0x5a, 0x1f])
        );
        assert_eq!(frame.stable_hash.len(), 64);
        assert_eq!(
            frame.capabilities,
            CloseableTabStripCapabilities {
                can_activate: true,
                can_close: true,
                has_active_tab: true,
                has_dirty_tab: true,
                has_pinned_tab: true,
            }
        );
    }

    #[test]
    fn conversion_is_deterministic_for_equivalent_generic_strips() {
        let build = || {
            CloseableTabStrip::new("tabs")
                .tab(katana_ui_core::molecule::structured::CloseableTab::new("a", "A").dirty(true))
                .tab(
                    katana_ui_core::molecule::structured::CloseableTab::new("b", "B")
                        .pinned(true)
                        .closeable(false),
                )
                .active_tab_id("a")
                .stable_state_id("stable-tabs")
        };

        let first = CloseableTabStripAdapter
            .close(&build())
            .expect("first conversion succeeds");
        let second = CloseableTabStripAdapter
            .close(&build())
            .expect("second conversion succeeds");

        assert_eq!(first, second);
    }

    #[test]
    fn pointer_click_selects_a_generic_tab_through_egui_response() {
        let context = egui::Context::default();
        let mut strip = CloseableTabStrip::new("tabs")
            .tab(katana_ui_core::molecule::structured::CloseableTab::new(
                "first", "one",
            ))
            .tab(katana_ui_core::molecule::structured::CloseableTab::new(
                "target",
                "selected-target-with-known-width",
            ))
            .active_tab_id("first");

        let adapter = CloseableTabStripAdapter;
        let first_frame = run_frame(&context, &adapter, &mut strip, Vec::new());
        let target = first_frame
            .tab_rects()
            .iter()
            .find(|(tab_id, _)| tab_id == "target")
            .expect("target tab response exists")
            .1
            .center();
        let _ = run_frame(
            &context,
            &adapter,
            &mut strip,
            vec![pointer_button(target, true)],
        );
        let release_frame = run_frame(
            &context,
            &adapter,
            &mut strip,
            vec![pointer_button(target, false)],
        );

        assert_eq!(
            strip.state().active_tab_id.as_ref().map(|id| id.as_str()),
            Some("target")
        );
        assert!(release_frame.events().iter().any(|event| {
            matches!(event, CloseableTabStripEvent::TabSelected { tab_id } if tab_id.as_str() == "target")
        }));
    }

    #[test]
    fn real_egui_render_exposes_group_and_tab_widget_responses() {
        let context = egui::Context::default();
        let adapter = CloseableTabStripAdapter;
        let mut strip = CloseableTabStrip::new("tabs")
            .group(CloseableTabGroup::new("documents", "Documents"))
            .tab(CloseableTab::new("readme", "README").group_id("documents"))
            .tab(CloseableTab::new("notes", "Notes"));

        let rendered = run_frame(&context, &adapter, &mut strip, Vec::new());

        assert!(rendered.closed_frame().revision > 0);
        assert!(rendered.widget_rect().height() > 0.0);
        assert_eq!(rendered.group_rects().len(), 1);
        assert_eq!(rendered.group_rects()[0].0, "documents");
        assert_eq!(rendered.tab_rects().len(), 2);
        assert!(
            rendered
                .group_rects()
                .iter()
                .all(|(_, rect)| rect.height() > 0.0)
        );
        assert!(
            rendered
                .tab_rects()
                .iter()
                .all(|(_, rect)| rect.width() > 0.0)
        );
    }

    fn run_frame(
        context: &egui::Context,
        adapter: &CloseableTabStripAdapter,
        strip: &mut CloseableTabStrip,
        events: Vec<egui::Event>,
    ) -> super::CloseableTabStripRender {
        let mut output = None;
        let _ = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN_SIZE)),
                events,
                ..egui::RawInput::default()
            },
            |ui| output = Some(adapter.show(ui, strip).expect("tab strip frame succeeds")),
        );
        output.expect("tab strip frame is produced")
    }

    fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }
}
