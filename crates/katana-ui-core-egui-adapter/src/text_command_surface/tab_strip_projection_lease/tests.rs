use super::*;
use katana_ui_core::molecule::RgbaColor;

#[test]
fn nested_descriptors_and_opaque_choices_are_constructible() {
    let child = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"child-group"),
        TabStripText::new("子グループ"),
    )
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"child-tab"),
            TabStripText::new("⭐️"),
        )
        .capabilities(TabStripTabCapabilities::new().selectable(true)),
    );
    let projection =
        TabStripProjection::new(7, TabStripCorrelation::from_opaque_bytes(b"correlation")).group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"root-group"),
                TabStripText::new("ルート"),
            )
            .group(child),
        );
    let lease = TabStripProjectionLease::new(projection);
    let debug = format!("{lease:?}");
    assert_eq!(debug, "TabStripProjectionLease(..)");
    let _swatch = TabStripSwatchTarget::from_opaque_bytes(b"swatch-token");
}

#[test]
fn opaque_debug_does_not_reveal_payload_or_presentation() {
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"secret-correlation"),
    )
    .tab(TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"secret-target"),
        TabStripText::new("秘密のタブ"),
    ));
    let debug = format!("{projection:?}");
    for forbidden in ["secret-correlation", "secret-target", "秘密のタブ"] {
        assert!(!debug.contains(forbidden), "debug leaked {forbidden}");
    }
}

#[test]
fn overlay_projection_keeps_menu_popup_and_swatch_routes_non_wire() {
    let swatch = TabStripSwatchDescriptor::new(
        TabStripSwatchTarget::from_opaque_bytes(b"secret-swatch"),
        RgbaColor::new(74, 144, 217, 255),
    )
    .selected(true)
    .accessibility_label(TabStripText::new("青 ⭐️"));
    let menu = TabStripContextMenuPresentation::new().entry(
        TabStripMenuEntry::submenu(
            TabStripText::new("グループ"),
            TabStripText::new("グループ操作"),
        )
        .child(TabStripMenuEntry::action(
            TabStripText::new("追加"),
            TabStripText::new("グループへ追加"),
            TabStripMenuOperation::MoveToGroup(TabStripGroupTarget::from_opaque_bytes(
                b"secret-group",
            )),
        )),
    );
    let popup = TabStripGroupPopupPresentation::new()
        .rename_placeholder(TabStripText::new("グループ名"))
        .entry(TabStripMenuEntry::action(
            TabStripText::new("解除"),
            TabStripText::new("グループを解除"),
            TabStripMenuOperation::Ungroup,
        ));
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"secret-correlation"),
    )
    .tab(
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"secret-tab"),
            TabStripText::new("秘密のタブ"),
        )
        .context_menu(menu),
    )
    .group(
        TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"secret-popup-group"),
            TabStripText::new("秘密のグループ"),
        )
        .swatch(swatch)
        .popup(popup),
    );

    let debug = format!("{projection:?}");
    for forbidden in [
        "secret-correlation",
        "secret-tab",
        "secret-group",
        "secret-swatch",
        "秘密のタブ",
        "秘密のグループ",
        "グループ操作",
        "青 ⭐️",
    ] {
        assert!(!debug.contains(forbidden), "debug leaked {forbidden}");
    }
}

#[test]
fn production_source_has_no_wire_or_legacy_boundary_symbols() {
    let source = include_str!("../tab_strip_projection_lease.rs");
    let production = source
        .split("#[cfg(test)]")
        .next()
        .expect("test module marker");
    for forbidden in [
        "Workspace",
        "CloseableTab",
        "SanitizedTab",
        "Serialize",
        "Deserialize",
        "derive(Clone)",
        "payload(",
        "fn target",
        "fn label",
        "fn order",
        "PathBuf",
    ] {
        assert!(
            !production.contains(forbidden),
            "legacy/wire symbol: {forbidden}"
        );
    }
}

