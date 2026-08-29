#[path = "sanitized_document_root_surface.rs"]
mod sanitized_document_root_surface;

use super::super::root::{
    EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootError, EguiTextCommandSurfaceRootOutput,
};
use super::super::types::TextCommandSurfaceStyle;
use super::sanitized_document_root_input::SanitizedDocumentRootInput;
use super::sanitized_document_root_style::resolve_style;
use super::sanitized_search_projection::SanitizedSearchProjection;
use super::sanitized_tab_projection::adapter::{
    SanitizedTabProjectionAdapter, SanitizedTabProjectionClosedEvent, SanitizedTabProjectionFrame,
};
use std::cell::Cell;
use std::rc::Rc;

/// Private process state for one retained generic document root.
pub(super) struct SanitizedDocumentRootProcess {
    pub(super) input: SanitizedDocumentRootInput,
    root: EguiTextCommandSurfaceRoot,
    style: TextCommandSurfaceStyle,
    tab_adapter: SanitizedTabProjectionAdapter,
    tab_frame: Option<SanitizedTabProjectionFrame>,
    tab_rendered: bool,
    pub(super) generation: Rc<Cell<u64>>,
    search_projection: Option<SanitizedSearchProjection>,
}

impl SanitizedDocumentRootProcess {
    pub(super) fn new(input: SanitizedDocumentRootInput) -> Result<Self, String> {
        let revision = input.revision;
        let mut input = input;
        let (surface, presentation) = sanitized_document_root_surface::from_input(&input);
        let search_projection = input.search_projection.take();
        let identity = input.identity.stable_fingerprint();
        let mut root = EguiTextCommandSurfaceRoot::with_identity(identity, surface)
            .map_err(|error| error.to_string())?;
        let _ = root.synchronize_presentation(presentation);
        let style = resolve_style(input.style).map_err(|error| error.to_string())?;
        let tab_adapter =
            SanitizedTabProjectionAdapter::from_projection(input.tab_projection.as_ref());
        Ok(Self {
            input,
            root,
            style,
            tab_adapter,
            tab_frame: None,
            tab_rendered: false,
            generation: Rc::new(Cell::new(revision)),
            search_projection,
        })
    }

    /// Synchronizes one complete host snapshot using the retained identity/revision policy.
    pub(super) fn synchronize(
        &mut self,
        input: SanitizedDocumentRootInput,
    ) -> Result<bool, SanitizedDocumentRootProcessError> {
        if !self.input.identity.same_identity(&input.identity) {
            return Err(SanitizedDocumentRootProcessError::IdentityChanged);
        }
        if input.revision < self.input.revision {
            return Err(SanitizedDocumentRootProcessError::StaleRevision {
                current: self.input.revision,
                received: input.revision,
            });
        }
        if input.revision == self.input.revision {
            if input.snapshot != self.input.snapshot
                || input.readonly != self.input.readonly
                || input.style != self.input.style
                || !input.same_command_projection_as(&self.input)
                || !input.same_search_projection_as(&self.input)
                || !input.same_context_projection_as(&self.input)
                || !input.same_tab_projection_as(&self.input)
            {
                return Err(SanitizedDocumentRootProcessError::RevisionConflict {
                    revision: input.revision,
                });
            }
            return Ok(false);
        }

        let mut input = input;
        let presentation = sanitized_document_root_surface::presentation_from_input(&input);
        let search_projection = input.search_projection.take();
        let changed = self.root.synchronize_presentation(presentation);
        self.style = resolve_style(input.style)
            .map_err(|error| SanitizedDocumentRootProcessError::Style(error.to_string()))?;
        self.tab_adapter
            .replace_projection(input.tab_projection.as_ref());
        self.tab_frame = None;
        self.tab_rendered = false;
        self.generation.set(input.revision);
        self.input = input;
        self.search_projection = search_projection;
        Ok(changed)
    }

