use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeSearchPresentation, CommandChromeText,
    CommandChromeToolbarPresentation, FloatingCommandToolbarVisibility, SearchControlIcons,
    SearchControlStrings, SearchResultSummaryTemplate,
};
use katana_ui_core::molecule::selection::ContextMenuItemKind;
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressAction, SourceAddressPresentation, SourceAddressStrip, SourceAddressSubmission,
};
use katana_ui_core::molecule::structured::{ReplaceMode, SearchOptions};
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAutomaticGutterPresentation, TextSurfacePresentation, TextSurfaceProps,
    TextSurfaceViewport,
};
use katana_ui_core_egui_adapter::context_menu::{
    ContextMenuPresentation, ContextMenuPresentationItem,
};
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurfaceCommandFamilyProjection, EguiTextCommandSurfacePresentation,
    TextCommandSurfaceStyle,
};
use katana_ui_core_egui_adapter::text_command_surface::{
    EguiTextCommandSurfaceFloatingPresentation, EguiTextCommandSurfaceHostProjectionEncoder,
    EguiTextCommandSurfaceHostProjectionLease, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceHostTargetToken, EguiTextCommandSurfacePresentationToken,
    EguiTextCommandSurfaceRootEventTransport, EguiTextCommandSurfaceRootFactory,
    EguiTextCommandSurfaceRootFactoryError, EguiTextCommandSurfaceSearchPresentation,
    KucOpaqueHostEffectBatch, KucRootEventBatchContext, KucRootEventBatchForwarder,
};
use katana_ui_core_egui_adapter::text_command_surface::{
    SourceAddressProjectionLease, SourceAddressSubmissionPort, SourceAddressSubmissionPortError,
};
use katana_ui_core_egui_adapter::text_command_surface::{
    TabStripCorrelation, TabStripProjection, TabStripProjectionLease, TabStripTabCapabilities,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
use std::cell::RefCell;
use std::rc::Rc;

trait Propagate<T> {
    fn propagate(self, context: &str) -> Result<T, String>;
}

impl<T, E: std::fmt::Debug> Propagate<T> for Result<T, E> {
    fn propagate(self, context: &str) -> Result<T, String> {
        self.map_err(|error| format!("{context}: {error:?}"))
    }
}

impl<T> Propagate<T> for Option<T> {
    fn propagate(self, context: &str) -> Result<T, String> {
        self.ok_or_else(|| context.to_owned())
    }
}

fn standard_style() -> Result<TextCommandSurfaceStyle, String> {
    TextCommandSurfaceStyle::standard().map_err(|error| format!("standard style: {error}"))
}

fn host_context_presentation() -> EguiTextCommandSurfacePresentation {
    let text = TextSurface::new(
        TextSurfaceProps::new(
            katana_ui_core::atom::TextArea::new("host-root-context-text")
                .value("# KLE / KUC\n\n日本語 ⭐️"),
            Vec::<UiTextSpan>::new(),
            TextSurfaceViewport::new(0, 0, 1280, 720),
        )
        .accessibility_label("KLE Storybook text surface")
        .context_target_label("KLE Storybook context target"),
    );
    let mut text = TextSurfacePresentation::from_props(text.props());
    text.selection_start = "# KLE / KUC\n\n".len();
    text.selection_end = text.selection_start + "日本語 ⭐️".len();
    EguiTextCommandSurfacePresentation {
        text_state_id: Some(UiStateId::new("host-root-context-text")),
        text,
        toolbar: Some(CommandChromeToolbarPresentation {
            actions: vec![CommandChromeAction::new(
                "generic-command",
                "Generic command",
            )],
            groups: Vec::new(),
            display_mode: Default::default(),
            density: Default::default(),
            overflow_strategy: Default::default(),
        }),
        floating: Some(EguiTextCommandSurfaceFloatingPresentation {
            toolbar: CommandChromeToolbarPresentation {
                actions: vec![CommandChromeAction::new(
                    "generic-selection",
                    "Generic selection",
                )],
                groups: Vec::new(),
                display_mode: Default::default(),
                density: Default::default(),
                overflow_strategy: Default::default(),
            },
            visibility: FloatingCommandToolbarVisibility::Visible,
        }),
        search: Some(EguiTextCommandSurfaceSearchPresentation {
            state_id: UiStateId::new("generic-search"),
            label: String::from("Generic search"),
            value: CommandChromeSearchPresentation {
                query: String::from("query"),
                options: SearchOptions::default(),
                result_count: Some(0),
                active_index: None,
                replace_mode: ReplaceMode::Hidden,
                replace_value: String::from("replacement"),
                strings: generic_search_strings(),
                capabilities: Default::default(),
                icons: SearchControlIcons::default(),
            },
        }),
        context_menu: Some(ContextMenuPresentation {
            visible: true,
            items: vec![
                ContextMenuPresentationItem::action("context.save", "保存 ⭐️"),
                ContextMenuPresentationItem {
                    id: String::from("context.authoring"),
                    label: String::from("Markdown editing ⭐️"),
                    accessibility_label: String::new(),
                    icon: None,
                    enabled: true,
                    checked: false,
                    kind: ContextMenuItemKind::Submenu,
                    children: vec![
                        ContextMenuPresentationItem::action("context.author.bold", "Bold ⭐️"),
                        ContextMenuPresentationItem::action(
                            "context.author.code-block",
                            "Code block ⭐️",
                        )
                        .child(ContextMenuPresentationItem::action(
                            "context.author.code-block.markdown",
                            "Markdown ⭐️",
                        )),
                    ],
                },
                ContextMenuPresentationItem {
                    id: String::from("context.ingest"),
                    label: String::from("Image ingest ⭐️"),
                    accessibility_label: String::new(),
                    icon: None,
                    enabled: true,
                    checked: false,
                    kind: ContextMenuItemKind::Submenu,
                    children: vec![ContextMenuPresentationItem::action(
                        "context.ingest.image-file",
                        "Image file ⭐️",
                    )],
                },
            ],
        }),
    }
}

fn generic_search_strings() -> SearchControlStrings {
    let text = |value: &str| CommandChromeText::new(value, value, value);
    SearchControlStrings {
        strip: text("Generic search"),
        query: text("Query"),
        replace: text("Replace"),
        match_case: text("Match case"),
        whole_word: text("Whole word"),
        use_regex: text("Regex"),
        previous: text("Previous"),
        next: text("Next"),
        replace_one: text("Replace one"),
        replace_all: text("Replace all"),
        close: text("Close"),
        result_summary: SearchResultSummaryTemplate {
            empty: String::new(),
            zero_results: String::from("0"),
            single_result: String::from("1 / 1"),
            indexed_result: String::from("{active} / {count}"),
            count_results: String::from("{count}"),
        },
    }
}

fn host_context_lease(
    revision: u64,
) -> Result<
    katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceHostProjectionLease,
    String,
> {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
        revision,
        b"host-root-context-target".to_vec(),
        host_context_presentation(),
        standard_style()?,
        EguiTextCommandSurfaceCommandFamilyProjection::new(
            Some(
                katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new(
                    "context-primary",
                ),
            ),
            Some(
                katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new(
                    "context-floating",
                ),
            ),
        ),
    )
    .propagate("host context token")?;
    Ok(katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
        token,
        |context: KucRootEventBatchContext| {
            if context.context_menu_events().is_empty() {
                Ok(None)
            } else {
                Ok(Some(KucOpaqueHostEffectBatch::from_handler(|| Ok(()))))
            }
        },
    ))
}

