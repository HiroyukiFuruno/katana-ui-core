#![cfg(feature = "egui")]
const CORE_MANIFEST: &str = include_str!("../Cargo.toml");
const CORE_ROOT: &str = include_str!("../src/lib.rs");

fn feature_members(feature: &str) -> Vec<&str> {
    let marker = format!("{feature} = [");
    let (_, tail) = CORE_MANIFEST
        .split_once(&marker)
        .expect("feature declaration");
    let (members, _) = tail.split_once(']').expect("feature array terminator");
    members.split('"').skip(1).step_by(2).collect::<Vec<_>>()
}

fn dependency_is_optional(dependency: &str) -> bool {
    let inline_marker = format!("{dependency} = {{");
    if let Some(line) = CORE_MANIFEST
        .lines()
        .find(|line| line.starts_with(&inline_marker))
    {
        return line.contains("optional = true");
    }

    let table_marker = format!("[dependencies.{dependency}]\n");
    CORE_MANIFEST
        .split_once(&table_marker)
        .and_then(|(_, tail)| tail.split_once("\n["))
        .is_some_and(|(table, _)| table.lines().any(|line| line == "optional = true"))
}

#[test]
fn optional_modules_share_one_public_package_boundary() {
    assert!(CORE_MANIFEST.contains("default = []"));
    assert_eq!(
        feature_members("egui"),
        [
            "dep:egui",
            "dep:hex",
            "dep:sha2",
            "text-raster",
            "svg-raster"
        ]
    );
    for dependency in ["egui", "cosmic-text", "resvg", "tiny-skia"] {
        assert!(
            dependency_is_optional(dependency),
            "{dependency} must remain optional"
        );
    }

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
