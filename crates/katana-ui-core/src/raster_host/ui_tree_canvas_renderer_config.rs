use super::{MeasuredNodeHeightCache, UiTreeCanvasRenderer};
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
        /* WHY: 以前の font metrics に基づくスクロール測定値を再利用しない。 */
        self.scroll_height_cache = std::cell::RefCell::new(MeasuredNodeHeightCache::default());
        self
    }
}
