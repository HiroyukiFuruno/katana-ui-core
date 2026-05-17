use floem::ViewId;

use super::AnchorRect;

pub(crate) struct ViewAnchor;

impl ViewAnchor {
    pub(crate) fn rect_for_view(anchor_id: ViewId, fallback: AnchorRect) -> AnchorRect {
        let Some(layout) = anchor_id.get_layout() else {
            return fallback;
        };
        let (x, y) = Self::origin_in_window(anchor_id);
        AnchorRect::new(x, y, layout.size.width, layout.size.height)
    }

    pub(crate) fn parent_origin_for_view(anchor_id: ViewId) -> (f32, f32) {
        anchor_id
            .parent()
            .map(Self::origin_in_window)
            .unwrap_or((0.0, 0.0))
    }

    fn origin_in_window(view_id: ViewId) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut current = Some(view_id);
        while let Some(id) = current {
            if let Some(layout) = id.get_layout() {
                x += layout.location.x;
                y += layout.location.y;
            }
            current = id.parent();
        }
        (x, y)
    }
}
