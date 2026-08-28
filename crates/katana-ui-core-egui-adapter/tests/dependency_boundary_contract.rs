const ADAPTER_MANIFEST: &str = include_str!("../Cargo.toml");
const CORE_MANIFEST: &str = include_str!("../../katana-ui-core/Cargo.toml");

#[test]
fn shared_adapter_is_the_only_egui_workspace_boundary() {
    assert!(ADAPTER_MANIFEST.contains("egui.workspace = true"));
    assert!(ADAPTER_MANIFEST.contains("katana-ui-core.workspace = true"));
    assert!(ADAPTER_MANIFEST.contains("katana-ui-core-text-raster.workspace = true"));
    assert!(ADAPTER_MANIFEST.contains("katana-ui-core-svg-raster.workspace = true"));
    assert!(!CORE_MANIFEST.contains("egui"));
}

#[test]
fn shared_adapter_does_not_import_consumer_or_host_crates() {
    for forbidden in [
        "katana-language-editor",
        "katana-document-viewer",
        "katana-render-runtime",
        "katana-ui =",
    ] {
        assert!(
            !ADAPTER_MANIFEST.contains(forbidden),
            "forbidden dependency: {forbidden}"
        );
    }
}
