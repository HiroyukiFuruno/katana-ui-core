//! Generic opaque context-menu projection.

#[path = "sanitized_context_projection/builder.rs"]
mod builder;
#[path = "sanitized_context_projection/item.rs"]
mod item;
#[path = "sanitized_context_projection/projection.rs"]
mod projection;
#[path = "sanitized_context_projection/target.rs"]
mod target;

pub use builder::SanitizedContextMenuProjectionBuilder;
pub use item::SanitizedContextMenuItem;
pub use projection::SanitizedContextMenuProjection;
pub(super) use target::ContextMenuCapability;
pub use target::{SanitizedContextMenuCapabilityRejection, SanitizedContextMenuTarget};

#[cfg(test)]
#[path = "sanitized_context_projection_inline_tests.rs"]
mod tests;