#[test]
fn builders_cover_optional_controls_capabilities_and_nested_routes() {
    let control =
        TabStripControlPresentation::new(TabStripText::new("tip"), TabStripText::new("a11y"));
    let navigation = TabStripNavigationPresentation::new(
        control,
        TabStripControlPresentation::new(TabStripText::new("next"), TabStripText::new("next a11y")),
    )
    .overflow(TabStripControlPresentation::new(
        TabStripText::new("more"),
        TabStripText::new("more a11y"),
    ));
    let tab_capabilities = TabStripTabCapabilities::new()
        .active(true)
        .dirty(true)
        .pinned(true)
        .selectable(true)
        .closeable(true)
        .draggable(true)
        .accepts_tab_drop(true)
        .groupable(true)
        .virtual_tab(true);
    let group_capabilities = TabStripGroupCapabilities::new()
        .collapsed(true)
        .collapsible(true)
        .menu_available(true)
        .renamable(true)
        .recolorable(true)
        .closeable(true)
        .ungroupable(true)
        .draggable(true)
        .accepts_tab_drop(true);
    let surface_capabilities = TabStripSurfaceCapabilities::new()
        .previous_available(true)
        .next_available(true)
        .overflow_available(true)
        .restore_available(true)
        .create_group_available(true)
        .tab_drop_at_end_available(true);
    let menu = TabStripContextMenuPresentation::new()
        .entry(TabStripMenuEntry::separator())
        .entry(
            TabStripMenuEntry::action(
                TabStripText::new("close"),
                TabStripText::new("close a11y"),
                TabStripMenuOperation::RequestClose,
            )
            .enabled(false)
            .checked(true),
        );
    let popup = TabStripGroupPopupPresentation::new()
        .rename_placeholder(TabStripText::new("rename"))
        .entry(TabStripMenuEntry::submenu(
            TabStripText::new("sub"),
            TabStripText::new("sub a11y"),
        ));
    let projection = TabStripProjection::new(3, TabStripCorrelation::from_opaque_bytes([1]))
        .capabilities(surface_capabilities)
        .navigation(navigation)
        .scroll_presentation(TabStripScrollPresentation::new().request_active_reveal(true))
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes([2]),
                TabStripText::new("tab"),
            )
            .tooltip(TabStripText::new("tooltip"))
            .accessibility_label(TabStripText::new("tab a11y"))
            .capabilities(tab_capabilities)
            .trailing_control(TabStripControlPresentation::new(
                TabStripText::new("close tip"),
                TabStripText::new("close a11y"),
            ))
            .context_menu(menu),
        )
        .group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes([3]),
                TabStripText::new("group"),
            )
            .accessibility_label(TabStripText::new("group a11y"))
            .capabilities(group_capabilities)
            .swatch(TabStripSwatchDescriptor::new(
                TabStripSwatchTarget::from_opaque_bytes([4]),
                RgbaColor::new(1, 2, 3, 255),
            ))
            .popup(popup),
        );

    let debug = format!("{projection:?}");
    assert!(debug.contains("TabStripProjection"));
    assert_eq!(
        format!("{:?}", TabStripScrollPresentation::new()),
        "TabStripScrollPresentation { request_active_reveal: false }"
    );
    assert_eq!(
        format!("{:?}", TabStripTabCapabilities::new()),
        "TabStripTabCapabilities { active: false, dirty: false, pinned: false, selectable: false, closeable: false, draggable: false, accepts_tab_drop: false, groupable: false, virtual_tab: false }"
    );
}

#[test]
fn target_copy_is_deep_for_route_and_same_target_checks_payload_bytes() {
    let target = TabStripTabTarget::from_opaque_bytes(b"tab-a");
    let copied = target.copy_for_route();
    let same_payload = copied.same_target(&TabStripTabTarget::from_opaque_bytes(b"tab-a"));
    let different_payload = copied.same_target(&TabStripTabTarget::from_opaque_bytes(b"tab-b"));
    assert!(same_payload);
    assert!(!different_payload);
    assert_ne!(target.payload.as_ptr(), copied.payload.as_ptr());
}

#[test]
fn group_and_swatch_copy_preserve_payload_without_exposing_bytes() {
    let group = TabStripGroupTarget::from_opaque_bytes([1u8, 2, 3]);
    let swatch = TabStripSwatchTarget::from_opaque_bytes(vec![9, 9, 9]);
    let group_copy = group.copy_for_route();
    let swatch_copy = swatch.copy_for_route();
    assert_eq!(group_copy.payload.as_ref(), group.payload.as_ref());
    assert_ne!(group_copy.payload.as_ptr(), group.payload.as_ptr());
    assert!(
        swatch_copy
            .payload
            .iter()
            .zip(swatch.payload.iter())
            .all(|(copied, source)| copied == source)
    );
    assert_ne!(swatch_copy.payload.as_ptr(), swatch.payload.as_ptr());
}

