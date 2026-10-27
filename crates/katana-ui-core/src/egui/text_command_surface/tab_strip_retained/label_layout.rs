use super::{RgbaColor, TabStripText};

pub(super) struct TabStripLabelInteraction {
    pub(super) route_path: Option<String>,
}

pub(super) struct TabStripLabelRenderRequest<'a> {
    pub(super) text: &'a TabStripText,
    pub(super) path: String,
    pub(super) x: f32,
    pub(super) bounds: egui::Rect,
    pub(super) active: bool,
    pub(super) active_reveal_pending: &'a mut bool,
    pub(super) interaction: TabStripLabelInteraction,
    pub(super) draggable: bool,
    pub(super) background_color: Option<RgbaColor>,
    pub(super) marker_color: Option<RgbaColor>,
}

pub(super) struct TabStripLabelRender {
    pub(super) advance: f32,
    pub(super) secondary_clicked: bool,
    pub(super) drag_started: bool,
    pub(super) drag_stopped: bool,
    pub(super) bounds: egui::Rect,
}
