use super::*;

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