#[test]
fn menu_entry_separator_and_chain_calls_are_strictly_localized() {
    let submenu =
        TabStripMenuEntry::submenu(TabStripText::new("submenu"), TabStripText::new("a11y"))
            .child(TabStripMenuEntry::action(
                TabStripText::new("child"),
                TabStripText::new("child a11y"),
                TabStripMenuOperation::CloseAll,
            ))
            .enabled(false)
            .checked(true);
    let separator = TabStripMenuEntry::separator();
    let context_menu = TabStripContextMenuPresentation::new()
        .entry(submenu)
        .entry(separator)
        .entry(TabStripMenuEntry::action(
            TabStripText::new("action"),
            TabStripText::new("action a11y"),
            TabStripMenuOperation::RequestClose,
        ));
    assert_eq!(context_menu.entries.len(), 3);
    assert!(context_menu.entries[1].separator);
    assert!(!context_menu.entries[1].enabled);
    assert!(!context_menu.entries[1].checked);
    assert!(context_menu.entries[1].operation.is_none());
    assert_eq!(context_menu.entries[1].children.len(), 0);
}

#[test]
fn debug_impls_do_not_reveal_inner_payload_lengths() {
    let tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes([1u8, 2, 3, 4]),
        TabStripText::new("sensitive-tab-label-314159"),
    )
    .tooltip(TabStripText::new("sensitive-tab-tooltip-271828"))
    .accessibility_label(TabStripText::new("sensitive-tab-a11y-161803"));
    let group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes([9u8, 8, 7]),
        TabStripText::new("sensitive-group-label-141421"),
    )
    .accessibility_label(TabStripText::new("sensitive-group-a11y-173205"));
    let swatch = TabStripSwatchDescriptor::new(
        TabStripSwatchTarget::from_opaque_bytes([3u8, 2, 1]),
        RgbaColor::new(1, 2, 3, 4),
    )
    .accessibility_label(TabStripText::new("swatch"));

    let tab_debug = format!("{tab:?}");
    assert!(tab_debug.starts_with("TabStripTabDescriptor {"));
    assert!(tab_debug.contains("label: \"<opaque>\""));
    for secret in [
        "sensitive-tab-label-314159",
        "sensitive-tab-tooltip-271828",
        "sensitive-tab-a11y-161803",
    ] {
        assert!(!tab_debug.contains(secret));
    }
    let group_debug = format!("{group:?}");
    assert!(group_debug.starts_with("TabStripGroupDescriptor {"));
    assert!(group_debug.contains("label: \"<opaque>\""));
    for secret in [
        "sensitive-group-label-141421",
        "sensitive-group-a11y-173205",
    ] {
        assert!(!group_debug.contains(secret));
    }
    assert_eq!(format!("{:?}", swatch), "TabStripSwatchDescriptor(..)");
    assert_eq!(format!("{:?}", &tab.target), "TabStripTabTarget(..)");
    assert_eq!(format!("{:?}", &group.target), "TabStripGroupTarget(..)");
    assert_eq!(format!("{:?}", &swatch.target), "TabStripSwatchTarget(..)");
    assert_eq!(
        format!("{:?}", TabStripText::new("value")),
        "TabStripText(..)"
    );
}

#[test]
fn navigation_and_control_formats_use_opaque_debug_shapes() {
    let control = TabStripControlPresentation::new(
        TabStripText::new("control-tooltip-secret"),
        TabStripText::new("control-a11y-secret"),
    );
    assert_eq!(
        format!("{control:?}"),
        "TabStripControlPresentation(..)"
    );
    let previous = TabStripControlPresentation::new(
        TabStripText::new("prev-tooltip"),
        TabStripText::new("prev-a11y"),
    );
    let next = TabStripControlPresentation::new(
        TabStripText::new("next-tooltip"),
        TabStripText::new("next-a11y"),
    );
    let navigation = TabStripNavigationPresentation::new(previous, next).overflow(
        TabStripControlPresentation::new(
            TabStripText::new("overflow-tooltip"),
            TabStripText::new("overflow-a11y"),
        ),
    );
    assert_eq!(
        format!("{:?}", &navigation),
        "TabStripNavigationPresentation(..)"
    );
}

