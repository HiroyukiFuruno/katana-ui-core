use crate::render_model::{UiTextSpan, UiTextSpanStyle};
use crate::text_raster::catalog_types::PlatformColorEmojiFaceRecord;
use crate::text_raster::model::{PlatformTextRasterError, RGBA_ALPHA_INDEX, RGBA_CHANNEL_COUNT};
use crate::theme::{FontFamily, FontToken};
use cosmic_text::{Attrs, Color, Family, Style as FontStyle, Weight};

use super::{BOLD_WEIGHT, REGULAR_WEIGHT, ResolvedTextFace, ResolvedTextFaces};

pub(super) fn normalized_runs(spans: &[UiTextSpan]) -> Vec<UiTextSpan> {
    spans.to_vec()
}

pub(super) fn attrs_for_span<'a>(
    font: &FontToken,
    span: &UiTextSpan,
    fallback_color_rgba: [u8; RGBA_CHANNEL_COUNT],
    emoji_face: &'a PlatformColorEmojiFaceRecord,
    text_faces: &'a ResolvedTextFaces,
) -> Result<Attrs<'a>, PlatformTextRasterError> {
    let style = &span.style;
    let selected_face = text_faces.selected_face(font.family, style, &span.text);
    Ok(Attrs::new()
        .family(family_for(
            font.family,
            style,
            &span.text,
            emoji_face,
            text_faces,
        )?)
        .weight(Weight(selected_face.map_or_else(
            || {
                if style.bold {
                    BOLD_WEIGHT
                } else {
                    font.weight.max(REGULAR_WEIGHT)
                }
            },
            ResolvedTextFace::weight,
        )))
        .style(selected_face.map_or_else(
            || {
                if style.italic {
                    FontStyle::Italic
                } else {
                    FontStyle::Normal
                }
            },
            ResolvedTextFace::style,
        ))
        .stretch(selected_face.map_or(cosmic_text::Stretch::Normal, ResolvedTextFace::stretch))
        .color(color_for(style, fallback_color_rgba)))
}

fn family_for<'a>(
    family: FontFamily,
    style: &UiTextSpanStyle,
    text: &str,
    emoji_face: &'a PlatformColorEmojiFaceRecord,
    text_faces: &'a ResolvedTextFaces,
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
            text_faces
                .selected_face(family, style, text)
                .map(|face| Family::Name(face.family()))
                .unwrap_or(Family::Monospace)
        } else {
            text_faces
                .selected_face(family, style, text)
                .map(|face| Family::Name(face.family()))
                .unwrap_or(Family::SansSerif)
        },
    )
}

fn color_for(style: &UiTextSpanStyle, fallback: [u8; RGBA_CHANNEL_COUNT]) -> Color {
    let color = (style.color_rgba[RGBA_ALPHA_INDEX] != 0).then_some(style.color_rgba);
    let [red, green, blue, alpha] = color.unwrap_or(fallback);
    Color::rgba(red, green, blue, alpha)
}

#[cfg(test)]
#[path = "attributes_candidate_tests.rs"]
mod candidate_tests;
#[cfg(test)]
#[path = "attributes_tests.rs"]
mod tests;