    /// Shows the retained KUC root exactly once for the caller's frame.
    pub(super) fn show(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
        let mut tab_frame = None;
        let output = ui
            .vertical(|ui| {
                let frame = self.tab_adapter.show(ui).map_err(|error| {
                    EguiTextCommandSurfaceRootError::Serialization(error.to_string())
                })?;
                self.tab_rendered = frame.has_render_facts();
                tab_frame = Some(frame);
                self.root.show(ui, &self.style)
            })
            .inner?;
        self.tab_frame = tab_frame;
        Ok(output)
    }

    pub(crate) fn take_tab_closed_events(&mut self) -> Vec<SanitizedTabProjectionClosedEvent> {
        self.tab_frame
            .take()
            .map(SanitizedTabProjectionFrame::into_closed_events)
            .unwrap_or_default()
    }

    pub(super) fn route_search_events(
        &self,
        events: &[katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent],
        revision: u64,
        root_identity_fingerprint: &str,
    ) -> Result<
        Vec<super::sanitized_search_event::SanitizedSearchEventTransport>,
        super::sanitized_search_projection::SanitizedSearchCapabilityRejection,
    > {
        self.search_projection.as_ref().map_or_else(
            || Ok(Vec::new()),
            |projection| {
                super::sanitized_search_event::route_search_events(
                    Some(projection),
                    events,
                    revision,
                    root_identity_fingerprint,
                )
            },
        )
    }

    #[cfg(test)]
    pub(super) fn search_options(
        &self,
    ) -> Option<katana_ui_core::molecule::structured::SearchOptions> {
        self.search_projection.as_ref().map(|projection| {
            super::sanitized_search_projection_adapter::SanitizedSearchPresentation::from(
                projection,
            )
            .value
            .options
        })
    }

    #[cfg(test)]
    pub(super) fn tab_rects(&self) -> &[(String, egui::Rect)] {
        self.tab_frame
            .as_ref()
            .map_or(&[], |frame| frame.boundary_facts().tab_rects)
    }

    #[cfg(test)]
    pub(super) fn tab_close_rects(&self) -> &[(String, egui::Rect)] {
        self.tab_frame
            .as_ref()
            .map_or(&[], |frame| frame.boundary_facts().close_rects)
    }
}

