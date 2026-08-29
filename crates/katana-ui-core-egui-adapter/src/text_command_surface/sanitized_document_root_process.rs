#[path = "sanitized_document_root_surface.rs"]
mod sanitized_document_root_surface;

use super::super::EguiTextCommandSurfaceCommandFamilyProjection;
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
    pub(super) fn new(input: SanitizedDocumentRootInput) -> Self {
        let revision = input.revision;
        let mut input = input;
        let (surface, presentation) = sanitized_document_root_surface::from_input(&input);
        let search_projection = input.search_projection.take();
        let identity = input.identity.stable_fingerprint();
        let mut root = EguiTextCommandSurfaceRoot::with_identity(identity, surface);
        let _ = root.synchronize_presentation(presentation);
        root.apply_command_family_projection(
            &EguiTextCommandSurfaceCommandFamilyProjection::legacy_compatibility(),
        );
        let style = resolve_style(input.style);
        let tab_adapter =
            SanitizedTabProjectionAdapter::from_projection(input.tab_projection.as_ref());
        Self {
            input,
            root,
            style,
            tab_adapter,
            tab_frame: None,
            tab_rendered: false,
            generation: Rc::new(Cell::new(revision)),
            search_projection,
        }
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
        self.root.apply_command_family_projection(
            &EguiTextCommandSurfaceCommandFamilyProjection::legacy_compatibility(),
        );
        self.style = resolve_style(input.style);
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
        ui.vertical(|ui| {
            let frame = self.tab_adapter.show(ui).map_err(tab_adapter_error)?;
            self.tab_rendered = frame.has_render_facts();
            tab_frame = Some(frame);
            self.root.show(ui, &self.style)
        })
        .inner
        .inspect(|_| {
            self.tab_frame = tab_frame;
        })
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
                super::sanitized_search_event::SanitizedSearchEventRouter::route_events(
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

fn tab_adapter_error(
    error: crate::closeable_tab_strip_adapter::CloseableTabStripAdapterError,
) -> EguiTextCommandSurfaceRootError {
    EguiTextCommandSurfaceRootError::Serialization(error.to_string())
}

#[cfg(test)]
pub(crate) fn search_projection_for_ime(
    query_target: impl Into<Vec<u8>>,
    replacement_target: impl Into<Vec<u8>>,
) -> super::SanitizedSearchProjection {
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

    let replacement_callback_record = std::rc::Rc::clone(&replacement_record);
    let replacement_callback = move |_, value| {
        replacement_callback_record.borrow_mut().push(value);
        Ok::<(), ()>(())
    };
    replacement_callback(
        super::SanitizedSearchTextOperation::Replacement,
        String::new(),
    )
    .expect("replacement callback fixture");
    replacement_record.borrow_mut().clear();

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
                .with_text_capability(replacement_callback),
        )
        .build()
        .expect("search projection is valid for tests")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SanitizedDocumentRootProcessError {
    IdentityChanged,
    StaleRevision { current: u64, received: u64 },
    RevisionConflict { revision: u64 },
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
    use super::{
        SanitizedDocumentRootProcess, SanitizedDocumentRootProcessError, tab_adapter_error,
    };
    use crate::text_command_surface::EguiTextCommandSurfaceRootError;
    use katana_ui_core::molecule::structured::CloseableTabStripEvent;
    use katana_ui_core::render_model::UiIconProps;

    #[test]
    fn tab_adapter_failure_maps_to_the_root_serialization_boundary() {
        let error = tab_adapter_error(
            crate::closeable_tab_strip_adapter::CloseableTabStripAdapterError::RevisionConflict,
        );
        assert!(matches!(
            error,
            EguiTextCommandSurfaceRootError::Serialization(_)
        ));
    }

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

    fn search_projection(label: &str, target: u8) -> SanitizedSearchProjection {
        SanitizedSearchProjectionBuilder::new()
            .localized_presentation(localized_search(label))
            .next_enabled(true)
            .next_target(SanitizedSearchTarget::from_opaque_bytes([target]))
            .build()
            .expect("検索投影は検証済み")
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
    ) -> SanitizedDocumentRootRecord {
        let mut output = None;
        crate::run_ui_discard(
            context,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 480.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    output = Some(process.show(ui).expect("root show succeeds"));
                });
            },
        );
        let output = output.expect("frame exists");
        SanitizedDocumentRootRecord::from_output(process.input.revision, &output)
    }

    #[test]
    fn identity_change_is_rejected_before_revision_policy() {
        let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"));

        assert_eq!(
            process.synchronize(input(4, b"two", "b")),
            Err(SanitizedDocumentRootProcessError::IdentityChanged)
        );
        assert_eq!(process.input.snapshot, "a");
    }

    #[test]
    fn stale_revision_is_rejected() {
        let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"));

        assert_eq!(
            process.synchronize(input(2, b"one", "b")),
            Err(SanitizedDocumentRootProcessError::StaleRevision {
                current: 3,
                received: 2,
            })
        );
        assert_eq!(process.input.snapshot, "a");
    }

    #[test]
    fn same_revision_requires_an_identical_snapshot() {
        let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"));

        assert_eq!(
            process.synchronize(input(3, b"one", "b")),
            Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
        );
        assert_eq!(process.synchronize(input(3, b"one", "a")), Ok(false));
    }

    #[test]
    fn same_revision_requires_an_identical_command_projection() {
        let mut process = SanitizedDocumentRootProcess::new(input_with_projection(
            3,
            b"one",
            "a",
            projection("first"),
        ));

        assert_eq!(
            process.synchronize(input_with_projection(3, b"one", "a", projection("second"))),
            Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
        );
        assert_eq!(
            process.synchronize(input_with_projection(3, b"one", "a", projection("first"))),
            Ok(false)
        );
    }

    #[test]
    fn same_revision_requires_an_identical_search_projection() {
        let mut process = SanitizedDocumentRootProcess::new(input_with_search_projection(
            3,
            b"one",
            "a",
            search_projection("次へ", 1),
        ));

        assert_eq!(
            process.synchronize(input_with_search_projection(
                3,
                b"one",
                "a",
                search_projection("次の一致", 2),
            )),
            Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
        );
        assert_eq!(
            process.synchronize(input_with_search_projection(
                3,
                b"one",
                "a",
                search_projection("次へ", 1),
            )),
            Ok(false)
        );
    }

    #[test]
    fn same_revision_requires_an_identical_context_projection() {
        let mut process = SanitizedDocumentRootProcess::new(input_with_context_projection(
            3,
            b"one",
            "a",
            context_projection("表示", 1),
        ));

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
    }

    #[test]
    fn same_revision_requires_an_identical_tab_projection() {
        let mut process = SanitizedDocumentRootProcess::new(input_with_tab_projection(
            3,
            b"one",
            "a",
            tab_projection("次の文書"),
        ));

        assert_eq!(
            process.synchronize(input_with_tab_projection(
                3,
                b"one",
                "a",
                tab_projection("別の文書"),
            )),
            Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
        );
    }

    #[test]
    fn newer_snapshot_is_synchronized_into_the_retained_root() {
        let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"));

        assert_eq!(process.synchronize(input(4, b"one", "b")), Ok(true));
        assert_eq!(process.input.revision, 4);
        assert_eq!(process.input.snapshot, "b");
    }

    #[test]
    fn real_egui_root_record_changes_when_command_projection_is_updated() {
        let context = egui::Context::default();
        let mut process = SanitizedDocumentRootProcess::new(input_with_projection(
            1,
            b"doc",
            "日本語 ⭐️",
            projection("first"),
        ));
        let first = render_record(&mut process, &context);

        process
            .synchronize(input_with_projection(
                2,
                b"doc",
                "日本語 ⭐️",
                projection("second"),
            ))
            .expect("projection revision update succeeds");

        let second = render_record(&mut process, &context);

        assert_ne!(first.record_hash(), second.record_hash());
    }

    #[test]
    fn real_egui_root_record_changes_when_tab_projection_is_added() {
        let context = egui::Context::default();
        let mut process = SanitizedDocumentRootProcess::new(input(1, b"doc", "本文"));
        let first = render_record(&mut process, &context);

        process
            .synchronize(input_with_tab_projection(
                2,
                b"doc",
                "本文",
                tab_projection("次の文書"),
            ))
            .expect("tab projection revision update succeeds");
        let second = render_record(&mut process, &context);

        assert!(process.tab_rendered);
        assert_ne!(first.record_hash(), second.record_hash());
    }

    #[test]
    fn physical_pointer_click_selects_tab_at_sanitized_root_boundary() {
        let context = egui::Context::default();
        let mut process = SanitizedDocumentRootProcess::new(input_with_tab_projection(
            1,
            b"doc",
            "本文 ⭐️",
            tab_projection("次の文書"),
        ));
        let _ = render_record(&mut process, &context);
        let target = process
            .tab_frame
            .as_ref()
            .expect("tab frame is retained")
            .boundary_facts()
            .tab_rects
            .iter()
            .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
            .map(|(_, rect)| rect.center())
            .expect("second tab widget rect is nonzero");

        let _ = render_record_with_events(
            &mut process,
            &context,
            vec![egui::Event::PointerButton {
                pos: target,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            }],
        );
        let _ = render_record_with_events(
            &mut process,
            &context,
            vec![egui::Event::PointerButton {
                pos: target,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            }],
        );

        assert_eq!(
            process.tab_adapter.active_tab_id(),
            Some("sanitized-tab-0-1")
        );
        let frame = process
            .tab_frame
            .as_ref()
            .expect("release frame is retained");
        let facts = frame.boundary_facts();
        assert!(facts.widget_rect.width() > 0.0);
        assert!(facts.closed_frame.has_closed_fact());
        assert!(facts.events.iter().any(|event| matches!(
            event,
            CloseableTabStripEvent::TabSelected { tab_id }
                if tab_id.as_str() == "sanitized-tab-0-1"
        )));
    }

    #[test]
    fn real_egui_root_record_changes_when_search_projection_is_updated() {
        let context = egui::Context::default();
        let mut process = SanitizedDocumentRootProcess::new(
            input_with_search_projection(1, b"doc", "日本語 ⭐️", search_projection("次へ", 1))
                .with_command_projection(projection("stable")),
        );
        let first = render_record(&mut process, &context);

        process
            .synchronize(
                input_with_search_projection(
                    2,
                    b"doc",
                    "日本語 ⭐️",
                    search_projection("次の一致", 2),
                )
                .with_command_projection(projection("stable")),
            )
            .expect("search projection revision update succeeds");

        let second = render_record(&mut process, &context);

        assert_ne!(first.record_hash(), second.record_hash());
    }

    #[test]
    fn real_egui_root_record_changes_when_context_projection_is_updated() {
        let context = egui::Context::default();
        let mut process = SanitizedDocumentRootProcess::new(input_with_context_projection(
            1,
            b"doc",
            "日本語 ⭐️",
            context_projection("表示", 1),
        ));

        let _ = render_record_with_events(
            &mut process,
            &context,
            vec![egui::Event::PointerButton {
                pos: egui::pos2(48.0, 8.0),
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            }],
        );
        let first = render_record_with_events(
            &mut process,
            &context,
            vec![egui::Event::PointerButton {
                pos: egui::pos2(48.0, 8.0),
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            }],
        );

        process
            .synchronize(input_with_context_projection(
                2,
                b"doc",
                "日本語 ⭐️",
                context_projection("別の表示", 2),
            ))
            .expect("context projection revision update succeeds");

        let second = render_record_with_events(&mut process, &context, Vec::new());

        assert_ne!(first.record_hash(), second.record_hash());
    }

    fn render_record_with_events(
        process: &mut SanitizedDocumentRootProcess,
        context: &egui::Context,
        events: Vec<egui::Event>,
    ) -> SanitizedDocumentRootRecord {
        let mut output = None;
        crate::run_ui_discard(
            context,
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
                    output = Some(process.show(ui).expect("root show succeeds"));
                });
            },
        );
        let output = output.expect("frame exists");
        SanitizedDocumentRootRecord::from_output(process.input.revision, &output)
    }
}
