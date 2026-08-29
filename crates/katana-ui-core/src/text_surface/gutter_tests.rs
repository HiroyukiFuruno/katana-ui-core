use super::{
    TextSurface, TextSurfaceAction, TextSurfaceEvent, TextSurfaceGraphemeBox, TextSurfaceGutter,
    TextSurfaceGutterRow, TextSurfaceLayout, TextSurfaceProps, TextSurfaceViewport,
};
use crate::atom::TextArea;
use crate::render_model::UiIconProps;
use crate::render_model::{UiRect, UiTextSpan};

const VIEWPORT_WIDTH: u32 = 640;
const VIEWPORT_HEIGHT: u32 = 320;
const GUTTER_WIDTH: u32 = 24;
const TEXT_ORIGIN_X: i32 = 24;
const GRAPHEME_WIDTH: u32 = 10;
const LINE_HEIGHT: u32 = 20;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gutter_reuses_line_box_geometry_and_emits_only_typed_generic_events() {
        let text_area = TextArea::new("editor").value("a\nb");
        let mut surface = TextSurface::new(
            TextSurfaceProps::new(
                text_area,
                vec![UiTextSpan::plain("a\nb")],
                TextSurfaceViewport::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
            )
            .gutter(
                TextSurfaceGutter::new(GUTTER_WIDTH)
                    .row(
                        TextSurfaceGutterRow::new(0, "1")
                            .accessibility_label("line one")
                            .accessibility_description("active line")
                            .visual_role("active-line"),
                    )
                    .row(
                        TextSurfaceGutterRow::new(1, "2")
                            .marker_id("marker.two")
                            .accessibility_label("line two"),
                    ),
            ),
        );
        let layout = TextSurfaceLayout::from_grapheme_boxes(
            "raster:gutter:1",
            UiRect::new(TEXT_ORIGIN_X, 0, GRAPHEME_WIDTH * 2, LINE_HEIGHT * 2),
            "a\nb",
            vec![
                grapheme_box(0, 0, 1, TEXT_ORIGIN_X, 0),
                grapheme_box(1, 1, 2, TEXT_ORIGIN_X + GRAPHEME_WIDTH as i32, 0),
                grapheme_box(2, 2, 3, TEXT_ORIGIN_X, LINE_HEIGHT as i32),
            ],
        );

        let frame = surface.frame(&layout);
        let row = surface.apply_action(TextSurfaceAction::ActivateGutterRow { logical_row: 0 });
        let marker = surface.apply_action(TextSurfaceAction::ActivateGutterMarker {
            logical_row: 1,
            marker_id: "marker.two".to_string(),
        });

        assert_eq!(2, frame.gutter.len());
        assert_eq!("active-line", frame.gutter[0].visual_role);
        assert_eq!(
            Some("active line".to_string()),
            frame.gutter[0].accessibility_description
        );
        assert_eq!(
            UiRect::new(0, 0, GUTTER_WIDTH, LINE_HEIGHT),
            frame.gutter[0].bounds
        );
        assert_eq!(
            UiRect::new(0, LINE_HEIGHT as i32, GUTTER_WIDTH, LINE_HEIGHT),
            frame.gutter[1].bounds
        );
        assert!(matches!(
            row.events.as_slice(),
            [TextSurfaceEvent::GutterRowActivated { logical_row: 0 }]
        ));
        assert!(matches!(
            marker.events.as_slice(),
            [TextSurfaceEvent::GutterMarkerActivated { logical_row: 1, marker_id }]
                if marker_id == "marker.two"
        ));
    }

    #[test]
    fn automatic_range_marker_resolves_current_utf8_layout_by_priority_and_input_order()
    -> Result<(), Box<dyn std::error::Error>> {
        use super::super::{
            TextSurfaceAutomaticGutterPresentation, TextSurfaceAutomaticGutterRangeOverride,
            TextSurfaceGutterRangeStartAnchor,
        };
        let source = "日本語\n⭐️\nthird";
        let high = TextSurfaceAutomaticGutterRangeOverride {
            byte_start: "日本語\n".len(),
            byte_end: "日本語\n⭐️".len(),
            start_anchor: TextSurfaceGutterRangeStartAnchor::ContainingLine,
            marker_id: "generic.high".to_string(),
            priority: 2,
            accessibility_label: "marker".to_string(),
            accessibility_description: Some("description".to_string()),
            visual_role: "attention".to_string(),
            icon: Some(UiIconProps::new(
                "<svg viewBox=\"0 0 8 8\"><path d=\"M1 1h6v6z\"/></svg>",
            )),
        };
        let low = TextSurfaceAutomaticGutterRangeOverride {
            marker_id: "generic.low".to_string(),
            priority: 1,
            ..high.clone()
        };
        let same_priority_later = TextSurfaceAutomaticGutterRangeOverride {
            marker_id: "generic.same-priority-later".to_string(),
            ..high.clone()
        };
        let gutter = TextSurfaceAutomaticGutterPresentation::new()
            .override_range(low)
            .override_range(high)
            .override_range(same_priority_later);
        let mut surface = TextSurface::new(TextSurfaceProps::new(
            TextArea::new("editor").value(source),
            vec![UiTextSpan::plain(source)],
            TextSurfaceViewport::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
        ));
        let mut presentation = super::super::TextSurfacePresentation::from_props(surface.props());
        presentation.automatic_gutter = Some(gutter);
        surface.synchronize_presentation(presentation);
        let layout = TextSurfaceLayout::from_grapheme_boxes(
            "range",
            UiRect::new(TEXT_ORIGIN_X, 0, 80, 60),
            source,
            vec![
                grapheme_box(0, 0, "日".len(), TEXT_ORIGIN_X, 0),
                grapheme_box(
                    1,
                    "日本語\n".len(),
                    "日本語\n⭐️".len(),
                    TEXT_ORIGIN_X,
                    LINE_HEIGHT as i32,
                ),
                grapheme_box(
                    2,
                    "日本語\n⭐️\n".len(),
                    source.len(),
                    TEXT_ORIGIN_X,
                    (LINE_HEIGHT * 2) as i32,
                ),
            ],
        );
        let frame = surface.frame(&layout);
        assert_eq!(Some("generic.high"), frame.gutter[1].marker_id.as_deref());
        assert!(frame.gutter[1].icon.is_some());
        let marker_bounds = frame.gutter[1].marker_bounds.ok_or_else(|| {
            std::io::Error::other("icon-bearing range marker must have KUC-derived bounds")
        })?;
        assert_ne!(frame.gutter[1].bounds, marker_bounds);
        assert!(marker_bounds.x >= frame.gutter[1].bounds.x);
        assert!(marker_bounds.y >= frame.gutter[1].bounds.y);
        Ok(())
    }

    #[test]
    fn automatic_range_marker_ignores_invalid_utf8_offsets_and_reresolves_after_controlled_update()
    {
        use super::super::{
            TextSurfaceAutomaticGutterPresentation, TextSurfaceAutomaticGutterRangeOverride,
            TextSurfaceGutterRangeStartAnchor,
        };
        let source = "日本語\n⭐️\nthird";
        let marker = TextSurfaceAutomaticGutterRangeOverride {
            byte_start: "日本語\n".len(),
            byte_end: "日本語\n⭐️".len(),
            start_anchor: TextSurfaceGutterRangeStartAnchor::ContainingLine,
            marker_id: "range.marker".to_string(),
            priority: 1,
            accessibility_label: "range marker".to_string(),
            accessibility_description: None,
            visual_role: "attention".to_string(),
            icon: Some(UiIconProps::new(
                "<svg viewBox=\"0 0 8 8\"><path d=\"M1 1h6v6z\"/></svg>",
            )),
        };
        let invalid = TextSurfaceAutomaticGutterRangeOverride {
            byte_start: "日".len().saturating_sub(1),
            marker_id: "invalid".to_string(),
            ..marker.clone()
        };
        let mut surface = TextSurface::new(TextSurfaceProps::new(
            TextArea::new("editor").value(source),
            vec![UiTextSpan::plain(source)],
            TextSurfaceViewport::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
        ));
        let mut presentation = super::super::TextSurfacePresentation::from_props(surface.props());
        presentation.automatic_gutter = Some(
            TextSurfaceAutomaticGutterPresentation::new()
                .override_range(invalid)
                .override_range(marker.clone()),
        );
        assert!(surface.synchronize_presentation(presentation));

        let updated = "prefix\n日本語\n⭐️\nthird";
        let mut updated_presentation =
            super::super::TextSurfacePresentation::from_props(surface.props());
        updated_presentation.value = updated.to_string();
        updated_presentation.spans = vec![UiTextSpan::plain(updated)];
        updated_presentation.automatic_gutter = Some(
            TextSurfaceAutomaticGutterPresentation::new().override_range(
                TextSurfaceAutomaticGutterRangeOverride {
                    byte_start: "prefix\n日本語\n".len(),
                    byte_end: "prefix\n日本語\n⭐️".len(),
                    ..marker
                },
            ),
        );
        assert!(surface.synchronize_presentation(updated_presentation));
        let layout = TextSurfaceLayout::from_grapheme_boxes(
            "updated-range",
            UiRect::new(TEXT_ORIGIN_X, 0, 80, 80),
            updated,
            vec![
                grapheme_box(0, 0, "prefix".len(), TEXT_ORIGIN_X, 0),
                grapheme_box(
                    1,
                    "prefix\n".len(),
                    "prefix\n日本語".len(),
                    TEXT_ORIGIN_X,
                    20,
                ),
                grapheme_box(
                    2,
                    "prefix\n日本語\n".len(),
                    "prefix\n日本語\n⭐️".len(),
                    TEXT_ORIGIN_X,
                    40,
                ),
                grapheme_box(
                    3,
                    "prefix\n日本語\n⭐️\n".len(),
                    updated.len(),
                    TEXT_ORIGIN_X,
                    60,
                ),
            ],
        );
        let frame = surface.frame(&layout);
        assert!(
            frame
                .gutter
                .iter()
                .all(|row| row.marker_id.as_deref() != Some("invalid"))
        );
        assert_eq!(
            Some(2),
            frame
                .gutter
                .iter()
                .find(|row| row.marker_id.as_deref() == Some("range.marker"))
                .map(|row| row.logical_row)
        );
    }

    #[test]
    fn gutter_builders_defaults_width_and_range_boundaries_are_total() {
        use super::super::{
            TextSurfaceAutomaticGutterOverride, TextSurfaceAutomaticGutterPresentation,
            TextSurfaceAutomaticGutterRangeOverride, TextSurfaceGutterRangeStartAnchor,
            TextSurfaceGutterRowId,
        };

        let row_id = TextSurfaceGutterRowId::for_logical_row(1);
        assert_eq!(row_id.as_str(), "kuc-gutter-row-1");
        let row_override = TextSurfaceAutomaticGutterOverride::default()
            .marker_id("row.marker")
            .accessibility_label("row label")
            .accessibility_description("row description")
            .visual_role("attention");
        assert_eq!(row_override.marker_id.as_deref(), Some("row.marker"));
        assert_eq!(
            row_override.accessibility_description.as_deref(),
            Some("row description")
        );

        let source = "日\nnext";
        let layout = TextSurfaceLayout::from_grapheme_boxes(
            "gutter-total",
            UiRect::new(TEXT_ORIGIN_X, 0, 50, 40),
            source,
            vec![
                grapheme_box(0, 0, "日".len(), TEXT_ORIGIN_X, 0),
                grapheme_box(1, "日\n".len(), source.len(), TEXT_ORIGIN_X, 20),
            ],
        );
        let following = TextSurfaceAutomaticGutterRangeOverride {
            byte_start: "日".len(),
            byte_end: "日\n".len(),
            start_anchor: TextSurfaceGutterRangeStartAnchor::FollowingLine,
            marker_id: "range.marker".into(),
            priority: 1,
            accessibility_label: "range label".into(),
            accessibility_description: None,
            visual_role: "range".into(),
            icon: None,
        };
        let invalid_reversed = TextSurfaceAutomaticGutterRangeOverride {
            byte_start: source.len(),
            byte_end: 0,
            ..following.clone()
        };
        let invalid_end = TextSurfaceAutomaticGutterRangeOverride {
            byte_start: 0,
            byte_end: source.len() + 1,
            ..following.clone()
        };
        let invalid_boundary = TextSurfaceAutomaticGutterRangeOverride {
            byte_start: 1,
            byte_end: "日".len(),
            ..following.clone()
        };
        let presentation = TextSurfaceAutomaticGutterPresentation::default()
            .override_row(row_id.clone(), TextSurfaceAutomaticGutterOverride::new())
            .override_row(row_id, row_override)
            .override_range(invalid_reversed)
            .override_range(invalid_end)
            .override_range(invalid_boundary)
            .override_range(following);
        assert_eq!(presentation.overrides.len(), 1);

        let controlled = TextSurfaceGutter::from_controlled_automatic(presentation);
        assert!(controlled.is_controlled_automatic());
        assert!(controlled.layout_derived_width(&layout) > 0);
        let rows = controlled.resolved_rows(&layout);
        assert_eq!(rows[1].marker_id.as_deref(), Some("range.marker"));

        let explicit = TextSurfaceGutter::new(GUTTER_WIDTH).row(
            TextSurfaceGutterRow::new(0, "one")
                .marker_id("explicit.marker")
                .accessibility_label("explicit label")
                .accessibility_description("explicit description")
                .visual_role("explicit"),
        );
        assert_eq!(explicit.layout_derived_width(&layout), GUTTER_WIDTH);
        assert_eq!(explicit.resolved_rows(&layout).len(), 1);
    }

    fn grapheme_box(
        grapheme_index: usize,
        byte_start: usize,
        byte_end: usize,
        x: i32,
        y: i32,
    ) -> TextSurfaceGraphemeBox {
        TextSurfaceGraphemeBox {
            grapheme_index,
            byte_start,
            byte_end,
            bounds: UiRect::new(x, y, GRAPHEME_WIDTH, LINE_HEIGHT),
        }
    }
}
