use crate::render_model::{
    UiCommonProps, UiNode, UiNodeKind, UiPanelProps, UiScrollbarModel, UiStateId,
};
use crate::theme::ThemeSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanelRegion {
    Root,
    Navigation,
    Preview,
    Details,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PanelState {
    state_id: UiStateId,
    common: UiCommonProps,
    theme: ThemeSnapshot,
    region: PanelRegion,
    scroll: UiPanelProps,
}

impl PanelState {
    fn new(region: PanelRegion, theme: ThemeSnapshot) -> Self {
        Self {
            state_id: UiStateId::next_for(UiNodeKind::Panel),
            common: UiCommonProps::default(),
            theme,
            region,
            scroll: UiPanelProps::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Panel {
    title: String,
    state: PanelState,
    children: Vec<UiNode>,
}

impl Panel {
    #[must_use]
    pub fn new(title: impl Into<String>, region: PanelRegion, theme: ThemeSnapshot) -> Self {
        Self {
            title: title.into(),
            state: PanelState::new(region, theme),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn vertical_scroll(
        mut self,
        scroll_y: u32,
        viewport_height: u32,
        content_height: u32,
        visible: bool,
    ) -> Self {
        self.state.scroll = self.state.scroll.with_vertical_scroll(
            scroll_y,
            viewport_height,
            content_height,
            visible,
        );
        self
    }

    #[must_use]
    pub fn horizontal_scroll(
        mut self,
        scroll_x: u32,
        viewport_width: u32,
        content_width: u32,
        visible: bool,
    ) -> Self {
        self.state.scroll = self.state.scroll.with_horizontal_scroll(
            scroll_x,
            viewport_width,
            content_width,
            visible,
        );
        self
    }

    #[must_use]
    pub fn common(mut self, value: UiCommonProps) -> Self {
        self.state.common = value;
        self
    }

    #[must_use]
    pub fn scrollbar(mut self, value: UiScrollbarModel) -> Self {
        self.state.scroll = self.state.scroll.scrollbar(value);
        self
    }

    #[must_use]
    pub fn horizontal_scrollbar(mut self, value: UiScrollbarModel) -> Self {
        self.state.scroll = self.state.scroll.horizontal_scrollbar(value);
        self
    }

    #[must_use]
    pub fn theme(&self) -> &ThemeSnapshot {
        &self.state.theme
    }

    #[must_use]
    pub fn region(&self) -> PanelRegion {
        self.state.region
    }
}

impl From<Panel> for UiNode {
    fn from(value: Panel) -> Self {
        let theme = value.state.theme;
        let mut node = UiNode::from_state(UiNodeKind::Panel, value.title, value.state.state_id)
            .common(value.state.common)
            .theme(&theme)
            .panel(value.state.scroll);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}

#[cfg(test)]
mod tests;
