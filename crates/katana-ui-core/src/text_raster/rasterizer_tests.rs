use super::*;
use crate::text_raster::{
    PlatformFontCatalog, PlatformTextFaceSelection, PlatformTextRasterConfig,
    PlatformTextRasterRequest,
};
use crate::theme::{FontFamily, FontToken};
use cosmic_text::{FontSystem, fontdb::Source};
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

const TEXT_COLOR: [u8; 4] = [245, 245, 245, 255];
const TEST_FONT_SIZE_PX: f32 = 18.0;
const TEST_FONT_WEIGHT: u16 = 400;

static NEXT_TEST_PATH: AtomicU64 = AtomicU64::new(0);

fn installed_font_candidate() -> io::Result<(PathBuf, String)> {
    FontSystem::new()
        .db()
        .faces()
        .find_map(|face| match &face.source {
            Source::File(path) => face
                .families
                .first()
                .map(|(family, _)| (path.clone(), family.to_string())),
            _ => None,
        })
        .ok_or_else(|| io::Error::other("a file-backed system font is required"))
}

fn missing_font_path() -> PathBuf {
    let serial = NEXT_TEST_PATH.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "kuc-first-candidate-{serial}-{}.ttf",
        std::process::id()
    ))
}

fn font(family: FontFamily) -> FontToken {
    FontToken {
        name: "candidate-selection".to_owned(),
        family,
        size: TEST_FONT_SIZE_PX,
        weight: TEST_FONT_WEIGHT,
    }
}

fn first_candidate_config(candidate: PathBuf) -> PlatformTextRasterConfig {
    let missing = missing_font_path();
    PlatformTextRasterConfig {
        proportional_candidates: vec![missing.clone(), candidate.clone()],
        monospace_candidates: vec![missing, candidate],
        emoji_candidates: Vec::new(),
        emoji_candidate_sha256: Vec::new(),
        cache_capacity: 4,
    }
}

#[test]
fn first_candidate_selection_reaches_regular_and_monospace_rasterization()
-> Result<(), Box<dyn std::error::Error>> {
    let (candidate, family) = installed_font_candidate()?;
    let config = first_candidate_config(candidate);
    let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
    let mut rasterizer = PlatformTextRasterizer::with_catalog_and_face_selection(
        catalog,
        config,
        PlatformTextFaceSelection::FirstCandidate,
    )?;

    assert_eq!(rasterizer.text_faces.proportional(), Some(family.as_str()));
    assert_eq!(rasterizer.text_faces.monospace(), Some(family.as_str()));

    let regular = rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        "Regular candidate",
        font(FontFamily::Proportional),
        TEXT_COLOR,
    ))?;
    let monospace = rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        "Monospace candidate",
        font(FontFamily::Monospace),
        TEXT_COLOR,
    ))?;

    assert!(regular.width > 0 && regular.height > 0);
    assert!(monospace.width > 0 && monospace.height > 0);
    assert!(!regular.rgba_pixels.is_empty());
    assert!(!monospace.rgba_pixels.is_empty());
    Ok(())
}

#[test]
fn unresolved_first_candidate_selection_keeps_generic_fallback_faces()
-> Result<(), Box<dyn std::error::Error>> {
    let missing = missing_font_path();
    let config = PlatformTextRasterConfig {
        proportional_candidates: vec![missing.clone()],
        monospace_candidates: vec![missing],
        emoji_candidates: Vec::new(),
        emoji_candidate_sha256: Vec::new(),
        cache_capacity: 4,
    };
    let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
    let mut rasterizer = PlatformTextRasterizer::with_catalog_and_face_selection(
        catalog,
        config,
        PlatformTextFaceSelection::FirstCandidate,
    )?;

    assert_eq!(rasterizer.text_faces, ResolvedTextFaces::default());
    let raster = rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        "System fallback",
        font(FontFamily::Proportional),
        TEXT_COLOR,
    ))?;

    assert!(raster.width > 0 && raster.height > 0);
    Ok(())
}
