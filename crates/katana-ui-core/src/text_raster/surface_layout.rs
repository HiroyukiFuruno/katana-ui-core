use crate::render_model::UiRect;
use crate::text_raster::model::{PlatformTextGraphemeBounds, PlatformTextRaster};
use crate::text_surface::{TextSurfaceGraphemeBox, TextSurfaceLayout, TextSurfacePoint};

const MIN_LAYOUT_EXTENT: u32 = 1;

impl PlatformTextRaster {
    #[must_use]
    pub fn text_surface_layout(
        &self,
        identity: impl Into<String>,
        origin: TextSurfacePoint,
    ) -> TextSurfaceLayout {
        let graphemes = self
            .grapheme_bounds
            .iter()
            .enumerate()
            .map(|(grapheme_index, bounds)| TextSurfaceGraphemeBox {
                grapheme_index,
                byte_start: bounds.byte_start,
                byte_end: bounds.byte_end,
                bounds: surface_rect(bounds, origin),
            })
            .collect::<Vec<_>>();
        let content_bounds = content_bounds(&graphemes, origin);
        TextSurfaceLayout::from_grapheme_boxes(identity, content_bounds, &self.text, graphemes)
    }

    #[must_use]
    pub fn text_surface_layout_with_composition(
        &self,
        identity: impl Into<String>,
        origin: TextSurfacePoint,
        source_start: usize,
        source_end: usize,
        preedit: impl Into<String>,
        caret_byte: usize,
    ) -> TextSurfaceLayout {
        self.text_surface_layout(identity, origin).with_composition(
            source_start,
            source_end,
            preedit,
            caret_byte,
        )
    }
}

fn surface_rect(bounds: &PlatformTextGraphemeBounds, origin: TextSurfacePoint) -> UiRect {
    UiRect::new(
        origin.x.saturating_add(bounds.x.round() as i32),
        origin.y.saturating_add(bounds.y.round() as i32),
        bounds.width.round().max(MIN_LAYOUT_EXTENT as f32) as u32,
        bounds.height.round().max(MIN_LAYOUT_EXTENT as f32) as u32,
    )
}

fn content_bounds(graphemes: &[TextSurfaceGraphemeBox], origin: TextSurfacePoint) -> UiRect {
    let right_edge = graphemes
        .iter()
        .map(|value| value.bounds.x.saturating_add(value.bounds.width as i32))
        .max()
        .unwrap_or(origin.x);
    let bottom_edge = graphemes
        .iter()
        .map(|value| value.bounds.y.saturating_add(value.bounds.height as i32))
        .max()
        .unwrap_or(origin.y);
    UiRect::new(
        origin.x,
        origin.y,
        u32::try_from(right_edge.saturating_sub(origin.x)).unwrap_or_default(),
        u32::try_from(bottom_edge.saturating_sub(origin.y)).unwrap_or_default(),
    )
}