fn complex_host_context_presentation() -> Result<EguiTextCommandSurfacePresentation, String> {
    let mut presentation = host_context_presentation();
    presentation.search = None;
    presentation.toolbar.as_mut().propagate("toolbar")?.actions = (0..12)
        .map(|index| {
            CommandChromeAction::new(format!("toolbar.{index}"), format!("Toolbar {index}"))
        })
        .collect();
    presentation
        .floating
        .as_mut()
        .propagate("floating toolbar")?
        .toolbar
        .actions = (0..12)
        .map(|index| {
            CommandChromeAction::new(format!("floating.{index}"), format!("Floating {index}"))
        })
        .collect();
    presentation.text.automatic_gutter = Some(TextSurfaceAutomaticGutterPresentation::new());

    let authoring_leafs = (0..14)
        .map(|index| {
            ContextMenuPresentationItem::action(
                format!("context.authoring.leaf-{index}"),
                format!("Markdown leaf {index} ⭐️"),
            )
        })
        .collect::<Vec<_>>();
    let code_kinds = [
        "text",
        "markdown",
        "bash",
        "zsh",
        "mermaid",
        "drawio",
        "plantuml",
        "json",
        "yaml",
        "toml",
        "rust",
        "typescript",
        "javascript",
        "python",
        "html",
        "css",
        "sql",
    ]
    .into_iter()
    .map(|kind| {
        ContextMenuPresentationItem::action(
            format!("context.authoring.code.{kind}"),
            format!("{kind} ⭐️"),
        )
    })
    .collect();
    let mut authoring = ContextMenuPresentationItem {
        id: String::from("context.authoring"),
        label: String::from("Markdown authoring ⭐️"),
        accessibility_label: String::new(),
        icon: None,
        enabled: true,
        checked: false,
        kind: ContextMenuItemKind::Submenu,
        children: authoring_leafs,
    };
    authoring.children.push(ContextMenuPresentationItem {
        id: String::from("context.authoring.code"),
        label: String::from("Code kind ⭐️"),
        accessibility_label: String::new(),
        icon: None,
        enabled: true,
        checked: false,
        kind: ContextMenuItemKind::Submenu,
        children: code_kinds,
    });
    presentation.context_menu = Some(ContextMenuPresentation {
        visible: true,
        items: vec![
            ContextMenuPresentationItem::action("context.save", "保存 ⭐️"),
            ContextMenuPresentationItem::action("context.format", "Format ⭐️"),
            authoring,
            ContextMenuPresentationItem {
                id: String::from("context.ingest"),
                label: String::from("Image ingest ⭐️"),
                accessibility_label: String::new(),
                icon: None,
                enabled: true,
                checked: false,
                kind: ContextMenuItemKind::Submenu,
                children: vec![ContextMenuPresentationItem {
                    id: String::from("context.ingest.image-file"),
                    label: String::from("Image file ⭐️"),
                    accessibility_label: String::new(),
                    icon: None,
                    enabled: false,
                    checked: false,
                    kind: ContextMenuItemKind::Action,
                    children: Vec::new(),
                }],
            },
        ],
    });
    Ok(presentation)
}

