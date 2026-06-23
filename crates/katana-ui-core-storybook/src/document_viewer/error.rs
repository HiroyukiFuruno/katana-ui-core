use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucMissingCapability {
    MarkdownBlockModel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KucViewerError {
    MissingCapability(KucMissingCapability),
}

impl Display for KucViewerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCapability(capability) => {
                write!(
                    formatter,
                    "KUC missing document viewer capability: {capability:?}"
                )
            }
        }
    }
}

impl Error for KucViewerError {}
