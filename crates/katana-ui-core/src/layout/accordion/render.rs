use floem::IntoView;
use floem::reactive::SignalGet;
use floem::views::{Decorators, container, h_stack};

pub(super) fn make_header_row(
    trigger: impl IntoView + 'static,
    style: HeaderRowStyle,
) -> impl IntoView {
    let HeaderRowStyle {
        header_bg,
        border_color,
        pad_v,
        pad_h,
        disabled,
        hover_color,
        gap,
    } = style;

    container(trigger).style(move |style| {
        let style = style
            .background(header_bg)
            .border(1.0)
            .border_color(border_color)
            .padding_vert(pad_v)
            .padding_horiz(pad_h)
            .width_full()
            .items_center()
            .gap(gap);

        if disabled {
            style
        } else {
            style.hover(move |hover_style| hover_style.background(hover_color))
        }
    })
}

#[derive(Clone, Copy)]
pub(super) struct HeaderRowStyle {
    pub(super) header_bg: floem::peniko::Color,
    pub(super) border_color: floem::peniko::Color,
    pub(super) pad_v: f32,
    pub(super) pad_h: f32,
    pub(super) disabled: bool,
    pub(super) hover_color: floem::peniko::Color,
    pub(super) gap: f32,
}

pub(super) fn make_body_view<IV: IntoView + 'static>(
    child: impl Fn() -> IV,
    open_ratio: floem::reactive::RwSignal<f32>,
    body_max_height: f32,
    pad_h: f32,
    pad_v: f32,
    body_border: bool,
    border_color: floem::peniko::Color,
) -> impl IntoView {
    container(child()).style(move |style| {
        let ratio = open_ratio.get().clamp(0.0, 1.0);
        let body_height = body_max_height * ratio;
        let style = style
            .height(body_height)
            .max_height(body_height)
            .padding_left(pad_h)
            .padding_right(pad_h)
            .padding_vert(pad_v);

        if body_border && ratio > 0.0 {
            style.border(1.0).border_color(border_color)
        } else {
            style
        }
    })
}

pub(super) fn make_trigger_wrapper(
    trigger: impl IntoView + 'static,
    tree_mode: super::AccordionTreeMode,
    tree_depth: usize,
    tree_show_lines: bool,
    line_color: floem::peniko::Color,
) -> impl IntoView {
    if tree_mode == super::AccordionTreeMode::Enabled {
        h_stack((
            super::view_helpers::build_tree_prefix(tree_depth, tree_show_lines, line_color)
                .into_any(),
            trigger.into_any(),
        ))
        .style(|style| style.items_center().width_full())
        .into_any()
    } else {
        trigger.into_any()
    }
}
