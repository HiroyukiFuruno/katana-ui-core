#![cfg(feature = "egui")]
const CORE_MANIFEST: &str = include_str!("../Cargo.toml");
const CORE_ROOT: &str = include_str!("../src/lib.rs");

#[test]
fn optional_modules_share_one_public_package_boundary() {
    assert!(CORE_MANIFEST.contains("default = []"));
    assert!(CORE_MANIFEST.contains(
        "egui = [\"dep:egui\", \"dep:hex\", \"dep:sha2\", \"text-raster\", \"svg-raster\"]"
    ));
    assert!(CORE_MANIFEST.contains("egui = { workspace = true, optional = true }"));
    assert!(CORE_MANIFEST.contains("cosmic-text = { workspace = true, optional = true }"));
    assert!(CORE_MANIFEST.contains("resvg = { workspace = true, optional = true }"));
    assert!(CORE_MANIFEST.contains("tiny-skia = { workspace = true, optional = true }"));

    assert!(CORE_ROOT.contains("#[cfg(feature = \"egui\")]\npub mod egui;"));
    assert!(CORE_ROOT.contains("#[cfg(feature = \"text-raster\")]\npub mod text_raster;"));
    assert!(CORE_ROOT.contains("#[cfg(feature = \"svg-raster\")]\npub mod svg_raster;"));

    for removed_package in [
        "katana-ui-core-egui-adapter",
        "katana-ui-core-text-raster",
        "katana-ui-core-svg-raster",
    ] {
        assert!(!CORE_MANIFEST.contains(removed_package));
    }
}

#[test]
fn public_package_does_not_import_consumer_or_host_crates() {
    for forbidden in [
        "katana-language-editor",
        "katana-document-viewer",
        "katana-render-runtime",
        "katana-ui =",
    ] {
        assert!(
            !CORE_MANIFEST.contains(forbidden),
            "forbidden dependency: {forbidden}"
        );
    }
}
