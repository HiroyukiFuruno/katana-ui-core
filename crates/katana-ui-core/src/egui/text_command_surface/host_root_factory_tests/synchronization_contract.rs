use super::*;

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
                            crate::atom::TextArea::new("empty-id").value("empty"),
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
            primary: Some(crate::molecule::command_chrome::CommandChromeFamilyId::new(
                "family-primary",
            )),
            floating: Some(crate::molecule::command_chrome::CommandChromeFamilyId::new(
                "family-floating",
            )),
        },
    )
    .expect("family token");

    assert!(root.synchronize(family_token).expect("synchronize family"));
}
