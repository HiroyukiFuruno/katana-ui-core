use super::view::{indeterminate_band_width, indeterminate_offset};
use floem::IntoView;
use floem::views::{Decorators, container, empty, h_stack};

pub(super) fn render_determinate(
    progress: f32,
    track_width: f32,
    size: f32,
    radius: f32,
    track_color: floem::peniko::Color,
    fill_color: floem::peniko::Color,
) -> impl IntoView {
    let fill_width = (progress * track_width).clamp(0.0, track_width);
    let remainder_width = (track_width - fill_width).max(0.0);

    container(
        h_stack((
            container(empty()).style(move |style| {
                style
                    .width(fill_width)
                    .height(size)
                    .background(fill_color)
                    .border_radius(radius)
            }),
            container(empty()).style(move |style| style.width(remainder_width).height(size)),
        ))
        .style(move |style| style.width(track_width)),
    )
    .style(move |style| {
        style
            .width(track_width)
            .height(size)
            .background(track_color)
            .border_radius(radius)
    })
}

pub(super) fn render_indeterminate(
    track_width: f32,
    size: f32,
    radius: f32,
    track_color: floem::peniko::Color,
    fill_color: floem::peniko::Color,
    frame: u64,
) -> impl IntoView {
    let band_width = indeterminate_band_width(track_width).min(track_width);
    let offset = indeterminate_offset(frame, track_width, band_width);
    let trailing = (track_width - band_width - offset).max(0.0);

    container(
        h_stack((
            container(empty()).style(move |style| style.width(offset).height(size)),
            container(empty()).style(move |style| {
                style
                    .width(band_width)
                    .height(size)
                    .background(fill_color)
                    .border_radius(radius)
            }),
            container(empty()).style(move |style| style.width(trailing).height(size)),
        ))
        .style(move |style| style.width(track_width)),
    )
    .style(move |style| {
        style
            .width(track_width)
            .height(size)
            .background(track_color)
            .border_radius(radius)
    })
}
