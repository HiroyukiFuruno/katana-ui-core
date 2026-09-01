use super::{SanitizedDocumentRootIdentity, SanitizedDocumentRootInput};
use crate::egui::text_command_surface::{
    SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
    SanitizedCommandTarget, SanitizedDocumentRootStyleKey,
};

#[test]
fn different_optional_projection_presence_is_not_equal() {
    let plain = SanitizedDocumentRootInput::new(
        1,
        SanitizedDocumentRootIdentity::from_opaque_bytes([1]),
        "snapshot",
        SanitizedDocumentRootStyleKey::Default,
    );
    let with_command = SanitizedDocumentRootInput::new(
        1,
        SanitizedDocumentRootIdentity::from_opaque_bytes([1]),
        "snapshot",
        SanitizedDocumentRootStyleKey::Default,
    )
    .with_command_projection(SanitizedCommandProjection::new([
        SanitizedCommandGroup::new(1, "group").item(SanitizedCommandItem::new(
            SanitizedCommandTarget::from_opaque_bytes([2]),
            1,
            "item",
        )),
    ]));

    assert!(!plain.same_command_projection_as(&with_command));
}
