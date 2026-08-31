use super::{AccessKitTargetClass, hash_serialized, target_class_name};

#[test]
fn status_bar_segment_and_diagnostics_fix_class_names_are_mapped() {
    assert_eq!(
        "status-bar-segment",
        target_class_name(AccessKitTargetClass::StatusBarSegment)
    );
    assert_eq!(
        "diagnostics-fix",
        target_class_name(AccessKitTargetClass::DiagnosticsFix)
    );
}

#[test]
fn snapshot_hash_serialization_fails_closed_for_unsupported_json_map_keys() {
    let unsupported_json_key = std::collections::BTreeMap::from([((1_u8, 2_u8), 3_u8)]);

    let error = hash_serialized(&unsupported_json_key)
        .expect_err("JSON object keys must not silently accept tuple semantics");

    assert!(error.contains("key must be a string"));
}
