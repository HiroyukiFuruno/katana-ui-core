mod actions;
mod model;

pub use actions::{SourceAddressAction, SourceAddressEvent, SourceAddressSubmission};
pub use model::{SourceAddressEntry, SourceAddressPresentation, SourceAddressStrip};

#[cfg(test)]
mod tests;
