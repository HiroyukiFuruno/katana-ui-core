use super::canvas::Canvas;
use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use crate::test_assert::KucTestExpect;
use katana_ui_core::atom::{Checkbox, ColorSwatch, Radio, SlideControl};
use katana_ui_core::molecule::{SettingsControl, SettingsField, SettingsList, SettingsSection};
use katana_ui_core::render_model::{UiInteractionState, UiNode, UiNodeKind};
use katana_ui_core::theme::ThemeSnapshot;

const HOVER_TEST_CANVAS_WIDTH: usize = 180;
const HOVER_TEST_CANVAS_HEIGHT: usize = 40;

#[test]
fn generic_checkbox_hover_draws_kuc_interactive_preset_border() {
    assert_hover_border(UiNode::from(Checkbox::new("Done")).interaction(hovered()));
}

#[test]
fn generic_radio_hover_draws_kuc_interactive_preset_border() {
    assert_hover_border(UiNode::from(Radio::new("Mode")).interaction(hovered()));
}

#[test]
fn generic_color_swatch_hover_draws_kuc_interactive_preset_border() {
    assert_hover_border(UiNode::from(ColorSwatch::new("Accent")).interaction(hovered()));
}

#[test]
fn generic_slide_control_hover_draws_kuc_interactive_preset_border() {
    assert_hover_border(UiNode::from(SlideControl::new("Opacity")).interaction(hovered()));
}

#[test]
fn settings_toggle_control_hover_draws_kuc_interactive_preset_border() {
    let list: UiNode = SettingsList::new("Settings")
        .section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        )
        .into();
    let toggle = find_kind(&list, UiNodeKind::Toggle)
        .kuc_expect("SettingsList should render a Toggle")
        .clone();

    assert_hover_border(toggle.interaction(hovered()));
}

#[test]
fn settings_field_row_hover_draws_border_around_label_and_control() {
    let list: UiNode = SettingsList::new("Settings")
        .section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        )
        .into();
    let field = find_kind(&list, UiNodeKind::FormField)
        .kuc_expect("SettingsList should render a FormField")
        .clone();

    assert_hover_border_at(
        field.interaction(hovered()),
        &[(0, 0), (HOVER_TEST_CANVAS_WIDTH - 1, 0)],
    );
}

#[test]
fn settings_section_header_hover_draws_kuc_interactive_preset_border() {
    let list: UiNode = SettingsList::new("Settings")
        .section(
            SettingsSection::new("display", "Display").field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            )),
        )
        .into();
    let section = find_kind(&list, UiNodeKind::Panel)
        .kuc_expect("SettingsList should render a section header")
        .clone();

    assert_hover_border(section.interaction(hovered()));
}

fn assert_hover_border(root: UiNode) {
    assert_hover_border_at(root, &[(0, 0)]);
}

fn assert_hover_border_at(root: UiNode, points: &[(usize, usize)]) {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(
        HOVER_TEST_CANVAS_WIDTH,
        HOVER_TEST_CANVAS_HEIGHT,
        palette.background,
    );

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: HOVER_TEST_CANVAS_WIDTH,
            height: HOVER_TEST_CANVAS_HEIGHT,
            scroll_y: 0.0,
        },
    );

    for (x, y) in points {
        assert_eq!(
            Some(palette.visual.hover_border),
            pixel_at(&canvas, *x, *y),
            "hover border missing at {x},{y}"
        );
    }
}

fn hovered() -> UiInteractionState {
    UiInteractionState {
        hovered: true,
        ..UiInteractionState::default()
    }
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas
        .pixels()
        .get(y.saturating_mul(canvas.width()).saturating_add(x))
        .copied()
}

fn find_kind(node: &UiNode, kind: UiNodeKind) -> Option<&UiNode> {
    if node.kind() == kind {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| find_kind(child, kind))
}