fn complex_host_context_lease(
    revision: u64,
) -> Result<
    katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceHostProjectionLease,
    String,
> {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
        revision,
        b"complex-host-root-context-target".to_vec(),
        complex_host_context_presentation()?,
        standard_style()?,
        EguiTextCommandSurfaceCommandFamilyProjection::new(
            Some(
                katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new(
                    "complex-primary",
                ),
            ),
            Some(
                katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new(
                    "complex-floating",
                ),
            ),
        ),
    )
    .propagate("complex host context token")?;
    Ok(katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
        token,
        |context: KucRootEventBatchContext| {
            if context.context_menu_events().is_empty() {
                Ok(None)
            } else {
                Ok(Some(KucOpaqueHostEffectBatch::from_handler(|| Ok(()))))
            }
        },
    ))
}

fn host_frame(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> Result<
    katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    String,
> {
    let mut frame = None;
    let mut render_error = None;
    let _ = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| match root.show(ui).propagate("host root show") {
            Ok(value) => frame = Some(value),
            Err(error) => render_error = Some(error),
        },
    );
    if let Some(error) = render_error {
        return Err(error);
    }
    frame.propagate("host root frame")
}

fn family_projection(
    primary: &str,
    floating: &str,
) -> EguiTextCommandSurfaceCommandFamilyProjection {
    EguiTextCommandSurfaceCommandFamilyProjection::new(
        Some(katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new(primary)),
        Some(katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new(floating)),
    )
}

fn source_address_lease() -> SourceAddressProjectionLease {
    SourceAddressProjectionLease::new(SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソース ⭐️",
        "ソースアドレス",
        "ソースアドレス入力",
    )))
}

fn tab_strip_lease() -> TabStripProjectionLease {
    TabStripProjectionLease::new(
        TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"facade-tab-correlation"),
        )
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"facade-tab-target"),
                TabStripText::new("日本語 ⭐️"),
            )
            .capabilities(TabStripTabCapabilities::new().active(true)),
        ),
    )
}

#[derive(Clone, Default)]
struct RecordingSourcePort(Rc<RefCell<Vec<String>>>);

impl SourceAddressSubmissionPort for RecordingSourcePort {
    fn forward_submission(
        &mut self,
        submission: SourceAddressSubmission,
    ) -> Result<(), SourceAddressSubmissionPortError> {
        self.0.borrow_mut().push(submission.into_draft());
        Ok(())
    }
}

#[test]
fn legacy_token_round_trips_with_the_shared_default_family() -> Result<(), String> {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"legacy-round-trip".to_vec(),
        {
            let mut presentation = host_context_presentation();
            presentation.toolbar = None;
            presentation.floating = None;
            presentation
        },
        standard_style()?,
    )
    .propagate("legacy token")?;
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .propagate("legacy token retain")?;
    let context = egui::Context::default();
    let mut result = None;
    let _ = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| result = Some(root.show(ui)),
    );
    assert!(
        result.propagate("legacy round-trip result")?.is_ok(),
        "legacy payload without command chrome must round-trip"
    );
    Ok(())
}

