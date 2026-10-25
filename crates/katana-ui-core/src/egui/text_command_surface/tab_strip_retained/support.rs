use super::{
    FontFamily, FontToken, OVERLAY_INPUT_RGBA, OVERLAY_LABEL_FONT_SIZE_PX,
    OVERLAY_LABEL_FONT_WEIGHT, OVERLAY_ROW_HEIGHT_PX, PREEDIT_RGBA, PRIMARY_TEXT_RGBA,
    SELECTION_RGBA, TabStripContextMenuPresentation, TabStripGroupDescriptor,
    TabStripPaintOperation, TabStripPaintOperationKind, TabStripPaintPlan, TabStripPaintTexture,
    TabStripProjection, TabStripRenameDraft, TabStripRootItem, TextSurfacePaintOperationKind,
    TextSurfacePaintStyle, TextSurfaceRasterStyle, UiRect,
};

pub(super) struct TabStripOverlayOutcome {
    pub(super) closed: bool,
    pub(super) submenu_path: Vec<usize>,
    pub(super) paint_plan: TabStripPaintPlan,
}

pub(super) struct TabStripOverlayPanel {
    pub(super) bounds: egui::Rect,
    pub(super) row_positions: Vec<f32>,
    pub(super) open_submenu: Option<usize>,
    pub(super) closed: bool,
}

pub(super) struct TabStripGroupPopupPrefix {
    pub(super) content_anchor: egui::Pos2,
    pub(super) bounds: egui::Rect,
    pub(super) operations: Vec<TabStripPaintOperation>,
    pub(super) closed: bool,
    pub(super) rename: Option<Box<TabStripRenameDraft>>,
}

impl TabStripOverlayPanel {
    pub(super) fn row_y(&self, index: usize) -> f32 {
        self.row_positions
            .get(index)
            .copied()
            .unwrap_or(self.bounds.min.y)
    }
}

pub(super) fn tab_menu_for_path<'a>(
    projection: &'a TabStripProjection,
    path: &str,
) -> Option<&'a TabStripContextMenuPresentation> {
    let mut tab_occurrence = 0;
    let mut group_occurrence = 0;
    for item in &projection.root_items {
        match item {
            TabStripRootItem::Tab(index) if path == format!("root-tab-{tab_occurrence}") => {
                return projection
                    .tabs
                    .get(*index)
                    .and_then(|tab| tab.context_menu.as_ref());
            }
            TabStripRootItem::Group(index) => {
                if let Some(group) = projection.groups.get(*index)
                    && let Some(menu) =
                        tab_menu_for_group(group, &format!("root-group-{group_occurrence}"), path)
                {
                    return Some(menu);
                }
            }
            _ => {}
        }
        match item {
            TabStripRootItem::Tab(_) => tab_occurrence += 1,
            TabStripRootItem::Group(_) => group_occurrence += 1,
        }
    }
    if !projection.root_items.is_empty() {
        return None;
    }
    for (index, tab) in projection.tabs.iter().enumerate() {
        if path == format!("root-tab-{index}") {
            return tab.context_menu.as_ref();
        }
    }
    projection
        .groups
        .iter()
        .enumerate()
        .find_map(|(index, group)| tab_menu_for_group(group, &format!("root-group-{index}"), path))
}

pub(super) fn tab_menu_for_group<'a>(
    group: &'a TabStripGroupDescriptor,
    prefix: &str,
    path: &str,
) -> Option<&'a TabStripContextMenuPresentation> {
    for (index, tab) in group.tabs.iter().enumerate() {
        if path == format!("{prefix}-tab-{index}") {
            return tab.context_menu.as_ref();
        }
    }
    group.groups.iter().enumerate().find_map(|(index, child)| {
        tab_menu_for_group(child, &format!("{prefix}-group-{index}"), path)
    })
}

pub(super) fn group_for_path<'a>(
    projection: &'a TabStripProjection,
    path: &str,
) -> Option<&'a TabStripGroupDescriptor> {
    let mut group_occurrence = 0;
    for item in &projection.root_items {
        if let TabStripRootItem::Group(index) = item
            && let Some(group) = projection.groups.get(*index)
            && let Some(found) =
                group_for_group(group, &format!("root-group-{group_occurrence}"), path)
        {
            return Some(found);
        }
        if matches!(item, TabStripRootItem::Group(_)) {
            group_occurrence += 1;
        }
    }
    if !projection.root_items.is_empty() {
        return None;
    }
    projection
        .groups
        .iter()
        .enumerate()
        .find_map(|(index, group)| group_for_group(group, &format!("root-group-{index}"), path))
}

