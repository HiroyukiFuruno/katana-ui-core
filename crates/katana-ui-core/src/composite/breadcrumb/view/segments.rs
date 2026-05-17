use crate::composite::breadcrumb::{BreadcrumbCrumb, BreadcrumbProps};
use crate::primitive::icon::{Icon, IconSize, IconSource};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::View;
use floem::views::{Decorators, button, container, h_stack, label};
use std::rc::Rc;

use super::ellipsis::RenderSegment;
use super::hover_tree::BreadcrumbHoverTree;

const BREADCRUMB_ICON_GAP: f32 = 4.0;
const BREADCRUMB_PADDING_H: f32 = 6.0;
const BREADCRUMB_PADDING_V: f32 = 3.0;
const BREADCRUMB_RADIUS: f32 = 6.0;
const BREADCRUMB_BORDER_WIDTH: f32 = 1.0;
const ELLIPSIS: &str = "…";

fn crumb_is_clickable(index: usize, crumb: &BreadcrumbCrumb, props: &BreadcrumbProps) -> bool {
    if crumb.disabled || crumb.on_click.is_none() {
        return false;
    }

    index + 1 != props.crumbs.len() || props.allow_last_click
}

fn crumb_label(
    label_text: String,
    icon: Option<IconSource>,
    text_color: Color,
    icon_color: Color,
    font_size: f32,
    theme: Theme,
) -> Box<dyn View> {
    let text = label(move || label_text.clone()).style(move |style| {
        style
            .font_size(font_size)
            .color(crate::floem_view::FloemColor::from_token(text_color))
    });

    match icon {
        Some(icon) => {
            let icon_view = Icon::new(icon)
                .size(IconSize::Sm)
                .color_override(icon_color)
                .view(theme)
                .into_any();
            h_stack((icon_view, text))
                .style(|style| style.gap(BREADCRUMB_ICON_GAP).items_center())
                .into_any()
        }
        None => text.into_any(),
    }
}

fn frame_node(node: Box<dyn View>, props: &BreadcrumbProps, theme: Theme) -> Box<dyn View> {
    let show_border = props.show_border;
    let show_background = props.show_background;
    container(node)
        .style(move |style| {
            let mut style = style
                .padding_horiz(BREADCRUMB_PADDING_H)
                .padding_vert(BREADCRUMB_PADDING_V)
                .border_radius(BREADCRUMB_RADIUS);

            style = if show_border {
                style.border(BREADCRUMB_BORDER_WIDTH).border_color(
                    crate::floem_view::FloemColor::from_token(theme.color.border),
                )
            } else {
                style.border(0.0)
            };

            if show_background {
                style.background(crate::floem_view::FloemColor::from_token(
                    theme.color.surface,
                ))
            } else {
                style
            }
        })
        .into_any()
}

pub(crate) struct BreadcrumbRender;

impl BreadcrumbRender {
    pub(crate) fn separator_node(separator: String, theme: Theme) -> Box<dyn View> {
        label(move || separator.clone())
            .style(move |style| {
                style.color(crate::floem_view::FloemColor::from_token(
                    theme.color.text_muted,
                ))
            })
            .into_any()
    }
}

fn clickable_segment(
    crumb: &BreadcrumbCrumb,
    props: &BreadcrumbProps,
    font_size: f32,
    theme: Theme,
) -> Box<dyn View> {
    let on_click: Rc<dyn Fn()> = crumb
        .on_click
        .as_ref()
        .cloned()
        .unwrap_or_else(|| Rc::new(|| {}));
    let label = frame_node(
        crumb_label(
            crumb.label.clone(),
            crumb.icon.clone(),
            theme.color.accent,
            theme.color.accent,
            font_size,
            theme.clone(),
        ),
        props,
        theme,
    );

    button(label)
        .action(move || (on_click)())
        .style(|style| style.border(0.0))
        .into_any()
}

fn child_tree_segment(
    crumb: &BreadcrumbCrumb,
    props: &BreadcrumbProps,
    font_size: f32,
    theme: Theme,
) -> Box<dyn View> {
    let trigger_crumb = crumb.clone();
    let trigger_props = props.clone();
    let trigger_theme = theme.clone();
    let trigger = frame_node(
        crumb_label(
            trigger_crumb.label.clone(),
            trigger_crumb.icon.clone(),
            trigger_theme.color.accent,
            trigger_theme.color.accent,
            font_size,
            trigger_theme.clone(),
        ),
        &trigger_props,
        trigger_theme,
    );

    BreadcrumbHoverTree::view(trigger, crumb.children.clone(), theme)
}

fn readonly_segment(
    crumb: &BreadcrumbCrumb,
    props: &BreadcrumbProps,
    font_size: f32,
    theme: Theme,
) -> Box<dyn View> {
    let color = if crumb.disabled {
        theme.color.text_disabled
    } else {
        theme.color.text
    };
    frame_node(
        crumb_label(
            crumb.label.clone(),
            crumb.icon.clone(),
            color,
            color,
            font_size,
            theme.clone(),
        ),
        props,
        theme,
    )
}

impl BreadcrumbRender {
    pub(crate) fn segment_view(
        segment: &RenderSegment,
        props: &BreadcrumbProps,
        font_size: f32,
        theme: Theme,
    ) -> Box<dyn View> {
        match segment {
            RenderSegment::Ellipsis => label(|| ELLIPSIS.to_string())
                .style(move |style| {
                    style
                        .color(crate::floem_view::FloemColor::from_token(
                            theme.color.text_muted,
                        ))
                        .font_size(font_size)
                })
                .into_any(),
            RenderSegment::Crumb(index, crumb) => {
                if !crumb.children.is_empty() {
                    return child_tree_segment(crumb, props, font_size, theme);
                }

                if crumb_is_clickable(*index, crumb, props) {
                    clickable_segment(crumb, props, font_size, theme)
                } else {
                    readonly_segment(crumb, props, font_size, theme)
                }
            }
        }
    }
}
