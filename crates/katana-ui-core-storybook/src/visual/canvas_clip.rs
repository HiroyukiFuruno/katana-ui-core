#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CanvasClip {
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) width: usize,
    pub(super) height: usize,
}

impl CanvasClip {
    pub(super) fn from_rect(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        canvas_width: usize,
        canvas_height: usize,
    ) -> Option<Self> {
        let right = x.saturating_add(width).min(canvas_width);
        let bottom = y.saturating_add(height).min(canvas_height);
        if x >= right || y >= bottom {
            return None;
        }
        Some(Self {
            x,
            y,
            width: right - x,
            height: bottom - y,
        })
    }

    pub(super) fn intersect(self, other: Self) -> Option<Self> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());
        if x >= right || y >= bottom {
            return None;
        }
        Some(Self {
            x,
            y,
            width: right - x,
            height: bottom - y,
        })
    }

    pub(super) const fn right(self) -> usize {
        self.x + self.width
    }

    pub(super) const fn bottom(self) -> usize {
        self.y + self.height
    }

    pub(super) fn contains(self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
}
