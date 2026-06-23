use super::dedicated_dod_common::Rect;
use super::layout_metrics::LayoutRect;
use super::screen_state_tabs::{TabsScreenAction, TabsScreenTab};

pub(super) const ADD_CLOSE_PRESET_INDEX: usize = 1;
pub(super) const PIN_PRESET_INDEX: usize = 2;
pub(super) const MOVE_PRESET_INDEX: usize = 3;
pub(super) const GROUP_PRESET_INDEX: usize = 4;
pub(super) const OVERFLOW_PRESET_INDEX: usize = 5;
pub(super) const STRIP_X: usize = 30;
pub(super) const STRIP_Y: usize = 38;
pub(super) const STRIP_WIDTH: usize = 470;
pub(super) const STRIP_HEIGHT: usize = 40;
pub(super) const STRIP_LEADING_INSET: usize = 8;
pub(super) const TAB_Y: usize = 45;
pub(super) const TAB_HEIGHT: usize = 26;
pub(super) const TAB_GAP: usize = 3;
pub(super) const TAB_LABEL_X: usize = 7;
pub(super) const TAB_CLOSE_SIZE: usize = 8;
pub(super) const TAB_CLOSE_AREA: usize = 18;
pub(super) const CLOSE_ICON_X_OFFSET: usize = 5;
pub(super) const CLOSE_ICON_Y_OFFSET: usize = 9;
pub(super) const PIN_ICON_SIZE: usize = 9;
pub(super) const PIN_ICON_X_OFFSET: usize = 4;
pub(super) const PIN_ICON_Y_OFFSET: usize = 8;
pub(super) const PIN_HEAD_X_OFFSET: usize = 3;
pub(super) const PIN_HEAD_WIDTH: usize = 3;
pub(super) const PIN_CROSS_Y_OFFSET: usize = 2;
pub(super) const PIN_STEM_X_OFFSET: usize = 4;
pub(super) const PIN_STEM_WIDTH: usize = 1;
pub(super) const PIN_STEM_HEIGHT: usize = 5;
pub(super) const DIRTY_SIZE: usize = 5;
pub(super) const DIRTY_RIGHT_OFFSET: usize = 27;
pub(super) const DIRTY_Y_OFFSET: usize = 6;
pub(super) const GROUP_HEADER_WIDTH: usize = 48;
pub(super) const GROUP_DOT_SIZE: usize = 8;
pub(super) const GROUP_DOT_X: usize = 7;
pub(super) const GROUP_DOT_Y: usize = 9;
pub(super) const GROUP_TEXT_X: usize = 18;
pub(super) const GROUP_UNDERLINE_HEIGHT: usize = 2;
pub(super) const CONTROLS_Y: usize = 84;
pub(super) const CONTROL_HEIGHT: usize = 20;
pub(super) const STATUS_X: usize = 42;
pub(super) const STATUS_Y: usize = 110;
pub(super) const STATUS_WIDTH: usize = 230;
pub(super) const STATUS_HEIGHT: usize = 18;
pub(super) const STATUS_TEXT_X_PADDING: usize = 8;
pub(super) const STATUS_TEXT_RIGHT_PADDING: usize = 12;
pub(super) const OVERFLOW_MENU_X: usize = 352;
pub(super) const OVERFLOW_MENU_Y: usize = 82;
pub(super) const OVERFLOW_MENU_WIDTH: usize = 132;
pub(super) const OVERFLOW_MENU_HEIGHT: usize = 42;
pub(super) const OVERFLOW_MENU_TEXT_X: usize = 8;
pub(super) const OVERFLOW_MENU_FIRST_TEXT_Y: usize = 8;
pub(super) const OVERFLOW_MENU_SECOND_TEXT_Y: usize = 24;
const OVERFLOW_BUTTON_X: usize = 296;
const OVERFLOW_BUTTON_WIDTH: usize = 34;
const ADD_BUTTON_X: usize = 42;
const ADD_BUTTON_WIDTH: usize = 24;
const CLOSE_BUTTON_X: usize = 72;
const CLOSE_BUTTON_WIDTH: usize = 48;
const MOVE_BUTTON_X: usize = 126;
const MOVE_BUTTON_WIDTH: usize = 52;
const GROUP_BUTTON_X: usize = 184;
const GROUP_BUTTON_WIDTH: usize = 54;
const PIN_BUTTON_X: usize = 244;
const PIN_BUTTON_WIDTH: usize = 46;
const TERMINAL_TAB_WIDTH: usize = 80;
const MEDIUM_TAB_WIDTH: usize = 74;
const PINNED_TAB_WIDTH: usize = 72;
const DEFAULT_TAB_WIDTH: usize = 68;
const CONTROL_COUNT: usize = 6;

pub(super) fn control_rects(
    origin_x: usize,
    origin_y: usize,
) -> [(TabsScreenAction, LayoutRect); CONTROL_COUNT] {
    [
        control_rect(
            TabsScreenAction::AddTab,
            origin_x,
            origin_y,
            ADD_BUTTON_X,
            ADD_BUTTON_WIDTH,
        ),
        control_rect(
            TabsScreenAction::CloseActive,
            origin_x,
            origin_y,
            CLOSE_BUTTON_X,
            CLOSE_BUTTON_WIDTH,
        ),
        control_rect(
            TabsScreenAction::MoveActiveRight,
            origin_x,
            origin_y,
            MOVE_BUTTON_X,
            MOVE_BUTTON_WIDTH,
        ),
        control_rect(
            TabsScreenAction::GroupActive,
            origin_x,
            origin_y,
            GROUP_BUTTON_X,
            GROUP_BUTTON_WIDTH,
        ),
        control_rect(
            TabsScreenAction::TogglePinActive,
            origin_x,
            origin_y,
            PIN_BUTTON_X,
            PIN_BUTTON_WIDTH,
        ),
        control_rect(
            TabsScreenAction::ToggleOverflow,
            origin_x,
            origin_y,
            OVERFLOW_BUTTON_X,
            OVERFLOW_BUTTON_WIDTH,
        ),
    ]
}

pub(super) fn overflow_button_rect(origin_x: usize, origin_y: usize) -> Rect {
    Rect::new(
        origin_x + OVERFLOW_BUTTON_X,
        origin_y + CONTROLS_Y,
        OVERFLOW_BUTTON_WIDTH,
        CONTROL_HEIGHT,
    )
}

pub(super) fn rect_to_common(rect: LayoutRect) -> Rect {
    Rect::new(rect.x, rect.y, rect.width, rect.height)
}

pub(super) fn tab_width(tab: &TabsScreenTab) -> usize {
    match tab.title.as_str() {
        "terminal" => TERMINAL_TAB_WIDTH,
        "scratch" | "preview" => MEDIUM_TAB_WIDTH,
        "readme" => PINNED_TAB_WIDTH,
        _ => DEFAULT_TAB_WIDTH,
    }
}

fn control_rect(
    action: TabsScreenAction,
    origin_x: usize,
    origin_y: usize,
    rel_x: usize,
    width: usize,
) -> (TabsScreenAction, LayoutRect) {
    (
        action,
        LayoutRect::new(
            origin_x + rel_x,
            origin_y + CONTROLS_Y,
            width,
            CONTROL_HEIGHT,
        ),
    )
}