#[test]
fn source_address_lease_is_consumed_by_facade_and_changes_render_and_accesskit_records()
-> Result<(), String> {
    let mut presentation = host_context_presentation();
    presentation.floating = None;
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"source-address-facade".to_vec(),
        presentation.clone(),
        standard_style()?,
    )
    .propagate("token")?;
    let mut legacy = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .propagate("legacy root")?;
    let legacy_frame = host_frame(&egui::Context::default(), &mut legacy, Vec::new())?;

    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"source-address-facade".to_vec(),
        presentation,
        standard_style()?,
    )
    .propagate("token")?;
    let mut with_source = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            EguiTextCommandSurfaceHostProjectionLease::new(token, |_context| Ok(None))
                .with_source_address(source_address_lease()),
        )
        .propagate("source-address root")?;
    let source_frame = host_frame(&egui::Context::default(), &mut with_source, Vec::new())?;

    assert_ne!(
        legacy_frame.record().rgba_hash(),
        source_frame.record().rgba_hash()
    );
    assert_ne!(
        legacy_frame.record().accessibility_snapshot_hash(),
        source_frame.record().accessibility_snapshot_hash()
    );
    Ok(())
}

#[test]
fn source_address_lease_rejects_stale_or_duplicate_revision_and_accepts_newer_replacement()
-> Result<(), String> {
    let mut presentation = host_context_presentation();
    presentation.floating = None;
    let token = |revision| -> Result<_, String> {
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            revision,
            b"source-address-revision".to_vec(),
            presentation.clone(),
            standard_style()?,
        )
        .propagate("token")
    };
    let lease = |revision, label: &str| -> Result<_, String> {
        let mut strip =
            SourceAddressStrip::new(SourceAddressPresentation::new(label, "説明", "入力"));
        let _ = strip.apply_action(SourceAddressAction::SetDraft(label.to_owned()));
        Ok(
            EguiTextCommandSurfaceHostProjectionLease::new(token(revision)?, |_context| Ok(None))
                .with_source_address(SourceAddressProjectionLease::new(strip)),
        )
    };

    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease(1, "初期ソース ⭐️")?)
        .propagate("retain source root")?;
    assert!(matches!(
        root.synchronize_with_lease(lease(1, "重複ソース")?),
        Err(EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision: 1 })
    ));
    assert!(matches!(
        root.synchronize_with_lease(lease(0, "古いソース")?),
        Err(EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision: 0 })
    ));

    let before = host_frame(&egui::Context::default(), &mut root, Vec::new())?;
    root.synchronize_with_lease(lease(2, "新しいソース ⭐️")?)
        .propagate("newer source lease")?;
    let after = host_frame(&egui::Context::default(), &mut root, Vec::new())?;
    assert_ne!(before.record().rgba_hash(), after.record().rgba_hash());
    assert_eq!(after.record().presentation_revision(), 2);
    Ok(())
}

#[test]
fn plain_newer_token_removes_a_prior_tab_strip_lease() -> Result<(), String> {
    let mut presentation = host_context_presentation();
    presentation.floating = None;
    let token = |revision| -> Result<_, String> {
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            revision,
            b"tab-strip-plain-replacement".to_vec(),
            presentation.clone(),
            standard_style()?,
        )
        .propagate("token")
    };
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            EguiTextCommandSurfaceHostProjectionLease::new(token(1)?, |_context| Ok(None))
                .with_tab_strip(tab_strip_lease()),
        )
        .propagate("retain tab strip root")?;
    let context = egui::Context::default();
    let before = host_frame(&context, &mut root, Vec::new())?;

    assert!(root.synchronize(token(2)?).propagate("plain replacement")?);
    let after = host_frame(&context, &mut root, Vec::new())?;

    assert_ne!(before.record().rgba_hash(), after.record().rgba_hash());
    assert_eq!(2, after.record().presentation_revision());
    Ok(())
}

#[test]
fn source_address_submission_port_is_attached_through_consuming_lease_without_wire_values()
-> Result<(), String> {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let port = RecordingSourcePort(Rc::clone(&calls));
    let lease = source_address_lease().with_submission_port(port);
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"source-address-port".to_vec(),
        host_context_presentation(),
        standard_style()?,
    )
    .propagate("token")?;
    let host_lease = EguiTextCommandSurfaceHostProjectionLease::new(token, |_context| Ok(None))
        .with_source_address(lease);
    let debug = format!("{host_lease:?}");
    assert!(!debug.contains("ソース"));
    assert!(!debug.contains("⭐️"));
    assert!(calls.borrow().is_empty());
    Ok(())
}

