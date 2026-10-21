use super::{MeasuredNodeHeightCache, UiTreeCanvasRenderer};
use crate::raster_host::ui_tree_canvas_text::text_lines::text_wrap::UiTreeTextWrap;
use katana_ui_core::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};

impl UiTreeCanvasRenderer {
    /// 本文の face 選択と typography を維持し、code span の raster 設定だけを置き換えます。
    /// 描画・折返し・測定・hit は同じ code renderer を共有します。
    #[must_use]
    pub fn with_code_text_raster_config(
        mut self,
        config: PlatformTextRasterConfig,
        face_selection: PlatformTextFaceSelection,
    ) -> Self {
        self.code_text = self.code_text.with_raster_config(config, face_selection);
        /* WHY: PLAIN_LINE_CACHE はrenderer設定をキーに含めないため、code face変更後に破棄する。 */
        UiTreeTextWrap::clear_plain_line_cache();
        /* WHY: 以前の font metrics に基づくスクロール測定値を再利用しない。 */
        self.scroll_height_cache = std::cell::RefCell::new(MeasuredNodeHeightCache::default());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raster_host::{Canvas, UiTreeRenderArea};
    use crate::render_model::{UiNode, UiNodeKind, UiTextProps, UiTextWrapMode};
    use crate::theme::ThemeSnapshot;

    #[test]
    fn code_override_clears_plain_code_wrap_cache() {
        let plain_code = UiNode::new(UiNodeKind::Text, "plain code ".repeat(24))
            .font_role("code")
            .text(UiTextProps {
                role: "code".into(),
                wrap: UiTextWrapMode::Wrap,
                ..UiTextProps::default()
            });
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 180,
            height: 140,
            scroll_y: 0.0,
        };
        let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
        let mut canvas = Canvas::new(180, 140, 0);
        renderer.render(&mut canvas, &plain_code, area);
        assert!(UiTreeTextWrap::cached_plain_line_count_for_test() > 0);

        let _configured = renderer.with_code_text_raster_config(
            PlatformTextRasterConfig::default(),
            PlatformTextFaceSelection::CandidateChain,
        );

        assert_eq!(0, UiTreeTextWrap::cached_plain_line_count_for_test());
    }
}
