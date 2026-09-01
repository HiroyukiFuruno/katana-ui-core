#[test]
fn opaque_group_target_debug_does_not_reveal_payload() {
    let target = SanitizedTabGroupTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]);

    assert_eq!(format!("{target:?}"), "SanitizedTabGroupTarget(..)");
}
