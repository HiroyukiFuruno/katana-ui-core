use crate::catalog_types::PlatformColorEmojiFaceRecord;
use crate::model::{
    PlatformTextGraphemeBounds, PlatformTextRasterError, PlatformTextRasterRequest,
    RGBA_ALPHA_INDEX, RGBA_CHANNEL_COUNT, TRANSPARENT_RGBA,
};
use cosmic_text::{
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, Style as FontStyle, SwashCache,
    Weight, Wrap,
};
use katana_ui_core::render_model::{UiTextSpan, UiTextSpanStyle};
use katana_ui_core::theme::{FontFamily, FontToken};
use std::collections::BTreeMap;
use unicode_segmentation::UnicodeSegmentation;

const FALLBACK_LAYOUT_WIDTH: f32 = 4096.0;
const FALLBACK_LAYOUT_HEIGHT: f32 = 4096.0;
const MAX_LAYOUT_WIDTH: f32 = 8192.0;
const MAX_RASTER_DIMENSION: usize = 8192;
const MAX_RASTER_PIXELS: usize = 16_777_216;
const MIN_FONT_SIZE_PX: f32 = 1.0;
const MIN_GRAPHEME_EXTENT_PX: f32 = 1.0;
const MIN_RASTER_DIMENSION: f32 = 1.0;
const MIN_GRAPHEME_COUNT: usize = 1;
const REGULAR_WEIGHT: u16 = 400;
const BOLD_WEIGHT: u16 = 700;
const OPAQUE_CHANNEL_VALUE: u8 = 255;

pub(crate) struct TextLayoutRasterizer;

pub(crate) struct LayoutRaster {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) rgba_pixels: Vec<[u8; RGBA_CHANNEL_COUNT]>,
    pub(crate) grapheme_bounds: Vec<PlatformTextGraphemeBounds>,
}

impl TextLayoutRasterizer {
    pub(crate) fn rasterize(
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        request: &PlatformTextRasterRequest,
        emoji_face: &PlatformColorEmojiFaceRecord,
    ) -> Result<LayoutRaster, PlatformTextRasterError> {
        let scale = request.normalized_scale_factor();
        let metrics = Metrics::new(
            request.font.size.max(MIN_FONT_SIZE_PX) * scale,
            request.normalized_line_height() * scale,
        );
        let mut buffer = Buffer::new(font_system, metrics);
        let mut buffer = buffer.borrow_with(font_system);
        buffer.set_wrap(Wrap::Word);
        buffer.set_size(
            Some(request.normalized_max_width(FALLBACK_LAYOUT_WIDTH, MAX_LAYOUT_WIDTH) * scale),
            Some(FALLBACK_LAYOUT_HEIGHT * scale),
        );
        let runs = normalized_runs(&request.spans);
        let rich_text = runs
            .iter()
            .map(|span| {
                attrs_for_span(&request.font, span, request.fallback_color_rgba, emoji_face)
                    .map(|attrs| (span.text.as_str(), attrs))
            })
            .collect::<Result<Vec<_>, _>>()?;
        buffer.set_rich_text(rich_text, &Attrs::new(), Shaping::Advanced, None);
        let source_text = request.text();
        let grapheme_bounds = collect_grapheme_bounds(&mut buffer, &source_text, scale);
        let (width, height) = raster_extent(&grapheme_bounds, scale)?;
        let rgba_pixels = collect_pixels(&mut buffer, swash_cache, width, height);
        Ok(LayoutRaster {
            width,
            height,
            rgba_pixels,
            grapheme_bounds,
        })
    }
}

fn normalized_runs(spans: &[UiTextSpan]) -> Vec<UiTextSpan> {
    spans.to_vec()
}

