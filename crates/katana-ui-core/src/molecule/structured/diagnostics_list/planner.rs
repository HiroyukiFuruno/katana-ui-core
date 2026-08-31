use super::{
    DiagnosticId, DiagnosticItem, DiagnosticScopeKey, DiagnosticSeverity, DiagnosticsGroupBy,
    DiagnosticsListOptions, DiagnosticsSortBy,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsGroup {
    pub key: String,
    pub item_ids: Vec<DiagnosticId>,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsVisibleSnapshot {
    pub visible_ids: Vec<DiagnosticId>,
    pub groups: Vec<DiagnosticsGroup>,
    pub total_count: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsListPlanner;

impl DiagnosticsListPlanner {
    #[must_use]
    pub fn snapshot(
        items: &[DiagnosticItem],
        options: &DiagnosticsListOptions,
    ) -> DiagnosticsVisibleSnapshot {
        let visible = Self::visible_items(items, options);
        let visible_ids = visible.iter().map(|it| it.id.clone()).collect::<Vec<_>>();
        DiagnosticsVisibleSnapshot {
            groups: Self::groups(&visible, options.group_by),
            total_count: visible_ids.len(),
            visible_ids,
        }
    }

    #[must_use]
    pub fn snapshot_for_scope(
        items: &[DiagnosticItem],
        options: &DiagnosticsListOptions,
        scope_key: Option<&DiagnosticScopeKey>,
    ) -> DiagnosticsVisibleSnapshot {
        let visible = Self::visible_items_for_scope(items, options, scope_key);
        let visible_ids = visible.iter().map(|it| it.id.clone()).collect::<Vec<_>>();
        DiagnosticsVisibleSnapshot {
            groups: Self::groups(&visible, options.group_by),
            total_count: visible_ids.len(),
            visible_ids,
        }
    }

    #[must_use]
    pub fn visible_items<'a>(
        items: &'a [DiagnosticItem],
        options: &DiagnosticsListOptions,
    ) -> Vec<&'a DiagnosticItem> {
        let mut visible = items
            .iter()
            .filter(|it| options.severity_filter.contains(&it.severity))
            .collect::<Vec<_>>();
        Self::sort(&mut visible, options.sort_by);
        visible
    }

    #[must_use]
    pub fn visible_items_for_scope<'a>(
        items: &'a [DiagnosticItem],
        options: &DiagnosticsListOptions,
        scope_key: Option<&DiagnosticScopeKey>,
    ) -> Vec<&'a DiagnosticItem> {
        let mut visible = items
            .iter()
            .filter(|item| {
                options.severity_filter.contains(&item.severity)
                    && scope_key.is_none_or(|key| item.scope_keys.contains(key))
            })
            .collect::<Vec<_>>();
        Self::sort(&mut visible, options.sort_by);
        visible
    }

    fn sort(items: &mut Vec<&DiagnosticItem>, sort_by: DiagnosticsSortBy) {
        match sort_by {
            DiagnosticsSortBy::Severity => items.sort_by_key(|it| it.severity),
            DiagnosticsSortBy::Location => items.sort_by_key(|it| {
                (
                    it.location.file.as_str(),
                    it.location.line,
                    it.location.column,
                )
            }),
            DiagnosticsSortBy::Source => items.sort_by_key(|it| it.source.as_str()),
            DiagnosticsSortBy::Order => {}
        }
    }

    fn groups(items: &[&DiagnosticItem], group_by: DiagnosticsGroupBy) -> Vec<DiagnosticsGroup> {
        match group_by {
            DiagnosticsGroupBy::Severity => severity_groups(items),
            DiagnosticsGroupBy::Source => keyed_groups(items, |it| it.source.clone()),
            DiagnosticsGroupBy::Location => keyed_groups(items, |it| it.location.file.clone()),
            DiagnosticsGroupBy::None => vec![DiagnosticsGroup {
                key: "all".to_string(),
                item_ids: items.iter().map(|it| it.id.clone()).collect(),
                count: items.len(),
            }],
        }
    }
}

fn severity_groups(items: &[&DiagnosticItem]) -> Vec<DiagnosticsGroup> {
    DiagnosticSeverity::all()
        .into_iter()
        .filter_map(|severity| {
            let ids = items
                .iter()
                .filter(|it| it.severity == severity)
                .map(|it| it.id.clone())
                .collect::<Vec<_>>();
            (!ids.is_empty()).then(|| DiagnosticsGroup {
                key: format!("{severity:?}"),
                count: ids.len(),
                item_ids: ids,
            })
        })
        .collect()
}

fn keyed_groups(
    items: &[&DiagnosticItem],
    key_for: impl Fn(&DiagnosticItem) -> String,
) -> Vec<DiagnosticsGroup> {
    let mut groups = Vec::<DiagnosticsGroup>::new();
    for item in items {
        let key = key_for(item);
        match groups.iter_mut().find(|group| group.key == key) {
            Some(group) => {
                group.item_ids.push(item.id.clone());
                group.count = group.item_ids.len();
            }
            None => groups.push(DiagnosticsGroup {
                key,
                item_ids: vec![item.id.clone()],
                count: 1,
            }),
        }
    }
    groups
}