#[test]
fn menu_entry_and_context_layout_debug_paths_are_covered() {
    let entry = TabStripMenuEntry::action(
        TabStripText::new("rename"),
        TabStripText::new("rename-a11y"),
        TabStripMenuOperation::RequestClose,
    )
    .checked(true);
    let menu = TabStripContextMenuPresentation::new().entry(entry);

    assert_eq!(
        format!("{:?}", &menu),
        "TabStripContextMenuPresentation(..)"
    );
    assert_eq!(format!("{:?}", menu.entries[0]), "TabStripMenuEntry(..)");
}

#[test]
fn popup_and_group_debug_variants_are_exercised() {
    let popup = TabStripGroupPopupPresentation::new()
        .rename_placeholder(TabStripText::new("group"))
        .entry(
            TabStripMenuEntry::submenu(TabStripText::new("sub"), TabStripText::new("sub-a11y"))
                .child(TabStripMenuEntry::action(
                    TabStripText::new("leaf"),
                    TabStripText::new("leaf-a11y"),
                    TabStripMenuOperation::CloseAll,
                )),
        );
    let mut tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes([1]),
        TabStripText::new("tab"),
    )
    .trailing_control(TabStripControlPresentation::new(
        TabStripText::new("tip"),
        TabStripText::new("tip-a11y"),
    ))
    .context_menu(
        TabStripContextMenuPresentation::new().entry(TabStripMenuEntry::action(
            TabStripText::new("close"),
            TabStripText::new("close-a11y"),
            TabStripMenuOperation::RequestClose,
        )),
    );
    assert_eq!(
        format!("{:?}", &popup),
        "TabStripGroupPopupPresentation(..)"
    );

    let group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes([2]),
        TabStripText::new("group"),
    )
    .popup(popup);

    tab.trailing_control = Some(TabStripControlPresentation::new(
        TabStripText::new("close"),
        TabStripText::new("close-tip"),
    ));
    tab.context_menu = Some(TabStripContextMenuPresentation::new().entry(
        TabStripMenuEntry::action(
            TabStripText::new("close"),
            TabStripText::new("close"),
            TabStripMenuOperation::CloseAll,
        ),
    ));

    let tab_debug = format!("{tab:?}");
    assert!(tab_debug.starts_with("TabStripTabDescriptor {"));
    for secret in ["tip-a11y", "close-tip", "close-a11y"] {
        assert!(!tab_debug.contains(secret));
    }
    let group_debug = format!("{group:?}");
    assert!(group_debug.starts_with("TabStripGroupDescriptor {"));
    for secret in ["sub-a11y", "leaf-a11y"] {
        assert!(!group_debug.contains(secret));
    }
}

#[test]
fn popup_and_context_builders_execute_debug_paths() {
    let mut context_menu = TabStripContextMenuPresentation::new();
    context_menu = context_menu.entry(TabStripMenuEntry::separator());
    assert_eq!(context_menu.entries.len(), 1);
    assert_eq!(
        format!("{:?}", context_menu),
        "TabStripContextMenuPresentation(..)"
    );

    let mut group_popup = TabStripGroupPopupPresentation::new();
    group_popup = group_popup
        .rename_placeholder(TabStripText::new("rename"))
        .entry(
            TabStripMenuEntry::submenu(
                TabStripText::new("submenu"),
                TabStripText::new("submenu a11y"),
            )
            .child(TabStripMenuEntry::action(
                TabStripText::new("reveal"),
                TabStripText::new("reveal a11y"),
                TabStripMenuOperation::Ungroup,
            )),
        );
    assert_eq!(
        format!("{:?}", group_popup),
        "TabStripGroupPopupPresentation(..)"
    );

    let swatch = TabStripSwatchDescriptor::new(
        TabStripSwatchTarget::from_opaque_bytes(b"target"),
        RgbaColor::new(1, 2, 3, 4),
    )
    .selected(true)
    .accessibility_label(TabStripText::new("label"));
    assert_eq!(format!("{:?}", swatch), "TabStripSwatchDescriptor(..)");
}
