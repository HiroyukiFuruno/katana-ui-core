use cosmic_text::FontSystem;

#[cfg(target_os = "macos")]
const MARKDOWN_PROPORTIONAL_FONT_CANDIDATES: &[&str] = &[
    "/System/Library/Fonts/\u{30d2}\u{30e9}\u{30ae}\u{30ce}\u{89d2}\u{30b4}\u{30b7}\u{30c3}\u{30af} W3.ttc",
    "/System/Library/Fonts/Hiragino Sans GB.ttc",
];

#[cfg(target_os = "macos")]
const MARKDOWN_MONOSPACE_FONT_CANDIDATES: &[&str] = &[
    "/System/Library/Fonts/Menlo.ttc",
    "/System/Library/Fonts/SFMono-Regular.otf",
    "/System/Library/Fonts/Monaco.ttf",
];

#[cfg(target_os = "macos")]
const MARKDOWN_EMOJI_FONT_CANDIDATES: &[&str] = &["/System/Library/Fonts/Apple Color Emoji.ttc"];

#[cfg(not(target_os = "macos"))]
const MARKDOWN_PROPORTIONAL_FONT_CANDIDATES: &[&str] = &[];

#[cfg(not(target_os = "macos"))]
const MARKDOWN_MONOSPACE_FONT_CANDIDATES: &[&str] = &[];

#[cfg(not(target_os = "macos"))]
const MARKDOWN_EMOJI_FONT_CANDIDATES: &[&str] = &[];

pub(super) fn font_system_with_markdown_fonts() -> FontSystem {
    let mut font_system = FontSystem::new();
    for path in MARKDOWN_PROPORTIONAL_FONT_CANDIDATES
        .iter()
        .chain(MARKDOWN_MONOSPACE_FONT_CANDIDATES)
        .chain(MARKDOWN_EMOJI_FONT_CANDIDATES)
    {
        let _ = font_system.db_mut().load_font_file(path);
    }
    font_system
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "macos")]
    #[test]
    fn loads_markdown_primary_font_family() {
        let font_system = font_system_with_markdown_fonts();
        let family = crate::visual::text_raster_font::MARKDOWN_PROPORTIONAL_FONT_FAMILY;

        assert!(
            font_system
                .db()
                .faces()
                .any(|face| face.families.iter().any(|(name, _)| name == family)),
            "expected markdown proportional family to be loaded: {family}"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn loads_katana_markdown_monospace_family() {
        let font_system = font_system_with_markdown_fonts();
        let family = crate::visual::text_raster_font::MARKDOWN_MONOSPACE_FONT_FAMILY;

        assert!(
            font_system
                .db()
                .faces()
                .any(|face| face.families.iter().any(|(name, _)| name == family)),
            "expected markdown monospace family to be loaded: {family}"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn loads_katana_markdown_emoji_family() {
        let font_system = font_system_with_markdown_fonts();
        let family = crate::visual::text_raster_font::APPLE_COLOR_EMOJI_FONT_FAMILY;

        assert!(
            font_system
                .db()
                .faces()
                .any(|face| face.families.iter().any(|(name, _)| name == family)),
            "expected markdown emoji family to be loaded: {family}"
        );
    }
}
