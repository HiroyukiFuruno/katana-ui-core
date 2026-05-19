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
        self.state.scroll =
            UiPanelProps::vertical_scroll(scroll_y, viewport_height, content_height, visible);
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
mod tests {
    use super::{Panel, PanelRegion};
    use crate::atom::Text;
    use crate::render_model::{
        UiNodeKind, UiRect, UiScrollbarModel, UiScrollbarPlacement, UiScrollbarVisibility, UiTree,
    };
    use crate::theme::ThemeSnapshot;

    #[test]
    fn panel_carries_theme_setting_to_render_model() {
        let tree = UiTree::new(
            Panel::new("Preview", PanelRegion::Preview, ThemeSnapshot::dark())
                .child(Text::new("Story")),
        );

        assert_eq!(UiNodeKind::Panel, tree.root().kind());
        assert_eq!("dark", tree.root().props().theme_id);
        assert_eq!(1, tree.root().children().len());
    }

    #[test]
    fn nested_panels_keep_independent_vertical_scroll_state() {
        let tree = UiTree::new(
            Panel::new("Parent", PanelRegion::Root, ThemeSnapshot::dark())
                .vertical_scroll(120, 600, 1800, true)
                .child(
                    Panel::new("Left", PanelRegion::Navigation, ThemeSnapshot::dark())
                        .vertical_scroll(24, 320, 900, true),
                )
                .child(
                    Panel::new("Right", PanelRegion::Details, ThemeSnapshot::dark())
                        .vertical_scroll(80, 320, 1200, true),
                ),
        );
        let left = &tree.root().children()[0];
        let right = &tree.root().children()[1];

        assert_eq!(120, tree.root().props().panel.scroll_y);
        assert_eq!(24, left.props().panel.scroll_y);
        assert_eq!(80, right.props().panel.scroll_y);
        assert_ne!(left.props().state_id, right.props().state_id);
    }

    #[test]
    fn panel_scrollbar_model_carries_bounds_visibility_and_drag_state() {
        let scrollbar = UiScrollbarModel::new(
            UiScrollbarVisibility::Always,
            UiScrollbarPlacement::Overlay,
            UiRect::new(280, 0, 8, 320),
            UiRect::new(280, 32, 8, 96),
            48,
        )
        .dragging(7, 32);
        let tree = UiTree::new(
            Panel::new("Preview", PanelRegion::Preview, ThemeSnapshot::dark())
                .vertical_scroll(0, 320, 1280, false)
                .scrollbar(scrollbar),
        );
        let panel = &tree.root().props().panel;

        assert!(panel.vertical_scrollbar_visible);
        assert_eq!(48, panel.scroll_y);
        assert_eq!(
            UiScrollbarVisibility::Always,
            panel.vertical_scrollbar.visibility
        );
        assert_eq!(
            UiScrollbarPlacement::Overlay,
            panel.vertical_scrollbar.placement
        );
        assert_eq!(
            UiRect::new(280, 0, 8, 320),
            panel.vertical_scrollbar.track_bounds
        );
        assert_eq!(
            UiRect::new(280, 32, 8, 96),
            panel.vertical_scrollbar.thumb_bounds
        );
        assert!(panel.vertical_scrollbar.drag_state.dragging);
        assert_eq!(Some(7), panel.vertical_scrollbar.drag_state.pointer_id);
    }
}
