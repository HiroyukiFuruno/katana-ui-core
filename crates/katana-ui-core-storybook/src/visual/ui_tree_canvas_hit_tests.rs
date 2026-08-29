use super::ui_tree_canvas_hit::UiTreeHostActionHitCollector;
use super::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use super::ui_tree_canvas_types::{UiTreeHitRect, UiTreeHostActionHit, UiTreeRenderArea};
use super::{Canvas, TextRenderer, UiTreeCanvasRenderer};
use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
use crate::visual::ui_tree_canvas_text_line_width::{SpanTextRenderers, span_part_width};
use katana_ui_core::atom::Button;
use katana_ui_core::atom::Checkbox;
use katana_ui_core::atom::ImageSurface;
use katana_ui_core::atom::Text;
use katana_ui_core::atom::Toggle;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::layout::{Row, Stack};
use katana_ui_core::molecule::Accordion;
use katana_ui_core::molecule::{SettingsControl, SettingsField, SettingsList, SettingsSection};
use katana_ui_core::molecule::{TreeNode, TreeView};
use katana_ui_core::render_model::{
    UiCommonProps, UiCursor, UiDimension, UiEdgeInsets, UiHostActionSpec, UiNode, UiNodeKind,
    UiPosition, UiScrollAreaProps, UiTextSpan, UiVariant, UiVisualRole, UiZIndex,
};
use katana_ui_core::theme::ThemeSnapshot;

const OVERLAY_BUTTON_MARGIN_PX: u16 = 8;

#[path = "ui_tree_canvas_hit_control_tests.rs"]
mod ui_tree_canvas_hit_control_tests;
#[path = "ui_tree_canvas_hit_media_frame_tests.rs"]
mod ui_tree_canvas_hit_media_frame_tests;
#[path = "ui_tree_canvas_hit_overlay_tests.rs"]
mod ui_tree_canvas_hit_overlay_tests;
#[path = "ui_tree_canvas_hit_overlay_visual_tests.rs"]
mod ui_tree_canvas_hit_overlay_visual_tests;
#[path = "ui_tree_canvas_hit_scroll_tests.rs"]
mod ui_tree_canvas_hit_scroll_tests;
#[path = "ui_tree_canvas_hit_settings_tree_tests.rs"]
mod ui_tree_canvas_hit_settings_tree_tests;
#[path = "ui_tree_canvas_hit_text_tests.rs"]
mod ui_tree_canvas_hit_text_tests;

fn text_hit_width(node: &UiNode, text: &str) -> usize {
    let metrics = UiTreeTextMetrics::for_node(node);
    let renderer = TextRenderer::load(&UiCoreFacade::default(), "body");
    let span = UiTextSpan::plain(text);
    span_part_width(
        SpanTextRenderers::new(&renderer, &renderer),
        &span,
        metrics,
        node.props().text.role == "code",
    )
}

fn top_right_overlay_margin() -> UiEdgeInsets {
    UiEdgeInsets {
        top: UiDimension::Px(OVERLAY_BUTTON_MARGIN_PX),
        right: UiDimension::Px(OVERLAY_BUTTON_MARGIN_PX),
        bottom: UiDimension::Px(0),
        left: UiDimension::Px(0),
    }
}

fn bottom_right_overlay_margin() -> UiEdgeInsets {
    UiEdgeInsets {
        top: UiDimension::Px(0),
        right: UiDimension::Px(OVERLAY_BUTTON_MARGIN_PX),
        bottom: UiDimension::Px(OVERLAY_BUTTON_MARGIN_PX),
        left: UiDimension::Px(0),
    }
}

fn action_ids(hits: &[UiTreeHostActionHit]) -> Vec<&str> {
    hits.iter()
        .map(|hit| hit.action.action_id.as_str())
        .collect()
}

fn bounds_for_color(canvas: &Canvas, color: u32) -> Option<(usize, usize, usize, usize)> {
    let mut min_x = canvas.width();
    let mut min_y = canvas.height();
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    let width = canvas.width();
    let height = canvas.height();

    for y in 0..height {
        for x in 0..width {
            let pixel = canvas.pixels()[y.saturating_mul(width) + x];
            if pixel == color {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    if min_x == canvas.width() {
        return None;
    }

    Some((
        min_x,
        min_y,
        max_x.saturating_sub(min_x) + 1,
        max_y.saturating_sub(min_y) + 1,
    ))
}
