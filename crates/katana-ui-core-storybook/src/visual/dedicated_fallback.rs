use super::canvas::Canvas;
use super::coverage;
use super::dedicated_atoms;
use super::dedicated_basic;
use super::dedicated_complex;
use super::dedicated_feedback;
use super::dedicated_node_labels;
use super::palette::VisualPalette;
use super::text::TextRenderer;
use katana_ui_core::render_model::{UiNode, UiNodeKind};

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    let label = dedicated_node_labels::label_for(node.kind());
    match node.kind() {
        UiNodeKind::Button | UiNodeKind::TextButton | UiNodeKind::IconTextButton => {
            dedicated_basic::button(canvas, text, palette, x, y, label);
        }
        UiNodeKind::SvgButton => dedicated_atoms::icon_button(canvas, text, palette, x, y, label),
        UiNodeKind::Badge | UiNodeKind::Chip | UiNodeKind::AttachmentChip => {
            dedicated_feedback::badge(canvas, text, palette, x, y, label);
        }
        UiNodeKind::Input | UiNodeKind::TextArea | UiNodeKind::SelectBox => {
            dedicated_basic::outlined_control(canvas, text, palette, x, y, label);
        }
        UiNodeKind::Checkbox | UiNodeKind::Radio => {
            dedicated_atoms::selection_control(canvas, text, palette, x, y, label);
        }
        UiNodeKind::Toggle => dedicated_basic::toggle(canvas, text, palette, x, y, label),
        UiNodeKind::Divider => dedicated_atoms::divider(canvas, text, palette, x, y, label),
        UiNodeKind::Spacer => dedicated_atoms::spacer(canvas, text, palette, x, y, label),
        UiNodeKind::KeyCap => dedicated_atoms::key_cap(canvas, text, palette, x, y, label),
        UiNodeKind::LoadingDots => {
            dedicated_atoms::loading_dots(canvas, text, palette, x, y, label)
        }
        UiNodeKind::Spinner => dedicated_atoms::spinner(canvas, text, palette, x, y, label),
        UiNodeKind::ProgressBar => {
            dedicated_feedback::progress(canvas, text, node, palette, x, y, label);
        }
        UiNodeKind::ColorSwatch => {
            dedicated_atoms::color_swatch(canvas, text, palette, x, y, label);
        }
        UiNodeKind::SlideControl => {
            dedicated_atoms::slide_control(canvas, text, palette, x, y, label)
        }
        UiNodeKind::CodeDiff => dedicated_complex::diff(canvas, text, palette, x, y, label),
        UiNodeKind::ColorPicker => {
            dedicated_complex::color_picker(canvas, text, palette, x, y, label);
        }
        UiNodeKind::Modal | UiNodeKind::ModalOverlay => {
            dedicated_feedback::overlay(canvas, text, palette, x, y, label);
        }
        kind if coverage::has_dedicated_renderer(kind) => {
            dedicated_basic::structured(canvas, text, palette, x, y, label);
        }
        _ => dedicated_basic::fallback(canvas, text, palette, x, y),
    }
}
