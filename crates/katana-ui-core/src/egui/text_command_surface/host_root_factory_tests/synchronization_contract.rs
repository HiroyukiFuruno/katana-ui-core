use super::*;
use crate::egui::text_command_surface::TabStripProjectionLease;

#[test]
fn preserve_state_lease_advances_a_matching_root_revision() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(
            EguiTextCommandSurfaceHostProjectionEncoder::token(
                3,
                b"preserve-state-success-target",
                presentation(),
                style.clone(),
            )
            .expect("initial token"),
        )
        .expect("retain initial root");
    let next = EguiTextCommandSurfaceHostProjectionLease::new(
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            4,
            b"preserve-state-success-target",
            presentation(),
            style,
        )
        .expect("next lease token"),
        |_context| Ok(None),
    );

    assert!(
        !root
            .synchronize_with_lease_preserving_state(next)
            .expect("matching pre-issued lease advances")
    );
    assert_eq!(root.process.presentation_revision(), 4);
}

#[test]
fn preserve_state_lease_attaches_tab_strip_without_resetting_text_state() {
    let style = TextCommandSurfaceStyle::standard().expect("standard style");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain(
            EguiTextCommandSurfaceHostProjectionEncoder::token(
                3,
                b"preserve-state-with-tabs-target",
                presentation(),
                style.clone(),
            )
            .expect("initial token"),
        )
        .expect("retain initial root");
    let before = root
        .process
        .root
        .selection_surface()
        .state()
        .text_area
        .value
        .clone();
    let tab_strip = TabStripProjectionLease::new(TabStripProjection::new(
        9,
        TabStripCorrelation::from_opaque_bytes(b"preserve-state-tabs"),
    ));
    let mut stale_presentation = presentation();
    stale_presentation.text.value = "stale lease value".to_owned();
    let next = EguiTextCommandSurfaceHostProjectionLease::new(
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            4,
            b"preserve-state-with-tabs-target",
            stale_presentation,
            style,
        )
        .expect("next lease token"),
        |_context| Ok(None),
    )
    .with_tab_strip(tab_strip);

    assert!(
        root.synchronize_with_lease_preserving_state(next)
            .expect("tab strip lease should synchronize")
    );
    assert_eq!(root.process.presentation_revision(), 4);
    assert_eq!(
        root.process
            .root
            .selection_surface()
            .state()
            .text_area
            .value,
        before
    );
    assert!(root.process.root.clear_tab_strip());
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
