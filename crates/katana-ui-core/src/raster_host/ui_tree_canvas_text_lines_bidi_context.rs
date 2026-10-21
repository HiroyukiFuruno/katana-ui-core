use crate::raster_host::text::RichTextBidiContext;
use std::ops::Range;

#[derive(Clone, Copy)]
enum BidiControl {
    Embedding(char),
    Isolate(char),
}

pub(super) fn bidi_context_for_fragment(
    text: &str,
    levels: &[unicode_bidi::Level],
    fragment_range: &Range<usize>,
) -> Option<RichTextBidiContext> {
    let mut start_stack = Vec::new();
    apply_bidi_controls(&mut start_stack, &text[..fragment_range.start], levels, 0);
    let mut end_stack = start_stack.clone();
    apply_bidi_controls(
        &mut end_stack,
        &text[fragment_range.clone()],
        levels,
        fragment_range.start,
    );
    let prefix: String = start_stack
        .iter()
        .map(|control| control.character())
        .collect();
    let suffix: String = end_stack
        .iter()
        .rev()
        .map(|control| control.closing_character())
        .collect();
    (!prefix.is_empty() || !suffix.is_empty()).then_some(RichTextBidiContext { prefix, suffix })
}

fn apply_bidi_controls(
    stack: &mut Vec<BidiControl>,
    text: &str,
    levels: &[unicode_bidi::Level],
    byte_offset: usize,
) {
    for (offset, character) in text.char_indices() {
        let byte_index = byte_offset + offset;
        match character {
            '\u{202a}' | '\u{202b}' | '\u{202d}' | '\u{202e}' => {
                stack.push(BidiControl::Embedding(character));
            }
            '\u{2066}' | '\u{2067}' => stack.push(BidiControl::Isolate(character)),
            '\u{2068}' => {
                let next_level = levels.get(byte_index + character.len_utf8());
                let isolate = if next_level.is_some_and(unicode_bidi::Level::is_rtl) {
                    '\u{2067}'
                } else {
                    '\u{2066}'
                };
                stack.push(BidiControl::Isolate(isolate));
            }
            '\u{202c}' => {
                if let Some(index) = stack
                    .iter()
                    .rposition(|control| matches!(control, BidiControl::Embedding(_)))
                {
                    stack.remove(index);
                }
            }
            '\u{2069}' => {
                if let Some(index) = stack
                    .iter()
                    .rposition(|control| matches!(control, BidiControl::Isolate(_)))
                {
                    stack.truncate(index);
                }
            }
            _ => {}
        }
    }
}

impl BidiControl {
    const fn character(self) -> char {
        match self {
            Self::Embedding(character) | Self::Isolate(character) => character,
        }
    }

    const fn closing_character(self) -> char {
        match self {
            Self::Embedding(_) => '\u{202c}',
            Self::Isolate(_) => '\u{2069}',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_bidi::BidiInfo;

    #[test]
    fn bidi_context_preserves_open_embeddings_and_isolates() {
        for (opening, expected_prefix, closing) in [
            ('\u{202a}', '\u{202a}', '\u{202c}'),
            ('\u{202b}', '\u{202b}', '\u{202c}'),
            ('\u{202d}', '\u{202d}', '\u{202c}'),
            ('\u{202e}', '\u{202e}', '\u{202c}'),
            ('\u{2066}', '\u{2066}', '\u{2069}'),
            ('\u{2067}', '\u{2067}', '\u{2069}'),
            ('\u{2068}', '\u{2066}', '\u{2069}'),
        ] {
            let text = format!("{opening}CODE{closing}");
            let bidi = BidiInfo::new(&text, None);
            let start = opening.len_utf8();
            let context = bidi_context_for_fragment(&text, &bidi.levels, &(start..start + 4))
                .expect("open bidi control must shape the fragment");

            assert_eq!(expected_prefix.to_string(), context.prefix);
            assert_eq!(closing.to_string(), context.suffix);
        }
    }

    #[test]
    fn fsi_resolves_to_rtl_isolate_and_pdi_does_not_leak_to_following_fragment() {
        let text = "\u{2068}אבג\u{2069}CODE";
        let bidi = BidiInfo::new(text, None);
        let hebrew_start = '\u{2068}'.len_utf8();
        let hebrew_end = hebrew_start + "אבג".len();
        let code_start = hebrew_end + '\u{2069}'.len_utf8();

        let hebrew_context =
            bidi_context_for_fragment(text, &bidi.levels, &(hebrew_start..hebrew_end))
                .expect("FSI must retain the resolved RTL isolate while rendering its fragment");
        assert_eq!("\u{2067}", hebrew_context.prefix);
        assert_eq!("\u{2069}", hebrew_context.suffix);
        assert!(
            bidi_context_for_fragment(text, &bidi.levels, &(code_start..text.len())).is_none(),
            "PDI must close the isolate before the next fragment",
        );
    }
}