fn attrs_for_span<'a>(
    font: &FontToken,
    span: &UiTextSpan,
    fallback_color_rgba: [u8; RGBA_CHANNEL_COUNT],
    emoji_face: &'a PlatformColorEmojiFaceRecord,
) -> Result<Attrs<'a>, PlatformTextRasterError> {
    let style = &span.style;
    Ok(Attrs::new()
        .family(family_for(font.family, style, &span.text, emoji_face)?)
        .weight(Weight(if style.bold {
            BOLD_WEIGHT
        } else {
            font.weight.max(REGULAR_WEIGHT)
        }))
        .style(if style.italic {
            FontStyle::Italic
        } else {
            FontStyle::Normal
        })
        .color(color_for(style, fallback_color_rgba)))
}

fn family_for<'a>(
    family: FontFamily,
    style: &UiTextSpanStyle,
    text: &str,
    emoji_face: &'a PlatformColorEmojiFaceRecord,
) -> Result<Family<'a>, PlatformTextRasterError> {
    if style.emoji {
        return emoji_face
            .resolved_family()
            .map(Family::Name)
            .ok_or_else(|| PlatformTextRasterError::ColorEmojiUnavailable {
                face: Box::new(emoji_face.clone()),
            });
    }
    Ok(
        if text.is_ascii() && (style.monospace || family == FontFamily::Monospace) {
            Family::Monospace
        } else {
            Family::SansSerif
        },
    )
}

fn color_for(style: &UiTextSpanStyle, fallback: [u8; RGBA_CHANNEL_COUNT]) -> Color {
    let color = (style.color_rgba[RGBA_ALPHA_INDEX] != 0).then_some(style.color_rgba);
    let [red, green, blue, alpha] = color.unwrap_or(fallback);
    Color::rgba(red, green, blue, alpha)
}

fn collect_grapheme_bounds(
    buffer: &mut cosmic_text::BorrowedWithFontSystem<'_, Buffer>,
    source_text: &str,
    scale: f32,
) -> Vec<PlatformTextGraphemeBounds> {
    let line_offsets = line_offsets(source_text);
    let mut bounds = BTreeMap::<(usize, usize), PlatformTextGraphemeBounds>::new();
    for run in buffer.layout_runs() {
        let line_offset = line_offsets.get(run.line_i).copied().unwrap_or_default();
        for glyph in run.glyphs {
            let cluster = &run.text[glyph.start..glyph.end];
            let graphemes = cluster.grapheme_indices(true).collect::<Vec<_>>();
            let grapheme_width = glyph.w / graphemes.len().max(MIN_GRAPHEME_COUNT) as f32 / scale;
            for (index, grapheme) in graphemes {
                let byte_start = line_offset + glyph.start + index;
                let byte_end = byte_start + grapheme.len();
                let candidate = PlatformTextGraphemeBounds {
                    byte_start,
                    byte_end,
                    x: glyph.x / scale + grapheme_width * grapheme_index(cluster, index) as f32,
                    y: run.line_top / scale,
                    width: grapheme_width.max(MIN_GRAPHEME_EXTENT_PX),
                    height: (run.line_height / scale).max(MIN_GRAPHEME_EXTENT_PX),
                };
                merge_bounds(&mut bounds, candidate);
            }
        }
    }
    bounds.into_values().collect()
}

fn grapheme_index(cluster: &str, byte_index: usize) -> usize {
    cluster
        .grapheme_indices(true)
        .position(|(index, _)| index == byte_index)
        .unwrap_or_default()
}

fn merge_bounds(
    bounds: &mut BTreeMap<(usize, usize), PlatformTextGraphemeBounds>,
    candidate: PlatformTextGraphemeBounds,
) {
    let key = (candidate.byte_start, candidate.byte_end);
    let Some(current) = bounds.get_mut(&key) else {
        bounds.insert(key, candidate);
        return;
    };
    let right = (current.x + current.width).max(candidate.x + candidate.width);
    let bottom = (current.y + current.height).max(candidate.y + candidate.height);
    current.x = current.x.min(candidate.x);
    current.y = current.y.min(candidate.y);
    current.width = (right - current.x).max(MIN_GRAPHEME_EXTENT_PX);
    current.height = (bottom - current.y).max(MIN_GRAPHEME_EXTENT_PX);
}

fn line_offsets(text: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(text.char_indices().filter_map(|(index, character)| {
            (character == '\n').then_some(index + MIN_GRAPHEME_COUNT)
        }))
        .collect()
}

