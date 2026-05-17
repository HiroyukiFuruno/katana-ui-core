use crate::floem_view::FloemColor;
use crate::theme::color::Color;
use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::views::{Decorators, container, empty, h_stack, h_stack_from_iter, label};

use super::ResolvedLoadingDots;
use super::types::{ACTIVE_ALPHA, ACTIVE_SCALE, INACTIVE_ALPHA, INACTIVE_SCALE};

const LABEL_GAP: f32 = 8.0;
const MAX_ALPHA_U8: f32 = 255.0;

pub(super) fn dots_row(resolved: ResolvedLoadingDots, frame: usize) -> impl IntoView {
    let active_index = resolved.active_dot_index(frame);
    let dot_size = resolved.dot_size;
    let color = resolved.color;
    let dots = h_stack_from_iter(
        (0..resolved.dot_count).map(|index| dot(active_index == Some(index), dot_size, color)),
    )
    .style(move |s| s.gap(resolved.dot_gap).items_center());

    match resolved.label {
        Some(label_text) => h_stack((label(move || label_text.clone()), dots))
            .style(|s| s.gap(LABEL_GAP).items_center())
            .into_any(),
        None => container(dots).into_any(),
    }
}

fn dot(is_active: bool, dot_size: f32, color: Color) -> impl IntoView {
    let scale = if is_active {
        ACTIVE_SCALE
    } else {
        INACTIVE_SCALE
    };
    let alpha = if is_active {
        ACTIVE_ALPHA
    } else {
        INACTIVE_ALPHA
    };

    let diameter = dot_size * scale;
    let radius = diameter / 2.0;
    let color = fade(&color, alpha);

    empty().style(move |s| {
        s.width(diameter)
            .height(diameter)
            .border_radius(radius)
            .background(color)
    })
}

fn fade(color: &crate::theme::color::Color, alpha_ratio: f32) -> PenikoColor {
    let a = f32::from(color.a) * alpha_ratio;
    let normalized = a.clamp(0.0, MAX_ALPHA_U8).round() as u8;

    FloemColor::from_token(crate::theme::color::Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: normalized,
    })
}
