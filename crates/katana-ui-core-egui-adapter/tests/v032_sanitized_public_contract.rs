use katana_ui_core::render_model::UiIconProps;
use katana_ui_core_egui_adapter::text_command_surface::{
    KucOpaqueHostEffectBatch, KucOpaqueHostEffectError, KucRootEventBatchDispatcher,
    SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
    SanitizedCommandProjection, SanitizedCommandTarget, SanitizedContextMenuItem,
    SanitizedContextMenuProjection, SanitizedContextMenuProjectionBuilder,
    SanitizedContextMenuTarget, SanitizedDocumentRootEventDispatchError,
    SanitizedDocumentRootEventForwardError, SanitizedDocumentRootEventForwarder,
    SanitizedDocumentRootEventTransport, SanitizedDocumentRootFactory,
    SanitizedDocumentRootFactoryError, SanitizedDocumentRootIdentity, SanitizedDocumentRootInput,
    SanitizedDocumentRootStyleKey, SanitizedSearchControlPresentation,
    SanitizedSearchLocalizedPresentation, SanitizedSearchOperationPresentation,
    SanitizedSearchOperationSlot, SanitizedSearchProjectionBuildError,
    SanitizedSearchProjectionBuilder, SanitizedSearchResultSummaryPresentation,
    SanitizedSearchTarget, SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation,
    SanitizedTab, SanitizedTabCapabilities, SanitizedTabClosePresentation, SanitizedTabProjection,
    SanitizedTabTarget,
};

#[derive(Default)]
struct CountingForwarder {
    calls: usize,
    reject_text: bool,
    reject_opaque: bool,
}

#[derive(Default)]
struct NullDispatcher {
    reject_text: bool,
    reject_opaque: bool,
}

impl KucRootEventBatchDispatcher for NullDispatcher {
    type Error = ();

    fn dispatch_text_events(
        &mut self,
        _events: Vec<katana_ui_core::text_surface::TextSurfaceEvent>,
    ) -> Result<(), Self::Error> {
        if self.reject_text { Err(()) } else { Ok(()) }
    }

    fn dispatch_toolbar_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_floating_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_search_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_context_menu_events(
        &mut self,
        _events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn consume_opaque_host_effect_batch(
        &mut self,
        effect_batch: KucOpaqueHostEffectBatch,
    ) -> Result<(), KucOpaqueHostEffectError> {
        if self.reject_opaque {
            Err(KucOpaqueHostEffectError)
        } else {
            effect_batch.consume_once()
        }
    }
}

impl SanitizedDocumentRootEventForwarder for CountingForwarder {
    type Error = ();

    fn forward_sanitized_document_root_event(
        &mut self,
        mut transport: SanitizedDocumentRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        let debug = format!("{transport:?}");
        assert_eq!(
            debug,
            "SanitizedDocumentRootEventTransport { payload: \"<opaque>\" }"
        );
        assert!(!debug.contains("本文"));
        let mut dispatcher = NullDispatcher {
            reject_text: self.reject_text,
            reject_opaque: self.reject_opaque,
        };
        if self.reject_text {
            assert_eq!(
                transport.dispatch_root_once(&mut dispatcher),
                Err(SanitizedDocumentRootEventDispatchError::Child(()))
            );
        } else if self.reject_opaque {
            assert_eq!(
                transport.dispatch_root_once(&mut dispatcher),
                Err(SanitizedDocumentRootEventDispatchError::OpaqueHostEffect)
            );
        } else {
            transport
                .dispatch_root_once(&mut dispatcher)
                .map_err(|_| ())?;
        }
        assert_eq!(
            transport.dispatch_root_once(&mut dispatcher),
            Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
        );
        Ok(())
    }
}

fn search_text(value: &str) -> SanitizedSearchTextPresentation {
    SanitizedSearchTextPresentation::new(value, format!("{value}-tooltip"), format!("{value}-a11y"))
}

fn search_localized() -> SanitizedSearchLocalizedPresentation {
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
            search_text("次へ"),
            search_text("置換"),
            search_text("すべて置換"),
            search_text("閉じる"),
        ),
        SanitizedSearchResultSummaryPresentation::new("待機", "一致なし", "1", "{i}", "{n}"),
        SanitizedSearchUnavailablePresentation::new("regex", "replace", "move", "close"),
    )
}

