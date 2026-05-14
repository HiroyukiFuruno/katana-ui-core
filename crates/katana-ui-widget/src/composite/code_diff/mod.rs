mod highlight;
mod model;
mod row;
mod split_model;
mod style;
mod types;
mod view;

pub use types::{
    CodeDiffCollapseOptions, CodeDiffError, CodeDiffLineKind, CodeDiffMode, CodeDiffModel,
    CodeDiffSource, CodeDiffSplitOrientation, CodeDiffTextRange,
};

use crate::theme::Theme;
use floem::IntoView;
use types::CodeDiffProps;

pub struct CodeDiff {
    pub(crate) props: CodeDiffProps,
}

impl CodeDiff {
    #[must_use]
    pub fn new(before: CodeDiffSource, after: CodeDiffSource) -> Self {
        Self {
            props: CodeDiffProps {
                before,
                after,
                mode: CodeDiffMode::Split,
                split_orientation: CodeDiffSplitOrientation::Horizontal,
                collapse: CodeDiffCollapseOptions::default(),
                show_header: true,
            },
        }
    }

    #[must_use]
    pub fn mode(mut self, mode: CodeDiffMode) -> Self {
        self.props.mode = mode;
        self
    }

    #[must_use]
    pub fn split_orientation(mut self, orientation: CodeDiffSplitOrientation) -> Self {
        self.props.split_orientation = orientation;
        self
    }

    #[must_use]
    pub fn collapse(mut self, collapse: CodeDiffCollapseOptions) -> Self {
        self.props.collapse = collapse;
        self
    }

    #[must_use]
    pub fn show_header(mut self, show_header: bool) -> Self {
        self.props.show_header = show_header;
        self
    }

    pub fn model(&self) -> Result<CodeDiffModel, CodeDiffError> {
        model::CodeDiffModelBuilder::build_model(&self.props.before, &self.props.after)
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        self.build_view(theme)
    }
}

#[cfg(test)]
mod tests;
