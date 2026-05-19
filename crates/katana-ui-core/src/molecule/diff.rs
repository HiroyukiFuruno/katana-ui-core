mod accessors;
mod actions;
mod engine;
mod engine_builder;
mod engine_text;
mod model;
mod types;

pub use model::CodeDiff;
pub use types::{
    CodeDiffBuildError, CodeDiffDirection, CodeDiffLine, CodeDiffLineHighlight, CodeDiffLineKind,
    CodeDiffMode, CodeDiffSide, CodeDiffSource, CodeDiffSummary, CodeDiffTextSource,
    CodeDiffWhitespace, CollapsedBlock, HighlightRange,
};
