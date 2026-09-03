#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LayoutRect {
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) width: usize,
    pub(super) height: usize,
}

impl LayoutRect {
    pub(super) const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub(super) const fn right(self) -> usize {
        self.x + self.width
    }

    pub(super) const fn bottom(self) -> usize {
        self.y + self.height
    }
}
