use unicode_segmentation::UnicodeSegmentation;

/// UTF-8 の論理一致と、表示用に grapheme 全体へ広げた強調範囲。
/// 空の一致は論理位置として保持し、表示強調には含めない。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHighlightProjection {
    logical_ranges: Vec<(usize, usize)>,
    display_ranges: Vec<(usize, usize)>,
}

/// 一致範囲が文字列の UTF-8 byte 範囲として不正な理由。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchHighlightProjectionError {
    Reversed {
        index: usize,
        start: usize,
        end: usize,
    },
    OutOfBounds {
        index: usize,
        start: usize,
        end: usize,
        len: usize,
    },
    InvalidUtf8Boundary {
        index: usize,
        start: usize,
        end: usize,
    },
}

impl std::fmt::Display for SearchHighlightProjectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "search highlight projection rejected: {self:?}")
    }
}
impl std::error::Error for SearchHighlightProjectionError {}

impl SearchHighlightProjection {
    /// 非空の一致が交差する全 grapheme を強調し、空の一致は強調しない。
    /// 非空一致の順序と重複を表示範囲にも保持する。空一致は論理範囲にのみ残る。
    /// 複数の不正がある場合は、入力順に逆向き、範囲外、UTF-8 境界の順で検査する。
    ///
    /// ```
    /// use katana_ui_core::egui::text_command_surface::SearchHighlightProjection;
    /// let projection = SearchHighlightProjection::new("⭐️", &[(3, 6), (6, 6)]).unwrap();
    /// assert_eq!(projection.logical_ranges(), &[(3, 6), (6, 6)]);
    /// assert_eq!(projection.display_ranges(), &[(0, 6)]);
    /// ```
    pub fn new(
        text: &str,
        matches: &[(usize, usize)],
    ) -> Result<Self, SearchHighlightProjectionError> {
        let mut display_ranges = Vec::new();
        for (index, &(start, end)) in matches.iter().enumerate() {
            if start > end {
                return Err(SearchHighlightProjectionError::Reversed { index, start, end });
            }
            if end > text.len() {
                return Err(SearchHighlightProjectionError::OutOfBounds {
                    index,
                    start,
                    end,
                    len: text.len(),
                });
            }
            if !text.is_char_boundary(start) || !text.is_char_boundary(end) {
                return Err(SearchHighlightProjectionError::InvalidUtf8Boundary {
                    index,
                    start,
                    end,
                });
            }
            if start == end {
                continue;
            }
            let mut begin = 0;
            let mut finish = text.len();
            for (offset, _) in text.grapheme_indices(true) {
                if offset <= start {
                    begin = offset;
                }
                if offset >= end {
                    finish = offset;
                    break;
                }
            }
            display_ranges.push((begin, finish));
        }
        Ok(Self {
            logical_ranges: matches.to_vec(),
            display_ranges,
        })
    }

    #[must_use]
    pub fn logical_ranges(&self) -> &[(usize, usize)] {
        &self.logical_ranges
    }
    #[must_use]
    pub fn display_ranges(&self) -> &[(usize, usize)] {
        &self.display_ranges
    }
}
