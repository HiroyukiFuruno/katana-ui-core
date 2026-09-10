use super::{
    Canvas, UiNode, UiTreeRenderArea, UiTreeTextContext, UiTreeTextLines, UiTreeTextMetrics,
    UiTreeTextRenderer, UiTreeTextTable, dimension_px, renderer_for_role, text_content_x,
};

impl UiTreeTextRenderer {
    /// 論理的な縦方向カーソルでテキストを描画し、canvas 操作は物理ピクセル境界で行う。
    ///
    /// 小数の行ボックスは正確な高さで論理カーソルを進める。ノードごとに丸めると、
    /// 31.5px の2行が本来の63pxではなく64pxを消費してしまう。
    pub(in crate::raster_host) fn draw_node_with_logical_cursor(
        canvas: &mut Canvas,
        context: UiTreeTextContext<'_>,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
    ) {
        let logical_start = *logical_y;
        let mut physical_y = logical_canvas_boundary(logical_start);
        Self::draw_node(canvas, context, node, x, &mut physical_y, area);
        *logical_y = logical_start + logical_advance_height(context, node, x, area);
    }
}

fn logical_advance_height(
    context: UiTreeTextContext<'_>,
    node: &UiNode,
    x: usize,
    area: UiTreeRenderArea,
) -> f32 {
    let requested_height = dimension_px(&node.props().common.height);
    if requested_height > 0 {
        return requested_height as f32;
    }
    let renderer = renderer_for_role(
        context.text,
        context.export_text,
        context.code_text,
        &node.props().font_role,
    );
    let metrics = UiTreeTextMetrics::for_node_with_typography(node, context.typography);
    let content_x = text_content_x(node, x);
    if node.props().text.role == "table" {
        return UiTreeTextTable::content_height(renderer, node, content_x, area, metrics) as f32;
    }
    UiTreeTextLines::line_count(renderer, context.code_text, node, content_x, area, metrics) as f32
        * metrics.line_box_height
}

fn logical_canvas_boundary(value: f32) -> usize {
    value.floor().max(0.0) as usize
}