#[test]
fn source_address_facade_forwards_two_input_submissions_across_distinct_root_transports()
-> Result<(), String> {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let port = RecordingSourcePort(Rc::clone(&calls));
    let mut presentation = host_context_presentation();
    presentation.floating = None;
    let token = |revision| -> Result<_, String> {
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            revision,
            b"source-address-input-submission".to_vec(),
            presentation.clone(),
            standard_style()?,
        )
        .propagate("source-address token")
    };

    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(
            EguiTextCommandSurfaceHostProjectionLease::new(token(1)?, |_context| Ok(None))
                .with_source_address(source_address_lease().with_submission_port(port.clone())),
        )
        .propagate("retain source-address host root")?;
    let context = egui::Context::default();
    context.enable_accesskit();

    let initial = host_frame(&context, &mut root, Vec::new())?;
    let initial_debug = format!("{initial:?}");
    assert!(!initial_debug.contains("draft-one"));
    assert!(!initial_debug.contains("draft-two"));
    initial
        .forward_events_once(&mut CompileOnlyForwarder)
        .propagate("forward initial root transport")?;

    let focused_one = host_frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(egui::pos2(80.0, 14.0)),
            egui::Event::PointerButton {
                pos: egui::pos2(80.0, 14.0),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: egui::pos2(80.0, 14.0),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ],
    )?;
    focused_one
        .forward_events_once(&mut CompileOnlyForwarder)
        .propagate("forward first focus transport")?;
    let typed_one = host_frame(
        &context,
        &mut root,
        vec![egui::Event::Text(String::from("draft-one"))],
    )?;
    typed_one
        .forward_events_once(&mut CompileOnlyForwarder)
        .propagate("forward first physical text transport")?;
    let submitted_one = host_frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }],
    )?;
    let first_debug = format!("{submitted_one:?}");
    let first_record_debug = format!("{:?}", submitted_one.record());
    for raw_draft in ["draft-one", "draft-two"] {
        assert!(!first_debug.contains(raw_draft));
        assert!(!first_record_debug.contains(raw_draft));
    }
    submitted_one
        .forward_events_once(&mut SourceAddressPortDispatchForwarder)
        .propagate("forward first submission transport")?;

    root.synchronize_with_lease(
        EguiTextCommandSurfaceHostProjectionLease::new(token(2)?, |_context| Ok(None))
            .with_source_address(source_address_lease().with_submission_port(port)),
    )
    .propagate("synchronize second source-address lease")?;

    let second_initial = host_frame(&context, &mut root, Vec::new())?;
    let second_initial_debug = format!("{second_initial:?}");
    assert!(!second_initial_debug.contains("draft-one"));
    assert!(!second_initial_debug.contains("draft-two"));
    second_initial
        .forward_events_once(&mut CompileOnlyForwarder)
        .propagate("forward second initial root transport")?;

    let focused_two = host_frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(egui::pos2(80.0, 14.0)),
            egui::Event::PointerButton {
                pos: egui::pos2(80.0, 14.0),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: egui::pos2(80.0, 14.0),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ],
    )?;
    focused_two
        .forward_events_once(&mut CompileOnlyForwarder)
        .propagate("forward second focus transport")?;
    let typed_two = host_frame(
        &context,
        &mut root,
        vec![egui::Event::Text(String::from("draft-two"))],
    )?;
    typed_two
        .forward_events_once(&mut CompileOnlyForwarder)
        .propagate("forward second physical text transport")?;
    let submitted_two = host_frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }],
    )?;
    let second_debug = format!("{submitted_two:?}");
    let second_record_debug = format!("{:?}", submitted_two.record());
    for raw_draft in ["draft-one", "draft-two"] {
        assert!(!second_debug.contains(raw_draft));
        assert!(!second_record_debug.contains(raw_draft));
    }
    submitted_two
        .forward_events_once(&mut SourceAddressPortDispatchForwarder)
        .propagate("forward second submission transport")?;

    assert_eq!(calls.borrow().as_slice(), ["draft-one", "draft-two"]);
    Ok(())
}

#[test]
fn legacy_token_with_both_slots_fails_without_exposing_family_bytes() -> Result<(), String> {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"legacy-dual-slot".to_vec(),
        host_context_presentation(),
        standard_style()?,
    )
    .propagate("legacy token")?;
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .propagate("retain token")?;
    let context = egui::Context::default();
    let mut result = None;
    let _ = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| result = Some(root.show(ui)),
    );
    let error = result
        .propagate("legacy result")?
        .err()
        .ok_or_else(|| "legacy dual-slot payload must fail".to_owned())?;
    let display = error.to_string();
    assert!(display.contains("command family"));
    assert!(!display.contains("default"));
    Ok(())
}

