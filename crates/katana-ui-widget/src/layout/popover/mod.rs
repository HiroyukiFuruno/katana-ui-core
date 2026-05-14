mod anchor;
mod ops;
mod placement;
mod placement_math;
mod types;
mod types_impl;
mod view;

pub(crate) use anchor::ViewAnchor;
pub use ops::PopoverOrigin;
pub use placement::{PlacementOrigin, PlacementResolver};
pub use types::{
    AnchorRect, AnchorRef, FreePlacement, Placement, Popover, PopoverChildren, PopoverOverlay,
    PopoverProps, ResolvedPopover,
};

#[cfg(test)]
mod tests;
