use super::hash_serialized;

#[test]
fn root_record_hash_serialization_fails_closed_for_unsupported_json_map_keys() {
    let unsupported_json_key = std::collections::BTreeMap::from([((1_u8, 2_u8), 3_u8)]);

    let error = hash_serialized(&unsupported_json_key)
        .expect_err("JSON object keys must not silently accept tuple semantics");

    assert!(error.contains("key must be a string"));
}