pub(super) fn group_path_for_target(
    projection: &TabStripProjection,
    target: &super::super::tab_strip_projection_lease::TabStripGroupTarget,
) -> Option<String> {
    let mut group_occurrence = 0;
    for item in &projection.root_items {
        if let super::super::tab_strip_projection_lease::TabStripRootItem::Group(index) = item {
            let prefix = format!("root-group-{group_occurrence}");
            if let Some(group) = projection.groups.get(*index)
                && let Some(path) = group_path_for_group(group, target, &prefix)
            {
                return Some(path);
            }
            group_occurrence += 1;
        }
    }
    if projection.root_items.is_empty() {
        for (index, group) in projection.groups.iter().enumerate() {
            if let Some(path) = group_path_for_group(group, target, &format!("root-group-{index}"))
            {
                return Some(path);
            }
        }
    }
    None
}

fn group_path_for_group(
    group: &TabStripGroupDescriptor,
    target: &super::super::tab_strip_projection_lease::TabStripGroupTarget,
    prefix: &str,
) -> Option<String> {
    if &group.target == target {
        return Some(prefix.to_owned());
    }
    for (index, child) in group.groups.iter().enumerate() {
        if let Some(path) = group_path_for_group(child, target, &format!("{prefix}-group-{index}"))
        {
            return Some(path);
        }
    }
    None
}

pub(super) fn group_for_group<'a>(
    group: &'a TabStripGroupDescriptor,
    prefix: &str,
    path: &str,
) -> Option<&'a TabStripGroupDescriptor> {
    if prefix == path {
        return Some(group);
    }
    group
        .groups
        .iter()
        .enumerate()
        .find_map(|(index, child)| group_for_group(child, &format!("{prefix}-group-{index}"), path))
}

pub(super) fn union_bounds(bounds: &[egui::Rect]) -> Option<egui::Rect> {
    let mut values = bounds.iter().copied();
    let first = values.next()?;
    Some(values.fold(first, |union, value| union.union(value)))
}

pub(super) fn rect_from_ui_rect(rect: UiRect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(rect.x as f32, rect.y as f32),
        egui::vec2(rect.width as f32, rect.height as f32),
    )
}

pub(super) fn rename_raster_style() -> TextSurfaceRasterStyle {
    TextSurfaceRasterStyle::new(
        FontToken {
            name: "kuc-tab-strip-overlay-input".to_owned(),
            family: FontFamily::Proportional,
            size: OVERLAY_LABEL_FONT_SIZE_PX,
            weight: OVERLAY_LABEL_FONT_WEIGHT,
        },
        PRIMARY_TEXT_RGBA,
        OVERLAY_ROW_HEIGHT_PX,
    )
}

pub(super) fn rename_paint_style() -> TextSurfacePaintStyle {
    TextSurfacePaintStyle {
        background_rgba: OVERLAY_INPUT_RGBA,
        gutter_background_rgba: OVERLAY_INPUT_RGBA,
        gutter_paints: Vec::new(),
        selection_rgba: SELECTION_RGBA,
        preedit_rgba: PREEDIT_RGBA,
        caret_rgba: PRIMARY_TEXT_RGBA,
        annotation_paints: Vec::new(),
    }
}

pub(super) fn append_text_surface_operations(
    operations: &mut Vec<TabStripPaintOperation>,
    paint_plan: &crate::egui::text_surface::TextSurfacePaintPlan,
) {
    operations.extend(paint_plan.operations.iter().map(|operation| {
        let kind = match &operation.kind {
            TextSurfacePaintOperationKind::Fill { bounds, color_rgba } => {
                TabStripPaintOperationKind::Fill {
                    bounds: *bounds,
                    color_rgba: *color_rgba,
                }
            }
            TextSurfacePaintOperationKind::Texture { bounds, texture } => {
                TabStripPaintOperationKind::Texture {
                    bounds: *bounds,
                    texture: TabStripPaintTexture {
                        identity: texture.identity.clone(),
                        width: texture.width,
                        height: texture.height,
                        rgba_pixels: texture.rgba_pixels.clone(),
                    },
                }
            }
        };
        TabStripPaintOperation {
            clip_bounds: operation.clip_bounds,
            kind,
        }
    }));
}

pub(super) fn ui_rect(rect: egui::Rect) -> UiRect {
    UiRect::new(
        rect.min.x.round() as i32,
        rect.min.y.round() as i32,
        rect.width().round().max(0.0) as u32,
        rect.height().round().max(0.0) as u32,
    )
}

#[cfg(test)]
#[path = "support_tests.rs"]
mod tests;
