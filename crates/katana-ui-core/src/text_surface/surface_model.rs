use super::props::TextSurfaceProps;
use super::state::TextSurfaceState;
use crate::text_selection::UiTextCaretPosition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSurface {
    pub(super) props: TextSurfaceProps,
    pub(super) state: TextSurfaceState,
    pub(super) caret_affinity: Option<UiTextCaretPosition>,
    pub(super) preferred_column: Option<i32>,
}
