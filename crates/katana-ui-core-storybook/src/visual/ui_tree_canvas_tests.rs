use super::ui_tree_canvas::is_outside_vertical_viewport;
use super::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use crate::visual::palette::VisualPalette;
use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
use katana_ui_core::atom::{Button, ImageSurface, Text, Toggle};
use katana_ui_core::layout::{Column, Row, Stack};
use katana_ui_core::molecule::{
    SettingsControl, SettingsControlOption, SettingsField, SettingsList, SettingsListDensity,
    SettingsSection,
};
use katana_ui_core::render_model::{
    UI_TASK_SET_STATE_ACTION_ID, UiBorder, UiCommonProps, UiContextMenuAnchor, UiContextMenuItem,
    UiContextMenuItemKind, UiContextMenuProps, UiDimension, UiEdgeInsets, UiHostActionPayload,
    UiHostActionSpec, UiInteractionState, UiNode, UiNodeKind, UiPosition, UiScrollAreaProps,
    UiTextProps, UiTextSpan, UiTextSpanStyle, UiTextWrapMode, UiTone, UiTreeNodeKind,
    UiTreeNodeProps, UiTreeProps, UiVariant, UiVisualRole, UiZIndex,
};
use katana_ui_core::theme::ThemeSnapshot;

#[path = "ui_tree_canvas_control_tests.rs"]
mod ui_tree_canvas_control_tests;
#[path = "ui_tree_canvas_document_alert_tests.rs"]
mod ui_tree_canvas_document_alert_tests;
#[path = "ui_tree_canvas_document_list_layout_tests.rs"]
mod ui_tree_canvas_document_list_layout_tests;
#[path = "ui_tree_canvas_document_quote_code_tests.rs"]
mod ui_tree_canvas_document_quote_code_tests;
#[path = "ui_tree_canvas_document_scroll_heading_tests.rs"]
mod ui_tree_canvas_document_scroll_heading_tests;
#[path = "ui_tree_canvas_image_measure_tests.rs"]
mod ui_tree_canvas_image_measure_tests;
#[path = "ui_tree_canvas_image_media_frame_tests.rs"]
mod ui_tree_canvas_image_media_frame_tests;
#[path = "ui_tree_canvas_image_text_tests.rs"]
mod ui_tree_canvas_image_text_tests;
#[path = "ui_tree_canvas_test_core_support.rs"]
mod ui_tree_canvas_test_core_support;
#[path = "ui_tree_canvas_test_document_support.rs"]
mod ui_tree_canvas_test_document_support;
#[path = "ui_tree_canvas_test_overlay_support.rs"]
mod ui_tree_canvas_test_overlay_support;
#[path = "ui_tree_canvas_text_height_tests.rs"]
mod ui_tree_canvas_text_height_tests;
#[path = "ui_tree_canvas_tree_tests.rs"]
mod ui_tree_canvas_tree_tests;
pub(in crate::visual) use ui_tree_canvas_test_core_support::*;
pub(in crate::visual) use ui_tree_canvas_test_document_support::*;
pub(in crate::visual) use ui_tree_canvas_test_overlay_support::*;

#[test]
fn rendered_ui_tree_text_is_selectable_from_canvas() {
    let mut canvas = Canvas::new(320, 200, 0);
    let root = UiNode::new(UiNodeKind::Text, "Viewer selectable text");
    UiTreeCanvasRenderer::new(ThemeSnapshot::dark()).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 16,
            y: 24,
            width: 280,
            height: 120,
            scroll_y: 0.0,
        },
    );

    let run = &canvas.text_runs()[0];
    assert_eq!(
        Some("Viewer selectable text".to_string()),
        canvas.copy_text_in_selection(
            Some((run.x(), run.y() + run.height() / 2)),
            Some((run.right(), run.y() + run.height() / 2)),
        )
    );
}
