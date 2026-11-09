use super::super::super::super::types::TextCommandSurfaceStyle;
use super::super::super::model::{
    KucDialogTarget, SearchFilterValidity, SearchModePresentation, SearchResultMode,
    SearchResultProjection, SelectionDialogProjection, SelectionDialogProjectionLease,
};
use super::super::{SelectionDialogOutput, SelectionDialogRetainedState};
use super::controls;
use crate::egui::text_surface::{EguiTextSurfaceDrawLayer, TextSurfacePaintOperationKind};
use crate::render_model::UiRect;
use crate::text_raster::{PlatformTextRasterConfig, PlatformTextRasterResources};
use crate::theme::{ColorToken, Rgba, ThemeSnapshot};

const TEST_SCREEN_WIDTH: f32 = 900.0;
const TEST_SCREEN_HEIGHT: f32 = 640.0;
const INVALID_UNDERLINE_HEIGHT_PX: u32 = 2;
const INVALID_COLOR: Rgba = [13, 117, 203, 231];

fn lease(
    revision: u64,
    include_validity: Option<SearchFilterValidity>,
    exclude_validity: Option<SearchFilterValidity>,
) -> SelectionDialogProjectionLease {
    let target = KucDialogTarget::from_opaque_bytes(b"mode");
    let mode = SearchResultMode {
        target: target.clone(),
        label: "All".into(),
        query: String::new(),
        include: Some("path".into()),
        exclude: Some("generated".into()),
        options: Vec::new(),
        history: Vec::new(),
        results: Vec::new(),
        searched: false,
    };
    let mut presentation = SearchModePresentation::new();
    if let Some(validity) = include_validity {
        presentation = presentation.include_validity(validity);
    }
    if let Some(validity) = exclude_validity {
        presentation = presentation.exclude_validity(validity);
    }
    SelectionDialogProjectionLease::new(
        revision,
        "Search",
        controls(),
        SelectionDialogProjection::Search(SearchResultProjection {
            modes: vec![mode],
            active_mode: target.clone(),
        }),
    )
    .expect("valid search lease")
    .search_mode_presentation(target, presentation)
    .expect("valid search mode presentation")
}

fn show(
    state: &mut SelectionDialogRetainedState,
    context: &egui::Context,
    style: &TextCommandSurfaceStyle,
) -> SelectionDialogOutput {
    let mut result = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(TEST_SCREEN_WIDTH, TEST_SCREEN_HEIGHT),
            )),
            ..Default::default()
        },
        |ui| result = Some(state.show(ui.ctx(), style)),
    );
    output.textures_delta.clear();
    result
        .expect("dialog frame")
        .expect("dialog paint")
        .expect("dialog remains open")
}

fn annotation_fills(output: &SelectionDialogOutput) -> Vec<(UiRect, Rgba, UiRect)> {
    output
        .paint_plan
        .operations
        .iter()
        .filter_map(|operation| match (&operation.layer, &operation.kind) {
            (
                EguiTextSurfaceDrawLayer::Annotation,
                TextSurfacePaintOperationKind::Fill { bounds, color_rgba },
            ) => Some((*bounds, *color_rgba, operation.clip_bounds)),
            _ => None,
        })
        .collect()
}

fn assert_underlines_field_bottom(
    output: &SelectionDialogOutput,
    underline: &(UiRect, Rgba, UiRect),
) {
    let (bounds, color, clip) = underline;
    let field = output
        .paint_plan
        .operations
        .iter()
        .filter_map(|operation| match (&operation.layer, &operation.kind) {
            (
                EguiTextSurfaceDrawLayer::Background,
                TextSurfacePaintOperationKind::Fill { bounds, .. },
            ) if bounds.height > INVALID_UNDERLINE_HEIGHT_PX => Some(*bounds),
            _ => None,
        })
        .find(|field| {
            field.x == bounds.x
                && field.width == bounds.width
                && field.y + field.height as i32 == bounds.y + bounds.height as i32
        })
        .expect("underline aligns with its text input field");
    assert_eq!(bounds.x, field.x);
    assert_eq!(bounds.width, field.width);
    assert_eq!(
        bounds.y,
        field.y + field.height as i32 - INVALID_UNDERLINE_HEIGHT_PX as i32
    );
    assert_eq!(bounds.height, INVALID_UNDERLINE_HEIGHT_PX);
    assert_eq!(*color, INVALID_COLOR);
    assert_eq!(
        *clip, field,
        "visible field clip is field bounds intersect UI clip"
    );
    assert!(field.x >= 0 && field.y >= 0);
    assert!(field.x + field.width as i32 <= TEST_SCREEN_WIDTH as i32);
    assert!(field.y + field.height as i32 <= TEST_SCREEN_HEIGHT as i32);
}

#[test]
fn search_filter_invalid_decoration_is_scoped_and_tracks_sync_in_paint_plan() {
    let mut theme = ThemeSnapshot::dark();
    theme.colors.retain(|token| token.name != "invalid");
    theme.colors.push(ColorToken {
        name: "invalid".into(),
        rgba: INVALID_COLOR,
    });
    let style = TextCommandSurfaceStyle::from_theme(&theme).expect("custom theme style");
    let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
    let mut state = SelectionDialogRetainedState::new(
        lease(
            1,
            Some(SearchFilterValidity::Invalid),
            Some(SearchFilterValidity::Valid),
        ),
        &resources,
    );
    let context = egui::Context::default();

    let include_invalid = show(&mut state, &context, &style);
    let include_lines = annotation_fills(&include_invalid);
    assert_eq!(include_lines.len(), 1, "only invalid include is decorated");
    assert_underlines_field_bottom(&include_invalid, &include_lines[0]);
    let include_bounds = include_lines[0].0;

    state
        .synchronize(lease(
            2,
            Some(SearchFilterValidity::Valid),
            Some(SearchFilterValidity::Invalid),
        ))
        .expect("switch validity and synchronize");
    let exclude_invalid = show(&mut state, &context, &style);
    let exclude_lines = annotation_fills(&exclude_invalid);
    assert_eq!(exclude_lines.len(), 1, "only invalid exclude is decorated");
    assert_underlines_field_bottom(&exclude_invalid, &exclude_lines[0]);
    assert_ne!(exclude_lines[0].0, include_bounds);
    assert!(exclude_lines[0].0.y > include_bounds.y);

    state
        .synchronize(lease(3, None, None))
        .expect("clear both validity states");
    let unspecified = show(&mut state, &context, &style);
    assert!(
        annotation_fills(&unspecified).is_empty(),
        "unspecified validity has no invalid decoration"
    );

    state
        .synchronize(lease(
            4,
            Some(SearchFilterValidity::Valid),
            Some(SearchFilterValidity::Valid),
        ))
        .expect("mark both filters valid");
    let valid = show(&mut state, &context, &style);
    assert!(
        annotation_fills(&valid).is_empty(),
        "valid filters have no invalid decoration"
    );
}
