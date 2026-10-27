use super::model::{
    KucDialogTarget, SelectionDialogProjection, SelectionDialogProjectionError,
    SelectionDialogProjectionLease,
};
use serde::{Deserialize, Serialize};

/// 検索欄にフォーカスがあるときの上下キーの用途。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchQueryArrowKeyPolicy {
    #[default]
    Results,
    History,
}

/// 履歴の新旧方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchQueryHistoryDirection {
    Older,
    Newer,
}

impl SelectionDialogProjectionLease {
    /// 指定した検索モードの検索欄における上下キーの用途を設定する。
    pub fn query_arrow_key_policy(
        mut self,
        mode: KucDialogTarget,
        policy: SearchQueryArrowKeyPolicy,
    ) -> Result<Self, SelectionDialogProjectionError> {
        if !matches!(&self.projection, SelectionDialogProjection::Search(search) if search.modes.iter().any(|item| item.target == mode))
        {
            return Err(SelectionDialogProjectionError::MissingMode);
        }
        self.query_arrow_policies.insert(mode, policy);
        Ok(self)
    }

    pub(crate) fn arrow_policy(&self, mode: &KucDialogTarget) -> SearchQueryArrowKeyPolicy {
        self.query_arrow_policies
            .get(mode)
            .copied()
            .unwrap_or_default()
    }
}
