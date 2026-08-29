use super::typed_text::{UiEmojiTextSegment, UiEmojiTextSegments};
use unicode_segmentation::UnicodeSegmentation;

impl UiEmojiTextSegments {
    #[must_use]
    pub fn split(text: impl AsRef<str>) -> Vec<UiEmojiTextSegment> {
        let text = text.as_ref();
        if text.is_empty() {
            return Vec::new();
        }

        let mut segments = Vec::new();
        let mut start = 0usize;
        let mut current = None;

        for (index, grapheme) in text.grapheme_indices(true) {
            let is_emoji = is_emoji_grapheme(grapheme);
            match current {
                None => current = Some(is_emoji),
                Some(emoji) if emoji == is_emoji => {}
                Some(emoji) => {
                    segments.push(UiEmojiTextSegment {
                        text: text[start..index].to_string(),
                        emoji,
                    });
                    start = index;
                    current = Some(is_emoji);
                }
            }
        }

        if let Some(emoji) = current {
            segments.push(UiEmojiTextSegment {
                text: text[start..].to_string(),
                emoji,
            });
        }

        segments
    }
}

fn is_emoji_grapheme(grapheme: &str) -> bool {
    if matches!(grapheme, "☆" | "⭐") {
        return false;
    }
    grapheme.chars().any(is_emoji_part)
}

fn is_emoji_part(character: char) -> bool {
    let code = character as u32;
    is_emoji_code(code) || is_emoji_range(code)
}

fn is_emoji_code(code: u32) -> bool {
    matches!(
        code,
        0x00A9
            | 0x00AE
            | 0x200D
            | 0x203C
            | 0x2049
            | 0x20E3
            | 0x2122
            | 0x2139
            | 0x23CF
            | 0x24C2
            | 0x25B6
            | 0x25C0
            | 0x3030
            | 0x303D
            | 0x3297
            | 0x3299
            | 0xFE0F
    )
}

fn is_emoji_range(code: u32) -> bool {
    matches!(
        code,
        0x2194..=0x21AA
            | 0x231A..=0x2328
            | 0x23E9..=0x23FA
            | 0x25AA..=0x25AB
            | 0x25FB..=0x25FE
            | 0x2600..=0x27BF
            | 0x2934..=0x2935
            | 0x2B05..=0x2B55
            | 0x1F000..=0x1FAFF
    )
}

#[cfg(test)]
mod tests {
    use super::UiEmojiTextSegments;

    #[test]
    fn split_marks_raw_emoji_runs_without_marking_surrounding_text() {
        let segments = UiEmojiTextSegments::split("Emoji: 🦀 text ⚠️");

        assert_eq!(
            segments
                .iter()
                .map(|segment| (segment.text.as_str(), segment.emoji))
                .collect::<Vec<_>>(),
            vec![
                ("Emoji: ", false),
                ("🦀", true),
                (" text ", false),
                ("⚠️", true),
            ]
        );
    }

    #[test]
    fn split_keeps_star_variation_selector_as_one_emoji_run() {
        let segments = UiEmojiTextSegments::split("Star ⭐️ mark");

        assert_eq!(
            segments
                .iter()
                .map(|segment| (segment.text.as_str(), segment.emoji))
                .collect::<Vec<_>>(),
            vec![("Star ", false), ("⭐️", true), (" mark", false)]
        );
    }

    #[test]
    fn split_keeps_text_presentation_stars_out_of_color_emoji_runs() {
        let segments = UiEmojiTextSegments::split("Stars ☆ ⭐ mark");

        assert_eq!(
            segments
                .iter()
                .map(|segment| (segment.text.as_str(), segment.emoji))
                .collect::<Vec<_>>(),
            vec![("Stars ☆ ⭐ mark", false)]
        );
    }
}
