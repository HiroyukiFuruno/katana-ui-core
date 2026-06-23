use super::{UiNodeKind, UiRect, UiSlotPlacement, UiSvgIconRenderPlan, UiSvgPaintPolicy, UiTree};
use serde::{Deserialize, Serialize};

pub const DEFAULT_SVG_ICON_BOX_PX: u32 = 16;
pub const SVG_ICON_SCALE_DENOMINATOR: u32 = 1_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSvgIconViewBox {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl UiSvgIconViewBox {
    #[must_use]
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSvgIconPixelPlan {
    pub node_kind: UiNodeKind,
    pub label: String,
    pub slot_label: String,
    pub placement: Option<UiSlotPlacement>,
    pub viewport: UiRect,
    pub view_box: Option<UiSvgIconViewBox>,
    pub scale_x_milli: u32,
    pub scale_y_milli: u32,
    pub paint_policy: UiSvgPaintPolicy,
    pub color_token: String,
    pub theme_token: String,
    pub callback: String,
    pub pixel_ready: bool,
}

impl UiSvgIconPixelPlan {
    #[must_use]
    pub fn collect_from_tree(tree: &UiTree) -> Vec<Self> {
        Self::from_render_plans(&UiSvgIconRenderPlan::collect_from_tree(tree))
    }

    #[must_use]
    pub fn from_render_plans(plans: &[UiSvgIconRenderPlan]) -> Vec<Self> {
        plans.iter().map(Self::from_render_plan).collect()
    }

    fn from_render_plan(plan: &UiSvgIconRenderPlan) -> Self {
        let viewport = UiRect::new(0, 0, DEFAULT_SVG_ICON_BOX_PX, DEFAULT_SVG_ICON_BOX_PX);
        let view_box = parse_view_box(&plan.view_box);
        let (scale_x_milli, scale_y_milli) = scale_milli(viewport, view_box);
        Self {
            node_kind: plan.node_kind,
            label: plan.label.clone(),
            slot_label: plan.slot_label.clone(),
            placement: plan.placement,
            viewport,
            view_box,
            scale_x_milli,
            scale_y_milli,
            paint_policy: plan.paint_policy,
            color_token: plan.color_token.clone(),
            theme_token: plan.theme_token.clone(),
            callback: plan.callback.clone(),
            pixel_ready: view_box.is_some(),
        }
    }
}

fn scale_milli(viewport: UiRect, view_box: Option<UiSvgIconViewBox>) -> (u32, u32) {
    let Some(view_box) = view_box else {
        return (0, 0);
    };
    (
        viewport.width * SVG_ICON_SCALE_DENOMINATOR / view_box.width,
        viewport.height * SVG_ICON_SCALE_DENOMINATOR / view_box.height,
    )
}

fn parse_view_box(value: &str) -> Option<UiSvgIconViewBox> {
    let mut parts = value
        .split(|character: char| character.is_ascii_whitespace() || character == ',')
        .filter(|part| !part.is_empty());
    let x = parse_i32(parts.next()?)?;
    let y = parse_i32(parts.next()?)?;
    let width = parse_positive_u32(parts.next()?)?;
    let height = parse_positive_u32(parts.next()?)?;
    if parts.next().is_some() {
        return None;
    }
    Some(UiSvgIconViewBox::new(x, y, width, height))
}

fn parse_i32(value: &str) -> Option<i32> {
    value.parse::<i32>().ok()
}

fn parse_positive_u32(value: &str) -> Option<u32> {
    let parsed = value.parse::<u32>().ok()?;
    (parsed > 0).then_some(parsed)
}
