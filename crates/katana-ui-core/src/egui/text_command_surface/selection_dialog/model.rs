use super::query_history::{SearchQueryArrowKeyPolicy, SearchQueryHistoryDirection};
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

/// host所有のトークン。KUCは比較するが、アプリケーション上の意味は解釈しない。
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KucDialogTarget(Vec<u8>);

impl KucDialogTarget {
    #[must_use]
    pub fn from_opaque_bytes(value: impl Into<Vec<u8>>) -> Self {
        Self(value.into())
    }

    pub(super) fn interaction_key(&self) -> String {
        format!("{:016x}", egui::Id::new(self).value())
    }
}

impl std::fmt::Debug for KucDialogTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("KucDialogTarget(..)")
    }
}

/// 表示するcontrol名はすべてlocalizationのためhostから供給される。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KucDialogControls {
    pub cancel: String,
    pub select: String,
    pub search: String,
    pub include: String,
    pub exclude: String,
    pub clear_history: String,
    pub delete_history: String,
    pub empty: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileSelectionItem {
    pub target: KucDialogTarget,
    pub label: String,
    pub depth: u16,
    pub navigable: bool,
    pub selectable: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileSelectionProjection {
    pub items: Vec<FileSelectionItem>,
    pub failure: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultOption {
    pub target: KucDialogTarget,
    pub label: String,
    pub enabled: bool,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultRow {
    pub target: KucDialogTarget,
    pub label: String,
    pub auxiliary_label: Option<String>,
    pub snippet: Option<String>,
    pub highlights: Vec<(usize, usize)>,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultMode {
    pub target: KucDialogTarget,
    pub label: String,
    pub query: String,
    pub include: Option<String>,
    pub exclude: Option<String>,
    pub options: Vec<SearchResultOption>,
    pub history: Vec<SearchResultRow>,
    pub results: Vec<SearchResultRow>,
    pub searched: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultProjection {
    pub modes: Vec<SearchResultMode>,
    pub active_mode: KucDialogTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionDialogProjection {
    File(FileSelectionProjection),
    Search(SearchResultProjection),
}

/// revision付きprojectionはdocument storageから独立して一つのretained rootに紐付く。
#[derive(Debug)]
pub struct SelectionDialogProjectionLease {
    pub(crate) revision: u64,
    pub(crate) title: String,
    pub(crate) controls: KucDialogControls,
    pub(crate) open: bool,
    pub(crate) disabled: bool,
    pub(crate) readonly: bool,
    pub(crate) projection: SelectionDialogProjection,
    pub(crate) query_arrow_policies:
        std::collections::HashMap<KucDialogTarget, SearchQueryArrowKeyPolicy>,
}

impl SelectionDialogProjectionLease {
    pub fn new(
        revision: u64,
        title: impl Into<String>,
        controls: KucDialogControls,
        projection: SelectionDialogProjection,
    ) -> Result<Self, SelectionDialogProjectionError> {
        validate(&projection)?;
        Ok(Self {
            revision,
            title: title.into(),
            controls,
            open: true,
            disabled: false,
            readonly: false,
            projection,
            query_arrow_policies: std::collections::HashMap::new(),
        })
    }

    #[must_use]
    pub fn open(mut self, value: bool) -> Self {
        self.open = value;
        self
    }
    #[must_use]
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    #[must_use]
    pub fn readonly(mut self, value: bool) -> Self {
        self.readonly = value;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionDialogOperation {
    ModeChanged {
        mode: KucDialogTarget,
    },
    QueryChanged {
        mode: KucDialogTarget,
        value: String,
    },
    QueryHistoryNavigated {
        mode: KucDialogTarget,
        target: Option<KucDialogTarget>,
        value: String,
        direction: SearchQueryHistoryDirection,
    },
    IncludeChanged {
        mode: KucDialogTarget,
        value: String,
    },
    ExcludeChanged {
        mode: KucDialogTarget,
        value: String,
    },
    OptionChanged {
        mode: KucDialogTarget,
        option: KucDialogTarget,
        selected: bool,
    },
    SearchRequested {
        mode: KucDialogTarget,
    },
    HistorySelected {
        mode: KucDialogTarget,
        target: KucDialogTarget,
    },
    HistoryDeleted {
        mode: KucDialogTarget,
        target: KucDialogTarget,
    },
    HistoryCleared {
        mode: KucDialogTarget,
    },
    ResultSelected {
        mode: KucDialogTarget,
        target: KucDialogTarget,
    },
    Navigated {
        target: KucDialogTarget,
    },
    FileSelected {
        target: KucDialogTarget,
    },
    Cancelled,
    Failed {
        message: String,
    },
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionDialogEvent {
    pub revision: u64,
    pub occurrence: u64,
    pub operation: SelectionDialogOperation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionDialogProjectionError {
    Rendering(String),
    DuplicateTarget,
    MissingMode,
    InvalidHighlight,
    StaleRevision { current: u64, received: u64 },
}

impl std::fmt::Display for SelectionDialogProjectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "selection dialog projection rejected: {self:?}")
    }
}
impl std::error::Error for SelectionDialogProjectionError {}

fn unique<'a>(
    targets: impl Iterator<Item = &'a KucDialogTarget>,
) -> Result<(), SelectionDialogProjectionError> {
    let mut seen = std::collections::HashSet::new();
    for target in targets {
        if !seen.insert(target) {
            return Err(SelectionDialogProjectionError::DuplicateTarget);
        }
    }
    Ok(())
}

fn validate(projection: &SelectionDialogProjection) -> Result<(), SelectionDialogProjectionError> {
    match projection {
        SelectionDialogProjection::File(files) => {
            unique(files.items.iter().map(|item| &item.target))
        }
        SelectionDialogProjection::Search(search) => {
            unique(search.modes.iter().map(|mode| &mode.target))?;
            if !search
                .modes
                .iter()
                .any(|mode| mode.target == search.active_mode)
            {
                return Err(SelectionDialogProjectionError::MissingMode);
            }
            for mode in &search.modes {
                unique(mode.options.iter().map(|item| &item.target))?;
                unique(mode.history.iter().map(|item| &item.target))?;
                unique(mode.results.iter().map(|item| &item.target))?;
                for row in mode.history.iter().chain(&mode.results) {
                    validate_row(row)?;
                }
            }
            Ok(())
        }
    }
}

fn validate_row(row: &SearchResultRow) -> Result<(), SelectionDialogProjectionError> {
    let text = row.snippet.as_deref().unwrap_or(&row.label);
    let boundaries = text
        .grapheme_indices(true)
        .map(|(offset, _)| offset)
        .chain(std::iter::once(text.len()))
        .collect::<std::collections::HashSet<_>>();
    for &(start, end) in &row.highlights {
        if start > end || !boundaries.contains(&start) || !boundaries.contains(&end) {
            return Err(SelectionDialogProjectionError::InvalidHighlight);
        }
    }
    Ok(())
}
