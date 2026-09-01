#[test]
fn public_record_has_no_child_payload_or_pixel_storage() {
    let source = include_str!("sanitized_document_root_record.rs");
    let public = source
        .split_once("pub struct SanitizedDocumentRootRecord")
        .expect("record declaration exists")
        .1
        .split_once("impl SanitizedDocumentRootRecord")
        .expect("record implementation exists")
        .0;
    for forbidden in [
        "child_geometry",
        "payload",
        "rgba_pixels",
        "texture",
        "accesskit_nodes",
    ] {
        assert!(!public.contains(forbidden), "record leaked `{forbidden}`");
    }
}