#[test]
fn explicit_distinct_families_render_both_slots_once() -> Result<(), String> {
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(
            EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
                1,
                b"distinct-families".to_vec(),
                host_context_presentation(),
                standard_style()?,
                family_projection("primary", "floating"),
            )
            .propagate("distinct token")?,
        )
        .propagate("retain distinct token")?;
    let context = egui::Context::default();
    let mut result = None;
    let _ = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| result = Some(root.show(ui)),
    );
    assert!(result.propagate("root result")?.is_ok());
    assert!(
        root.synchronize(
            EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
                2,
                b"distinct-families".to_vec(),
                host_context_presentation(),
                standard_style()?,
                family_projection("primary-next", "floating-next"),
            )
            .propagate("synchronization token")?,
        )
        .propagate("synchronize distinct families")?
    );
    let mut synchronized_result = None;
    let _ = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| synchronized_result = Some(root.show(ui)),
    );
    assert!(
        synchronized_result
            .propagate("synchronized root result")?
            .is_ok()
    );
    Ok(())
}

#[test]
fn explicit_same_family_fails_before_render_without_exposing_family_bytes() -> Result<(), String> {
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(
            EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
                1,
                b"same-family".to_vec(),
                host_context_presentation(),
                standard_style()?,
                family_projection("private-family", "private-family"),
            )
            .propagate("same-family token")?,
        )
        .propagate("retain same-family token")?;
    let context = egui::Context::default();
    let mut result = None;
    let _ = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| result = Some(root.show(ui)),
    );
    let display = result
        .propagate("root result")?
        .err()
        .ok_or_else(|| "same family must fail before render".to_owned())?
        .to_string();
    assert!(!display.contains("private-family"));
    Ok(())
}

#[test]
fn revision_conflict_includes_family_projection() -> Result<(), String> {
    let factory = EguiTextCommandSurfaceRootFactory::new();
    let mut root = factory
        .retain(
            EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
                1,
                b"family-revision".to_vec(),
                host_context_presentation(),
                standard_style()?,
                family_projection("primary-a", "floating-a"),
            )
            .propagate("initial token")?,
        )
        .propagate("retain initial token")?;
    let conflict = root.synchronize(
        EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
            1,
            b"family-revision".to_vec(),
            host_context_presentation(),
            standard_style()?,
            family_projection("primary-b", "floating-b"),
        )
        .propagate("conflicting token")?,
    );
    assert!(matches!(
        conflict,
        Err(EguiTextCommandSurfaceRootFactoryError::RevisionConflict { revision: 1 })
    ));
    Ok(())
}

#[test]
fn lease_debug_and_token_debug_do_not_expose_family_interpretation() -> Result<(), String> {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
        1,
        b"opaque-family-contract".to_vec(),
        host_context_presentation(),
        standard_style()?,
        family_projection("opaque-primary", "opaque-floating"),
    )
    .propagate("token")?;
    let token_debug = format!("{token:?}");
    assert!(!token_debug.contains("opaque-primary"));
    assert!(!token_debug.contains("opaque-floating"));
    let lease = katana_ui_core_egui_adapter::text_command_surface::EguiTextCommandSurfaceHostProjectionLease::new(
        token,
        |_context: KucRootEventBatchContext| {
            Ok::<_, katana_ui_core_egui_adapter::text_command_surface::KucOpaqueHostEffectError>(
                None,
            )
        },
    );
    let lease_debug = format!("{lease:?}");
    assert!(!lease_debug.contains("opaque-primary"));
    assert!(!lease_debug.contains("opaque-floating"));
    Ok(())
}

