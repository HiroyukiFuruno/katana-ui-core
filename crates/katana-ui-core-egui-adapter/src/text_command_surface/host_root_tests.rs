use super::*;
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};

const VIEWPORT_WIDTH: u32 = 320;
const VIEWPORT_HEIGHT: u32 = 180;

pub(super) fn presentation() -> EguiTextCommandSurfacePresentation {
    let surface = TextSurface::new(TextSurfaceProps::new(
        katana_ui_core::atom::TextArea::new("host-root-unit").value("opaque text"),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
    ));
    EguiTextCommandSurfacePresentation {
        text_state_id: None,
        text: TextSurfacePresentation::from_props(surface.props()),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: None,
    }
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

    let encoder = <EguiTextCommandSurfaceHostProjectionEncoder as Default>::default();
    assert!(
        encoder
            .encode(
                1,
                b"legacy-encode".to_vec(),
                presentation(),
                TextCommandSurfaceStyle::standard()
            )
            .is_ok()
    );
    assert!(
        EguiTextCommandSurfaceHostProjectionEncoder::token(
            2,
            b"legacy-token".to_vec(),
            presentation(),
            TextCommandSurfaceStyle::standard()
        )
        .is_ok()
    );
    assert!(
        encoder
            .encode_with_command_families(
                3,
                b"family-encode".to_vec(),
                presentation(),
                TextCommandSurfaceStyle::standard(),
                EguiTextCommandSurfaceCommandFamilyProjection::default()
            )
            .is_ok()
    );
    assert!(
        EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
            4,
            b"family-token".to_vec(),
            presentation(),
            TextCommandSurfaceStyle::standard(),
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
