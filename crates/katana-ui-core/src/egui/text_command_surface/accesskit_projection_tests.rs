use super::{AccessKitTextInputBounds, AccessKitTextInputNode, AccessKitTextInputRole};

fn node(role: egui::accesskit::Role, value: Option<&str>) -> egui::accesskit::Node {
    let mut node = egui::accesskit::Node::new(role);
    if let Some(value) = value {
        node.set_value(value);
    }
    node
}

#[test]
fn actual_accesskit_node_projection_reads_setter_results() {
    let mut actual = node(egui::accesskit::Role::MultilineTextInput, Some("⭐️入力"));
    actual.set_bounds(egui::accesskit::Rect {
        x0: 1.0,
        y0: 2.0,
        x1: 3.0,
        y1: 4.0,
    });

    let projection = AccessKitTextInputNode::from_accesskit_node(&actual);

    assert!(projection.is_text_input());
    assert_eq!(projection.role, AccessKitTextInputRole::MultilineTextInput);
    assert_eq!(projection.value.as_deref(), Some("⭐️入力"));
    assert_eq!(
        projection.scalar_sequence,
        vec![0x2b50, 0xfe0f, 0x5165, 0x529b]
    );
    assert_eq!(
        projection.bounds,
        Some(AccessKitTextInputBounds {
            x0_bits: 1.0_f64.to_bits(),
            y0_bits: 2.0_f64.to_bits(),
            x1_bits: 3.0_f64.to_bits(),
            y1_bits: 4.0_f64.to_bits(),
        })
    );
}

#[test]
fn actual_accesskit_node_projection_rejects_non_text_and_missing_properties() {
    let text =
        AccessKitTextInputNode::from_accesskit_node(&node(egui::accesskit::Role::TextInput, None));
    let other = AccessKitTextInputNode::from_accesskit_node(&node(
        egui::accesskit::Role::Button,
        Some("button"),
    ));

    assert!(text.is_text_input());
    assert_eq!(text.value, None);
    assert!(text.scalar_sequence.is_empty());
    assert_eq!(text.bounds, None);
    assert!(!other.is_text_input());
    assert_eq!(other.role, AccessKitTextInputRole::Other);
}

#[test]
fn text_input_projection_hash_binds_role_value_scalars_and_bounds() {
    let node = AccessKitTextInputNode {
        role: AccessKitTextInputRole::TextInput,
        value: Some("入力".into()),
        scalar_sequence: vec![0x5165, 0x529b],
        bounds: Some(AccessKitTextInputBounds {
            x0_bits: 0.0_f64.to_bits(),
            y0_bits: 0.0_f64.to_bits(),
            x1_bits: 1.0_f64.to_bits(),
            y1_bits: 1.0_f64.to_bits(),
        }),
    };
    let mut changed = node.clone();
    changed.scalar_sequence.push(0x20);
    let mut multiline = node.clone();
    multiline.role = AccessKitTextInputRole::MultilineTextInput;
    let mut other = node.clone();
    other.role = AccessKitTextInputRole::Other;
    let missing = AccessKitTextInputNode {
        role: AccessKitTextInputRole::TextInput,
        value: None,
        scalar_sequence: Vec::new(),
        bounds: None,
    };
    let escaped = AccessKitTextInputNode {
        value: Some("\"\\\n入力".into()),
        scalar_sequence: vec![0x22, 0x5c, 0xa, 0x5165, 0x529b],
        ..node.clone()
    };

    assert_ne!(node.snapshot_hash(), changed.snapshot_hash());
    assert_ne!(node.snapshot_hash(), multiline.snapshot_hash());
    assert_ne!(node.snapshot_hash(), other.snapshot_hash());
    assert!(!missing.snapshot_hash().is_empty());
    assert_eq!(escaped.snapshot_hash(), escaped.clone().snapshot_hash());
}
