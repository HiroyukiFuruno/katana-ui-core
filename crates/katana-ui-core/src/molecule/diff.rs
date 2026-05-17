mod model;
mod types;

pub use model::CodeDiff;
pub use types::{
    CodeDiffDirection, CodeDiffLine, CodeDiffLineKind, CodeDiffMode, CodeDiffSource,
    CodeDiffWhitespace, CollapsedBlock, HighlightRange,
};
