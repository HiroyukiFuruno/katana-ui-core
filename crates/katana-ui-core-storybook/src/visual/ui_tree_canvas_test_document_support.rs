use super::*;

pub(in crate::visual) const ALERT_BLOCK_HEIGHT: u16 = 46;
pub(in crate::visual) const RENDER_CANVAS_WIDTH: usize = 160;
pub(in crate::visual) const RENDER_CANVAS_HEIGHT: usize = 48;
pub(in crate::visual) const CONTEXT_MENU_ANCHOR_X: i32 = 10;
pub(in crate::visual) const CONTEXT_MENU_ANCHOR_Y: i32 = 12;
pub(in crate::visual) const CONTEXT_MENU_MIN_WIDTH: u32 = 180;
pub(in crate::visual) const VIEWER_HEADING_HEIGHT: u16 = 48;
pub(in crate::visual) const VIEWER_BLOCKQUOTE_HEIGHT: u16 = 46;
pub(in crate::visual) const VIEWER_CODE_BLOCK_HEIGHT: u16 = 59;
pub(in crate::visual) const VIEWER_LIST_ITEM_HEIGHT: u16 = 23;
pub(in crate::visual) const VIEWER_TABLE_HEIGHT: u16 = 104;
pub(in crate::visual) const VIEWER_TABLE_WIDTH: u16 = 1168;
pub(in crate::visual) const VIEWER_CODE_TOP_PADDING: u16 = 14;
pub(in crate::visual) const VIEWER_SURFACE_PADDING: u16 = 56;
pub(in crate::visual) const DOCUMENT_CODE_LEFT_PADDING: u16 = 24;
pub(in crate::visual) const DOCUMENT_LIST_DEPTH_INDENT: u16 = 40;
pub(in crate::visual) const DOCUMENT_QUOTE_DEPTH_INDENT: u16 = 32;
pub(in crate::visual) const DOCUMENT_QUOTE_BULLET_PADDING: u16 = 28;

pub(in crate::visual) fn alert_with_tone(label: &str, tone: UiTone) -> UiNode {
    UiNode::new(UiNodeKind::Text, label)
        .text(UiTextProps {
            role: "alert".to_string(),
            ..UiTextProps::default()
        })
        .severity(tone)
        .height(UiDimension::Px(ALERT_BLOCK_HEIGHT))
}

pub(in crate::visual) fn code_text_node() -> UiNode {
    UiNode::new(UiNodeKind::Text, "main.rs").text(UiTextProps {
        role: "code".to_string(),
        wrap: UiTextWrapMode::NoWrap,
        ..UiTextProps::default()
    })
}

pub(in crate::visual) fn viewer_table_stack_column_for_test() -> UiNode {
    UiNode::new(UiNodeKind::Column, "")
        .common(viewer_surface_padding_common())
        .child(text_block_for_test(
            "### Heading",
            "heading",
            VIEWER_HEADING_HEIGHT,
        ))
        .child(text_block_for_test(
            "> A blockquote",
            "blockquote",
            VIEWER_BLOCKQUOTE_HEIGHT,
        ))
        .child(code_block_for_test(
            "let code = \"directly after quote\";",
            VIEWER_CODE_BLOCK_HEIGHT,
        ))
        .child(text_block_for_test(
            "- A list item",
            "list-item",
            VIEWER_LIST_ITEM_HEIGHT,
        ))
        .child(
            text_block_for_test(
                "| Header |\n| --- |\n| Table after list |",
                "table",
                VIEWER_TABLE_HEIGHT,
            )
            .width(UiDimension::Px(VIEWER_TABLE_WIDTH)),
        )
}

pub(in crate::visual) fn text_block_for_test(label: &str, role: &str, height: u16) -> UiNode {
    UiNode::new(UiNodeKind::Text, label)
        .text(UiTextProps {
            role: role.to_string(),
            ..UiTextProps::default()
        })
        .height(UiDimension::Px(height))
}

pub(in crate::visual) fn code_block_for_test(label: &str, height: u16) -> UiNode {
    UiNode::new(UiNodeKind::Text, label)
        .text(UiTextProps {
            role: "code".to_string(),
            ..UiTextProps::default()
        })
        .font_role("code")
        .common(document_code_common(VIEWER_CODE_TOP_PADDING))
        .height(UiDimension::Px(height))
}

pub(in crate::visual) fn viewer_surface_padding_common() -> UiCommonProps {
    UiCommonProps::default().padding(UiEdgeInsets {
        top: UiDimension::Px(VIEWER_SURFACE_PADDING),
        right: UiDimension::Px(VIEWER_SURFACE_PADDING),
        bottom: UiDimension::Px(VIEWER_SURFACE_PADDING),
        left: UiDimension::Px(VIEWER_SURFACE_PADDING),
    })
}

pub(in crate::visual) fn document_code_common(top_padding_px: u16) -> UiCommonProps {
    UiCommonProps::default()
        .padding(UiEdgeInsets {
            top: UiDimension::Px(top_padding_px),
            left: UiDimension::Px(DOCUMENT_CODE_LEFT_PADDING),
            ..UiEdgeInsets::default()
        })
        .border(UiBorder::solid(1, 0, "document.code.border"))
}

pub(in crate::visual) fn list_depth_common(depth: u16) -> UiCommonProps {
    UiCommonProps::default().margin(UiEdgeInsets {
        left: UiDimension::Px(depth.saturating_mul(DOCUMENT_LIST_DEPTH_INDENT)),
        ..UiEdgeInsets::default()
    })
}

pub(in crate::visual) fn quote_depth_common(depth: u16) -> UiCommonProps {
    UiCommonProps::default().margin(quote_depth_margin(depth))
}

pub(in crate::visual) fn quote_bullet_common(depth: u16) -> UiCommonProps {
    quote_depth_common(depth).padding(UiEdgeInsets {
        left: UiDimension::Px(DOCUMENT_QUOTE_BULLET_PADDING),
        ..UiEdgeInsets::default()
    })
}

pub(in crate::visual) fn quote_depth_margin(depth: u16) -> UiEdgeInsets {
    UiEdgeInsets {
        left: UiDimension::Px(depth.saturating_mul(DOCUMENT_QUOTE_DEPTH_INDENT)),
        ..UiEdgeInsets::default()
    }
}

pub(in crate::visual) fn vertical_bounds_for_color_in_x_range(
    canvas: &Canvas,
    color: u32,
    start_x: usize,
    end_x: usize,
) -> Option<VerticalBounds> {
    let mut top = None;
    let mut bottom = None;
    let end_x = end_x.min(canvas.width());
    for y in 0..canvas.height() {
        let found = (start_x..end_x).any(|x| pixel_at(canvas, x, y) == Some(color));
        if found {
            top.get_or_insert(y);
            bottom = Some(y);
        }
    }
    Some(VerticalBounds {
        top: top?,
        bottom: bottom?,
    })
}
