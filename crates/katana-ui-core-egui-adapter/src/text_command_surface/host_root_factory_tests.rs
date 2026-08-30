use super::*;
use crate::text_command_surface::{
    TabStripControlPresentation, TabStripCorrelation, TabStripNavigationPresentation,
    TabStripProjection, TabStripScrollPresentation, TabStripText,
};
use egui::RawInput;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeToolbarPresentation, FloatingCommandToolbarVisibility,
};
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressPresentation, SourceAddressStrip,
};
use katana_ui_core::molecule::{DiagnosticsList, StatusBar, StatusBarSegment};
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

#[test]
fn empty_status_diagnostics_lease_is_opaque_and_consumes_to_empty_parts() {
    let lease = StatusDiagnosticsProjectionLease::default();
    assert_eq!(format!("{lease:?}"), "StatusDiagnosticsProjectionLease(..)");
    let (status, diagnostics) = lease.into_parts();
    assert!(status.is_none());
    assert!(diagnostics.is_none());
}

#[test]
fn opaque_token_and_encoder_entry_points_execute_in_the_unit_crate() {
    let target = EguiTextCommandSurfaceHostTargetToken::from_opaque_bytes(b"target".to_vec());
    assert_eq!(
        format!("{target:?}"),
        "EguiTextCommandSurfaceHostTargetToken(..)"
    );
    let opaque = EguiTextCommandSurfacePresentationToken::from_opaque_bytes(
        7,
        EguiTextCommandSurfaceHostTargetToken::from_opaque_bytes(b"opaque".to_vec()),
        b"payload".to_vec(),
    );
    assert!(format!("{opaque:?}").contains("<opaque>"));

    let encoded = EguiTextCommandSurfacePresentationToken::from_encoded(
        8,
        EguiTextCommandSurfaceHostTargetToken::from_opaque_bytes(b"encoded".to_vec()),
        b"encoded-payload".to_vec(),
    );
    assert_eq!(encoded.revision, 8);

    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let encoder = <EguiTextCommandSurfaceHostProjectionEncoder as Default>::default();
    assert!(
        encoder
            .encode(1, b"legacy-encode".to_vec(), presentation(), style.clone(),)
            .is_ok()
    );
    assert!(
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            2,
            b"legacy-token".to_vec(),
            presentation(),
            style.clone(),
        )
        .is_ok()
    );
    assert!(
        encoder
            .encode_with_command_families(
                3,
                b"family-encode".to_vec(),
                presentation(),
                style.clone(),
                EguiTextCommandSurfaceCommandFamilyProjection::default()
            )
            .is_ok()
    );
    assert!(
        EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
            4,
            b"family-token".to_vec(),
            presentation(),
            style,
            EguiTextCommandSurfaceCommandFamilyProjection::default()
        )
        .is_ok()
    );
    let _factory = <EguiTextCommandSurfaceRootFactory as Default>::default();
}

#[test]
fn empty_host_target_fails_closed_before_payload_decode() {
    let token = EguiTextCommandSurfacePresentationToken::from_opaque_bytes(
        1,
        EguiTextCommandSurfaceHostTargetToken::from_opaque_bytes(Vec::new()),
        b"not-json".to_vec(),
    );
    assert!(matches!(
        EguiTextCommandSurfaceRootFactory::new().retain(token),
        Err(EguiTextCommandSurfaceRootFactoryError::InvalidToken(
            "host target is empty"
        ))
    ));
}

#[test]
fn retain_then_identity_change_fails_closed() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"identity-target-a",
        presentation(),
        style.clone(),
    )
    .expect("token A");
    let token_switched = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"identity-target-b",
        presentation(),
        style,
    )
    .expect("token B");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("retain initial");

    assert!(matches!(
        root.synchronize(token_switched),
        Err(EguiTextCommandSurfaceRootFactoryError::IdentityChanged)
    ));
}

