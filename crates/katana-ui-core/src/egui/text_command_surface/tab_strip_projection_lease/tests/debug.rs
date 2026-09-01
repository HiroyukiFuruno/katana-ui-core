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
    assert_eq!(format!("{:?}", tab.target), "TabStripTabTarget(..)");
    assert_eq!(format!("{:?}", group.target), "TabStripGroupTarget(..)");
    assert_eq!(format!("{:?}", swatch.target), "TabStripSwatchTarget(..)");
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
    assert_eq!(format!("{control:?}"), "TabStripControlPresentation(..)");
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
        format!("{:?}", navigation),
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
        format!("{:?}", menu),
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
        format!("{:?}", popup),
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
