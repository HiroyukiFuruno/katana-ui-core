use super::common::{TEXT_COLOR, font, has_alpha_pixels};
use crate::{
    PlatformTextGraphemeRange, PlatformTextRasterConfig, PlatformTextRasterError,
    PlatformTextRasterRequest, PlatformTextRasterizer,
};
use std::path::Path;

#[test]
fn star_variation_selector_has_one_grapheme_bound_and_hit_range()
-> Result<(), Box<dyn std::error::Error>> {
    let text = "日本語 ⭐️ input";
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let raster = match rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        text,
        font(),
        TEXT_COLOR,
    )) {
        Ok(raster) => raster,
        Err(PlatformTextRasterError::ColorEmojiUnavailable { face }) => {
            #[cfg(target_os = "macos")]
            return Err(PlatformTextRasterError::ColorEmojiUnavailable { face }.into());
            #[cfg(not(target_os = "macos"))]
            assert!(!face.is_available());
            #[cfg(not(target_os = "macos"))]
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let start = text.find("⭐️").ok_or("star fixture missing")?;
    let end = start + "⭐️".len();
    let bounds = raster
        .grapheme_bounds
        .iter()
        .find(|bounds| bounds.byte_start == start && bounds.byte_end == end)
        .ok_or("star must have one grapheme bound")?;

    assert_eq!(
        Some((start, end)),
        raster
            .hit_test(bounds.x + bounds.width / 2.0, bounds.y + 1.0)
            .map(|hit| (hit.byte_start, hit.byte_end))
    );
    Ok(())
}

#[test]
fn grapheme_edit_boundaries_keep_star_variation_selector_atomic()
-> Result<(), Box<dyn std::error::Error>> {
    let text = "かな⭐️";
    let star_start = text.find('⭐').ok_or("star byte offset")?;
    let star_end = text.len();

    assert_eq!(
        PlatformTextGraphemeRange::previous(text, star_end),
        Some(PlatformTextGraphemeRange {
            byte_start: star_start,
            byte_end: star_end,
        })
    );
    assert_eq!(
        PlatformTextGraphemeRange::next(text, star_start),
        Some(PlatformTextGraphemeRange {
            byte_start: star_start,
            byte_end: star_end,
        })
    );
    Ok(())
}

#[test]
fn editor_fixture_japanese_and_star_rasterize_with_bounded_line_positions()
-> Result<(), Box<dyn std::error::Error>> {
    let text = "Storybook fixture\nInteractively edited editor seed with 日本語 and ⭐️\n";
    let mut request = PlatformTextRasterRequest::from_text(text, font(), TEXT_COLOR);
    request.max_width_px = Some(760.0);
    request.scale_factor = 2.0;
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let raster = rasterizer.rasterize(&request)?;

    let bounds = raster
        .grapheme_bounds
        .iter()
        .filter_map(|bounds| {
            text.get(bounds.byte_start..bounds.byte_end)
                .map(|text| (text, bounds.y))
        })
        .collect::<Vec<_>>();
    assert!(bounds.iter().any(|(text, _)| *text == "日"));
    assert!(bounds.iter().any(|(text, _)| *text == "⭐️"));
    let japanese_y = bounds
        .iter()
        .find_map(|(text, y)| (*text == "日").then_some(*y))
        .ok_or("Japanese bound")?;
    let star_y = bounds
        .iter()
        .find_map(|(text, y)| (*text == "⭐️").then_some(*y))
        .ok_or("star bound")?;
    assert!(japanese_y < 100.0, "Japanese y drifted: {japanese_y}");
    assert!(star_y < 160.0, "star y drifted: {star_y}");
    let japanese = raster
        .grapheme_bounds
        .iter()
        .find(|bounds| text.get(bounds.byte_start..bounds.byte_end) == Some("日"))
        .ok_or("Japanese bound")?;
    assert!(has_alpha_pixels(&raster, japanese, request.scale_factor));
    Ok(())
}

#[test]
fn zwj_emoji_has_one_grapheme_bound_and_hit_range() -> Result<(), Box<dyn std::error::Error>> {
    let text = "engineer 🧑‍💻";
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let raster = match rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        text,
        font(),
        TEXT_COLOR,
    )) {
        Ok(raster) => raster,
        Err(PlatformTextRasterError::ColorEmojiUnavailable { face }) => {
            #[cfg(target_os = "macos")]
            return Err(PlatformTextRasterError::ColorEmojiUnavailable { face }.into());
            #[cfg(not(target_os = "macos"))]
            assert!(!face.is_available());
            #[cfg(not(target_os = "macos"))]
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let start = text.find("🧑‍💻").ok_or("ZWJ fixture missing")?;
    let end = start + "🧑‍💻".len();
    let bounds = raster
        .grapheme_bounds
        .iter()
        .find(|bounds| bounds.byte_start == start && bounds.byte_end == end)
        .ok_or("ZWJ emoji must have one grapheme bound")?;

    assert_eq!(
        Some((start, end)),
        raster
            .hit_test(bounds.x + bounds.width / 2.0, bounds.y + 1.0)
            .map(|hit| (hit.byte_start, hit.byte_end))
    );
    Ok(())
}

#[cfg(target_os = "macos")]
#[test]
fn platform_emoji_uses_color_pixels_when_apple_color_emoji_is_available()
-> Result<(), Box<dyn std::error::Error>> {
    let config = PlatformTextRasterConfig::default();
    let mut rasterizer = PlatformTextRasterizer::new(config);
    let raster = rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        "⭐️",
        font(),
        TEXT_COLOR,
    ))?;
    assert_eq!(
        Some("Apple Color Emoji"),
        raster.report.resolved_emoji_font_family.as_deref()
    );
    assert!(raster.report.color_emoji_font_available);
    let face = &raster.report.emoji_face;
    assert_eq!(Some("Apple Color Emoji"), face.resolved_family());
    assert_eq!(
        Some(Path::new("/System/Library/Fonts/Apple Color Emoji.ttc")),
        face.source_file_path.as_deref()
    );
    assert!(face.raw_file_sha256.is_some());

    let star_bounds = raster
        .grapheme_bounds
        .iter()
        .find(|bounds| raster.text.get(bounds.byte_start..bounds.byte_end) == Some("⭐️"))
        .ok_or("isolated color star grapheme bound")?;
    let star_crop = raster
        .grapheme_crop(star_bounds, 1.0)
        .ok_or("isolated color star crop")?;
    assert!(star_crop.chromatic_pixel_count() > 0);

    let outline = rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        "☆",
        font(),
        TEXT_COLOR,
    ))?;
    let outline_bounds = outline
        .grapheme_bounds
        .iter()
        .find(|bounds| outline.text.get(bounds.byte_start..bounds.byte_end) == Some("☆"))
        .ok_or("isolated outline star grapheme bound")?;
    let outline_crop = outline
        .grapheme_crop(outline_bounds, 1.0)
        .ok_or("isolated outline star crop")?;
    assert_eq!(0, outline_crop.chromatic_pixel_count());
    assert_ne!(star_crop.sha256(), outline_crop.sha256());
    Ok(())
}
