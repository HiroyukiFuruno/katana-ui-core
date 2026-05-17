use super::types::{
    DEFAULT_EXPANDED_PANEL_WIDTH, DEFAULT_HOVER_HANDLE_WIDTH, DEFAULT_SIDE_MENU_WIDTH,
    SIDE_MENU_CLICK_COOLDOWN_MS, SIDE_MENU_HOVER_DELAY_MS, SIDE_MENU_PANEL_GAP, SideMenuPopMode,
    SideMenuSide,
};
use super::view::SideMenu;
use crate::primitive::icon::IconSource;

const TEST_ICON: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1 1'></svg>";

const _: () = {
    assert!(DEFAULT_SIDE_MENU_WIDTH >= DEFAULT_HOVER_HANDLE_WIDTH);
    assert!(DEFAULT_SIDE_MENU_WIDTH < DEFAULT_EXPANDED_PANEL_WIDTH / 3.0);
};

const _: () = {
    assert!(SIDE_MENU_HOVER_DELAY_MS >= 200);
    assert!(SIDE_MENU_CLICK_COOLDOWN_MS >= SIDE_MENU_HOVER_DELAY_MS);
};

#[test]
fn expansion_panel_follows_side_direction() {
    let left_x = SideMenuSide::Left.expansion_panel_x(10.0, 52.0, 240.0);
    let right_x = SideMenuSide::Right.expansion_panel_x(300.0, 52.0, 240.0);

    assert_eq!(left_x, 10.0 + 52.0 + SIDE_MENU_PANEL_GAP);
    assert_eq!(right_x, 300.0 - 240.0 - SIDE_MENU_PANEL_GAP);
}

#[test]
fn initial_pop_records_open_pop_state() {
    let menu = SideMenu::new([super::types::SideMenuItem::new(
        IconSource::SvgBytes(TEST_ICON),
        || {},
    )])
    .initial_pop(0, SideMenuPopMode::Popover);

    assert_eq!(
        menu.initial_pop_state(),
        Some((0, SideMenuPopMode::Popover))
    );
}
