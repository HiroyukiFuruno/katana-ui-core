use super::{
    ContextMenuCapability, SHA256_SIGNATURE_LENGTH, SanitizedContextMenuActivationTransport,
    SanitizedContextMenuCapabilityRejection, SanitizedContextMenuItem,
    SanitizedContextMenuProjection,
};
use crate::egui::text_command_surface::SanitizedContextMenuTarget;
use crate::molecule::selection::ContextMenuEvent;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn reentrant_capability_is_rejected_without_consuming_callback() {
    let capability: ContextMenuCapability = Rc::new(RefCell::new(Some(Box::new(|| Ok(())))));
    let _guard = capability.borrow_mut();
    let mut event = SanitizedContextMenuActivationTransport {
        target: super::SanitizedContextMenuRoutedTarget {
            signature: [0; SHA256_SIGNATURE_LENGTH],
            capability: Some(capability.clone()),
        },
        revision: 1,
        correlation: "opaque".to_owned(),
    };

    assert_eq!(
        event.invoke_once(),
        Err(SanitizedContextMenuCapabilityRejection::Reentrant)
    );
}

fn context_menu_command_id(target: &SanitizedContextMenuTarget) -> String {
    let mut digest = Sha256::new();
    digest.update((target.opaque().len() as u64).to_le_bytes());
    digest.update(target.opaque());
    format!(
        concat!("kuc-context-menu-", "{}"),
        hex::encode(digest.finalize())
    )
}

#[test]
fn route_context_menu_events_supports_enabled_leaf_activation() {
    let called = Rc::new(RefCell::new(false));
    let leaf = SanitizedContextMenuTarget::from_opaque_bytes([1, 2, 3]).with_unit_capability({
        let called = called.clone();
        move || {
            *called.borrow_mut() = true;
            Ok::<(), ()>(())
        }
    });
    let leaf_id = context_menu_command_id(&leaf);
    let projection =
        SanitizedContextMenuProjection::new([SanitizedContextMenuItem::new(leaf, 0, "item")]);

    let mut transports = super::route_context_menu_events(
        Some(&projection),
        &[ContextMenuEvent::ItemSelected {
            path: vec![0],
            command: leaf_id.clone(),
        }],
        7,
        "root",
    )
    .expect("leaf target should route");

    assert_eq!(transports.len(), 1);
    let debug = format!("{:?}", transports[0]);
    assert_eq!(
        debug,
        "SanitizedContextMenuActivationTransport { payload: \"<opaque>\" }"
    );
    assert!(!debug.contains("root"));
    assert!(!debug.contains(&leaf_id));
    assert_eq!(transports[0].invoke_once(), Ok(()));
    assert!(*called.borrow());
    assert_eq!(
        transports[0].invoke_once(),
        Err(SanitizedContextMenuCapabilityRejection::Missing)
    );
}

#[test]
fn route_context_menu_events_skips_disabled_and_submenu_paths_as_none() {
    let submenu = SanitizedContextMenuItem::new(
        SanitizedContextMenuTarget::from_opaque_bytes([9, 9, 9]),
        1,
        "submenu",
    );
    let parent = SanitizedContextMenuItem::new(
        SanitizedContextMenuTarget::from_opaque_bytes([1, 2, 3]),
        0,
        "parent",
    )
    .submenu_item(submenu);
    let leaf_projection = SanitizedContextMenuProjection::new([parent]);
    let parent_id = context_menu_command_id(leaf_projection.items()[0].target());

    let transports = super::route_context_menu_events(
        Some(&leaf_projection),
        &[ContextMenuEvent::ItemSelected {
            path: vec![0],
            command: parent_id,
        }],
        1,
        "root",
    )
    .expect("submenu target is routed as None");
    assert!(transports.is_empty());
}

#[test]
fn route_context_menu_events_returns_missing_for_invalid_paths_or_capabilities() {
    let without_callback = SanitizedContextMenuProjection::new([SanitizedContextMenuItem::new(
        SanitizedContextMenuTarget::from_opaque_bytes([4, 5, 6]),
        0,
        "item",
    )]);
    let command_id =
        context_menu_command_id(&SanitizedContextMenuTarget::from_opaque_bytes([4, 5, 6]));
    assert!(matches!(
        super::route_context_menu_events(
            Some(&without_callback),
            &[ContextMenuEvent::ItemSelected {
                path: vec![1],
                command: command_id,
            }],
            1,
            "root"
        ),
        Err(SanitizedContextMenuCapabilityRejection::Missing)
    ));

    assert!(matches!(
        super::route_context_menu_events(
            Some(&without_callback),
            &[ContextMenuEvent::ItemSelected {
                path: vec![0],
                command: "wrong".to_owned(),
            }],
            1,
            "root"
        ),
        Err(SanitizedContextMenuCapabilityRejection::Missing)
    ));

    let without_capability = SanitizedContextMenuProjection::new([SanitizedContextMenuItem::new(
        SanitizedContextMenuTarget::from_opaque_bytes([7, 8, 9]),
        0,
        "missing",
    )]);
    let missing_id =
        context_menu_command_id(&SanitizedContextMenuTarget::from_opaque_bytes([7, 8, 9]));
    assert!(matches!(
        super::route_context_menu_events(
            Some(&without_capability),
            &[ContextMenuEvent::ItemSelected {
                path: vec![0],
                command: missing_id,
            }],
            1,
            "root"
        ),
        Err(SanitizedContextMenuCapabilityRejection::Missing)
    ));
}
