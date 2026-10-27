use super::super::super::accesskit_evidence::{self, AccessKitTargetClass};
use crate::egui::command_chrome::CommandChromePaintStyle;
use crate::egui::tab_strip_paint::{
    TabStripPaintOperation, TabStripPaintOperationKind, TabStripPaintTexture,
};
use crate::egui::texture_cache::RgbaTextureCache;
use crate::render_model::UiRect;

const ALPHA_INDEX: usize = 3;
const PADDING: f32 = 10.0;

pub(super) struct GlobalActionRow<'a> {
    pub(super) rect: egui::Rect,
    pub(super) texture: TabStripPaintTexture,
    pub(super) logical_texture_size: egui::Vec2,
    pub(super) enabled: bool,
    pub(super) response_id: egui::Id,
    pub(super) label: &'a str,
    pub(super) identity: &'a str,
    pub(super) class: AccessKitTargetClass,
}

pub(super) fn paint_row(
    ui: &egui::Ui,
    cache: &mut RgbaTextureCache,
    operations: &mut Vec<TabStripPaintOperation>,
    canvas: egui::Rect,
    row: GlobalActionRow<'_>,
    style: &CommandChromePaintStyle,
) {
    let GlobalActionRow {
        rect,
        texture,
        logical_texture_size,
        enabled,
        response_id,
        label,
        identity,
        class,
    } = row;
    let visible = rect.intersect(ui.clip_rect());
    if !visible.is_positive() {
        return;
    }
    let background = if enabled {
        style.action_rgba
    } else {
        style.disabled_action_rgba
    };
    ui.painter().rect_filled(
        rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(
            background[0],
            background[1],
            background[2],
            background[ALPHA_INDEX],
        ),
    );
    operations.push(TabStripPaintOperation {
        clip_bounds: ui_rect(canvas.intersect(ui.clip_rect())),
        kind: TabStripPaintOperationKind::Fill {
            bounds: ui_rect(rect),
            color_rgba: background,
        },
    });
    let text_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.min.x + PADDING,
            rect.center().y - logical_texture_size.y / 2.0,
        ),
        logical_texture_size,
    );
    let handle = cache.texture_for_rgba(
        ui.ctx(),
        &texture.identity,
        texture.width as usize,
        texture.height as usize,
        &texture.rgba_pixels,
    );
    ui.painter().image(
        handle.id(),
        text_rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
    operations.push(TabStripPaintOperation {
        clip_bounds: ui_rect(canvas.intersect(ui.clip_rect())),
        kind: TabStripPaintOperationKind::Texture {
            bounds: ui_rect(text_rect),
            texture,
        },
    });
    accesskit_evidence::publish_labeled_button_accesskit(
        ui,
        response_id,
        label,
        !enabled,
        ui_rect(visible),
        identity,
        class,
    );
}

pub(in crate::egui::text_command_surface::root) fn ui_rect(rect: egui::Rect) -> UiRect {
    UiRect::new(
        rect.min.x.round() as i32,
        rect.min.y.round() as i32,
        rect.width().round().max(0.0) as u32,
        rect.height().round().max(0.0) as u32,
    )
}