#[cfg(test)]
pub(crate) fn search_projection_for_ime(
    query_target: impl Into<Vec<u8>>,
    replacement_target: impl Into<Vec<u8>>,
) -> Result<super::SanitizedSearchProjection, String> {
    let query_record = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
    let replacement_record = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
    fn text(value: &str) -> super::SanitizedSearchTextPresentation {
        super::SanitizedSearchTextPresentation::new(
            value,
            format!("{value} ⭐️"),
            format!("{value} ⭐️"),
        )
    }

    fn localized() -> super::SanitizedSearchLocalizedPresentation {
        super::SanitizedSearchLocalizedPresentation::new(
            super::SanitizedSearchControlPresentation::new(
                text("検索"),
                text("検索語"),
                text("置換語"),
                text("大文字小文字"),
                text("単語一致"),
                text("正規表現"),
            ),
            super::SanitizedSearchOperationPresentation::new(
                text("前へ"),
                text("次へ"),
                text("置換"),
                text("すべて置換"),
                text("閉じる"),
            ),
            super::SanitizedSearchResultSummaryPresentation::new(
                "検索待機 ⭐️",
                "一致なし",
                "1件",
                "{active} / {count}",
                "{count}件",
            ),
            super::SanitizedSearchUnavailablePresentation::new(
                "正規表現は利用不可",
                "置換は利用不可",
                "移動は利用不可",
                "閉じる操作は利用不可",
            ),
        )
    }

    super::SanitizedSearchProjectionBuilder::new()
        .localized_presentation(localized())
        .query_target(
            super::SanitizedSearchTarget::from_opaque_bytes(query_target).with_text_capability({
                let record = query_record;
                move |_, value| {
                    record.borrow_mut().push(value);
                    Ok::<(), ()>(())
                }
            }),
        )
        .replacement_target(
            super::SanitizedSearchTarget::from_opaque_bytes(replacement_target)
                .with_text_capability({
                    let record = replacement_record;
                    move |_, value| {
                        record.borrow_mut().push(value);
                        Ok::<(), ()>(())
                    }
                }),
        )
        .build()
        .map_err(|error| format!("{error:?}"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SanitizedDocumentRootProcessError {
    IdentityChanged,
    StaleRevision { current: u64, received: u64 },
    RevisionConflict { revision: u64 },
    Style(String),
}

#[cfg(test)]
mod tests {
    use super::super::sanitized_document_root_input::{
        SanitizedDocumentRootIdentity, SanitizedDocumentRootInput,
    };
    use super::super::sanitized_document_root_record::SanitizedDocumentRootRecord;
    use super::super::sanitized_document_root_style::SanitizedDocumentRootStyleKey;
    use super::super::{
        SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
        SanitizedCommandTarget, SanitizedSearchControlPresentation,
        SanitizedSearchLocalizedPresentation, SanitizedSearchOperationPresentation,
        SanitizedSearchProjection, SanitizedSearchProjectionBuilder,
        SanitizedSearchResultSummaryPresentation, SanitizedSearchTarget,
        SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation, SanitizedTab,
        SanitizedTabCapabilities, SanitizedTabGroup, SanitizedTabProjection, SanitizedTabTarget,
    };
    use super::{SanitizedDocumentRootProcess, SanitizedDocumentRootProcessError};
    use katana_ui_core::molecule::structured::CloseableTabStripEvent;
    use katana_ui_core::render_model::UiIconProps;

    fn input(revision: u64, identity: &[u8], snapshot: &str) -> SanitizedDocumentRootInput {
        SanitizedDocumentRootInput::new(
            revision,
            SanitizedDocumentRootIdentity::from_opaque_bytes(identity.to_vec()),
            snapshot,
            SanitizedDocumentRootStyleKey::Default,
        )
    }

    fn input_with_projection(
        revision: u64,
        identity: &[u8],
        snapshot: &str,
        command_projection: SanitizedCommandProjection,
    ) -> SanitizedDocumentRootInput {
        input(revision, identity, snapshot).with_command_projection(command_projection)
    }

    fn input_with_search_projection(
        revision: u64,
        identity: &[u8],
        snapshot: &str,
        search_projection: SanitizedSearchProjection,
    ) -> SanitizedDocumentRootInput {
        input(revision, identity, snapshot).with_search_projection(search_projection)
    }

    fn context_projection(label: &str, target: u8) -> super::super::SanitizedContextMenuProjection {
        super::super::SanitizedContextMenuProjectionBuilder::new()
            .item(super::super::SanitizedContextMenuItem::new(
                super::super::SanitizedContextMenuTarget::from_opaque_bytes([target]),
                1,
                label,
            ))
            .build()
    }

    fn input_with_context_projection(
        revision: u64,
        identity: &[u8],
        snapshot: &str,
        context_projection: super::super::SanitizedContextMenuProjection,
    ) -> SanitizedDocumentRootInput {
        input(revision, identity, snapshot).with_context_projection(context_projection)
    }

    fn input_with_tab_projection(
        revision: u64,
        identity: &[u8],
        snapshot: &str,
        tab_projection: SanitizedTabProjection,
    ) -> SanitizedDocumentRootInput {
        input(revision, identity, snapshot).with_tab_projection(tab_projection)
    }

    fn projection(label: &str) -> SanitizedCommandProjection {
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "g").item(
            SanitizedCommandItem::new(
                SanitizedCommandTarget::from_opaque_bytes(label.as_bytes()),
                1,
                label,
            )
            .with_icon(UiIconProps::new("<svg/>")),
        )])
    }

    fn search_text(value: &str) -> SanitizedSearchTextPresentation {
        SanitizedSearchTextPresentation::new(value, format!("{value} ⭐️"), format!("{value} ⭐️"))
    }

    fn localized_search(next: &str) -> SanitizedSearchLocalizedPresentation {
        SanitizedSearchLocalizedPresentation::new(
            SanitizedSearchControlPresentation::new(
                search_text("検索"),
                search_text("検索語"),
                search_text("置換"),
                search_text("大文字小文字"),
                search_text("単語"),
                search_text("正規表現"),
            ),
            SanitizedSearchOperationPresentation::new(
                search_text("前へ"),
                search_text(next),
                search_text("置換"),
                search_text("すべて置換"),
                search_text("閉じる"),
            ),
            SanitizedSearchResultSummaryPresentation::new(
                "検索待機 ⭐️",
                "一致なし ⭐️",
                "一件 ⭐️",
                "位置 ⭐️",
                "件数 ⭐️",
            ),
            SanitizedSearchUnavailablePresentation::new(
                "正規表現は利用不可 ⭐️",
                "置換は利用不可 ⭐️",
                "移動は利用不可 ⭐️",
                "閉じる操作は利用不可 ⭐️",
            ),
        )
    }

    fn search_projection(label: &str, target: u8) -> Result<SanitizedSearchProjection, String> {
        SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized_search(label))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([target]))
            .build()
            .map_err(|error| format!("{error:?}"))
    }

    fn tab_projection(second_label: &str) -> SanitizedTabProjection {
        SanitizedTabProjection::new([SanitizedTabGroup::new(
            crate::text_command_surface::sanitized_document_root::sanitized_tab_projection::SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "ドキュメント",
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 0, "最初")
                .with_capabilities(
                    SanitizedTabCapabilities::new()
                        .active_state(true)
                        .close_state(true),
                ),
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([2]), 1, second_label)
                .with_capabilities(SanitizedTabCapabilities::new().close_state(true)),
        )])
    }

    fn render_record(
        process: &mut SanitizedDocumentRootProcess,
        context: &egui::Context,
    ) -> Result<SanitizedDocumentRootRecord, String> {
        let mut output = None;
        let mut platform_output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 480.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    output = Some(process.show(ui).map_err(|error| error.to_string()));
                });
            },
        );
        platform_output.textures_delta.clear();
        let output = output.ok_or_else(|| "frame output was not produced".to_owned())??;
        Ok(SanitizedDocumentRootRecord::from_output(
            process.input.revision,
            &output,
        ))
    }

    #[test]
    fn identity_change_is_rejected_before_revision_policy() -> Result<(), String> {
        let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"))?;

        assert_eq!(
            process.synchronize(input(4, b"two", "b")),
            Err(SanitizedDocumentRootProcessError::IdentityChanged)
        );
        assert_eq!(process.input.snapshot, "a");
        Ok(())
    }

    #[test]
    fn stale_revision_is_rejected() -> Result<(), String> {
        let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"))?;

        assert_eq!(
            process.synchronize(input(2, b"one", "b")),
            Err(SanitizedDocumentRootProcessError::StaleRevision {
                current: 3,
                received: 2,
            })
        );
        assert_eq!(process.input.snapshot, "a");
        Ok(())
    }

    #[test]
    fn same_revision_requires_an_identical_snapshot() -> Result<(), String> {
        let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"))?;

        assert_eq!(
            process.synchronize(input(3, b"one", "b")),
            Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
        );
        assert_eq!(process.synchronize(input(3, b"one", "a")), Ok(false));
        Ok(())
    }

    #[test]
    fn same_revision_requires_an_identical_command_projection() -> Result<(), String> {
        let mut process = SanitizedDocumentRootProcess::new(input_with_projection(
            3,
            b"one",
            "a",
            projection("first"),
        ))?;

        assert_eq!(
            process.synchronize(input_with_projection(3, b"one", "a", projection("second"))),
            Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
        );
        assert_eq!(
            process.synchronize(input_with_projection(3, b"one", "a", projection("first"))),
            Ok(false)
        );
        Ok(())
    }

    #[test]
    fn same_revision_requires_an_identical_search_projection() -> Result<(), String> {
        let mut process = SanitizedDocumentRootProcess::new(input_with_search_projection(
            3,
            b"one",
            "a",
            search_projection("次へ", 1)?,
        ))?;

        assert_eq!(
            process.synchronize(input_with_search_projection(
                3,
                b"one",
                "a",
                search_projection("次の一致", 2)?,
            )),
            Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
        );
        assert_eq!(
            process.synchronize(input_with_search_projection(
                3,
                b"one",
                "a",
                search_projection("次へ", 1)?,
            )),
            Ok(false)
        );
        Ok(())
    }

    #[test]
    fn same_revision_requires_an_identical_context_projection() -> Result<(), String> {
        let mut process = SanitizedDocumentRootProcess::new(input_with_context_projection(
            3,
            b"one",
            "a",
            context_projection("表示", 1),
        ))?;

        assert_eq!(
            process.synchronize(input_with_context_projection(
                3,
                b"one",
                "a",
                context_projection("別の表示", 2),
            )),
            Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
        );
        assert_eq!(
            process.synchronize(input_with_context_projection(
                3,
                b"one",
                "a",
                context_projection("表示", 1),
            )),
            Ok(false)
        );
        Ok(())
    }

    #[test]
    fn same_revision_requires_an_identical_tab_projection() -> Result<(), String> {
        let mut process = SanitizedDocumentRootProcess::new(input_with_tab_projection(
            3,
            b"one",
            "a",
            tab_projection("次の文書"),
        ))?;

        assert_eq!(
            process.synchronize(input_with_tab_projection(
                3,
                b"one",
                "a",
                tab_projection("別の文書"),
            )),
            Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
        );
        Ok(())
    }

    #[test]
    fn newer_snapshot_is_synchronized_into_the_retained_root() -> Result<(), String> {
        let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"))?;

        assert_eq!(process.synchronize(input(4, b"one", "b")), Ok(true));
        assert_eq!(process.input.revision, 4);
        assert_eq!(process.input.snapshot, "b");
        Ok(())
    }

    #[test]
    fn real_egui_root_record_changes_when_command_projection_is_updated() -> Result<(), String> {
        let context = egui::Context::default();
        let mut process = SanitizedDocumentRootProcess::new(input_with_projection(
            1,
            b"doc",
            "日本語 ⭐️",
            projection("first"),
        ))?;
        let first = render_record(&mut process, &context)?;

        process
            .synchronize(input_with_projection(
                2,
                b"doc",
                "日本語 ⭐️",
                projection("second"),
            ))
            .map_err(|error| format!("{error:?}"))?;

        let second = render_record(&mut process, &context)?;

        assert_ne!(first.record_hash(), second.record_hash());
        Ok(())
    }

    #[test]
    fn real_egui_root_record_changes_when_tab_projection_is_added() -> Result<(), String> {
        let context = egui::Context::default();
        let mut process = SanitizedDocumentRootProcess::new(input(1, b"doc", "本文"))?;
        let first = render_record(&mut process, &context)?;

        process
            .synchronize(input_with_tab_projection(
                2,
                b"doc",
                "本文",
                tab_projection("次の文書"),
            ))
            .map_err(|error| format!("{error:?}"))?;
        let second = render_record(&mut process, &context)?;

        assert!(process.tab_rendered);
        assert_ne!(first.record_hash(), second.record_hash());
        Ok(())
    }

    #[test]
    fn physical_pointer_click_selects_tab_at_sanitized_root_boundary() -> Result<(), String> {
        let context = egui::Context::default();
        let mut process = SanitizedDocumentRootProcess::new(input_with_tab_projection(
            1,
            b"doc",
            "本文 ⭐️",
            tab_projection("次の文書"),
        ))?;
        let _ = render_record(&mut process, &context)?;
        let target = process
            .tab_frame
            .as_ref()
            .ok_or_else(|| "tab frame is not retained".to_owned())?
            .boundary_facts()
            .tab_rects
            .iter()
            .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
            .map(|(_, rect)| rect.center())
            .ok_or_else(|| "second tab widget rect is zero or absent".to_owned())?;

        let _ = render_record_with_events(
            &mut process,
            &context,
            vec![egui::Event::PointerButton {
                pos: target,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            }],
        )?;
        let _ = render_record_with_events(
            &mut process,
            &context,
            vec![egui::Event::PointerButton {
                pos: target,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            }],
        )?;

        assert_eq!(
            process.tab_adapter.active_tab_id(),
            Some("sanitized-tab-0-1")
        );
        let frame = process
            .tab_frame
            .as_ref()
            .ok_or_else(|| "release frame is not retained".to_owned())?;
        let facts = frame.boundary_facts();
        assert!(facts.widget_rect.width() > 0.0);
        assert!(facts.closed_frame.has_closed_fact());
        assert!(facts.events.iter().any(|event| matches!(
            event,
            CloseableTabStripEvent::TabSelected { tab_id }
                if tab_id.as_str() == "sanitized-tab-0-1"
        )));
        Ok(())
    }

    #[test]
    fn real_egui_root_record_changes_when_search_projection_is_updated() -> Result<(), String> {
        let context = egui::Context::default();
        let mut process = SanitizedDocumentRootProcess::new(
            input_with_search_projection(1, b"doc", "日本語 ⭐️", search_projection("次へ", 1)?)
                .with_command_projection(projection("stable")),
        )?;
        let first = render_record(&mut process, &context)?;

        process
            .synchronize(
                input_with_search_projection(
                    2,
                    b"doc",
                    "日本語 ⭐️",
                    search_projection("次の一致", 2)?,
                )
                .with_command_projection(projection("stable")),
            )
            .map_err(|error| format!("{error:?}"))?;

        let second = render_record(&mut process, &context)?;

        assert_ne!(first.record_hash(), second.record_hash());
        Ok(())
    }

    #[test]
    fn real_egui_root_record_changes_when_context_projection_is_updated() -> Result<(), String> {
        let context = egui::Context::default();
        let mut process = SanitizedDocumentRootProcess::new(input_with_context_projection(
            1,
            b"doc",
            "日本語 ⭐️",
            context_projection("表示", 1),
        ))?;

        let _ = render_record_with_events(
            &mut process,
            &context,
            vec![egui::Event::PointerButton {
                pos: egui::pos2(48.0, 8.0),
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            }],
        )?;
        let first = render_record_with_events(
            &mut process,
            &context,
            vec![egui::Event::PointerButton {
                pos: egui::pos2(48.0, 8.0),
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            }],
        )?;

        process
            .synchronize(input_with_context_projection(
                2,
                b"doc",
                "日本語 ⭐️",
                context_projection("別の表示", 2),
            ))
            .map_err(|error| format!("{error:?}"))?;

        let second = render_record_with_events(&mut process, &context, Vec::new())?;

        assert_ne!(first.record_hash(), second.record_hash());
        Ok(())
    }

    fn render_record_with_events(
        process: &mut SanitizedDocumentRootProcess,
        context: &egui::Context,
        events: Vec<egui::Event>,
    ) -> Result<SanitizedDocumentRootRecord, String> {
        let mut output = None;
        let mut platform_output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 480.0),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    output = Some(process.show(ui).map_err(|error| error.to_string()));
                });
            },
        );
        platform_output.textures_delta.clear();
        let output = output.ok_or_else(|| "frame output was not produced".to_owned())??;
        Ok(SanitizedDocumentRootRecord::from_output(
            process.input.revision,
            &output,
        ))
    }
}
