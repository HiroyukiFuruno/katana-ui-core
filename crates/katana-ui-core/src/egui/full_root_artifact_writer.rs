mod metadata;
mod metadata_types;
mod process;
#[cfg(test)]
mod tests;
mod validation;

pub use metadata_types::{FullRootArtifact, FullRootArtifactError};
pub use process::FullRootArtifactWriter;
