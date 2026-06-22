use super::span_style::span_color;
use super::{UiTreeTextLineContext, UiTreeTextLines};
use crate::visual::canvas::Canvas;
use crate::visual::text::TextRenderer;
use crate::visual::ui_tree_canvas_text_line_width::{
    SpanTextRenderers, preserves_whitespace, span_part_width, span_visible_part_bounds,
    whitespace_width,
};
use crate::visual::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use crate::visual::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::atom::Text;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::render_model::{UiDimension, UiNode, UiTextSpan, UiTextSpanStyle};
use katana_ui_core::theme::{ColorToken, ThemeSnapshot};

const TEST_BACKGROUND: u32 = 0x151515;
const STRIKE_COLOR: u32 = 0xcc6633;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;

#[path = "ui_tree_canvas_text_lines_decoration_tests.rs"]
mod decoration_tests;
#[path = "ui_tree_canvas_text_lines_html_tests.rs"]
mod html_tests;
#[path = "ui_tree_canvas_text_lines_spacing_tests.rs"]
mod spacing_tests;
#[path = "ui_tree_canvas_text_lines_test_support.rs"]
mod support;
#[cfg(test)]
pub(super) use support::*;