fn search_projection_full()
-> katana_ui_core_egui_adapter::text_command_surface::SanitizedSearchProjection {
    SanitizedSearchProjectionBuilder::new()
        .localized_presentation(search_localized())
        .query_target(
            SanitizedSearchTarget::from_opaque_bytes([1])
                .with_text_capability(|_op, _value| Ok::<(), ()>(())),
        )
        .replacement_target(
            SanitizedSearchTarget::from_opaque_bytes([2])
                .with_text_capability(|_op, _value| Ok::<(), ()>(())),
        )
        .match_case_target(
            SanitizedSearchTarget::from_opaque_bytes([3])
                .with_unit_capability(|_operation| Ok::<(), ()>(())),
        )
        .whole_word_target(
            SanitizedSearchTarget::from_opaque_bytes([4])
                .with_unit_capability(|_operation| Ok::<(), ()>(())),
        )
        .regex_target(
            SanitizedSearchTarget::from_opaque_bytes([5])
                .with_unit_capability(|_operation| Ok::<(), ()>(())),
        )
        .close_enabled(true)
        .close_target(
            SanitizedSearchTarget::from_opaque_bytes([6])
                .with_unit_capability(|_operation| Ok::<(), ()>(())),
        )
        .next_enabled(true)
        .next_target(
            SanitizedSearchTarget::from_opaque_bytes([7])
                .with_unit_capability(|_operation| Ok::<(), ()>(())),
        )
        .previous_enabled(true)
        .previous_target(
            SanitizedSearchTarget::from_opaque_bytes([8])
                .with_unit_capability(|_operation| Ok::<(), ()>(())),
        )
        .replace_enabled(true)
        .replace_target(
            SanitizedSearchTarget::from_opaque_bytes([9])
                .with_text_capability(|_op, _value| Ok::<(), ()>(())),
        )
        .replace_all_enabled(true)
        .replace_all_target(
            SanitizedSearchTarget::from_opaque_bytes([10])
                .with_text_capability(|_op, _value| Ok::<(), ()>(())),
        )
        .build()
        .expect("complete search projection")
}

fn command_projection() -> SanitizedCommandProjection {
    let dropdown = SanitizedCommandDropdownItem::new(
        SanitizedCommandTarget::from_opaque_bytes([9, 10, 11]),
        1,
        "子",
    )
    .tooltip_text("子");

    let item =
        SanitizedCommandItem::new(SanitizedCommandTarget::from_opaque_bytes([1, 2]), 1, "実行")
            .dropdown_item(dropdown)
            .with_icon(UiIconProps::new("<svg/>"))
            .tooltip_text("実行")
            .visible_state(false);

    let group = SanitizedCommandGroup::new(1, "操作").item(item);
    SanitizedCommandProjection::new([group])
}

fn context_projection() -> SanitizedContextMenuProjection {
    SanitizedContextMenuProjectionBuilder::new()
        .item(
            SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes([11]),
                1,
                "貼り付け",
            )
            .with_icon(UiIconProps::new("<svg/>")),
        )
        .item(
            SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes([12]),
                2,
                "コピー",
            )
            .submenu_item(
                SanitizedContextMenuItem::new(
                    SanitizedContextMenuTarget::from_opaque_bytes([13]),
                    0,
                    "サブ",
                )
                .enabled_state(false)
                .checked_state(true),
            ),
        )
        .build()
}

fn tab_projection() -> SanitizedTabProjection {
    let tab = SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 1, "タブ")
        .with_icon(UiIconProps::new("<svg/>"))
        .with_capabilities(
            SanitizedTabCapabilities::new()
                .active_state(true)
                .dirty_state(true)
                .pinned_state(false)
                .close_state(true),
        )
        .with_close_presentation(SanitizedTabClosePresentation::new("×", "閉じる", "閉じる"));

    let _ = tab;
    SanitizedTabProjection::default()
}

fn input_base() -> SanitizedDocumentRootInput {
    SanitizedDocumentRootInput::new(
        1,
        SanitizedDocumentRootIdentity::from_opaque_bytes([1, 2, 3]),
        "本文",
        SanitizedDocumentRootStyleKey::Default,
    )
    .with_readonly(false)
    .with_command_projection(command_projection())
    .with_floating_command_projection(command_projection())
    .with_context_projection(context_projection())
    .with_tab_projection(tab_projection())
    .with_search_projection(search_projection_full())
}

#[test]
fn command_projection_debug_and_target_api_no_payload_leak() {
    let projection = command_projection();
    let debug = format!("{:?}", projection);
    assert!(debug.contains("SanitizedCommandProjection"));
    assert!(!debug.contains("[1, 2, 3]"));

    let target = SanitizedCommandTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]);
    let callback_target = target.with_unit_capability(|| Ok::<(), ()>(()));
    assert_eq!(
        format!("{:?}", callback_target),
        "SanitizedCommandTarget(..)"
    );

    let identity = SanitizedDocumentRootIdentity::from_opaque_bytes([91, 92, 93]);
    assert_eq!(format!("{identity:?}"), "SanitizedDocumentRootIdentity(..)");
    assert!(!format!("{identity:?}").contains("91"));
}

#[test]
fn context_projection_debug_and_target_api_no_payload_leak() {
    let projection = context_projection();
    let debug = format!("{:?}", projection);
    assert!(debug.contains("SanitizedContextMenuProjection"));
    assert!(!debug.contains("[11, 12]"));

    let target = SanitizedContextMenuTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]);
    let callback_target = target.with_unit_capability(|| Ok::<(), ()>(()));
    assert_eq!(
        format!("{:?}", callback_target),
        "SanitizedContextMenuTarget(..)"
    );
}

