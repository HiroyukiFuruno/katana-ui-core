use super::canvas::Canvas;
use super::text::TextRenderer;
use super::ui_tree_canvas_choice_control::UiTreeChoiceControlRenderer;
use super::ui_tree_canvas_context_menu::UiTreeContextMenuRenderer;
use super::ui_tree_canvas_control::UiTreeControlRenderer;
use super::ui_tree_canvas_grid::UiTreeGridRenderer;
use super::ui_tree_canvas_hit_metrics::{
    INDENT, NODE_GAP, TEXT_HEIGHT, absolute_child_rect, has_absolute_child, is_absolute,
    render_origin_y,
};
use super::ui_tree_canvas_layout::UiTreeLayoutRenderer;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_scroll;
use super::ui_tree_canvas_scroll_height_cache::MeasuredNodeHeightCache;
use super::ui_tree_canvas_settings::{UiTreeSettingsContext, UiTreeSettingsRenderer};
use super::ui_tree_canvas_text::{UiTreeTextContext, UiTreeTextRenderer};
use super::ui_tree_canvas_text_metrics::{UiTreeDocumentTypography, UiTreeTextMetrics};
use super::ui_tree_canvas_tree::UiTreeViewRenderer;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiVisualRole};
use katana_ui_core::theme::ThemeSnapshot;

#[path = "ui_tree_canvas_geometry.rs"]
mod geometry;
#[path = "ui_tree_canvas_renderer_methods.rs"]
mod renderer_methods;
#[path = "ui_tree_canvas_renderer_types.rs"]
mod renderer_types;
pub(in crate::raster_host) use geometry::is_outside_vertical_viewport;
use geometry::{
    ContainerPadding, child_container_x, child_render_area, dimension_px, draw_hover_background,
    draw_hover_surface, draw_label, gap_after_child, remaining_width, should_draw_container_label,
    stack_frame_height,
};
pub use renderer_types::UiTreeCanvasRenderer;

const TEXT_SIZE: f32 = 14.0;
const DOCUMENT_BODY_FONT_ROLE: &str = "document-body";
const DOCUMENT_EXPORT_BODY_FONT_ROLE: &str = "document-export-body";

impl UiTreeCanvasRenderer {
    #[must_use]
    pub fn new(theme: ThemeSnapshot) -> Self {
        let facade = UiCoreFacade::new(theme.clone());
        Self {
            palette: UiTreeCanvasPalette::from_theme(&theme),
            text: TextRenderer::load(&facade, facade.default_font_role()),
            document_text: TextRenderer::load(&facade, DOCUMENT_BODY_FONT_ROLE),
            export_text: TextRenderer::load(&facade, DOCUMENT_EXPORT_BODY_FONT_ROLE),
            code_text: TextRenderer::load(&facade, "code"),
            typography: UiTreeDocumentTypography::from_theme(&theme),
            scroll_height_cache: std::cell::RefCell::new(MeasuredNodeHeightCache::default()),
        }
    }

    pub fn render(&self, canvas: &mut Canvas, root: &UiNode, area: UiTreeRenderArea) {
        canvas.with_clip(area.x, area.y, area.width, area.height, &mut |canvas| {
            let mut y = render_origin_y(root, area);
            self.render_node(canvas, root, area.x, &mut y, area, self.palette);
        });
    }

    pub(super) fn measured_scroll_node_height(
        &self,
        node: &UiNode,
        text_context: UiTreeTextContext<'_>,
        x: usize,
        area: UiTreeRenderArea,
    ) -> usize {
        self.scroll_height_cache
            .borrow_mut()
            .height(node, text_context, x, area)
    }