#[test]
fn synchronize_rejects_stale_revision() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        2,
        b"stale-target",
        presentation(),
        style.clone(),
    )
    .expect("token");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("retain initial");
    let stale = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"stale-target",
        presentation(),
        style,
    )
    .expect("stale token");

    assert!(matches!(
        root.synchronize(stale),
        Err(EguiTextCommandSurfaceRootFactoryError::StaleRevision {
            current: 2,
            received: 1
        })
    ));
}

#[test]
fn synchronize_detects_revision_conflict_for_same_revision_but_different_payload() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"conflict-target",
        presentation(),
        style.clone(),
    )
    .expect("initial token");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("retain initial");

    let mut changed = presentation();
    changed.text.value = String::from("updated text body");
    let conflict =
        EguiTextCommandSurfaceHostProjectionEncoder::token(1, b"conflict-target", changed, style)
            .expect("conflict token");

    assert!(matches!(
        root.synchronize(conflict),
        Err(EguiTextCommandSurfaceRootFactoryError::RevisionConflict { revision: 1 })
    ));
}

#[test]
fn synchronize_with_lease_rejects_duplicate_revision() {
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        3,
        b"lease-target",
        presentation(),
        TextCommandSurfaceStyle::standard().expect("standard style"),
    )
    .expect("token");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(
            EguiTextCommandSurfaceHostProjectionEncoder::token(
                3,
                b"lease-target",
                presentation(),
                TextCommandSurfaceStyle::standard().expect("standard style"),
            )
            .expect("token duplicate"),
        )
        .expect("retain initial");
    let lease = EguiTextCommandSurfaceHostProjectionLease::new(token, |_context| Ok(None));

    assert!(matches!(
        root.synchronize_with_lease(lease),
        Err(EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision: 3 })
    ));
}

#[test]
fn retain_rejects_empty_identity_for_process() {
    assert!(matches!(
        HostRootProcess::retain(
            super::host_root_token_codec::DecodedRootPresentation {
                identity: String::new(),
                presentation: EguiTextCommandSurfacePresentation {
                    text_state_id: None,
                    text: TextSurfacePresentation::from_props(
                        TextSurface::new(TextSurfaceProps::new(
                            katana_ui_core::atom::TextArea::new("empty-id").value("empty"),
                            Vec::<UiTextSpan>::new(),
                            TextSurfaceViewport::new(0, 0, 240, 120),
                        ))
                        .props()
                    ),
                    toolbar: None,
                    floating: None,
                    search: None,
                    context_menu: None,
                },
                style: TextCommandSurfaceStyle::standard().expect("standard style"),
                command_families: None,
            },
            1,
        ),
        Err(EguiTextCommandSurfaceRootFactoryError::InvalidToken(
            "host target identity is empty"
        ))
    ));
}

#[test]
fn synchronize_with_same_presentation_and_same_revision_is_no_change() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        2,
        b"same-revision-target",
        presentation(),
        style.clone(),
    )
    .expect("same revision token");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("retain initial");

    let same_token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        2,
        b"same-revision-target",
        presentation(),
        style,
    )
    .expect("same revision token");

    assert_eq!(root.synchronize(same_token).expect("synchronize"), false);
}

#[test]
fn synchronize_with_versioned_family_projection_updates_root_projection() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"family-target",
        presentation(),
        style.clone(),
    )
    .expect("legacy token");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("retain initial");
    let family_token = EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
        2,
        b"family-target",
        presentation(),
        style,
        EguiTextCommandSurfaceCommandFamilyProjection {
            primary: Some(
                katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new(
                    "family-primary",
                ),
            ),
            floating: Some(
                katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new(
                    "family-floating",
                ),
            ),
        },
    )
    .expect("family token");

    assert!(root.synchronize(family_token).expect("synchronize family"));
}

