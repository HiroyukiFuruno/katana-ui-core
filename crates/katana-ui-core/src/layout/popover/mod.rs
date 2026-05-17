mod anchor;
mod contract;
mod ops;
mod placement;
mod placement_math;
mod types;
mod types_impl;
mod view;

pub(crate) use anchor::ViewAnchor;
#[doc(hidden)]
pub use contract::{PopoverInteractionState, PopoverTransition};
pub use ops::PopoverOrigin;
pub use placement::{PlacementOrigin, PlacementResolver};
pub use types::{
    AnchorRect, AnchorRef, Placement, Popover, PopoverChildren, PopoverOverlay, PopoverProps,
    ResolvedPopover,
};

#[cfg(test)]
mod tests;
