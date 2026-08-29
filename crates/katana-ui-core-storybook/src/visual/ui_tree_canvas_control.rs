use super::canvas::Canvas;
use super::switch_control;
use super::text::TextRenderer;
use super::ui_tree_canvas_checkbox::UiTreeCheckboxRenderer;
use super::ui_tree_canvas_entry::UiTreeEntryRenderer;
use super::ui_tree_canvas_hit_metrics::{
    TEXT_HEIGHT, button_dimensions, dimension_px, toggle_dimensions,
};
use super::ui_tree_canvas_hover::UiTreeCanvasHover;
use super::ui_tree_canvas_loading::UiTreeLoadingRenderer;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_separator::UiTreeSeparatorRenderer;
use super::ui_tree_canvas_svg_icon::SvgIconRaster;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::{UiNode, UiVariant};

const TEXT_SIZE: f32 = 14.0;
const TOGGLE_LABEL_GAP: usize = 8;
const BUTTON_LABEL_X_PADDING: usize = 6;
const BUTTON_DEFAULT_ROW_GAP: usize = 8;
const TOGGLE_LABEL_Y_OFFSET: usize = 2;
const COMPACT_BUTTON_LABEL_Y_OFFSET: usize = 5;
const SURFACE_ICON_SIZE: usize = 20;

pub(super) struct UiTreeControlRenderer;

impl UiTreeControlRenderer {
    pub(super) fn draw_checkbox(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
    ) {
        UiTreeCheckboxRenderer::draw(canvas, text, node, x, y, palette);
    }

    pub(super) fn draw_divider(
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        UiTreeSeparatorRenderer::draw_divider(canvas, node, x, y, area, palette);
    }

    pub(super) fn draw_loading(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
    ) {
        UiTreeLoadingRenderer::draw(canvas, text, node, x, y, palette);
    }

    pub(super) fn draw_button(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
    ) {
        let (width, height) = button_dimensions(node);
        if is_icon_button(node) {
            Self::draw_icon_button(canvas, text, node, x, *y, width, height, palette);
        } else if is_outline_button(node) {
            Self::draw_outline_button(canvas, text, node, x, *y, width, height, palette);
        } else {
            canvas.fill_rect(x, *y, width, height, palette.selection);
            Self::draw_button_label(
                canvas,
                text,
                node,
                x.saturating_add(BUTTON_LABEL_X_PADDING),
                *y,
                height,
                palette,
            );
        }
        UiTreeCanvasHover::draw_node_border(canvas, node, x, *y, width, height, palette);
        let requested_height = dimension_px(&node.props().common.height);
        if requested_height == 0 {
            *y = y.saturating_add(TEXT_HEIGHT + BUTTON_DEFAULT_ROW_GAP);
        }
    }

    pub(super) fn draw_toggle(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
        show_label: bool,
    ) {
        let (toggle_width, toggle_height) = toggle_dimensions();
        switch_control::draw_switch(
            canvas,
            &palette.visual,
            x,
            *y,
            toggle_width,
            toggle_height,
            node.props().checked,
        );
        UiTreeCanvasHover::draw_node_border(
            canvas,
            node,
            x,
            *y,
            toggle_width,
            toggle_height,
            palette,
        );
        if show_label {
            text.draw(
                canvas,
                &node.props().label,
                x + toggle_width + TOGGLE_LABEL_GAP,
                (*y).saturating_add(TOGGLE_LABEL_Y_OFFSET),
                TEXT_SIZE,
                palette.text,
            );
        }
        *y = y.saturating_add(toggle_height.max(TEXT_HEIGHT));
    }

    pub(super) fn draw_text_entry(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
        show_label: bool,
    ) {
        UiTreeEntryRenderer::draw_text_entry(canvas, text, node, x, y, palette, show_label);
    }

    pub(super) fn draw_select(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
        show_label: bool,
    ) {
        UiTreeEntryRenderer::draw_select(canvas, text, node, x, y, palette, show_label);
    }