#[test]
fn retain_with_lease_rejects_a_versioned_duplicate_family() {
    let family =
        katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new("duplicate-family");
    let toolbar = CommandChromeToolbarPresentation {
        actions: vec![CommandChromeAction::new("primary", "Primary")],
        groups: Vec::new(),
        display_mode: Default::default(),
        density: Default::default(),
        overflow_strategy: Default::default(),
    };
    let mut duplicate_family_presentation = presentation();
    duplicate_family_presentation.toolbar = Some(toolbar.clone());
    duplicate_family_presentation.floating = Some(
        crate::text_command_surface::EguiTextCommandSurfaceFloatingPresentation {
            toolbar,
            visibility: FloatingCommandToolbarVisibility::Visible,
        },
    );
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
        1,
        b"duplicate-family-lease-target",
        duplicate_family_presentation,
        TextCommandSurfaceStyle::standard().expect("standard style"),
        EguiTextCommandSurfaceCommandFamilyProjection::new(Some(family.clone()), Some(family)),
    )
    .expect("versioned token");

    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(EguiTextCommandSurfaceHostProjectionLease::new(
            token,
            |_context| Ok(None),
        ))
        .expect("retain with versioned duplicate family token");
    let mut result = None;
    let context = egui::Context::default();
    let mut output = context.run_ui(
        RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..RawInput::default()
        },
        |ui| result = Some(root.show(ui)),
    );
    output.textures_delta.clear();
    match result.expect("render result") {
        Err(EguiTextCommandSurfaceRootFactoryError::Root(message)) => {
            assert!(
                message.contains("command family is mounted"),
                "unexpected root error: {message}"
            );
        }
        Err(error) => panic!("unexpected render error: {error}"),
        Ok(_) => panic!("duplicate family must fail closed during rendering"),
    }
}

#[test]
fn synchronize_with_lease_attaches_source_address_tab_strip_and_aux_lease_slots() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let token = EguiTextCommandSurfaceHostProjectionEncoder::token(
        1,
        b"router-slot-target",
        presentation(),
        style.clone(),
    )
    .expect("router lease token");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(token)
        .expect("retain initial");

    let status_bar = StatusBar::new("status");
    let diagnostics = DiagnosticsList::new("diagnostics");
    let source_address = SourceAddressProjectionLease::new(SourceAddressStrip::new(
        SourceAddressPresentation::new("source-address", "show source address", "アクセシビリティ"),
    ));
    let tab_strip = TabStripProjectionLease::new(
        TabStripProjection::new(
            2,
            TabStripCorrelation::from_opaque_bytes("tab-strip-correlation"),
        )
        .navigation(
            TabStripNavigationPresentation::new(
                TabStripControlPresentation::new(
                    TabStripText::new("prev"),
                    TabStripText::new("prev-a11y"),
                ),
                TabStripControlPresentation::new(
                    TabStripText::new("next"),
                    TabStripText::new("next-a11y"),
                ),
            )
            .overflow(TabStripControlPresentation::new(
                TabStripText::new("more"),
                TabStripText::new("more-a11y"),
            )),
        )
        .scroll_presentation(TabStripScrollPresentation::default().request_active_reveal(true)),
    );
    let status = StatusDiagnosticsProjectionLease::new()
        .with_status_bar(status_bar.segment(StatusBarSegment::new("status-segment", "status")))
        .with_diagnostics_list(diagnostics);
    let editor_viewport = EditorViewportProjectionLease::new(
        katana_ui_core::render_model::UiImageSurfaceProps::new(
            "preview",
            1,
            1,
            vec![255, 0, 0, 255],
        )
        .expect("editor preview"),
    )
    .with_split_ratio_percent(50)
    .expect("split ratio");
    let lease = EguiTextCommandSurfaceHostProjectionLease::new(
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            2,
            b"router-slot-target",
            presentation(),
            TextCommandSurfaceStyle::standard().expect("standard style"),
        )
        .expect("updated lease token"),
        |_context| Ok(None),
    )
    .with_source_address(source_address)
    .with_tab_strip(tab_strip)
    .with_status_diagnostics(status)
    .with_editor_viewport(editor_viewport);

    assert!(root.synchronize_with_lease(lease).is_ok());
}
