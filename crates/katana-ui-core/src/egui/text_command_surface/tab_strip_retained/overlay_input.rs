#[derive(Default)]
pub(super) struct TabStripOverlayInput {
    pub(super) primary_press: Option<egui::Id>,
    pub(super) opened_by_secondary: Option<(String, egui::Rect)>,
}
