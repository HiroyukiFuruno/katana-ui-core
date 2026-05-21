use super::render::{CANVAS_HEIGHT, VIEWPORT_HEIGHT};
#[cfg(test)]
use super::render::{HEIGHT, WIDTH};

pub(super) const CONTENT_HEIGHT: usize = CANVAS_HEIGHT;
pub(super) const MAX_SCROLL_Y: usize = CONTENT_HEIGHT - VIEWPORT_HEIGHT;
pub(super) const SCROLL_STEP: usize = 80;

pub(super) const NAV_WIDTH: usize = 280;
pub(super) const NAV_FIRST_ROW_Y: usize = 132;
pub(super) const NAV_ROW_X: usize = 14;
pub(super) const NAV_ROW_WIDTH: usize = 252;
pub(super) const NAV_ROW_HEIGHT: usize = 24;
pub(super) const NAV_ROW_STEP: usize = 28;
pub(super) const BRAND_X: usize = 22;
pub(super) const THEME_CONTROL_Y: usize = 64;
pub(super) const THEME_CONTROL_WIDTH: usize = 118;
pub(super) const THEME_CONTROL_HEIGHT: usize = 24;
pub(super) const THEME_CONTROL_GAP: usize = 0;
pub(super) const SCROLLBAR_CONTROL_Y: usize = 96;
pub(super) const SCROLLBAR_CONTROL_WIDTH: usize = 118;
pub(super) const SCROLLBAR_CONTROL_HEIGHT: usize = 22;
pub(super) const PREVIEW_X: usize = 310;
pub(super) const PRESET_ACTIVE_Y: usize = 104;
pub(super) const PRESET_INACTIVE_Y: usize = PRESET_ACTIVE_Y;
pub(super) const PRESET_WIDTH: usize = 106;
pub(super) const PRESET_ACTIVE_HEIGHT: usize = 32;
pub(super) const PRESET_INACTIVE_HEIGHT: usize = PRESET_ACTIVE_HEIGHT;
pub(super) const PRESET_GAP: usize = 0;
pub(super) const PRESET_TEXT_X_OFFSET: usize = 14;
pub(super) const PRESET_TAB_COUNT: usize = 7;
#[cfg(test)]
pub(super) const PRESET_INTERACTIVE_INDEX: usize = 1;
#[cfg(test)]
pub(super) const PRESET_CONTAINER_PADDING: usize = 5;
pub(super) const PRESET_ACTIVE_BOTTOM_BORDER_HEIGHT: usize = 3;
pub(super) const INSPECTOR_X: usize = 1072;
pub(super) const INSPECTOR_Y: usize = 22;
pub(super) const INSPECTOR_WIDTH: usize = 334;
pub(super) const INSPECTOR_HEIGHT: usize = VIEWPORT_HEIGHT - 44;
const INSPECTOR_SECTION_X_OFFSET: usize = 28;
const INSPECTOR_FIRST_ROW_Y_OFFSET: usize = 112;
const INSPECTOR_SETTING_ROW_WIDTH_OFFSET: usize = 56;
const INSPECTOR_SETTING_ROW_HEIGHT: usize = 22;

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

    pub(super) fn right(self) -> usize {
        self.x + self.width
    }

    pub(super) fn bottom(self) -> usize {
        self.y + self.height
    }

    pub(super) fn contains(self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    #[cfg(test)]
    pub(super) fn overlaps(self, other: Self) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    #[cfg(test)]
    pub(super) fn inside_canvas(self) -> bool {
        self.right() <= WIDTH && self.bottom() <= HEIGHT
    }

    #[cfg(test)]
    pub(super) fn inside_content(self) -> bool {
        self.right() <= WIDTH && self.bottom() <= CONTENT_HEIGHT
    }
}

pub(super) fn light_theme_rect() -> LayoutRect {
    LayoutRect::new(
        BRAND_X,
        THEME_CONTROL_Y,
        THEME_CONTROL_WIDTH,
        THEME_CONTROL_HEIGHT,
    )
}

pub(super) fn dark_theme_rect() -> LayoutRect {
    LayoutRect::new(
        BRAND_X + THEME_CONTROL_WIDTH + THEME_CONTROL_GAP,
        THEME_CONTROL_Y,
        THEME_CONTROL_WIDTH,
        THEME_CONTROL_HEIGHT,
    )
}

pub(super) fn scrollbar_on_rect() -> LayoutRect {
    LayoutRect::new(
        BRAND_X,
        SCROLLBAR_CONTROL_Y,
        SCROLLBAR_CONTROL_WIDTH,
        SCROLLBAR_CONTROL_HEIGHT,
    )
}

pub(super) fn scrollbar_off_rect() -> LayoutRect {
    LayoutRect::new(
        BRAND_X + SCROLLBAR_CONTROL_WIDTH + THEME_CONTROL_GAP,
        SCROLLBAR_CONTROL_Y,
        SCROLLBAR_CONTROL_WIDTH,
        SCROLLBAR_CONTROL_HEIGHT,
    )
}

pub(super) fn preset_tab_rect(index: usize) -> LayoutRect {
    LayoutRect::new(
        PREVIEW_X + index * (PRESET_WIDTH + PRESET_GAP),
        PRESET_ACTIVE_Y,
        PRESET_WIDTH,
        PRESET_ACTIVE_HEIGHT,
    )
}

pub(super) fn preset_tab_visual_rect(index: usize, active: bool) -> LayoutRect {
    let y = if active {
        PRESET_ACTIVE_Y
    } else {
        PRESET_INACTIVE_Y
    };
    let height = if active {
        PRESET_ACTIVE_HEIGHT
    } else {
        PRESET_INACTIVE_HEIGHT
    };
    LayoutRect::new(
        PREVIEW_X + index * (PRESET_WIDTH + PRESET_GAP),
        y,
        PRESET_WIDTH,
        height,
    )
}

#[cfg(test)]
pub(super) fn preset_container_rect() -> LayoutRect {
    LayoutRect::new(
        PREVIEW_X - PRESET_CONTAINER_PADDING,
        PRESET_ACTIVE_Y - PRESET_CONTAINER_PADDING,
        PRESET_TAB_COUNT * PRESET_WIDTH
            + (PRESET_TAB_COUNT - 1) * PRESET_GAP
            + PRESET_CONTAINER_PADDING * 2,
        PRESET_ACTIVE_HEIGHT + PRESET_CONTAINER_PADDING * 2,
    )
}

pub(super) fn navigation_hit_rect(row_y: usize) -> LayoutRect {
    LayoutRect::new(NAV_ROW_X, row_y, NAV_ROW_WIDTH, NAV_ROW_HEIGHT)
}

pub(super) fn button_setting_hit_rect() -> LayoutRect {
    LayoutRect::new(
        INSPECTOR_X + INSPECTOR_SECTION_X_OFFSET,
        INSPECTOR_Y + INSPECTOR_FIRST_ROW_Y_OFFSET,
        INSPECTOR_WIDTH - INSPECTOR_SETTING_ROW_WIDTH_OFFSET,
        INSPECTOR_SETTING_ROW_HEIGHT,
    )
}

#[cfg(test)]
pub(super) fn inspector_rect() -> LayoutRect {
    LayoutRect::new(INSPECTOR_X, INSPECTOR_Y, INSPECTOR_WIDTH, INSPECTOR_HEIGHT)
}

#[cfg(test)]
mod tests;
