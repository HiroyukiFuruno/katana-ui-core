use crate::render_model::{UiNode, UiNodeKind, UiStateId};
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
    theme: ThemeSnapshot,
    region: PanelRegion,
}

impl PanelState {
    fn new(region: PanelRegion, theme: ThemeSnapshot) -> Self {
        Self {
            state_id: UiStateId::next_for(UiNodeKind::Panel),
            theme,
            region,
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
        let mut node =
            UiNode::from_state(UiNodeKind::Panel, value.title, value.state.state_id).theme(&theme);
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
    use crate::render_model::{UiNodeKind, UiTree};
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
}
