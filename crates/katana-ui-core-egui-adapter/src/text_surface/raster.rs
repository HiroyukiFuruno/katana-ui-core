use super::artifact_model::EguiTextSurfaceError;
use super::model::TextSurfaceRasterStyle;
use katana_ui_core::render_model::{UiRect, UiTextSpan, UiTextSpanStyle};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceGraphemeBox, TextSurfaceLayout, TextSurfacePoint,
};
use katana_ui_core::theme::FontFamily;
use katana_ui_core_text_raster::{
    PlatformTextRaster, PlatformTextRasterRequest, PlatformTextRasterizer,
};

pub(super) struct RasterFrame {
    pub raster: PlatformTextRaster,
    pub identity: String,
}

pub(super) fn rasterize_surface(
    rasterizer: &mut PlatformTextRasterizer,
    surface: &TextSurface,
    style: &TextSurfaceRasterStyle,
    max_width_px: f32,
    scale_factor: f32,
    identity: String,
) -> Result<RasterFrame, EguiTextSurfaceError> {
    let request = PlatformTextRasterRequest {
        spans: surface_spans(surface, style),
        font: style.font.clone(),
        fallback_color_rgba: style.fallback_color_rgba,
        line_height_px: style.line_height_px,
        max_width_px: Some(max_width_px.max(1.0)),
        scale_factor,
    };
    let raster = rasterizer.rasterize(&request)?;
    Ok(RasterFrame { raster, identity })
}

pub(super) fn rasterize_placeholder(
    rasterizer: &mut PlatformTextRasterizer,
    surface: &TextSurface,
    style: &TextSurfaceRasterStyle,
    max_width_px: f32,
    scale_factor: f32,
    identity: String,
) -> Result<Option<RasterFrame>, EguiTextSurfaceError> {
    let state = &surface.state().text_area;
    let placeholder = &surface.props().text_area.options().placeholder;
    if !state.value.is_empty() || state.composition.is_some() || placeholder.is_empty() {
        return Ok(None);
    }
    let request = PlatformTextRasterRequest {
        spans: UiTextSpan::emoji_marked_spans(placeholder, base_span_style(style)),
        font: style.font.clone(),
        fallback_color_rgba: style.fallback_color_rgba,
        line_height_px: style.line_height_px,
        max_width_px: Some(max_width_px.max(1.0)),
        scale_factor,
    };
    let raster = rasterizer.rasterize(&request)?;
    Ok(Some(RasterFrame { raster, identity }))
}

pub(super) fn layout_for_surface(
    frame: &RasterFrame,
    surface: &TextSurface,
    origin: TextSurfacePoint,
) -> katana_ui_core::text_surface::TextSurfaceLayout {
    if surface.state().text_area.value.is_empty() && surface.state().text_area.composition.is_none()
    {
        return empty_layout(frame.identity.clone(), surface, origin);
    }
    composed_layout(&frame.raster, surface, origin, frame.identity.clone())
}

pub(super) fn rasterize_gutter_label(
    rasterizer: &mut PlatformTextRasterizer,
    label: &str,
    style: &TextSurfaceRasterStyle,
    scale_factor: f32,
) -> Result<PlatformTextRaster, EguiTextSurfaceError> {
    let base_style = base_span_style(style);
    let request = PlatformTextRasterRequest {
        spans: UiTextSpan::emoji_marked_spans(label, base_style),
        font: style.font.clone(),
        fallback_color_rgba: style.fallback_color_rgba,
        line_height_px: style.line_height_px,
        max_width_px: None,
        scale_factor,
    };
    Ok(rasterizer.rasterize(&request)?)
}

fn surface_spans(surface: &TextSurface, style: &TextSurfaceRasterStyle) -> Vec<UiTextSpan> {
    let source = &surface.state().text_area.value;
    if source.is_empty() && surface.state().text_area.composition.is_none() {
        return UiTextSpan::emoji_marked_spans(" ", base_span_style(style));
    }
    let original = &surface.props().spans;
    let original_text = original
        .iter()
        .map(|span| span.text.as_str())
        .collect::<String>();
    if surface.state().text_area.composition.is_none() && original_text == *source {
        return original.clone();
    }
    UiTextSpan::emoji_marked_spans(composed_text(surface), base_span_style(style))
}

fn empty_layout(
    identity: String,
    surface: &TextSurface,
    origin: TextSurfacePoint,
) -> TextSurfaceLayout {
    let bounds = UiRect::new(
        origin.x,
        origin.y,
        1,
        surface.props().viewport.height.max(1),
    );
    TextSurfaceLayout::from_grapheme_boxes(
        identity,
        bounds,
        String::new(),
        vec![TextSurfaceGraphemeBox {
            grapheme_index: 0,
            byte_start: 0,
            byte_end: 0,
            bounds,
        }],
    )
}

fn composed_layout(
    raster: &PlatformTextRaster,
    surface: &TextSurface,
    origin: TextSurfacePoint,
    identity: String,
) -> katana_ui_core::text_surface::TextSurfaceLayout {
    let state = &surface.state().text_area;
    let Some(composition) = state.composition.as_ref() else {
        return raster.text_surface_layout(identity, origin);
    };
    raster.text_surface_layout_with_composition(
        identity,
        origin,
        state.selection.start,
        state.selection.end,
        &composition.preedit,
        composition.caret,
    )
}

fn composed_text(surface: &TextSurface) -> String {
    let state = &surface.state().text_area;
    let Some(composition) = state.composition.as_ref() else {
        return state.value.clone();
    };
    let (start, end) =
        safe_composition_range(&state.value, state.selection.start, state.selection.end);
    let (before, remainder) = state.value.split_at(start);
    let (_, after) = remainder.split_at(end - start);
    format!("{before}{}{after}", composition.preedit)
}

fn safe_composition_range(value: &str, start: usize, end: usize) -> (usize, usize) {
    let boundary = |index: usize| {
        let mut index = index.min(value.len());
        while !value.is_char_boundary(index) {
            index -= 1;
        }
        index
    };
    let start = boundary(start);
    let end = boundary(end);
    if start <= end {
        (start, end)
    } else {
        (end, start)
    }
}

fn base_span_style(style: &TextSurfaceRasterStyle) -> UiTextSpanStyle {
    UiTextSpanStyle {
        monospace: matches!(style.font.family, FontFamily::Monospace),
        color_rgba: style.fallback_color_rgba,
        ..UiTextSpanStyle::default()
    }
}

#[cfg(test)]
mod tests {
    use super::safe_composition_range;

    #[test]
    fn composition_range_clamps_reorders_and_preserves_utf8_boundaries() {
        assert_eq!((0, 3), safe_composition_range("日本", 1, 5));
        assert_eq!((0, 6), safe_composition_range("日本", usize::MAX, 1));
    }
}
