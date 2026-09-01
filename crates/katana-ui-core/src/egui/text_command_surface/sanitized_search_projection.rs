#[path = "sanitized_search_projection/builder.rs"]
mod builder;
#[path = "sanitized_search_projection/builder_types.rs"]
mod builder_types;
#[path = "sanitized_search_projection/operation.rs"]
mod operation;
#[path = "sanitized_search_projection/presentation.rs"]
mod presentation;
#[path = "sanitized_search_projection/projection.rs"]
mod projection;
#[path = "sanitized_search_projection/types.rs"]
mod types;

pub use builder_types::SanitizedSearchProjectionBuilder;
pub use presentation::{
    SanitizedSearchControlPresentation, SanitizedSearchLocalizedPresentation,
    SanitizedSearchOperationPresentation, SanitizedSearchResultSummaryPresentation,
    SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation,
};
pub use projection::SanitizedSearchProjection;
pub(crate) use types::{SanitizedSearchCapability, TextCapability, UnitCapability};
pub use types::{
    SanitizedSearchCapabilityRejection, SanitizedSearchOperationSlot,
    SanitizedSearchProjectionBuildError, SanitizedSearchTarget, SanitizedSearchTextOperation,
    SanitizedSearchUnitOperation,
};

#[cfg(test)]
#[path = "sanitized_search_projection_inline_tests.rs"]
mod tests;
