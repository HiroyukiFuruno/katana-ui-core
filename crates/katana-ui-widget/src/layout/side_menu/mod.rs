mod expand_panel;
mod helpers;
mod interaction;
mod ops;
mod overlay_effect;
mod render;
#[cfg(test)]
mod tests;
mod types;
mod view;

pub use types::{
    DEFAULT_EXPANDED_PANEL_WIDTH, DEFAULT_HOVER_HANDLE_WIDTH, DEFAULT_SIDE_MENU_WIDTH,
    SideMenuExpandMode, SideMenuItem, SideMenuItemPlacement, SideMenuItemPop, SideMenuPopMode,
    SideMenuProps, SideMenuSide,
};
pub use view::SideMenu;
