mod ellipsis;
mod hover_tree;
mod segments;
pub(super) use ellipsis::BreadcrumbSegments;
#[cfg(test)]
pub(super) use ellipsis::RenderSegment;

use crate::composite::breadcrumb::Breadcrumb;
use crate::theme::Theme;
use floem::IntoView;
use floem::View;
use floem::views::{Decorators, h_stack_from_iter};

const BREADCRUMB_GAP: f32 = 4.0;

pub(super) fn build_view(breadcrumb: Breadcrumb, theme: Theme) -> impl IntoView {
    let separator = breadcrumb.props.separator.clone();
    let props = breadcrumb.props;
    let font_size = theme.typography.body.font_size;
    let render_segments =
        BreadcrumbSegments::apply_ellipsis(&props.crumbs, props.max_visible_crumbs);
    let segment_count = render_segments.len();
    let mut nodes: Vec<Box<dyn View>> = Vec::with_capacity(render_segments.len().saturating_mul(2));

    for (display_index, segment) in render_segments.into_iter().enumerate() {
        nodes.push(segments::BreadcrumbRender::segment_view(
            &segment,
            &props,
            font_size,
            theme.clone(),
        ));

        if display_index + 1 < segment_count {
            nodes.push(segments::BreadcrumbRender::separator_node(
                separator.clone(),
                theme.clone(),
            ));
        }
    }

    h_stack_from_iter(nodes)
        .style(move |style| style.gap(BREADCRUMB_GAP).items_center().width_full())
}
