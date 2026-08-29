use super::props::TextSurfaceProps;
use super::state::TextSurfaceState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSurface {
    pub(super) props: TextSurfaceProps,
    pub(super) state: TextSurfaceState,
}
