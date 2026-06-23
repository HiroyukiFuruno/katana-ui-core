mod adapter_coverage_plan;
mod button_layout;
mod command;
mod common;
mod common_builder;
mod common_types;
mod context_menu_item;
mod context_menu_props;
mod host_action_plan;
mod host_action_settings;
mod host_action_task;
mod host_action_text;
mod host_action_tree;
mod host_action_types;
mod identity;
mod image_surface_props;
mod image_surface_render_plan;
mod image_surface_transform;
mod interactive_preset;
mod kind;
mod props;
mod svg_icon_pixel_plan;
mod svg_icon_render_plan;
mod text_area_props;
mod tree;
mod tree_builder;
mod tree_model;
mod tree_props;
mod tree_semantics;
mod typed;

pub use adapter_coverage_plan::UiAdapterCoveragePlan;
pub use button_layout::{
    UiButtonLayoutDto, UiButtonLayoutPatchDto, UiButtonLayoutPreset, UiButtonLayoutSpec,
};
pub use command::{RenderContext, UiCommand, UiTreeDiff};
pub use common::UiCommonProps;
pub use common_types::{
    UiAlignItems, UiBorder, UiCursor, UiDimension, UiDisplay, UiEdgeInsets, UiJustifyContent,
    UiLayoutAxis, UiOverflow, UiPointerEvents, UiPosition, UiZIndex,
};
pub use context_menu_item::{UiContextMenuDividerTone, UiContextMenuItem, UiContextMenuItemKind};
pub use context_menu_props::{
    UiContextMenuAnchor, UiContextMenuPlacement, UiContextMenuProps, UiContextMenuRect,
};
pub use host_action_settings::{UiSettingsFieldControlTarget, UiSettingsSectionToggleTarget};
pub use host_action_task::{UiTaskControlAction, UiTaskControlMenuItem, UiTaskMarker};
pub use host_action_text::{UiAccordionToggleAction, UiTextSpanAction};
pub use host_action_tree::UiTreeRowActionTarget;
pub use host_action_types::{
    UI_CODE_COPY_ACTION_ID, UI_DISCLOSURE_TOGGLE_ACTION_ID, UI_IMAGE_HIGHLIGHT_ACTION_ID,
    UI_LINK_OPEN_ACTION_ID, UI_SETTINGS_FIELD_ACTIVATE_ACTION_ID,
    UI_SETTINGS_SECTION_TOGGLE_ACTION_ID, UI_TASK_SET_STATE_ACTION_ID, UI_TASK_STATE_ID_PREFIX,
    UI_TASK_TOGGLE_ACTION_ID, UI_TREE_ROW_ACTION_ID, UiHostActionKind, UiHostActionPayload,
    UiHostActionPlan, UiHostActionSpec, UiSettingsFieldControlActionPayload,
    UiSettingsSectionToggleActionPayload, UiSurfaceControlActionPayload,
    UiTaskControlActionPayload, UiTaskControlStateActionPayload, UiTreeRowActionKind,
    UiTreeRowActionPayload,
};
pub use identity::{UiNodeId, UiStateId};
pub use image_surface_props::{
    UiImageSurfaceFit, UiImageSurfaceHighlight, UiImageSurfaceProps, UiImageSurfaceValidationError,
};
pub use image_surface_render_plan::UiImageSurfaceRenderPlan;
pub use image_surface_transform::UiImageSurfaceTransform;
pub use interactive_preset::UiInteractivePreset;
pub use kind::UiNodeKind;
pub use props::{UiInteractionState, UiProps, UiSize, UiTone, UiVariant, UiVisualRole};
pub use svg_icon_pixel_plan::{
    DEFAULT_SVG_ICON_BOX_PX, SVG_ICON_SCALE_DENOMINATOR, UiSvgIconPixelPlan, UiSvgIconViewBox,
};
pub use svg_icon_render_plan::UiSvgIconRenderPlan;
pub use text_area_props::{
    UiTextAreaNewlineKey, UiTextAreaProps, UiTextAreaSubmitKey, UiTextAreaTabBehavior,
    UiTextAreaWrapPolicy,
};
pub use tree::UiNode;
pub use tree_model::UiTree;
pub use tree_props::{
    UiTreeLineStyle, UiTreeNodeKind, UiTreeNodeProps, UiTreeProps, UiTreeToggleTriggerArea,
};
pub use tree_semantics::UiTreeSemantics;
pub use typed::{
    UiAnimationState, UiButtonProps, UiClearActionSpec, UiColorBlendingMode, UiColorPickerProps,
    UiColorPickerTriggerKind, UiColorSwatchProps, UiCommandResultProps,
    UiDisclosureIndicatorPosition, UiDisclosureProps, UiDisclosureTriggerArea, UiDismissAction,
    UiDragHandleProps, UiDragPreviewProps, UiDropIndicatorProps, UiFormFieldProps, UiIconProps,
    UiLoadingProps, UiModalParentInteraction, UiModalPlacement, UiModalPresentation, UiModalProps,
    UiModalSize, UiPanelProps, UiPopoverFocusManagement, UiPopoverPlacement, UiPopoverProps,
    UiProgressMode, UiRect, UiScrollAreaAxis, UiScrollAreaProps, UiScrollbarDragState,
    UiScrollbarModel, UiScrollbarPlacement, UiScrollbarVisibility, UiSearchControlProps,
    UiSearchReplaceMode, UiShortcutProps, UiSkeletonAnimation, UiSkeletonProps, UiSkeletonShape,
    UiSlotActionSpec, UiSlotPlacement, UiSlotSpec, UiSplitPaneAxis, UiSplitPaneHandleProps,
    UiSplitPaneProps, UiSplitPaneResizeMode, UiStatusProps, UiSvgPaintPolicy, UiTextEntryProps,
    UiTextProps, UiTextSpan, UiTextSpanStyle, UiTextWrapMode,
};

#[cfg(test)]
mod tests {
    use super::{UiNode, UiNodeKind, UiTree};

    #[test]
    fn tree_keeps_children_order() {
        let tree = UiTree::new(
            UiNode::new(UiNodeKind::Row, "row")
                .child(UiNode::new(UiNodeKind::Text, "a"))
                .child(UiNode::new(UiNodeKind::Text, "b")),
        );
        assert_eq!(2, tree.root().children().len());
    }

    #[test]
    fn duplicate_components_get_unique_state_ids() {
        let tree = UiTree::new(
            UiNode::new(UiNodeKind::Row, "row")
                .child(UiNode::new(UiNodeKind::Button, "Save"))
                .child(UiNode::new(UiNodeKind::Button, "Save")),
        );
        let first = &tree.root().children()[0];
        let second = &tree.root().children()[1];

        assert_ne!(first.id(), second.id());
        assert_ne!(first.props().state_id, second.props().state_id);
    }
}
