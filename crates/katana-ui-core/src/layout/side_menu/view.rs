use super::render;
use super::types::{
    DEFAULT_SIDE_MENU_WIDTH, SideMenuExpandMode, SideMenuItem, SideMenuPopMode, SideMenuProps,
    SideMenuSide,
};
use crate::theme::Theme;

#[derive(Clone)]
pub struct SideMenu {
    props: SideMenuProps,
}

impl SideMenu {
    #[must_use]
    pub fn new(items: impl IntoIterator<Item = SideMenuItem>) -> Self {
        Self {
            props: SideMenuProps {
                side: SideMenuSide::Left,
                width: DEFAULT_SIDE_MENU_WIDTH,
                expand_mode: SideMenuExpandMode::default(),
                items: items.into_iter().collect(),
                initial_pop: None,
            },
        }
    }
    #[must_use]
    pub fn side(mut self, side: SideMenuSide) -> Self {
        self.props.side = side;
        self
    }
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.props.width = width.max(0.0);
        self
    }
    #[must_use]
    pub fn hover_expand(mut self, collapsed_width: f32) -> Self {
        self.props.expand_mode = SideMenuExpandMode::Hover {
            collapsed_width: collapsed_width.max(0.0),
        };
        self
    }
    #[must_use]
    pub fn fixed(mut self) -> Self {
        self.props.expand_mode = SideMenuExpandMode::Fixed;
        self
    }
    #[must_use]
    pub fn initial_pop(mut self, index: usize, mode: SideMenuPopMode) -> Self {
        self.props.initial_pop = Some((index, mode));
        self
    }
    #[cfg(test)]
    pub(crate) fn initial_pop_state(&self) -> Option<(usize, SideMenuPopMode)> {
        self.props.initial_pop
    }
    #[must_use]
    pub fn add_item(mut self, item: SideMenuItem) -> Self {
        self.props.items.push(item);
        self
    }
    #[must_use]
    pub fn view(self, theme: Theme) -> impl floem::IntoView {
        render::render(self.props, theme)
    }
}
