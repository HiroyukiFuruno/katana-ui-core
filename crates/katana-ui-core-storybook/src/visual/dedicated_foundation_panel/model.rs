use super::super::dedicated_dod_common::Rect;
use super::super::panel_screen_state::PanelChildKey;
use super::super::render_context::ScenarioContext;
use katana_ui_core::render_model::UiPanelProps;
use katana_ui_core::render_model::{UiNode, UiNodeKind};

pub(super) const ROOT_X: usize = 16;
pub(super) const ROOT_Y: usize = 24;
pub(super) const ROOT_WIDTH: usize = 644;
pub(super) const ROOT_HEIGHT: usize = 300;
pub(super) const NAV_SLOT: PanelSlot = PanelSlot::new(
    "nav",
    "Navigation panel",
    PanelChildKey::Navigation,
    32,
    64,
    128,
    192,
);
pub(super) const PREVIEW_SLOT: PanelSlot = PanelSlot::new(
    "preview",
    "Preview panel",
    PanelChildKey::Preview,
    174,
    64,
    296,
    192,
);
pub(super) const DETAILS_SLOT: PanelSlot = PanelSlot::new(
    "details",
    "Details panel",
    PanelChildKey::Details,
    484,
    64,
    136,
    192,
);
pub(super) const TEXT_X_OFFSET: usize = 8;
pub(super) const TEXT_Y_OFFSET: usize = 7;
pub(super) const LABEL_SIZE: f32 = 8.0;
pub(super) const STATUS_X: usize = 32;
pub(super) const STATUS_Y: usize = 264;
pub(super) const STATUS_WIDTH: usize = 138;
pub(super) const STATUS_HEIGHT: usize = 22;
pub(super) const STATUS_GAP: usize = 8;
pub(super) const STATUS_TEXT_X: usize = 7;
pub(super) const STATUS_TEXT_Y: usize = 5;
pub(super) const VERTICAL_PRESET_INDEX: usize = 1;
pub(super) const HORIZONTAL_PRESET_INDEX: usize = 2;
pub(super) const SCROLLBAR_PRESET_INDEX: usize = 3;
pub(super) const NESTED_PRESET_INDEX: usize = 4;
const VERTICAL_PRESET_SCROLL_Y: u32 = 220;
const HORIZONTAL_PRESET_SCROLL_X: u32 = 280;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PanelSlot {
    pub(super) label: &'static str,
    pub(super) node_label: &'static str,
    pub(super) key: PanelChildKey,
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) width: usize,
    pub(super) height: usize,
}

impl PanelSlot {
    const fn new(
        label: &'static str,
        node_label: &'static str,
        key: PanelChildKey,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            label,
            node_label,
            key,
            x,
            y,
            width,
            height,
        }
    }

    pub(super) fn rect(self, x: usize, y: usize) -> Rect {
        Rect::new(x + self.x, y + self.y, self.width, self.height)
    }

    fn contains(self, origin_x: usize, origin_y: usize, x: usize, y: usize) -> bool {
        let rect = self.rect(origin_x, origin_y);
        x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
    }
}

pub(super) fn child_panel<'a>(root: &'a UiNode, label: &str) -> Option<&'a UiNode> {
    root.children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::Panel && it.props().label == label)
}

pub(super) fn panel_props_for_slot(
    slot: PanelSlot,
    props: &UiPanelProps,
    scenario: ScenarioContext<'_>,
) -> UiPanelProps {
    let mut next = props.clone();
    let child = scenario.screen_state.panel.child(slot.key);
    match scenario.preset_index {
        VERTICAL_PRESET_INDEX => {
            set_horizontal(&mut next, child.scroll_x, false);
            set_vertical(
                &mut next,
                child.scroll_y.max(VERTICAL_PRESET_SCROLL_Y),
                true,
            );
        }
        HORIZONTAL_PRESET_INDEX => {
            set_horizontal(
                &mut next,
                child.scroll_x.max(HORIZONTAL_PRESET_SCROLL_X),
                true,
            );
            set_vertical(&mut next, child.scroll_y, false);
        }
        NESTED_PRESET_INDEX => {
            set_horizontal(&mut next, child.scroll_x, true);
            set_vertical(&mut next, child.scroll_y, true);
        }
        _ => {
            set_horizontal(&mut next, child.scroll_x, false);
            set_vertical(&mut next, child.scroll_y, false);
        }
    }
    apply_visibility(&mut next, component_scrollbars_visible(scenario, slot.key));
    next
}

pub(super) fn component_scrollbars_visible(
    scenario: ScenarioContext<'_>,
    panel: PanelChildKey,
) -> bool {
    scenario.screen_state.panel.child(panel).scrollbar_visible
}

pub(in crate::visual) fn panel_at(
    origin_x: usize,
    origin_y: usize,
    x: usize,
    y: usize,
) -> Option<PanelChildKey> {
    [NAV_SLOT, PREVIEW_SLOT, DETAILS_SLOT]
        .into_iter()
        .find(|slot| slot.contains(origin_x, origin_y, x, y))
        .map(|slot| slot.key)
}

fn apply_visibility(props: &mut UiPanelProps, visible: bool) {
    props.vertical_scrollbar_visible = visible && props.content_height > props.viewport_height;
    props.horizontal_scrollbar_visible = visible && props.content_width > props.viewport_width;
}

fn set_vertical(props: &mut UiPanelProps, scroll_y: u32, visible: bool) {
    props.scroll_y = scroll_y.min(props.content_height.saturating_sub(props.viewport_height));
    props.vertical_scrollbar_visible = visible && props.content_height > props.viewport_height;
}

fn set_horizontal(props: &mut UiPanelProps, scroll_x: u32, visible: bool) {
    props.scroll_x = scroll_x.min(props.content_width.saturating_sub(props.viewport_width));
    props.horizontal_scrollbar_visible = visible && props.content_width > props.viewport_width;
}