    fn draw_icon_button(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        palette: UiTreeCanvasPalette,
    ) {
        if surface_icon_role(node).is_some() {
            Self::draw_surface_icon(canvas, node, x, y, width, height, palette.text);
            return;
        }
        let label_width = text.measure_width(&node.props().label, TEXT_SIZE);
        let label_x = x.saturating_add(width.saturating_sub(label_width) / 2);
        Self::draw_button_label(canvas, text, node, label_x, y, height, palette);
    }

    fn draw_surface_icon(
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) {
        let left = x.saturating_add(width.saturating_sub(SURFACE_ICON_SIZE) / 2);
        let top = y.saturating_add(height.saturating_sub(SURFACE_ICON_SIZE) / 2);
        let icon = &node.props().icon;
        let _ = SvgIconRaster::draw(canvas, icon, left, top, SURFACE_ICON_SIZE, color);
    }

    fn draw_outline_button(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        palette: UiTreeCanvasPalette,
    ) {
        canvas.stroke_rect(x, y, width, height, palette.muted_border);
        let label_width = text.measure_width(&node.props().label, TEXT_SIZE);
        let label_x = x.saturating_add(width.saturating_sub(label_width) / 2);
        Self::draw_button_label(canvas, text, node, label_x, y, height, palette);
    }

    fn draw_button_label(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: usize,
        height: usize,
        palette: UiTreeCanvasPalette,
    ) {
        if node.props().label.is_empty() {
            return;
        }
        let label_y = if height <= TEXT_HEIGHT {
            y.saturating_add(COMPACT_BUTTON_LABEL_Y_OFFSET)
        } else {
            y.saturating_add(height.saturating_sub(TEXT_HEIGHT) / 2)
        };
        text.draw(
            canvas,
            &node.props().label,
            x,
            label_y,
            TEXT_SIZE,
            palette.text,
        );
    }
}

fn is_icon_button(node: &UiNode) -> bool {
    matches!(node.props().variant, UiVariant::Icon)
}

fn surface_icon_role(node: &UiNode) -> Option<&str> {
    let role = node.props().icon.role.as_str();
    if role.starts_with("surface.") {
        Some(role)
    } else {
        None
    }
}

fn is_outline_button(node: &UiNode) -> bool {
    matches!(node.props().variant, UiVariant::Outline)
}

#[cfg(test)]
mod tests {
    use super::super::canvas::Canvas;
    use super::super::text::TextRenderer;
    use super::super::ui_tree_canvas::UiTreeCanvasRenderer;
    use super::super::ui_tree_canvas_palette::UiTreeCanvasPalette;
    use super::super::ui_tree_canvas_types::UiTreeRenderArea;
    use katana_ui_core::atom::{Button, Spinner, Toggle};
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::render_model::{
        UiDimension, UiIconProps, UiInteractionState, UiNode, UiTree, UiVariant,
    };
    use katana_ui_core::theme::ThemeSnapshot;

    const BACKGROUND: u32 = 0x101010;

    #[test]
    fn loading_renderer_changes_pixels_by_animation_phase() {
        let first = render_spinner(0);
        let second = render_spinner(1);

        assert_ne!(first.pixels(), second.pixels());
    }

    #[test]
    fn icon_button_keeps_transparent_base_until_hover_border() {
        let canvas = render_button(UiVariant::Icon);
        let palette = palette();

        assert_eq!(0, count_color(&canvas, palette.selection));
        assert_eq!(0, count_color(&canvas, palette.muted_border));
    }

    #[test]
    fn outline_button_keeps_outline_base() {
        let canvas = render_button(UiVariant::Outline);
        let palette = palette();

        assert_eq!(0, count_color(&canvas, palette.selection));
        assert!(count_color(&canvas, palette.muted_border) > 0);
    }

    #[test]
    fn surface_icon_button_draws_icon_without_label_text_run() {
        let canvas = render_surface_icon_button();
        let icon_pixels = count_non_background(&canvas);

        assert!(
            icon_pixels >= 60,
            "surface icon raster must be visible enough for Retina diagram controls: icon_pixels={icon_pixels}"
        );
        assert!(
            canvas.text_runs().is_empty(),
            "surface icon controls must not render fallback text labels"
        );
    }

