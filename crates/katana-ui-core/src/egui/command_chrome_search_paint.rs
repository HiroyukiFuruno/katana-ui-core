use super::command_chrome_artifact::{
    CommandChromePaintOperation, CommandChromePaintOperationKind, CommandChromePaintPlan,
    CommandChromePaintTexture,
};
use super::command_chrome_paint::{
    layered_fill, plan_surface_bounds, texture, texture_from_raster,
};
use super::command_chrome_types::{
    CommandChromePaintStyle, EguiCommandChromeDrawLayer, EguiCommandChromeSearchFrameRecord,
    RenderedRaster,
};
use crate::egui::text_surface::{
    EguiTextSurfaceDrawLayer, TextSurfacePaintOperationKind, TextSurfacePaintPlan,
};
use crate::render_model::{RGBA_CHANNEL_COUNT, UiRect};

pub(super) struct SearchControlPaintSource {
    bounds: UiRect,
    raster: RenderedRaster,
    state: SearchControlPaintState,
}

pub(super) struct SearchControlPaintState {
    pub(super) icon: bool,
    pub(super) action: bool,
    pub(super) disabled: bool,
    pub(super) active: bool,
    pub(super) active_rgba: [u8; RGBA_CHANNEL_COUNT],
    pub(super) hovered: bool,
    pub(super) padding_px: u32,
}

impl SearchControlPaintSource {
    pub(super) fn new(
        bounds: UiRect,
        raster: RenderedRaster,
        state: SearchControlPaintState,
    ) -> Self {
        Self {
            bounds,
            raster,
            state,
        }
    }
}

pub(super) fn build_search_paint_plan(
    record: &EguiCommandChromeSearchFrameRecord,
    query: &TextSurfacePaintPlan,
    replace: Option<&TextSurfacePaintPlan>,
    controls: &[SearchControlPaintSource],
    style: &CommandChromePaintStyle,
) -> CommandChromePaintPlan {
    let mut operations = convert_text_surface_paint_plan(query);
    if let Some(replace) = replace {
        operations.extend(convert_text_surface_paint_plan(replace));
    }
    for control in controls {
        append_control_operations(&mut operations, record.bounds, control, style);
    }
    CommandChromePaintPlan {
        surface_bounds: plan_surface_bounds(record.bounds, &operations),
        operations,
    }
}

fn convert_text_surface_paint_plan(
    plan: &TextSurfacePaintPlan,
) -> Vec<CommandChromePaintOperation> {
    plan.operations
        .iter()
        .map(|operation| CommandChromePaintOperation {
            layer: match operation.layer {
                EguiTextSurfaceDrawLayer::Background
                | EguiTextSurfaceDrawLayer::Gutter
                | EguiTextSurfaceDrawLayer::Selection
                | EguiTextSurfaceDrawLayer::Preedit
                | EguiTextSurfaceDrawLayer::Annotation => EguiCommandChromeDrawLayer::PanelFill,
                EguiTextSurfaceDrawLayer::PlaceholderTexture
                | EguiTextSurfaceDrawLayer::TextTexture => EguiCommandChromeDrawLayer::TextTexture,
                EguiTextSurfaceDrawLayer::Caret => EguiCommandChromeDrawLayer::FocusRing,
            },
            clip_bounds: operation.clip_bounds,
            kind: match &operation.kind {
                TextSurfacePaintOperationKind::Fill { bounds, color_rgba } => {
                    CommandChromePaintOperationKind::Fill {
                        bounds: *bounds,
                        color_rgba: *color_rgba,
                    }
                }
                TextSurfacePaintOperationKind::Texture { bounds, texture } => {
                    CommandChromePaintOperationKind::Texture {
                        bounds: *bounds,
                        texture: CommandChromePaintTexture {
                            identity: texture.identity.clone(),
                            width: texture.width,
                            height: texture.height,
                            rgba_pixels: texture.rgba_pixels.clone(),
                        },
                    }
                }
            },
        })
        .collect()
}

fn append_control_operations(
    operations: &mut Vec<CommandChromePaintOperation>,
    clip_bounds: UiRect,
    control: &SearchControlPaintSource,
    style: &CommandChromePaintStyle,
) {
    if control.state.action {
        let fill = if control.state.disabled {
            style.disabled_action_rgba
        } else if control.state.active {
            control.state.active_rgba
        } else if control.state.hovered {
            style.hovered_action_rgba
        } else {
            style.action_rgba
        };
        operations.push(layered_fill(
            EguiCommandChromeDrawLayer::ActionFill,
            clip_bounds,
            control.bounds,
            fill,
        ));
    }
    let raster_bounds = UiRect::new(
        control
            .bounds
            .x
            .saturating_add_unsigned(control.state.padding_px),
        control.bounds.y.saturating_add_unsigned(
            control.bounds.height.saturating_sub(control.raster.height) / 2,
        ),
        control.raster.width,
        control.raster.height,
    );
    operations.push(texture(
        if control.state.icon {
            EguiCommandChromeDrawLayer::IconTexture
        } else {
            EguiCommandChromeDrawLayer::TextTexture
        },
        clip_bounds,
        raster_bounds,
        texture_from_raster(&control.raster),
    ));
}
