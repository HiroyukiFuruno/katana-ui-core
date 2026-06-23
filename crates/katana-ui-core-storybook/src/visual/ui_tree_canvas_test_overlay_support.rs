use super::*;
use crate::test_assert::KucTestExpect;

pub(in crate::visual) const DIAGRAM_PIXEL_RGBA: [u8; 4] = [0x20, 0x24, 0x28, 0xff];
pub(in crate::visual) const DIAGRAM_MEDIA_FRAME_HEIGHT: u16 = 200;
pub(in crate::visual) const OVERLAY_CONTROL_SIZE: u16 = 28;
pub(in crate::visual) const OVERLAY_MARGIN: u16 = 8;

pub(in crate::visual) fn diagram_media_frame_root() -> UiNode {
    let image = ImageSurface::from_rgba("diagram", "diagram", 1, 1, DIAGRAM_PIXEL_RGBA.to_vec())
        .kuc_expect("test image surface should be valid");
    UiNode::from(
        Stack::new()
            .child(
                UiNode::from(image)
                    .visual_role(UiVisualRole::MediaFrame)
                    .height(UiDimension::Px(DIAGRAM_MEDIA_FRAME_HEIGHT)),
            )
            .child(
                UiNode::from(
                    Column::new()
                        .child(
                            Row::new()
                                .child(overlay_control_spacer())
                                .child(overlay_control_button("up"))
                                .child(overlay_control_button("+")),
                        )
                        .child(
                            Row::new()
                                .child(overlay_control_button("left"))
                                .child(overlay_control_button("reset"))
                                .child(overlay_control_button("right")),
                        )
                        .child(
                            Row::new()
                                .child(overlay_control_spacer())
                                .child(overlay_control_button("down"))
                                .child(overlay_control_button("-")),
                        ),
                )
                .position(UiPosition::Absolute)
                .margin(bottom_right_overlay_margin())
                .z_index(UiZIndex::value(2)),
            )
            .child(
                UiNode::from(
                    Row::new()
                        .child(overlay_control_button("full"))
                        .child(overlay_control_button("copy")),
                )
                .position(UiPosition::Absolute)
                .margin(top_right_overlay_margin())
                .z_index(UiZIndex::value(2)),
            ),
    )
    .height(UiDimension::Px(DIAGRAM_MEDIA_FRAME_HEIGHT))
}

pub(in crate::visual) fn overlay_control_button(label: &str) -> UiNode {
    UiNode::from(Button::new(label))
        .variant(UiVariant::Icon)
        .style_class("surface-overlay-button")
        .width(UiDimension::Px(OVERLAY_CONTROL_SIZE))
        .height(UiDimension::Px(OVERLAY_CONTROL_SIZE))
}

pub(in crate::visual) fn overlay_control_spacer() -> UiNode {
    UiNode::new(UiNodeKind::Stack, "")
        .width(UiDimension::Px(OVERLAY_CONTROL_SIZE))
        .height(UiDimension::Px(OVERLAY_CONTROL_SIZE))
}

pub(in crate::visual) fn top_right_overlay_margin() -> UiEdgeInsets {
    UiEdgeInsets {
        top: UiDimension::Px(OVERLAY_MARGIN),
        right: UiDimension::Px(OVERLAY_MARGIN),
        bottom: UiDimension::Px(0),
        left: UiDimension::Px(0),
    }
}

pub(in crate::visual) fn bottom_right_overlay_margin() -> UiEdgeInsets {
    UiEdgeInsets {
        top: UiDimension::Px(0),
        right: UiDimension::Px(OVERLAY_MARGIN),
        bottom: UiDimension::Px(OVERLAY_MARGIN),
        left: UiDimension::Px(0),
    }
}
