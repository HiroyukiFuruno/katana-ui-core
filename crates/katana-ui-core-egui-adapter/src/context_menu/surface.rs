use super::accessibility::ContextMenuAccessibility;
use super::interaction::consume_item_click;
use super::presentation::full_path;
use super::types::{ContextMenuPresentationItem, EguiContextMenuItemFrame};
use crate::text_surface::TextSurfaceContextTargetAnchor;
use katana_ui_core::molecule::selection::{
    ContextMenuAction, ContextMenuPlacementResolver, ContextMenuSize, ContextMenuViewport,
};
use katana_ui_core::render_model::{UiContextMenuAnchor, UiContextMenuRect, UiRect};

pub(super) struct MenuAreaOutput {
    pub(super) actions: Vec<ContextMenuAction>,
    pub(super) item_frames: Vec<EguiContextMenuItemFrame>,
}

pub(super) fn menu_items(
    ui: &mut egui::Ui,
    area_id: egui::Id,
    menu_bounds: UiRect,
    items: &[ContextMenuPresentationItem],
    submenu_path: &[usize],
    vertical_scroll_offset: f32,
) -> MenuAreaOutput {
    let mut actions = Vec::new();
    let mut item_frames = Vec::new();
    for (index, item) in items.iter().enumerate() {
        let item_bounds = menu_item_bounds(menu_bounds, index, vertical_scroll_offset);
        if !contains_rect(menu_bounds, item_bounds) {
            continue;
        }
        let rect = egui_rect(item_bounds);
        let item_id = area_id.with(("item", index));
        let response = ui.interact(rect, item_id, egui::Sense::click());
        ContextMenuAccessibility::publish_item(ui, item_id, item, item_bounds);
        item_frames.push(EguiContextMenuItemFrame {
            id: item.id.clone(),
            bounds: item_bounds,
            disabled: !item.enabled,
            checked: item.checked,
        });
        if response.clicked()
            && let Some(action) = consume_item_click(item, full_path(submenu_path, index))
        {
            actions.push(action);
        }
    }
    MenuAreaOutput {
        actions,
        item_frames,
    }
}

pub(super) fn menu_bounds(
    anchor: &TextSurfaceContextTargetAnchor,
    measured_width: u32,
    measured_height: u32,
) -> UiRect {
    let viewport = anchor.viewport_bounds();
    let local = local_anchor(anchor, viewport);
    let result = ContextMenuPlacementResolver::resolve(
        &local,
        ContextMenuSize::new(measured_width.min(viewport.width), measured_height),
        ContextMenuViewport::new(viewport.width, viewport.height),
        &[],
    );
    UiRect::new(
        viewport.x.saturating_add(result.x),
        viewport.y.saturating_add(result.y),
        measured_width.min(viewport.width),
        result.render_height,
    )
}

pub(super) fn local_anchor(
    anchor: &TextSurfaceContextTargetAnchor,
    viewport: UiRect,
) -> UiContextMenuAnchor {
    match anchor.anchor() {
        UiContextMenuAnchor::Pointer { x, y } => UiContextMenuAnchor::Pointer {
            x: x.saturating_sub(viewport.x),
            y: y.saturating_sub(viewport.y),
        },
        UiContextMenuAnchor::VirtualRect(rect) => {
            UiContextMenuAnchor::VirtualRect(UiContextMenuRect::new(
                rect.x.saturating_sub(viewport.x),
                rect.y.saturating_sub(viewport.y),
                rect.width,
                rect.height,
            ))
        }
        UiContextMenuAnchor::NodeId(id) => UiContextMenuAnchor::NodeId(id.clone()),
    }
}

pub(super) fn egui_rect(bounds: UiRect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(bounds.x as f32, bounds.y as f32),
        egui::vec2(bounds.width as f32, bounds.height as f32),
    )
}

fn menu_item_bounds(bounds: UiRect, index: usize, vertical_scroll_offset: f32) -> UiRect {
    let row_offset = u32::try_from(index)
        .unwrap_or(u32::MAX)
        .saturating_mul(super::types::ROW_HEIGHT_PX);
    let offset = vertical_scroll_offset.round() as i32;
    UiRect::new(
        bounds.x,
        bounds
            .y
            .saturating_add(super::types::MENU_PADDING_PX as i32)
            .saturating_add_unsigned(row_offset)
            .saturating_sub(offset),
        bounds.width,
        super::types::ROW_HEIGHT_PX,
    )
}

pub(super) fn contains(bounds: UiRect, point: egui::Pos2) -> bool {
    point.x >= bounds.x as f32
        && point.x < bounds.x.saturating_add_unsigned(bounds.width) as f32
        && point.y >= bounds.y as f32
        && point.y < bounds.y.saturating_add_unsigned(bounds.height) as f32
}

fn contains_rect(bounds: UiRect, item: UiRect) -> bool {
    item.x >= bounds.x
        && item.y >= bounds.y
        && item.x.saturating_add_unsigned(item.width)
            <= bounds.x.saturating_add_unsigned(bounds.width)
        && item.y.saturating_add_unsigned(item.height)
            <= bounds.y.saturating_add_unsigned(bounds.height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_anchor_remains_opaque_when_localized() {
        let anchor = TextSurfaceContextTargetAnchor {
            anchor: UiContextMenuAnchor::NodeId("opaque-node".into()),
            selection: katana_ui_core::text_selection::UiTextSelectionRange::caret(0),
            viewport_bounds: UiRect::new(10, 20, 100, 80),
        };
        assert_eq!(
            local_anchor(&anchor, UiRect::new(10, 20, 100, 80)),
            UiContextMenuAnchor::NodeId("opaque-node".into())
        );
    }

    #[test]
    fn pointer_virtual_anchor_and_geometry_helpers_cover_the_local_surface() {
        let viewport = UiRect::new(10, 20, 100, 80);
        for (anchor, expected) in [
            (
                UiContextMenuAnchor::Pointer { x: 14, y: 26 },
                UiContextMenuAnchor::Pointer { x: 4, y: 6 },
            ),
            (
                UiContextMenuAnchor::VirtualRect(UiContextMenuRect::new(15, 27, 8, 9)),
                UiContextMenuAnchor::VirtualRect(UiContextMenuRect::new(5, 7, 8, 9)),
            ),
        ] {
            let target = TextSurfaceContextTargetAnchor {
                anchor,
                selection: katana_ui_core::text_selection::UiTextSelectionRange::caret(0),
                viewport_bounds: viewport,
            };
            assert_eq!(local_anchor(&target, viewport), expected);
        }

        let bounds = UiRect::new(0, 0, 20, 20);
        assert!(contains(bounds, egui::pos2(10.0, 10.0)));
        assert!(!contains(bounds, egui::pos2(20.0, 20.0)));
        assert!(contains_rect(bounds, UiRect::new(1, 1, 18, 18)));
        assert!(!contains_rect(bounds, UiRect::new(1, 1, 20, 20)));
    }
}
