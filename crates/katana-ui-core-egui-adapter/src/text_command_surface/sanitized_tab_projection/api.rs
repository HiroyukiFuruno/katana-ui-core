#[test]
fn public_api_is_constructor_only_and_stays_generic() {
    let entry_source = include_str!("../sanitized_tab_projection.rs");
    let entry_api_source = entry_source
        .split("#[cfg(test)]")
        .next()
        .unwrap_or(entry_source);
    let api_source = [
        entry_api_source,
        include_str!("../sanitized_tab_projection/types.rs"),
        include_str!("../sanitized_tab_projection/logic_a.rs"),
        include_str!("../sanitized_tab_projection/logic_b.rs"),
    ]
    .join("\n");
    let lower = api_source.to_ascii_lowercase();
    for forbidden in [
        "document",
        "path",
        "markdown",
        "katana_language",
        "katana::",
        "kle",
        "geometry",
        "coordinate",
        "serialize",
        "clone",
        "payload",
        "pub fn stable_fingerprint",
        "pub fn target",
        "pub fn groups",
        "pub fn tabs",
        "pub fn capabilities",
        "pub fn visible_label",
        "pub fn tooltip",
        "pub fn accessibility_label",
    ] {
        assert!(
            !lower.contains(forbidden),
            "forbidden tab projection API term: {forbidden}"
        );
    }

    let public_methods = api_source.lines().filter_map(|line| {
        let declaration = line.trim_start();
        (declaration.starts_with("pub fn ") || declaration.starts_with("pub const fn "))
            .then_some(declaration)
    });
    let allowed = [
        "from_opaque_bytes",
        "new",
        "active_state",
        "dirty_state",
        "pinned_state",
        "close_state",
        "collapse_state",
        "menu_state",
        "rename_state",
        "recolor_state",
        "ungroup_state",
        "drag_state",
        "with_icon",
        "with_capabilities",
        "with_close_presentation",
        "tab",
        "group",
    ];
    for declaration in public_methods {
        assert!(
            allowed
                .iter()
                .any(|name| declaration.contains(&format!("fn {name}"))),
            "unexpected public tab projection method: {declaration}"
        );
    }

    assert!(api_source.contains("visible_label: impl Into<String>"));
    assert!(api_source.contains("tooltip: impl Into<String>"));
    assert!(api_source.contains("accessibility_label: impl Into<String>"));
    assert!(!api_source.contains("visible: bool"));
    assert!(!api_source.contains("derive(Debug"));
}
