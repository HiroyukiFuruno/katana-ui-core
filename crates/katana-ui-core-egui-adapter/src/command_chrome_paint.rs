use super::command_chrome_artifact::{
    CommandChromePaintOperation, CommandChromePaintOperationKind, CommandChromePaintPlan,
    CommandChromePaintTexture,
};
use super::command_chrome_dropdown::DropdownPresentation;
use super::command_chrome_types::{
    CommandChromePaintStyle, EguiCommandChromeDrawLayer, EguiCommandChromeFrameRecord,
    RenderedAction, RenderedRaster,
};
use crate::texture_cache::RgbaTextureCache;
use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect};

const ACTION_PADDING_PX: u32 = 8;

pub(super) struct ActionPaintSource {
    primary_bounds: UiRect,
    secondary_bounds: Option<UiRect>,
    primary_hovered: bool,
    secondary_hovered: bool,
    disabled: bool,
    rendered: RenderedAction,
}

impl ActionPaintSource {
    pub(super) fn new(
        primary_bounds: UiRect,
        secondary_bounds: Option<UiRect>,
        primary_hovered: bool,
        secondary_hovered: bool,
        disabled: bool,
        rendered: RenderedAction,
    ) -> Self {
        Self {
            primary_bounds,
            secondary_bounds,
            primary_hovered,
            secondary_hovered,
            disabled,
            rendered,
        }
    }
}

pub(super) struct DropdownPaintSource {
    bounds: UiRect,
    background_rgba: [u8; RGBA_CHANNEL_COUNT],
    actions: Vec<ActionPaintSource>,
}

impl DropdownPaintSource {
    pub(super) fn new(
        bounds: UiRect,
        background_rgba: [u8; RGBA_CHANNEL_COUNT],
        actions: Vec<ActionPaintSource>,
    ) -> Self {
        Self {
            bounds,
            background_rgba,
            actions,
        }
    }
}

pub(super) fn build_toolbar_paint_plan(
    record: &EguiCommandChromeFrameRecord,
    actions: &[ActionPaintSource],
    dropdown: Option<&DropdownPresentation>,
    style: &CommandChromePaintStyle,
) -> CommandChromePaintPlan {
    let mut operations = Vec::new();
    for action in actions {
        append_action_operations(&mut operations, record.bounds, action, style);
    }
    if let Some(dropdown) = dropdown {
        operations.push(fill(
            dropdown.paint.bounds,
            dropdown.paint.bounds,
            dropdown.paint.background_rgba,
        ));
        for action in &dropdown.paint.actions {
            append_action_operations(&mut operations, dropdown.paint.bounds, action, style);
        }
    }
    CommandChromePaintPlan {
        surface_bounds: plan_surface_bounds(record.bounds, &operations),
        operations,
    }
}

pub(super) fn plan_surface_bounds(
    toolbar_bounds: UiRect,
    operations: &[CommandChromePaintOperation],
) -> UiRect {
    operations.iter().fold(toolbar_bounds, |bounds, operation| {
        union_bounds(bounds, operation.clip_bounds)
    })
}

pub(super) fn union_bounds(left: UiRect, right: UiRect) -> UiRect {
    let x = left.x.min(right.x);
    let y = left.y.min(right.y);
    let right_edge = left
        .x
        .saturating_add_unsigned(left.width)
        .max(right.x.saturating_add_unsigned(right.width));
    let bottom_edge = left
        .y
        .saturating_add_unsigned(left.height)
        .max(right.y.saturating_add_unsigned(right.height));
    UiRect::new(
        x,
        y,
        u32::try_from(right_edge.saturating_sub(x)).unwrap_or_default(),
        u32::try_from(bottom_edge.saturating_sub(y)).unwrap_or_default(),
    )
}

