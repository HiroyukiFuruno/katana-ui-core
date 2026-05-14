use floem::View;
use std::rc::Rc;

use crate::primitive::icon::IconSource;

pub const DEFAULT_EXPANDED_PANEL_WIDTH: f32 = 240.0;
pub const DEFAULT_HOVER_HANDLE_WIDTH: f32 = 8.0;
pub const DEFAULT_SIDE_MENU_WIDTH: f32 = 52.0;
pub const SIDE_MENU_PANEL_GAP: f32 = 2.0;
pub const SIDE_MENU_HOVER_DELAY_MS: u64 = 250;
pub const SIDE_MENU_CLICK_COOLDOWN_MS: u64 = 250;

/// Side placement of the side menu.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SideMenuSide {
    #[default]
    Left,
    Right,
}

impl SideMenuSide {
    #[must_use]
    pub fn expansion_panel_x(self, rail_x: f32, rail_width: f32, panel_width: f32) -> f32 {
        match self {
            Self::Left => rail_x + rail_width + SIDE_MENU_PANEL_GAP,
            Self::Right => rail_x - panel_width - SIDE_MENU_PANEL_GAP,
        }
    }
}

/// Expansion strategy for the menu container.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SideMenuExpandMode {
    #[default]
    Fixed,
    Hover {
        collapsed_width: f32,
    },
}

/// How an item popout is displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideMenuPopMode {
    Modal,
    Popover,
    Expand,
}

/// Vertical group placement inside the side rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SideMenuItemPlacement {
    #[default]
    Top,
    Bottom,
}

pub type SideMenuIconAction = Rc<dyn Fn()>;
pub type SideMenuPopContentFactory = dyn Fn() -> Box<dyn View>;

/// Pop content definition for one icon.
#[derive(Clone)]
pub struct SideMenuItemPop {
    pub mode: SideMenuPopMode,
    pub content: Rc<SideMenuPopContentFactory>,
}

/// One icon action entry.
#[derive(Clone)]
pub struct SideMenuItem {
    pub icon: IconSource,
    pub on_activate: SideMenuIconAction,
    pub pop: Option<SideMenuItemPop>,
    pub selected: bool,
    pub placement: SideMenuItemPlacement,
}

impl SideMenuItem {
    #[must_use]
    pub fn new(icon: IconSource, on_activate: impl Fn() + 'static) -> Self {
        Self {
            icon,
            on_activate: Rc::new(on_activate),
            pop: None,
            selected: false,
            placement: SideMenuItemPlacement::Top,
        }
    }

    #[must_use]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    #[must_use]
    pub fn bottom(mut self) -> Self {
        self.placement = SideMenuItemPlacement::Bottom;
        self
    }

    #[must_use]
    pub fn placement(mut self, placement: SideMenuItemPlacement) -> Self {
        self.placement = placement;
        self
    }

    #[must_use]
    pub fn with_pop(
        mut self,
        mode: SideMenuPopMode,
        content: impl Fn() -> Box<dyn View> + 'static,
    ) -> Self {
        self.pop = Some(SideMenuItemPop {
            mode,
            content: Rc::new(content),
        });
        self
    }

    #[must_use]
    pub fn with_modal_pop(self, content: impl Fn() -> Box<dyn View> + 'static) -> Self {
        self.with_pop(SideMenuPopMode::Modal, content)
    }

    #[must_use]
    pub fn with_popover_pop(self, content: impl Fn() -> Box<dyn View> + 'static) -> Self {
        self.with_pop(SideMenuPopMode::Popover, content)
    }

    #[must_use]
    pub fn with_expand_pop(self, content: impl Fn() -> Box<dyn View> + 'static) -> Self {
        self.with_pop(SideMenuPopMode::Expand, content)
    }
}

/// Raw SideMenu properties.
#[derive(Clone)]
pub struct SideMenuProps {
    pub side: SideMenuSide,
    pub width: f32,
    pub expand_mode: SideMenuExpandMode,
    pub items: Vec<SideMenuItem>,
    pub initial_pop: Option<(usize, SideMenuPopMode)>,
}
