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

#[path = "host_root_factory_tests.rs"]
mod factory_tests;
#[path = "host_root_lease_tests.rs"]
mod lease_tests;

#[test]
fn host_root_factory_error_variants_are_rendered_and_debuggable() {
    let invalid_token = EguiTextCommandSurfaceRootFactoryError::InvalidToken("unknown root format");
    assert_eq!(
        invalid_token.to_string(),
        "invalid root token: unknown root format"
    );
    assert!(format!("{invalid_token:?}").contains("InvalidToken"));

    let identity_changed = EguiTextCommandSurfaceRootFactoryError::IdentityChanged;
    assert_eq!(
        identity_changed.to_string(),
        "root identity cannot change while retained"
    );
    assert!(format!("{identity_changed:?}").contains("IdentityChanged"));

    let stale_revision = EguiTextCommandSurfaceRootFactoryError::StaleRevision {
        current: 7,
        received: 3,
    };
    assert_eq!(
        stale_revision.to_string(),
        "stale root presentation revision 3; current is 7"
    );
    assert!(format!("{stale_revision:?}").contains("StaleRevision"));

    let revision_conflict =
        EguiTextCommandSurfaceRootFactoryError::RevisionConflict { revision: 9 };
    assert_eq!(
        revision_conflict.to_string(),
        "root presentation revision 9 was already retained"
    );
    assert!(format!("{revision_conflict:?}").contains("RevisionConflict"));

    let decode = EguiTextCommandSurfaceRootFactoryError::Decode("corrupt token".to_string());
    assert_eq!(
        decode.to_string(),
        "root presentation token decode failed: corrupt token"
    );
    assert!(format!("{decode:?}").contains("Decode"));

    let root = EguiTextCommandSurfaceRootFactoryError::Root("upstream error".to_string());
    assert_eq!(root.to_string(), "upstream error");
    assert!(format!("{root:?}").contains("Root"));

    let opaque_host_effect = EguiTextCommandSurfaceRootFactoryError::OpaqueHostEffect;
    assert_eq!(
        opaque_host_effect.to_string(),
        "opaque host effect router failed"
    );
    assert!(format!("{opaque_host_effect:?}").contains("OpaqueHostEffect"));

    let opaque_host_effect_rejected =
        EguiTextCommandSurfaceRootFactoryError::OpaqueHostEffectRejected;
    assert_eq!(
        opaque_host_effect_rejected.to_string(),
        "opaque host effect batch was rejected"
    );
    assert!(format!("{opaque_host_effect_rejected:?}").contains("OpaqueHostEffectRejected"));

    let duplicate_lease = EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision: 11 };
    assert_eq!(
        duplicate_lease.to_string(),
        "root lease revision 11 was already consumed"
    );
    assert!(format!("{duplicate_lease:?}").contains("DuplicateLease"));

    let surface_conversion = EguiTextCommandSurfaceRootFactoryError::from(
        crate::text_command_surface::EguiTextCommandSurfaceError::DuplicateCommandFamilyMount {
            family: katana_ui_core::molecule::command_chrome::CommandChromeFamilyId::new(
                "duplicate",
            ),
        },
    );
    assert_eq!(
        surface_conversion.to_string(),
        "command family is mounted in both primary and floating slots"
    );

    let root_conversion = EguiTextCommandSurfaceRootFactoryError::from(
        crate::text_command_surface::EguiTextCommandSurfaceRootError::Serialization(
            "root serialization".to_owned(),
        ),
    );
    assert_eq!(
        root_conversion.to_string(),
        "text-command root serialization failed: root serialization"
    );
}