pub(super) fn paint_command_chrome(
    ui: &egui::Ui,
    cache: &mut RgbaTextureCache,
    plan: &CommandChromePaintPlan,
) {
    for operation in &plan.operations {
        let painter = ui
            .painter()
            .with_clip_rect(egui_rect(operation.clip_bounds));
        match &operation.kind {
            CommandChromePaintOperationKind::Fill { bounds, color_rgba } => {
                painter.rect_filled(egui_rect(*bounds), 0.0, color(*color_rgba));
            }
            CommandChromePaintOperationKind::Texture { bounds, texture } => {
                let texture = cache.texture_for_rgba(
                    ui.ctx(),
                    &texture.identity,
                    texture.width as usize,
                    texture.height as usize,
                    &texture.rgba_pixels,
                );
                painter.image(
                    texture.id(),
                    egui_rect(*bounds),
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
        }
    }
}

fn append_action_operations(
    operations: &mut Vec<CommandChromePaintOperation>,
    clip_bounds: UiRect,
    action: &ActionPaintSource,
    style: &CommandChromePaintStyle,
) {
    operations.push(fill(
        clip_bounds,
        action.primary_bounds,
        action_fill(action.disabled, action.primary_hovered, style),
    ));
    if let Some(secondary_bounds) = action.secondary_bounds {
        operations.push(fill(
            clip_bounds,
            secondary_bounds,
            action_fill(action.disabled, action.secondary_hovered, style),
        ));
    }
    append_texture_operations(
        operations,
        clip_bounds,
        action.primary_bounds,
        &action.rendered,
    );
}

fn append_texture_operations(
    operations: &mut Vec<CommandChromePaintOperation>,
    clip_bounds: UiRect,
    action_bounds: UiRect,
    rendered: &RenderedAction,
) {
    let mut x = action_bounds.x.saturating_add_unsigned(ACTION_PADDING_PX);
    if let Some(icon) = &rendered.icon {
        let bounds = raster_bounds(x, action_bounds, icon);
        x = x.saturating_add_unsigned(icon.width);
        operations.push(texture(
            EguiCommandChromeDrawLayer::IconTexture,
            clip_bounds,
            bounds,
            texture_from_raster(icon),
        ));
    }
    if let Some(label) = &rendered.label {
        operations.push(texture(
            EguiCommandChromeDrawLayer::TextTexture,
            clip_bounds,
            raster_bounds(x, action_bounds, label),
            texture_from_raster(label),
        ));
    }
}

fn action_fill(
    disabled: bool,
    hovered: bool,
    style: &CommandChromePaintStyle,
) -> [u8; RGBA_CHANNEL_COUNT] {
    if disabled {
        style.disabled_action_rgba
    } else if hovered {
        style.hovered_action_rgba
    } else {
        style.action_rgba
    }
}

fn fill(
    clip_bounds: UiRect,
    bounds: UiRect,
    color_rgba: [u8; RGBA_CHANNEL_COUNT],
) -> CommandChromePaintOperation {
    layered_fill(
        EguiCommandChromeDrawLayer::ActionFill,
        clip_bounds,
        bounds,
        color_rgba,
    )
}

pub(super) fn layered_fill(
    layer: EguiCommandChromeDrawLayer,
    clip_bounds: UiRect,
    bounds: UiRect,
    color_rgba: [u8; RGBA_CHANNEL_COUNT],
) -> CommandChromePaintOperation {
    CommandChromePaintOperation {
        layer,
        clip_bounds,
        kind: CommandChromePaintOperationKind::Fill { bounds, color_rgba },
    }
}

pub(super) fn texture(
    layer: EguiCommandChromeDrawLayer,
    clip_bounds: UiRect,
    bounds: UiRect,
    texture: CommandChromePaintTexture,
) -> CommandChromePaintOperation {
    CommandChromePaintOperation {
        layer,
        clip_bounds,
        kind: CommandChromePaintOperationKind::Texture { bounds, texture },
    }
}

pub(super) fn texture_from_raster(raster: &RenderedRaster) -> CommandChromePaintTexture {
    CommandChromePaintTexture {
        identity: raster.identity.clone(),
        width: raster.physical_width,
        height: raster.physical_height,
        rgba_pixels: raster.pixels.clone(),
    }
}

fn raster_bounds(x: i32, action_bounds: UiRect, raster: &RenderedRaster) -> UiRect {
    UiRect::new(
        x,
        action_bounds
            .y
            .saturating_add_unsigned(action_bounds.height.saturating_sub(raster.height) / 2),
        raster.width,
        raster.height,
    )
}

fn egui_rect(bounds: UiRect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(bounds.x as f32, bounds.y as f32),
        egui::vec2(bounds.width as f32, bounds.height as f32),
    )
}

fn color([red, green, blue, alpha]: [u8; RGBA_CHANNEL_COUNT]) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(red, green, blue, alpha)
}
