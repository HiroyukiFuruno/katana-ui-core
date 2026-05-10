/// Split direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

/// Properties for `SplitPane`.
#[derive(Debug, Clone)]
pub struct SplitPaneProps {
    pub direction: Direction,
    pub ratio: f32,
    pub min_ratio: f32,
    pub max_ratio: f32,
}
