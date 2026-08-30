use super::accessibility::publish_item;
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
        publish_item(ui, item_id, item, item_bounds);
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
    use crate::context_menu::types::{MENU_PADDING_PX, ROW_HEIGHT_PX};

    #[test]
    fn menu_item_bounds_advances_by_row_height_and_respects_vertical_scroll_offset() {
        let bounds = UiRect::new(0, 0, 100, 100);
        let first = menu_item_bounds(bounds, 0, 0.0);
        let second = menu_item_bounds(bounds, 1, 0.0);
        let row_height = ROW_HEIGHT_PX as i32;
        assert_eq!(first.y + row_height, second.y);
        assert!(contains_rect(bounds, first));
        assert!(contains_rect(bounds, second));

        let scrolled = menu_item_bounds(bounds, 0, 5.0);
        assert_eq!(scrolled.y, bounds.y + MENU_PADDING_PX as i32 - 5);
    }

    #[test]
    fn contains_rect_detects_out_of_range_items() {
        let bounds = UiRect::new(10, 20, 40, 20);
        assert!(contains(bounds, egui::Pos2::new(10.0, 20.0)));
        assert!(!contains(bounds, egui::Pos2::new(51.0, 39.0)));
        assert!(contains_rect(bounds, UiRect::new(10, 20, 20, 20),));
        assert!(!contains_rect(bounds, UiRect::new(45, 20, 10, 10),));

        let menu_bounds = UiRect::new(
            0,
            0,
            100,
            ROW_HEIGHT_PX.saturating_add(MENU_PADDING_PX.saturating_mul(2)),
        );
        let items = [
            ContextMenuPresentationItem::action("visible", "Visible"),
            ContextMenuPresentationItem::action("clipped", "Clipped"),
        ];
        let context = egui::Context::default();
        let mut frames = None;
        let mut output = context.run_ui(egui::RawInput::default(), |ui| {
            frames = Some(
                menu_items(
                    ui,
                    egui::Id::new("clipped-menu-items"),
                    menu_bounds,
                    &items,
                    &[],
                    0.0,
                )
                .item_frames,
            );
        });
        output.textures_delta.clear();
        assert_eq!(
            frames
                .expect("menu item frames")
                .iter()
                .map(|frame| frame.id.as_str())
                .collect::<Vec<_>>(),
            vec!["visible"]
        );
    }
}
