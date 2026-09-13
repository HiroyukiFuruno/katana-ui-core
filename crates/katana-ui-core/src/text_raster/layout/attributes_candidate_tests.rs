use super::*;
use crate::render_model::UiTextSpan;
use crate::text_raster::{
    PlatformColorEmojiAvailability, PlatformColorEmojiUnavailableReason, PlatformFontCatalog,
    PlatformFontCatalogFingerprint, PlatformFontProfile, PlatformTextFaceSelection,
    PlatformTextRasterConfig,
};
use crate::theme::{FontFamily, FontToken};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping};

const TEST_FONT_SIZE_PX: f32 = 16.0;
const REGULAR_WEIGHT: u16 = 400;
const FINGERPRINT_BYTES: usize = 32;
const LAYOUT_WIDTH_PX: f32 = 1024.0;
const LAYOUT_HEIGHT_PX: f32 = 128.0;
const LATIN_GLYPH_COUNT: usize = 5;

fn font() -> FontToken {
    FontToken {
        name: "candidate".to_owned(),
        family: FontFamily::Proportional,
        size: TEST_FONT_SIZE_PX,
        weight: REGULAR_WEIGHT,
    }
}
fn face() -> PlatformColorEmojiFaceRecord {
    PlatformColorEmojiFaceRecord {
        platform_profile: PlatformFontProfile::Unsupported,
        family_identity: String::new(),
        source_file_path: None,
        raw_file_sha256: None,
        catalog_fingerprint: PlatformFontCatalogFingerprint::from_bytes([0; FINGERPRINT_BYTES]),
        availability: PlatformColorEmojiAvailability::Unavailable(
            PlatformColorEmojiUnavailableReason::NoCandidates,
        ),
    }
}
fn candidate() -> Option<(std::path::PathBuf, u32)> {
    FontSystem::new().db().faces().find_map(|face| {
        let cosmic_text::fontdb::Source::File(path) = &face.source else {
            return None;
        };
        let mut db = cosmic_text::fontdb::Database::new();
        db.load_font_file(path).ok()?;
        let mut fs = FontSystem::new_with_locale_and_db(String::new(), db);
        let face = fs.db().faces().next()?;
        let (id, weight, index) = (face.id, face.weight, face.index);
        (weight.0 != REGULAR_WEIGHT
            && fs.get_font(id, weight).is_some_and(|font| {
                "Latin"
                    .chars()
                    .all(|c| font.as_swash().charmap().map(c) != 0)
            }))
        .then(|| (path.clone(), index))
    })
}
fn shaped(fs: &mut FontSystem, attrs: Attrs<'_>) -> Vec<cosmic_text::fontdb::FaceInfo> {
    let mut b = Buffer::new(fs, Metrics::new(TEST_FONT_SIZE_PX, TEST_FONT_SIZE_PX));
    let font_ids = {
        let mut b = b.borrow_with(fs);
        b.set_size(Some(LAYOUT_WIDTH_PX), Some(LAYOUT_HEIGHT_PX));
        b.set_rich_text([("Latin", attrs)], &Attrs::new(), Shaping::Advanced, None);
        b.layout_runs()
            .flat_map(|r| r.glyphs.iter())
            .map(|glyph| glyph.font_id)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        LATIN_GLYPH_COUNT,
        font_ids.len(),
        "Latin must shape every glyph"
    );
    font_ids
        .into_iter()
        .map(|font_id| fs.db().face(font_id).expect("face").clone())
        .collect()
}
#[test]
fn non_regular_first_candidate_shapes_every_latin_variant_with_its_source_and_index() {
    let (path, index) = candidate().expect("non-400 Latin-capable first face is required");
    let config = PlatformTextRasterConfig {
        proportional_candidates: vec![path.clone()],
        monospace_candidates: vec![path.clone()],
        emoji_candidates: Vec::new(),
        emoji_candidate_sha256: Vec::new(),
        cache_capacity: 1,
    };
    let catalog = PlatformFontCatalog::new(config.catalog_policy());
    let faces = ResolvedTextFaces::from_candidate_faces(catalog.regular_font_faces());
    let emoji = face();
    for (bold, italic, weight, monospace) in [
        (false, false, REGULAR_WEIGHT, false),
        (false, false, 300, false),
        (true, false, 700, false),
        (false, true, 650, false),
        (false, false, 900, true),
    ] {
        let mut token = font();
        token.family = if monospace {
            FontFamily::Monospace
        } else {
            FontFamily::Proportional
        };
        token.weight = weight;
        let mut span = UiTextSpan::plain("Latin");
        span.style.bold = bold;
        span.style.italic = italic;
        span.style.monospace = monospace;
        let attrs = attrs_for_span(&token, &span, [u8::MAX; RGBA_CHANNEL_COUNT], &emoji, &faces)
            .expect("attrs");
        let got = catalog
            .with_font_system_for_face_selection(PlatformTextFaceSelection::FirstCandidate, |fs| {
                shaped(fs, attrs)
            })
            .expect("font system");
        assert!(got.into_iter().all(|face| {
            face.index == index
                && matches!(face.source,cosmic_text::fontdb::Source::File(ref p)|cosmic_text::fontdb::Source::SharedFile(ref p,_) if p==&path)
        }));
    }
}
