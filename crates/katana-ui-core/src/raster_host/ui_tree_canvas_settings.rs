use super::canvas::Canvas;
use super::text::TextRenderer;
use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_hover::UiTreeCanvasHover;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::molecule::SettingsListLayoutMetrics;
use katana_ui_core::render_model::{UiNode, UiNodeKind};

const TEXT_SIZE: f32 = 14.0;
const SETTINGS_TITLE_TEXT_Y_OFFSET: usize = 4;
const SETTINGS_PANEL_TEXT_X_OFFSET: usize = 4;
const SETTINGS_PANEL_TEXT_Y_OFFSET: usize = 3;

pub(super) struct UiTreeSettingsRenderer;

pub(super) struct UiTreeSettingsContext<'a> {
    pub(super) renderer: &'a UiTreeCanvasRenderer,
    pub(super) text: &'a TextRenderer,
    pub(super) area: UiTreeRenderArea,
    pub(super) palette: UiTreeCanvasPalette,
}

impl UiTreeSettingsRenderer {
    pub(super) fn draw_settings_list(
        canvas: &mut Canvas,
        context: UiTreeSettingsContext<'_>,
        node: &UiNode,
        x: usize,
        y: &mut usize,
    ) {
        canvas.fill_rect(
            x,
            *y,
            context.area.width.saturating_sub(x),
            1,
            context.palette.muted_border,
        );
        context.text.draw(
            canvas,
            &node.props().label,
            x,
            *y + SETTINGS_TITLE_TEXT_Y_OFFSET,
            TEXT_SIZE,
            context.palette.text,
        );
        *y = y.saturating_add(to_usize(metrics().title_height()));
        for child in node.children() {
            context.renderer.render_node(
                canvas,
                child,
                x.saturating_add(to_usize(metrics().child_indent())),
                y,
                context.area,
                context.palette,
            );
        }
    }

    pub(super) fn draw_panel(
        canvas: &mut Canvas,
        context: UiTreeSettingsContext<'_>,
        node: &UiNode,
        x: usize,
        y: &mut usize,
    ) {
        let width = context
            .area
            .width
            .saturating_sub(x.saturating_sub(context.area.x));
        canvas.fill_rect(
            x,
            *y,
            width,
            to_usize(metrics().section_height()),
            context.palette.quote_background,
        );
        context.text.draw(
            canvas,
            &node.props().label,
            x + SETTINGS_PANEL_TEXT_X_OFFSET,
            *y + SETTINGS_PANEL_TEXT_Y_OFFSET,
            TEXT_SIZE,
            context.palette.text,
        );
        UiTreeCanvasHover::draw_node_border(
            canvas,
            node,
            x,
            *y,
            width,
            to_usize(metrics().section_height()),
            context.palette,
        );
        *y = y.saturating_add(to_usize(metrics().section_height()));
        for child in node.children() {
            if duplicate_panel_label(node, child) {
                continue;
            }
            context.renderer.render_node(
                canvas,
                child,
                x.saturating_add(to_usize(metrics().child_indent())),
                y,
                context.area,
                context.palette,
            );
        }
    }

    pub(super) fn draw_form_field(
        canvas: &mut Canvas,
        context: UiTreeSettingsContext<'_>,
        node: &UiNode,
        x: usize,
        y: &mut usize,
    ) {
        let row_top = *y;
        context.text.draw(
            canvas,
            &node.props().label,
            x,
            row_top + 2,
            TEXT_SIZE,
            context.palette.text,
        );
        let mut control_y = row_top;
        for child in node.children() {
            context.renderer.render_node(
                canvas,
                child,
                x.saturating_add(to_usize(metrics().field_label_width())),
                &mut control_y,
                context.area,
                context.palette,
            );
        }
        let next_y = control_y.max(row_top.saturating_add(to_usize(metrics().field_height())));
        UiTreeCanvasHover::draw_node_border(
            canvas,
            node,
            x,
            row_top,
            context
                .area
                .width
                .saturating_sub(x.saturating_sub(context.area.x)),
            next_y.saturating_sub(row_top),
            context.palette,
        );
        *y = next_y;
    }
}

fn duplicate_panel_label(parent: &UiNode, child: &UiNode) -> bool {
    child.kind() == UiNodeKind::Text && child.props().label == parent.props().label
}

const fn metrics() -> SettingsListLayoutMetrics {
    SettingsListLayoutMetrics::DEFAULT
}

const fn to_usize(value: u32) -> usize {
    value as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::atom::Text;
    use katana_ui_core::render_model::UiNode;
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn panel_renders_nonduplicate_child_content() {
        let root = UiNode::new(UiNodeKind::Panel, "Parent").child(Text::new("Child"));
        let mut canvas = Canvas::new(240, 100, 0);
        UiTreeCanvasRenderer::new(ThemeSnapshot::dark()).render(
            &mut canvas,
            &root,
            UiTreeRenderArea {
                x: 4,
                y: 4,
                width: 220,
                height: 90,
                scroll_y: 0.0,
            },
        );

        assert!(canvas.text_runs().iter().any(|run| run.text() == "Parent"));
        assert!(canvas.text_runs().iter().any(|run| run.text() == "Child"));
    }
}