fn raster_extent(
    bounds: &[PlatformTextGraphemeBounds],
    scale: f32,
) -> Result<(usize, usize), PlatformTextRasterError> {
    let width = bounds
        .iter()
        .map(|bounds| bounds.x + bounds.width)
        .fold(MIN_RASTER_DIMENSION, f32::max);
    let height = bounds
        .iter()
        .map(|bounds| bounds.y + bounds.height)
        .fold(MIN_RASTER_DIMENSION, f32::max);
    let width = raster_dimension(width * scale)?;
    let height = raster_dimension(height * scale)?;
    let pixels = width
        .checked_mul(height)
        .ok_or(PlatformTextRasterError::RasterTooLarge {
            width,
            height,
            max_pixels: MAX_RASTER_PIXELS,
        })?;
    if pixels > MAX_RASTER_PIXELS {
        return Err(PlatformTextRasterError::RasterTooLarge {
            width,
            height,
            max_pixels: MAX_RASTER_PIXELS,
        });
    }
    Ok((width, height))
}

fn raster_dimension(value: f32) -> Result<usize, PlatformTextRasterError> {
    if !value.is_finite() {
        return Err(PlatformTextRasterError::NonFiniteLayoutExtent);
    }
    let dimension = value.ceil().max(MIN_RASTER_DIMENSION) as usize;
    if dimension > MAX_RASTER_DIMENSION {
        return Err(PlatformTextRasterError::RasterTooLarge {
            width: dimension,
            height: dimension,
            max_pixels: MAX_RASTER_PIXELS,
        });
    }
    Ok(dimension)
}