#[test]
fn host_root_fresh_lease_context_open_then_leaf_capture_preserves_context_state()
-> Result<(), String> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(host_context_lease(1)?)
        .propagate("retain host root")?;

    root.synchronize_with_lease(host_context_lease(2)?)
        .propagate("synchronize initial lease")?;
    let initial = host_frame(&context, &mut root, Vec::new())?;
    initial
        .forward_events_once(&mut CompileOnlyForwarder)
        .propagate("forward initial frame once")?;
    let mut open_request = initial
        .interaction_locator()
        .request_context_open()
        .propagate("initial context target")?;
    let mut open_input = egui::RawInput::default();
    open_request
        .apply_to_raw_input_once(&mut open_input)
        .propagate("queue context opener")?;

    root.synchronize_with_lease(host_context_lease(3)?)
        .propagate("synchronize opener lease")?;
    let opened = host_frame(&context, &mut root, open_input.events)?;
    opened
        .forward_events_once(&mut CompileOnlyForwarder)
        .propagate("forward opener frame once")?;
    let mut leaf_request = opened
        .interaction_locator()
        .request(katana_ui_core_egui_adapter::text_command_surface::KucInteractionSelector::new(
            "context.save",
            katana_ui_core_egui_adapter::text_command_surface::KucInteractionActionClass::ContextMenuItem,
        ))
        .propagate("context.save on opener frame")?;

    root.synchronize_with_lease(host_context_lease(4)?)
        .propagate("synchronize leaf lease")?;
    let mut leaf_input = egui::RawInput::default();
    leaf_request
        .apply_to_raw_input_once(&mut leaf_input)
        .propagate("queue context leaf")?;
    let captured = host_frame(&context, &mut root, leaf_input.events)?;
    captured
        .forward_events_once(&mut CompileOnlyForwarder)
        .propagate("forward leaf frame once")?;
    Ok(())
}

#[test]
fn host_root_complex_kle_context_fixture_preserves_current_frame_context_leaf() -> Result<(), String>
{
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(complex_host_context_lease(1)?)
        .propagate("retain complex host root")?;

    root.synchronize_with_lease(complex_host_context_lease(2)?)
        .propagate("synchronize initial complex lease")?;
    let initial = host_frame(&context, &mut root, Vec::new())?;
    let mut open_request = initial
        .interaction_locator()
        .request_context_open()
        .propagate("complex context target")?;
    let mut open_input = egui::RawInput::default();
    open_request
        .apply_to_raw_input_once(&mut open_input)
        .propagate("queue complex context opener")?;

    root.synchronize_with_lease(complex_host_context_lease(3)?)
        .propagate("synchronize complex opener lease")?;
    let opened = host_frame(&context, &mut root, open_input.events)?;
    assert!(opened
        .interaction_locator()
        .request(katana_ui_core_egui_adapter::text_command_surface::KucInteractionSelector::new(
            "context.save",
            katana_ui_core_egui_adapter::text_command_surface::KucInteractionActionClass::ContextMenuItem,
        ))
        .is_ok());
    Ok(())
}

fn compile_only_root_factory_signature(
    factory: &EguiTextCommandSurfaceRootFactory,
    token: EguiTextCommandSurfacePresentationToken,
) -> Result<EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceRootFactoryError> {
    factory.retain(token)
}

struct CompileOnlyForwarder;

impl KucRootEventBatchForwarder for CompileOnlyForwarder {
    type Error = ();