#[test]
fn tab_projection_debug_and_builder_contract() {
    let projection = tab_projection();
    let debug = format!("{:?}", projection);
    assert!(debug.contains("SanitizedTabProjection"));

    let tab_target = SanitizedTabTarget::from_opaque_bytes([0xde, 0xad]);
    assert_eq!(format!("{:?}", tab_target), "SanitizedTabTarget(..)");
}

#[test]
fn search_projection_rejects_enabled_without_target() {
    let error = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(search_localized())
        .next_enabled(true)
        .build()
        .err()
        .expect("missing target should fail");

    assert!(matches!(
        error,
        SanitizedSearchProjectionBuildError::EnabledOperationWithoutTarget {
            operation: SanitizedSearchOperationSlot::Next,
        }
    ));
}

#[test]
fn search_projection_round_trip_build_complete() {
    let projection = search_projection_full();
    let _ = projection;
}

#[test]
fn document_root_factory_synchronize_paths() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory.retain(input_base()).expect("retain succeeds");

    let unchanged = input_base();
    assert_eq!(
        root.synchronize(unchanged)
            .expect("same revision keeps unchanged"),
        false
    );

    let stale = SanitizedDocumentRootInput::new(
        0,
        SanitizedDocumentRootIdentity::from_opaque_bytes([1, 2, 3]),
        "本文",
        SanitizedDocumentRootStyleKey::Default,
    )
    .with_search_projection(search_projection_full());
    assert!(matches!(
        root.synchronize(stale),
        Err(SanitizedDocumentRootFactoryError::StaleRevision { .. })
    ));

    let changed = SanitizedDocumentRootInput::new(
        2,
        SanitizedDocumentRootIdentity::from_opaque_bytes([1, 2, 3]),
        "更新本文",
        SanitizedDocumentRootStyleKey::Default,
    )
    .with_search_projection(search_projection_full());
    assert!(root.synchronize(changed).expect("higher revision updates"));
}

#[test]
fn document_root_render_record_accessors_and_forwarders() {
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input_base())
        .expect("retain succeeds");

    let context = egui::Context::default();
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            frame = Some(root.show(ui).expect("show succeeds"));
        },
    );
    output.textures_delta.clear();

    let frame = frame.expect("frame exists");
    let record = frame.record();
    assert_eq!(record.revision(), 1);
    assert_eq!(record.presentation_revision(), 1);
    assert_eq!(record.state_revision(), 0);
    assert!(record.rgba_hash().len() > 0);
    assert!(record.paint_plan_hash().len() > 0);
    assert!(record.record_hash().len() > 0);
    assert!(record.accessibility_snapshot_hash().len() > 0);

    let dims = record.dimensions();
    assert!(dims.width() > 0);
    assert!(dims.height() > 0);

    let record_debug = format!("{:?}", record);
    assert!(!record_debug.contains("payload"));
    assert!(!record_debug.contains("[1, 2, 3]"));

    let frame_debug = format!("{:?}", frame);
    assert!(!frame_debug.contains("payload"));
    assert!(!frame_debug.contains("[1, 2, 3]"));

    let mut forwarder = CountingForwarder::default();
    let receipt = frame
        .forward_events_once(&mut forwarder)
        .expect("forward once succeeds");
    assert_eq!(forwarder.calls, 1);
    assert_eq!(receipt.event_cardinality(), 0);
    assert_eq!(receipt.state_revision(), record.state_revision());
    assert!(!receipt.correlation_fingerprint().is_empty());
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );

    let mut rejecting_root = SanitizedDocumentRootFactory::new()
        .retain(input_base())
        .expect("rejecting root retain succeeds");
    let mut rejecting_frame = None;
    let mut rejecting_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            rejecting_frame = Some(rejecting_root.show(ui).expect("rejecting show succeeds"));
        },
    );
    rejecting_output.textures_delta.clear();
    let rejecting_frame = rejecting_frame.expect("rejecting frame exists");
    let mut rejecting_forwarder = CountingForwarder {
        calls: 0,
        reject_text: true,
        reject_opaque: false,
    };
    rejecting_frame
        .forward_events_once(&mut rejecting_forwarder)
        .expect("host handles the typed child rejection");
    assert_eq!(rejecting_forwarder.calls, 1);

    let mut opaque_root = SanitizedDocumentRootFactory::new()
        .retain(input_base())
        .expect("opaque root retain succeeds");
    let mut opaque_frame = None;
    let mut opaque_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            opaque_frame = Some(opaque_root.show(ui).expect("opaque show succeeds"));
        },
    );
    opaque_output.textures_delta.clear();
    let opaque_frame = opaque_frame.expect("opaque frame exists");
    let mut opaque_forwarder = CountingForwarder {
        calls: 0,
        reject_text: false,
        reject_opaque: true,
    };
    opaque_frame
        .forward_events_once(&mut opaque_forwarder)
        .expect("host observes the typed opaque rejection");
    assert_eq!(opaque_forwarder.calls, 1);
}