    #[test]
    fn surface_overlay_icon_button_keeps_katana_transparent_button_fill() {
        let canvas = render_surface_icon_button();
        let palette = palette();

        assert_eq!(
            0,
            count_color(&canvas, palette.visual.surface),
            "KatanA diagram controls keep transparent button fill until hover"
        );
        assert_eq!(
            0,
            count_color(&canvas, palette.visual.border),
            "KatanA diagram controls do not draw a permanent square button border"
        );
    }

    #[test]
    fn toggle_can_draw_its_label_for_embedded_control_hosts() {
        let node: UiNode = Toggle::new("Enabled").into();
        let mut canvas = Canvas::new(180, 48, BACKGROUND);
        let text = TextRenderer::load(&UiCoreFacade::default(), "body");
        let mut y = 6;

        super::UiTreeControlRenderer::draw_toggle(
            &mut canvas,
            &text,
            &node,
            6,
            &mut y,
            palette(),
            true,
        );

        assert!(canvas.text_runs().iter().any(|run| run.text() == "Enabled"));
    }

    fn render_spinner(phase: u16) -> Canvas {
        let mut node: UiNode = Spinner::new("Loading").into();
        node = node.interaction(UiInteractionState {
            animation_phase: phase,
            ..UiInteractionState::default()
        });
        let tree = UiTree::new(node);
        let mut canvas = Canvas::new(140, 40, 0x111111);
        UiTreeCanvasRenderer::new(ThemeSnapshot::dark()).render(
            &mut canvas,
            tree.root(),
            UiTreeRenderArea {
                x: 4,
                y: 4,
                width: 120,
                height: 32,
                scroll_y: 0.0,
            },
        );
        canvas
    }

    fn render_button(variant: UiVariant) -> Canvas {
        let node: UiNode = Button::new("")
            .variant(variant)
            .width(UiDimension::px(28))
            .height(UiDimension::px(28))
            .into();
        let mut canvas = Canvas::new(40, 40, BACKGROUND);
        let text = TextRenderer::load(&UiCoreFacade::default(), "body");
        let mut y = 6;
        super::UiTreeControlRenderer::draw_button(&mut canvas, &text, &node, 6, &mut y, palette());
        canvas
    }

    fn render_surface_icon_button() -> Canvas {
        const MATERIAL_PAN_UP: &str = r##"<svg fill="#FFFFFF" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"><path d="M440-647 244-451q-12 12-28 11.5T188-452q-11-12-11.5-28t11.5-28l264-264q6-6 13-8.5t15-2.5q8 0 15 2.5t13 8.5l264 264q11 11 11 27.5T772-452q-12 12-28.5 12T715-452L520-647v447q0 17-11.5 28.5T480-160q-17 0-28.5-11.5T440-200v-447Z"/></svg>"##;
        let node: UiNode = Button::new("pan-up")
            .variant(UiVariant::Icon)
            .width(UiDimension::px(28))
            .height(UiDimension::px(28))
            .into();
        let node = node
            .icon(UiIconProps::new(MATERIAL_PAN_UP).role("surface.pan-up"))
            .style_class("surface-overlay-button");
        let mut canvas = Canvas::new(40, 40, BACKGROUND);
        let text = TextRenderer::load(&UiCoreFacade::default(), "body");
        let mut y = 6;
        super::UiTreeControlRenderer::draw_button(&mut canvas, &text, &node, 6, &mut y, palette());
        canvas
    }

    fn palette() -> UiTreeCanvasPalette {
        UiTreeCanvasPalette::from_theme(&ThemeSnapshot::dark())
    }

    fn count_color(canvas: &Canvas, color: u32) -> usize {
        canvas
            .pixels()
            .iter()
            .filter(|pixel| **pixel == color)
            .count()
    }

    fn count_non_background(canvas: &Canvas) -> usize {
        canvas
            .pixels()
            .iter()
            .filter(|pixel| **pixel != BACKGROUND)
            .count()
    }
}
