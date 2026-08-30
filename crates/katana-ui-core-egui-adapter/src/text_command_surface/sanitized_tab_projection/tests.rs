use super::{
    SanitizedTab, SanitizedTabCapabilities, SanitizedTabClosePresentation, SanitizedTabGroup,
    SanitizedTabGroupCapabilities, SanitizedTabGroupTarget, SanitizedTabProjection,
    SanitizedTabTarget,
};
use katana_ui_core::render_model::UiIconProps;

include!("tests/construction.rs");
include!("tests/fingerprints.rs");
include!("../sanitized_tab_projection/coverage.rs");
include!("../sanitized_tab_projection/api.rs");
include!("../sanitized_tab_projection/debug.rs");