    pub(super) fn text_context(&self, palette: UiTreeCanvasPalette) -> UiTreeTextContext<'_> {
        UiTreeTextContext {
            text: &self.document_text,
            export_text: &self.export_text,
            code_text: &self.code_text,
            palette,
            typography: self.typography,
        }
    }

    pub(super) fn render_node(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let start_y = *y;
        let requested_height = dimension_px(&node.props().common.height);
        if requested_height > 0 && is_outside_vertical_viewport(*y, requested_height, area) {
            *y = start_y.saturating_add(requested_height);
            return;
        }
        draw_hover_background(canvas, node, x, *y, area, palette);
        match node.kind() {
            UiNodeKind::TreeView => {
                UiTreeViewRenderer::draw(canvas, &self.text, node, x, y, area, palette)
            }
            UiNodeKind::Row => {
                UiTreeLayoutRenderer::draw_row(self, canvas, node, x, y, area, palette)
            }
            UiNodeKind::Accordion => self.draw_accordion(canvas, node, x, y, area, palette),
            UiNodeKind::Checkbox => {
                UiTreeControlRenderer::draw_checkbox(canvas, &self.text, node, x, y, palette)
            }
            UiNodeKind::Radio => {
                UiTreeChoiceControlRenderer::draw_radio(canvas, &self.text, node, x, y, palette)
            }
            UiNodeKind::ColorSwatch => UiTreeChoiceControlRenderer::draw_color_swatch(
                canvas, &self.text, node, x, y, palette,
            ),
            UiNodeKind::SlideControl => UiTreeChoiceControlRenderer::draw_slide_control(
                canvas, &self.text, node, x, y, palette,
            ),
            UiNodeKind::Toggle => {
                UiTreeControlRenderer::draw_toggle(canvas, &self.text, node, x, y, palette, false)
            }
            UiNodeKind::Input | UiNodeKind::TextArea | UiNodeKind::SearchBox => {
                UiTreeControlRenderer::draw_text_entry(
                    canvas, &self.text, node, x, y, palette, false,
                );
            }
            UiNodeKind::SelectBox | UiNodeKind::ComboBox => {
                UiTreeControlRenderer::draw_select(canvas, &self.text, node, x, y, palette, false);
            }
            UiNodeKind::SettingsList => UiTreeSettingsRenderer::draw_settings_list(
                canvas,
                self.settings_context(area, palette),
                node,
                x,
                y,
            ),
            UiNodeKind::ContextMenu => {
                UiTreeContextMenuRenderer::draw(canvas, &self.text, node, y, palette);
            }
            UiNodeKind::Panel => UiTreeSettingsRenderer::draw_panel(
                canvas,
                self.settings_context(area, palette),
                node,
                x,
                y,
            ),
            UiNodeKind::FormField => UiTreeSettingsRenderer::draw_form_field(
                canvas,
                self.settings_context(area, palette),
                node,
                x,
                y,
            ),
            UiNodeKind::Divider => {
                UiTreeControlRenderer::draw_divider(canvas, node, x, y, area, palette)
            }
            UiNodeKind::ImageSurface => self.draw_image(canvas, node, x, y, area, palette),
            UiNodeKind::Button | UiNodeKind::TextButton | UiNodeKind::IconTextButton => {
                UiTreeControlRenderer::draw_button(canvas, &self.text, node, x, y, palette);
            }
            UiNodeKind::Spinner | UiNodeKind::LoadingDots => {
                UiTreeControlRenderer::draw_loading(canvas, &self.text, node, x, y, palette);
            }
            UiNodeKind::ScrollArea => {
                ui_tree_canvas_scroll::draw_scroll_area(self, canvas, node, x, y, area, palette);
            }
            UiNodeKind::Grid => {
                UiTreeGridRenderer::draw(canvas, &self.text, node, x, y, area, palette);
            }
            UiNodeKind::Text => {
                let text_context = UiTreeTextContext {
                    text: &self.document_text,
                    export_text: &self.export_text,
                    code_text: &self.code_text,
                    palette,
                    typography: self.typography,
                };
                if requested_height > 0 {
                    canvas.with_clip(
                        x,
                        start_y,
                        remaining_width(area, x),
                        requested_height,
                        &mut |canvas| {
                            UiTreeTextRenderer::draw_node(canvas, text_context, node, x, y, area);
                        },
                    );
                } else {
                    UiTreeTextRenderer::draw_node(canvas, text_context, node, x, y, area);
                }
            }
            _ => self.draw_container(canvas, node, x, y, area, palette),
        }
        if requested_height > 0 {
            *y = start_y.saturating_add(requested_height);
        }
    }
}

#[cfg(test)]
mod direct_render_tests {
    use super::*;
    use katana_ui_core::molecule::Accordion;

    #[test]
    fn render_node_skips_offscreen_fixed_height_and_dispatches_accordion() {
        let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
        let mut canvas = Canvas::new(200, 100, 0);
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 200,
            height: 100,
            scroll_y: 0.0,
        };

        let fixed = UiNode::new(UiNodeKind::Text, "outside").height(UiDimension::Px(20));
        let mut y = 120;
        renderer.render_node(&mut canvas, &fixed, 0, &mut y, area, renderer.palette);
        assert_eq!(140, y);

        let accordion = UiNode::from(Accordion::new("Details").open(true));
        let mut accordion_y = 0;
        renderer.render_node(
            &mut canvas,
            &accordion,
            0,
            &mut accordion_y,
            area,
            renderer.palette,
        );
        assert!(accordion_y > 0);
    }

    #[test]
    fn input_family_nodes_share_the_text_entry_frame_contract() {
        let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 200,
            height: 100,
            scroll_y: 0.0,
        };

        for kind in [
            UiNodeKind::Input,
            UiNodeKind::TextArea,
            UiNodeKind::SearchBox,
        ] {
            let node = UiNode::new(kind, "Label").placeholder("Value");
            let mut canvas = Canvas::new(area.width, area.height, renderer.palette.background);
            let mut y = 0;

            renderer.render_node(&mut canvas, &node, 0, &mut y, area, renderer.palette);

            assert_eq!(20, y, "{kind:?} must consume one entry row");
            assert!(
                canvas.text_runs().iter().any(|run| run.text() == "Value"),
                "{kind:?} must render the same placeholder value contract"
            );
        }
    }

    #[test]
    fn select_family_nodes_share_the_selector_arrow_contract() {
        let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 200,
            height: 100,
            scroll_y: 0.0,
        };

        for kind in [UiNodeKind::SelectBox, UiNodeKind::ComboBox] {
            let node = UiNode::new(kind, "Label").placeholder("Choice");
            let mut canvas = Canvas::new(area.width, area.height, renderer.palette.background);
            let mut y = 0;

            renderer.render_node(&mut canvas, &node, 0, &mut y, area, renderer.palette);

            assert_eq!(20, y, "{kind:?} must consume one selector row");
            assert!(
                canvas.pixels().contains(&renderer.palette.selection),
                "{kind:?} must render the selector affordance"
            );
        }
    }
}
