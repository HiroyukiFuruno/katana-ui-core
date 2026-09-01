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
    assert!(swatch_copy
        .payload
        .iter()
        .zip(swatch.payload.iter())
        .all(|(copied, source)| copied == source));
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
