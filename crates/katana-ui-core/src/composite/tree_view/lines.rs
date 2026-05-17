use super::types::{TreeViewLineKind, TreeViewLineStyle};
use crate::floem_view::FloemColor;
use floem::views::{Decorators, container, empty, h_stack_from_iter};
use floem::{IntoView, View};

const LINE_SEGMENTS: usize = 48;
const DASH_WIDTH: f32 = 8.0;
const DASH_GAP: f32 = 4.0;

fn line_segment(width: f32, thickness: f32, color: floem::peniko::Color) -> Box<dyn View> {
    container(empty())
        .style(move |style| style.width(width).height(thickness).background(color))
        .into_any()
}

pub(super) fn horizontal_line(style: TreeViewLineStyle) -> Box<dyn View> {
    let color = FloemColor::from_token(style.color);
    match style.kind {
        TreeViewLineKind::Solid => container(empty())
            .style(move |view| view.height(style.thickness).width_full().background(color))
            .into_any(),
        TreeViewLineKind::Dashed | TreeViewLineKind::Dotted => {
            let width = if style.kind == TreeViewLineKind::Dotted {
                style.thickness
            } else {
                DASH_WIDTH
            };
            let segments = (0..LINE_SEGMENTS)
                .map(|_| line_segment(width, style.thickness, color))
                .collect::<Vec<_>>();
            h_stack_from_iter(segments)
                .style(|view| view.gap(DASH_GAP).width_full())
                .into_any()
        }
    }
}