fn collect_pixels(
    buffer: &mut cosmic_text::BorrowedWithFontSystem<'_, Buffer>,
    swash_cache: &mut SwashCache,
    width: usize,
    height: usize,
) -> Vec<[u8; RGBA_CHANNEL_COUNT]> {
    let mut pixels = vec![TRANSPARENT_RGBA; width * height];
    buffer.draw(
        swash_cache,
        Color::rgba(
            OPAQUE_CHANNEL_VALUE,
            OPAQUE_CHANNEL_VALUE,
            OPAQUE_CHANNEL_VALUE,
            OPAQUE_CHANNEL_VALUE,
        ),
        |left, top, width_in_pixels, height_in_pixels, color| {
            for y in 0..height_in_pixels {
                for x in 0..width_in_pixels {
                    let x = left + x as i32;
                    let y = top + y as i32;
                    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
                        continue;
                    }
                    pixels[y as usize * width + x as usize] =
                        [color.r(), color.g(), color.b(), color.a()];
                }
            }
        },
    );
    pixels
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bounds(x: f32, y: f32, width: f32, height: f32) -> PlatformTextGraphemeBounds {
        PlatformTextGraphemeBounds {
            byte_start: 0,
            byte_end: 1,
            x,
            y,
            width,
            height,
        }
    }

    #[test]
    fn layout_helpers_cover_merge_extent_color_and_unicode_boundaries() {
        assert_eq!(line_offsets("a\nb\n"), vec![0, 2, 4]);
        assert_eq!(grapheme_index("a⭐️", 0), 0);
        assert_eq!(grapheme_index("a⭐️", 1), 1);
        assert_eq!(grapheme_index("a⭐️", 99), 0);

        let mut merged = BTreeMap::new();
        merge_bounds(&mut merged, bounds(2.0, 3.0, 4.0, 5.0));
        merge_bounds(&mut merged, bounds(1.0, 2.0, 8.0, 9.0));
        let merged = merged.get(&(0, 1)).expect("merged bounds");
        assert_eq!(
            (merged.x, merged.y, merged.width, merged.height),
            (1.0, 2.0, 8.0, 9.0)
        );

        assert_eq!(raster_extent(&[], 1.0).expect("minimum extent"), (1, 1));
        assert!(matches!(
            raster_extent(&[bounds(0.0, 0.0, 8192.0, 8192.0)], 1.0),
            Err(PlatformTextRasterError::RasterTooLarge { .. })
        ));
        assert_eq!(raster_dimension(1.1).expect("finite dimension"), 2);
        assert_eq!(
            raster_dimension(f32::INFINITY),
            Err(PlatformTextRasterError::NonFiniteLayoutExtent)
        );
        assert!(matches!(
            raster_dimension(8193.0),
            Err(PlatformTextRasterError::RasterTooLarge { .. })
        ));

        assert_eq!(
            color_for(&UiTextSpanStyle::default(), [1, 2, 3, 4]),
            Color::rgba(1, 2, 3, 4)
        );
        let styled = UiTextSpanStyle {
            color_rgba: [5, 6, 7, 8],
            ..UiTextSpanStyle::default()
        };
        assert_eq!(color_for(&styled, [1, 2, 3, 4]), Color::rgba(5, 6, 7, 8));
        assert!(normalized_runs(&[UiTextSpan::plain("x")])[0].text == "x");

        let styled_span = UiTextSpan {
            text: "styled".to_owned(),
            style: UiTextSpanStyle {
                bold: true,
                italic: true,
                ..UiTextSpanStyle::default()
            },
            link_target: String::new(),
        };
        let emoji_face = unavailable_emoji_face();
        let attrs = attrs_for_span(
            &FontToken {
                name: "styled".to_owned(),
                family: FontFamily::Proportional,
                size: 14.0,
                weight: REGULAR_WEIGHT,
            },
            &styled_span,
            [1, 2, 3, 4],
            &emoji_face,
        )
        .expect("styled attributes");
        assert_eq!(attrs.metadata, 0);
    }

    fn unavailable_emoji_face() -> crate::PlatformColorEmojiFaceRecord {
        crate::PlatformColorEmojiFaceRecord {
            platform_profile: crate::PlatformFontProfile::Unsupported,
            family_identity: String::new(),
            source_file_path: None,
            raw_file_sha256: None,
            catalog_fingerprint: crate::PlatformFontCatalogFingerprint::from_bytes([0; 32]),
            availability: crate::PlatformColorEmojiAvailability::Unavailable(
                crate::PlatformColorEmojiUnavailableReason::NoCandidates,
            ),
        }
    }

    #[test]
    fn family_for_ascii_prefers_monospace_and_non_ascii_reverts_to_sans_serif() {
        let monospace_style = UiTextSpanStyle {
            monospace: true,
            ..UiTextSpanStyle::default()
        };

        assert_eq!(
            family_for(
                FontFamily::Proportional,
                &monospace_style,
                "abc",
                &unavailable_emoji_face(),
            )
            .expect("ascii text respects monospace family"),
            Family::Monospace
        );
        assert_eq!(
            family_for(
                FontFamily::Monospace,
                &UiTextSpanStyle::default(),
                "abc",
                &unavailable_emoji_face(),
            )
            .expect("ascii monospace font family resolves monospace"),
            Family::Monospace
        );
        assert_eq!(
            family_for(
                FontFamily::Monospace,
                &UiTextSpanStyle::default(),
                "漢字",
                &unavailable_emoji_face(),
            )
            .expect("unicode text resolves sans-serif"),
            Family::SansSerif
        );
    }

    #[test]
    fn family_for_emoji_uses_resolved_face_or_returns_typed_error() {
        let emoji_style = UiTextSpanStyle {
            emoji: true,
            ..UiTextSpanStyle::default()
        };
        let unresolved = unavailable_emoji_face();
        assert!(matches!(
            family_for(FontFamily::Proportional, &emoji_style, "⭐", &unresolved),
            Err(PlatformTextRasterError::ColorEmojiUnavailable { .. })
        ));

        let resolved = crate::PlatformColorEmojiFaceRecord {
            family_identity: "Test Emoji".to_string(),
            availability: crate::PlatformColorEmojiAvailability::Resolved,
            ..unresolved
        };
        assert_eq!(
            family_for(FontFamily::Proportional, &emoji_style, "⭐", &resolved)
                .expect("emoji uses resolved family"),
            Family::Name("Test Emoji")
        );
    }
}
