use crate::render_model::UiRect;
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTextGlyphBox {
    pub grapheme_index: usize,
    pub byte_range: Range<usize>,
    pub bounds: UiRect,
    pub baseline_y: i32,
    pub text: String,
}

impl UiTextGlyphBox {
    #[must_use]
    pub fn new(
        grapheme_index: usize,
        byte_range: Range<usize>,
        bounds: UiRect,
        baseline_y: i32,
    ) -> Self {
        Self {
            grapheme_index,
            byte_range,
            bounds,
            baseline_y,
            text: String::new(),
        }
    }

    #[must_use]
    pub fn with_text(mut self, value: impl Into<String>) -> Self {
        self.text = value.into();
        self
    }

    #[must_use]
    pub fn caret_x_before(&self) -> i32 {
        self.bounds.x
    }

    #[must_use]
    pub fn caret_x_after(&self) -> i32 {
        self.bounds.x + self.bounds.width as i32
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTextLineBox {
    pub byte_range: Range<usize>,
    pub glyphs: Vec<UiTextGlyphBox>,
}

impl UiTextLineBox {
    #[must_use]
    pub fn new(byte_range: Range<usize>, glyphs: Vec<UiTextGlyphBox>) -> Self {
        Self { byte_range, glyphs }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTextSelectionRange {
    pub anchor: usize,
    pub focus: usize,
}

impl UiTextSelectionRange {
    #[must_use]
    pub const fn new(anchor: usize, focus: usize) -> Self {
        Self { anchor, focus }
    }

    #[must_use]
    pub const fn caret(position: usize) -> Self {
        Self {
            anchor: position,
            focus: position,
        }
    }

    #[must_use]
    pub const fn is_collapsed(self) -> bool {
        self.anchor == self.focus
    }

    #[must_use]
    pub fn ordered(self) -> Range<usize> {
        self.anchor.min(self.focus)..self.anchor.max(self.focus)
    }

    #[must_use]
    pub const fn caret_position(self) -> usize {
        self.focus
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiTextPasteResult {
    pub text: String,
    pub selection: UiTextSelectionRange,
}
