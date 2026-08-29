use crate::catalog_types::PlatformColorEmojiFaceRecord;
use crate::model::{PlatformTextRasterError, RGBA_ALPHA_INDEX, RGBA_CHANNEL_COUNT};
use cosmic_text::{Attrs, Color, Family, Style as FontStyle, Weight};
use katana_ui_core::render_model::{UiTextSpan, UiTextSpanStyle};
use katana_ui_core::theme::{FontFamily, FontToken};

use super::{BOLD_WEIGHT, REGULAR_WEIGHT};

pub(super) fn normalized_runs(spans: &[UiTextSpan]) -> Vec<UiTextSpan> {
    spans.to_vec()
}

pub(super) fn attrs_for_span<'a>(
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

#[cfg(test)]
#[path = "attributes_tests.rs"]
mod tests;
