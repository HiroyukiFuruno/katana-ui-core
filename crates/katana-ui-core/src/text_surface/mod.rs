mod accessibility;
mod accessibility_tree;
mod action;
mod annotation;
mod focus_request;
mod frame;
mod frame_record;
mod gutter;
mod gutter_types;
mod layout;
mod layout_gutter;
mod layout_model;
mod layout_scroll;
mod props;
mod scroll_request;
mod scroll_request_types;
mod scroll_request_value;
mod state;
mod surface;
mod surface_controlled;
mod surface_focus;
mod surface_interaction;
mod surface_model;
mod surface_scroll;

pub use accessibility::{
    TextSurfaceAccessibilityAction, TextSurfaceAccessibilityActionKind,
    TextSurfaceAccessibilityLabels, TextSurfaceAccessibilityNode, TextSurfaceAccessibilityTarget,
    TextSurfaceAccessibilityTree,
};
pub use action::{
    TextSurfaceAction, TextSurfaceActionOutcome, TextSurfaceClipboardOperation, TextSurfaceEvent,
    TextSurfaceHistoryOperation, TextSurfaceLayoutAction,
};
pub use annotation::{TextSurfaceAnnotation, TextSurfaceAnnotationStyle};
pub use focus_request::{
    TextSurfaceFocusRequest, TextSurfaceFocusRequestAcknowledgement, TextSurfaceFocusRequestResult,
    TextSurfaceFocusRequestToken,
};
pub use frame_record::{
    TextSurfaceFrameRecord, TextSurfaceGutterFrame, TextSurfacePreeditFrame,
    TextSurfaceSelectionFrame,
};
pub use gutter_types::{
    TextSurfaceAutomaticGutterOverride, TextSurfaceAutomaticGutterPresentation,
    TextSurfaceAutomaticGutterRangeOverride, TextSurfaceGutter, TextSurfaceGutterRangeStartAnchor,
    TextSurfaceGutterRow, TextSurfaceGutterRowId,
};
pub use layout_model::{
    TextSurfaceCompositionLayout, TextSurfaceGraphemeBox, TextSurfaceLayout, TextSurfaceLineBox,
};
pub use props::{
    TextSurfacePoint, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
    TextSurfaceViewportSizing,
};
pub use scroll_request_types::{
    TextSurfaceLogicalPixels, TextSurfaceScrollAlignment, TextSurfaceScrollRequest,
    TextSurfaceScrollRequestAcknowledgement, TextSurfaceScrollRequestRejection,
    TextSurfaceScrollRequestResult, TextSurfaceScrollRequestToken, TextSurfaceScrollTarget,
};
pub use state::{TextSurfaceScrollBounds, TextSurfaceState};
pub use surface_model::TextSurface;

#[cfg(test)]
mod accessibility_tests;
#[cfg(test)]
mod clipboard_tests;
#[cfg(test)]
mod focus_request_tests;
#[cfg(test)]
mod gutter_tests;
#[cfg(test)]
mod interaction_tests;
#[cfg(test)]
mod scroll_precision_tests;
#[cfg(test)]
mod scroll_request_tests;
#[cfg(test)]
mod synchronization_identity_tests;
#[cfg(test)]
mod synchronization_presentation_tests;
#[cfg(test)]
mod synchronization_tests;
#[cfg(test)]
mod tests;
