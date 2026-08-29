use super::layout_model::TextSurfaceLayout;
use super::scroll_request::{aligned_scroll_offset, target_bounds};
use super::scroll_request_types::{
    TextSurfaceScrollRequestAcknowledgement, TextSurfaceScrollRequestResult,
};
use super::state::TextSurfaceScrollBounds;
use super::surface_model::TextSurface;
use crate::render_model::UiRect;

impl TextSurface {
    /// Synchronizes renderer-measured extents so all TextSurface scroll inputs use one bound.
    pub fn synchronize_scroll_bounds(&mut self, content_bounds: UiRect, viewport_bounds: UiRect) {
        let measured = TextSurfaceScrollBounds::from_extents(
            content_bounds.width,
            content_bounds.height,
            viewport_bounds.width,
            viewport_bounds.height,
        );
        self.state.scroll_bounds = Some(match self.state.scroll_bounds {
            Some(previous) => TextSurfaceScrollBounds {
                max_x: previous.max_x.max(measured.max_x),
                max_y: previous.max_y.max(measured.max_y),
            },
            None => measured,
        });
        self.clamp_scroll_offset();
    }

    /// Applies a pending controlled request after the adapter has produced the current layout.
    /// This does not route through the interaction action pipeline.
    pub fn resolve_controlled_scroll_request(
        &mut self,
        layout: &TextSurfaceLayout,
        viewport_bounds: UiRect,
    ) -> Option<TextSurfaceScrollRequestResult> {
        self.resolve_controlled_scroll_request_with_scale(layout, viewport_bounds, 1.0)
    }

    /// Resolves a request using the display scale measured by the adapter.
    pub fn resolve_controlled_scroll_request_with_scale(
        &mut self,
        layout: &TextSurfaceLayout,
        viewport_bounds: UiRect,
        scale_factor: f32,
    ) -> Option<TextSurfaceScrollRequestResult> {
        let request = self.props.scroll_request.clone()?;
        if self.state.last_scroll_request_token.as_ref() == Some(&request.token) {
            return None;
        }
        let bounds = self.state.scroll_bounds?;
        self.state.last_scroll_request_token = Some(request.token.clone());
        let target = match target_bounds(layout, &request.target) {
            Ok(target) => target,
            Err(reason) => {
                return Some(TextSurfaceScrollRequestResult::Rejected {
                    token: request.token,
                    reason,
                });
            }
        };
        (self.state.scroll_x, self.state.scroll_y) = aligned_scroll_offset(
            self.state.scroll_x,
            self.state.scroll_y,
            target,
            viewport_bounds,
            &request,
            bounds,
            scale_factor,
        );
        Some(TextSurfaceScrollRequestResult::Acknowledged(
            TextSurfaceScrollRequestAcknowledgement {
                token: request.token,
                target_bounds: target,
                scroll_x: self.state.scroll_x,
                scroll_y: self.state.scroll_y,
            },
        ))
    }
}
