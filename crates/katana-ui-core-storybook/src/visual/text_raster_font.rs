use super::text_raster::RichTextRasterSpan;
use super::text_raster_color::text_color;
use cosmic_text::{Attrs, Family, Style as FontStyle, Weight};
use katana_ui_core::theme::{FontFamily, FontToken};

const REGULAR_WEIGHT: u16 = 400;
const KATANA_MARKDOWN_PROPORTIONAL_WEIGHT: u16 = 300;
const DOCUMENT_BODY_FONT_ROLE: &str = "document-body";
const DOCUMENT_CODE_FONT_ROLE: &str = "document-code";
#[cfg(target_os = "macos")]
pub(super) const APPLE_COLOR_EMOJI_FONT_FAMILY: &str = "Apple Color Emoji";
#[cfg(target_os = "macos")]
pub(super) const MARKDOWN_PROPORTIONAL_FONT_FAMILY: &str =
    "\u{30d2}\u{30e9}\u{30ae}\u{30ce}\u{89d2}\u{30b4}\u{30b7}\u{30c3}\u{30af}";
#[cfg(target_os = "macos")]
pub(super) const MARKDOWN_MONOSPACE_FONT_FAMILY: &str = "Menlo";

pub(super) fn attrs_for_text<'a>(
    font: &'a FontToken,
    text: &str,
    emoji: bool,
    italic: bool,
) -> Attrs<'a> {
    Attrs::new()
        .family(family_for_font(font, text, emoji))
        .weight(Weight(weight_for_font(font, emoji)))
        .style(if italic {
            FontStyle::Italic
        } else {
            FontStyle::Normal
        })
}

pub(super) fn attrs_for_rich_span<'a>(
    span: &'a RichTextRasterSpan<'a>,
    _scale_factor: f32,
) -> Attrs<'a> {
    attrs_for_text(span.font, span.text, span.emoji, span.style.is_italic())
        .color(text_color(span.style.color()))
}

fn family_for_font(font: &FontToken, text: &str, emoji: bool) -> Family<'static> {
    if font.name == DOCUMENT_BODY_FONT_ROLE && font.family == FontFamily::Proportional && !emoji {
        return katana_markdown_proportional_family();
    }
    if font.name == DOCUMENT_CODE_FONT_ROLE && font.family == FontFamily::Monospace && !emoji {
        return katana_markdown_monospace_family();
    }
    family_for_text(font.family, text, emoji)
}

fn weight_for_font(font: &FontToken, emoji: bool) -> u16 {
    if font.name == DOCUMENT_BODY_FONT_ROLE && font.family == FontFamily::Proportional && !emoji {
        if font.weight > REGULAR_WEIGHT {
            return font.weight;
        }
        return KATANA_MARKDOWN_PROPORTIONAL_WEIGHT;
    }
    font.weight.max(REGULAR_WEIGHT)
}

fn family_for_text(family: FontFamily, text: &str, emoji: bool) -> Family<'static> {
    if emoji {
        return os_emoji_font_family();
    }
    match family {
        FontFamily::Proportional => os_proportional_font_family(),
        FontFamily::Monospace if text.is_ascii() => os_monospace_font_family(),
        FontFamily::Monospace => os_proportional_font_family(),
    }
}

#[cfg(target_os = "macos")]
fn os_emoji_font_family() -> Family<'static> {
    Family::Name(APPLE_COLOR_EMOJI_FONT_FAMILY)
}

#[cfg(not(target_os = "macos"))]
fn os_emoji_font_family() -> Family<'static> {
    Family::SansSerif
}

#[cfg(target_os = "macos")]
fn katana_markdown_proportional_family() -> Family<'static> {
    Family::Name(MARKDOWN_PROPORTIONAL_FONT_FAMILY)
}

#[cfg(not(target_os = "macos"))]
fn katana_markdown_proportional_family() -> Family<'static> {
    Family::SansSerif
}

#[cfg(target_os = "macos")]
fn katana_markdown_monospace_family() -> Family<'static> {
    Family::Name(MARKDOWN_MONOSPACE_FONT_FAMILY)
}

#[cfg(not(target_os = "macos"))]
fn katana_markdown_monospace_family() -> Family<'static> {
    Family::Monospace
}

#[cfg(target_os = "macos")]
fn os_proportional_font_family() -> Family<'static> {
    Family::SansSerif
}

#[cfg(not(target_os = "macos"))]
fn os_proportional_font_family() -> Family<'static> {
    Family::SansSerif
}

#[cfg(target_os = "macos")]
fn os_monospace_font_family() -> Family<'static> {
    Family::Monospace
}

#[cfg(not(target_os = "macos"))]
fn os_monospace_font_family() -> Family<'static> {
    Family::Monospace
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;

    #[test]
    fn proportional_text_uses_export_surface_default_family() {
        assert_eq!(
            Family::SansSerif,
            family_for_text(FontFamily::Proportional, "KatanA", false)
        );
    }

    #[test]
    fn document_body_text_uses_katana_preview_family() {
        let font = FontToken {
            name: DOCUMENT_BODY_FONT_ROLE.to_string(),
            family: FontFamily::Proportional,
            size: 14.0,
            weight: 400,
        };

        assert_eq!(
            Family::Name(MARKDOWN_PROPORTIONAL_FONT_FAMILY),
            family_for_font(&font, "KatanA", false)
        );
    }

    #[test]
    fn document_body_text_uses_katana_preview_light_weight() {
        let font = FontToken {
            name: DOCUMENT_BODY_FONT_ROLE.to_string(),
            family: FontFamily::Proportional,
            size: 14.0,
            weight: 400,
        };

        assert_eq!(
            KATANA_MARKDOWN_PROPORTIONAL_WEIGHT,
            weight_for_font(&font, false)
        );
    }

    #[test]
    fn document_body_bold_text_keeps_bold_weight() {
        let font = FontToken {
            name: DOCUMENT_BODY_FONT_ROLE.to_string(),
            family: FontFamily::Proportional,
            size: 14.0,
            weight: 700,
        };

        assert_eq!(700, weight_for_font(&font, false));
    }

    #[test]
    fn monospace_ascii_uses_export_surface_monospace_family() {
        assert_eq!(
            Family::Monospace,
            family_for_text(FontFamily::Monospace, "fn main() {}", false)
        );
    }

    #[test]
    fn document_code_text_uses_katana_preview_monospace_family() {
        let font = FontToken {
            name: DOCUMENT_CODE_FONT_ROLE.to_string(),
            family: FontFamily::Monospace,
            size: 12.0,
            weight: 400,
        };

        assert_eq!(
            Family::Name(MARKDOWN_MONOSPACE_FONT_FAMILY),
            family_for_font(&font, "fn main() {}", false)
        );
    }

    #[test]
    fn emoji_text_uses_os_emoji_family() {
        assert_eq!(
            Family::Name(APPLE_COLOR_EMOJI_FONT_FAMILY),
            family_for_text(FontFamily::Proportional, "🌏", true)
        );
    }
}
