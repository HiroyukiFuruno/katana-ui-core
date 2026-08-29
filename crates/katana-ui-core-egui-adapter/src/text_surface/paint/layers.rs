use super::super::model::EguiTextSurfaceDrawLayer;

pub struct PaintLayers;

impl PaintLayers {
    pub(crate) fn build(has_placeholder: bool) -> Vec<EguiTextSurfaceDrawLayer> {
        let mut values = vec![
            EguiTextSurfaceDrawLayer::Background,
            EguiTextSurfaceDrawLayer::Gutter,
            EguiTextSurfaceDrawLayer::Selection,
            EguiTextSurfaceDrawLayer::Preedit,
            EguiTextSurfaceDrawLayer::Annotation,
        ];
        if has_placeholder {
            values.push(EguiTextSurfaceDrawLayer::PlaceholderTexture);
        }
        values.extend([
            EguiTextSurfaceDrawLayer::TextTexture,
            EguiTextSurfaceDrawLayer::Caret,
        ]);
        values
    }
}
