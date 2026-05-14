use crate::primitive::icon::{Icon, IconSize, IconSource};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::View;
use floem::views::{Decorators, container, h_stack, label};

const INDENT_STEP: f32 = 16.0;
const DISCLOSURE_SIZE: f32 = 10.0;
const DISCLOSURE_SLOT_WIDTH: f32 = 12.0;
const ITEM_ICON_SIZE: f32 = 14.0;
const ITEM_ICON_SLOT_WIDTH: f32 = 18.0;
const CHEVRON_DOWN: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><path d='M4 6l4 4 4-4' fill='none' stroke='currentColor' stroke-width='1.8' stroke-linecap='round' stroke-linejoin='round'/></svg>";
const CHEVRON_RIGHT: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><path d='M6 4l4 4-4 4' fill='none' stroke='currentColor' stroke-width='1.8' stroke-linecap='round' stroke-linejoin='round'/></svg>";

pub(super) fn indent_blocks(
    indent: usize,
    row_height: f32,
    line_color: floem::peniko::Color,
) -> Vec<Box<dyn View>> {
    (0..indent)
        .map(|_| {
            h_stack((
                container(label(|| ""))
                    .style(move |style| style.width(1.0).height(row_height).background(line_color)),
                container(label(|| ""))
                    .style(move |style| style.width(INDENT_STEP - 1.0).height(row_height)),
            ))
            .into_any()
        })
        .collect()
}

pub(super) fn disclosure_icon(
    expanded: bool,
    row_height: f32,
    color: Color,
    theme: Theme,
) -> impl IntoView {
    let source = if expanded {
        IconSource::SvgBytes(CHEVRON_DOWN)
    } else {
        IconSource::SvgBytes(CHEVRON_RIGHT)
    };
    let icon = Icon::new(source)
        .size(IconSize::Pt(DISCLOSURE_SIZE))
        .color_override(color)
        .view(theme);

    container(icon).style(move |style| {
        style
            .width(DISCLOSURE_SLOT_WIDTH)
            .height(row_height)
            .items_center()
            .justify_center()
    })
}

pub(super) fn disclosure_spacer(row_height: f32) -> impl IntoView {
    container(label(|| "")).style(move |style| {
        style
            .width(DISCLOSURE_SLOT_WIDTH)
            .height(row_height)
            .items_center()
    })
}

pub(super) fn item_icon(
    source: IconSource,
    row_height: f32,
    color: Color,
    theme: Theme,
) -> impl IntoView {
    let icon = Icon::new(source)
        .size(IconSize::Pt(ITEM_ICON_SIZE))
        .color_override(color)
        .view(theme);

    container(icon).style(move |style| {
        style
            .width(ITEM_ICON_SLOT_WIDTH)
            .height(row_height)
            .items_center()
            .justify_center()
    })
}