    fn forward_root_event_batch(
        &mut self,
        _transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct SourceAddressPortDispatchForwarder;

struct SourceAddressPortDispatcher;

impl katana_ui_core_egui_adapter::text_command_surface::KucRootEventBatchDispatcher
    for SourceAddressPortDispatcher
{
    type Error = ();

    fn dispatch_text_events(
        &mut self,
        _events: Vec<katana_ui_core::text_surface::TextSurfaceEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
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
}

impl KucRootEventBatchForwarder for SourceAddressPortDispatchForwarder {
    type Error = ();

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        transport
            .dispatch_once(&mut SourceAddressPortDispatcher)
            .map(|_| ())
            .map_err(|_| ())
    }
}

#[test]
fn public_facade_compiles_with_opaque_tokens_and_one_shot_transport() -> Result<(), String> {
    let _factory = EguiTextCommandSurfaceRootFactory::new();
    let _encoder = EguiTextCommandSurfaceHostProjectionEncoder::new();
    let _target = EguiTextCommandSurfaceHostTargetToken::from_opaque_bytes(b"host-target");
    let _forwarder = CompileOnlyForwarder;
    let _factory_signature = compile_only_root_factory_signature;
    Ok(())
}

#[test]
fn opaque_tokens_and_transport_have_no_clone_or_serialize_derives() -> Result<(), String> {
    let source = include_str!("../src/text_command_surface/host_root/types.rs");
    for type_name in [
        "EguiTextCommandSurfaceHostTargetToken",
        "EguiTextCommandSurfacePresentationToken",
    ] {
        let declaration = source
            .split_once(&format!("pub struct {type_name}"))
            .and_then(|(_, value)| value.split_once("\n}\n"))
            .map(|(value, _)| value)
            .propagate("opaque token declaration was not found")?;
        assert!(!declaration.contains("Clone"), "{type_name} became Clone");
        assert!(
            !declaration.contains("Serialize"),
            "{type_name} became Serialize"
        );
    }

    let event_source = include_str!("../src/text_command_surface/root_event.rs");
    let transport_marker = "pub struct EguiTextCommandSurfaceRootEventTransport";
    let transport_start = event_source
        .find(transport_marker)
        .propagate("opaque event transport declaration was not found")?;
    let declaration_start = event_source[..transport_start]
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .filter(|line| line.trim_start().starts_with("#[derive("))
        .and_then(|line| event_source[..transport_start].rfind(line))
        .unwrap_or(transport_start);
    let transport = event_source[declaration_start..]
        .split_once("\n}\n")
        .map(|(value, _)| value)
        .propagate("opaque event transport declaration was not found")?;
    assert!(!transport.contains("Clone"));
    assert!(!transport.contains("Serialize"));
    Ok(())
}

#[test]
fn public_facade_signatures_reject_child_and_presentation_concrete_types() -> Result<(), String> {
    let source = include_str!("../src/text_command_surface/host_root.rs");
    let root_sections = [source
        .split_once("impl EguiTextCommandSurfaceRootFactory")
        .and_then(|(_, value)| {
            value.split_once("impl Default for EguiTextCommandSurfaceRootFactory")
        })
        .map(|(value, _)| value)];
    let module_sources = [
        include_str!("../src/text_command_surface/host_root/frame.rs"),
        include_str!("../src/text_command_surface/host_root/errors.rs"),
    ];
    let forbidden = [
        "EguiTextCommandSurfacePresentation,",
        "EguiTextCommandSurfacePresentation>",
        "EguiTextCommandSurfacePresentation)",
        "EguiTextCommandSurfaceFloatingPresentation,",
        "EguiTextCommandSurfaceFloatingPresentation>",
        "EguiTextCommandSurfaceSearchPresentation,",
        "EguiTextCommandSurfaceSearchPresentation>",
        "TextCommandSurfaceStyle",
        "TextSurfaceEvent",
        "CommandChromeToolbarEvent",
        "FloatingCommandToolbarEvent",
        "CommandChromeSearchEvent",
        "ContextMenuEvent",
        "PaintPlan",
        "TextureId",
        "egui::Id",
        "PlatformTextRasterConfig",
        "EguiTextCommandSurfaceRootError",
    ];
    for section in root_sections
        .into_iter()
        .chain(module_sources.into_iter().map(Some))
    {
        let section = section.propagate("facade public section was not found")?;
        for forbidden_name in forbidden {
            assert!(
                !section.contains(forbidden_name),
                "public facade section leaked `{forbidden_name}`"
            );
        }
    }
    assert!(!source.contains("pub fn with_text_raster_config"));
    Ok(())
}

#[test]
fn compatibility_types_are_hidden_and_storybook_uses_only_the_facade_root() -> Result<(), String> {
    let module_source = include_str!("../src/text_command_surface.rs");
    assert!(module_source.contains("#[doc(hidden)]\npub use root"));
    assert!(module_source.contains("#[doc(hidden)]\npub use types"));

    let storybook_source =
        include_str!("../../katana-ui-core-storybook/src/visual/text_command_root_storybook.rs");
    for forbidden in [
        "EguiTextCommandSurfaceRoot,",
        "EguiTextCommandSurfaceRootOutput",
        "EguiTextCommandSurfaceAdapter",
        "EguiTextCommandSurface,",
        "ArtifactCompositor",
        "PaintPlan",
        "TextureId",
        "egui::Id",
    ] {
        assert!(
            !storybook_source.contains(forbidden),
            "full-root Storybook leaked `{forbidden}`"
        );
    }
    assert!(storybook_source.contains("EguiTextCommandSurfaceHostRoot"));
    assert!(storybook_source.contains("EguiTextCommandSurfaceRootFactory"));
    assert!(module_source.contains("EguiTextCommandSurfaceHostProjectionEncoder"));
    assert!(!module_source.contains("EguiTextCommandSurfaceRootStorybookBuilder"));
    assert!(storybook_source.contains("root.show(ui)"));
    Ok(())
}

#[test]
fn host_projection_boundary_keeps_target_bytes_and_rgba_private() -> Result<(), String> {
    let source = include_str!("../src/text_command_surface/host_root/types.rs");
    assert!(source.contains("pub struct EguiTextCommandSurfaceHostProjectionEncoder"));
    assert!(!source.contains("String::from_utf8"));
    assert!(
        !include_str!("../src/text_command_surface/host_root_facade.rs")
            .contains("pub fn rgba_pixels")
    );
    assert!(
        !include_str!("../src/text_command_surface/host_root_token_codec.rs")
            .contains("target.payload.to_vec()")
    );
    Ok(())
}
