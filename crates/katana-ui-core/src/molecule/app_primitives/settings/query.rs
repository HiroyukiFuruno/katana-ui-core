use super::{SettingsField, SettingsList, SettingsSection};

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsVisibleSection<'a> {
    pub section: &'a SettingsSection,
    pub fields: Vec<&'a SettingsField>,
}

pub(super) fn visible_section<'a>(
    list: &SettingsList,
    section: &'a SettingsSection,
) -> Option<SettingsVisibleSection<'a>> {
    if list.collapsed_section_ids.contains(&section.id) {
        return Some(SettingsVisibleSection {
            section,
            fields: Vec::new(),
        });
    }
    let fields = visible_fields_for_query(section, list.query.as_deref());
    if fields.is_empty() && !section_matches(section, list.query.as_deref()) {
        return None;
    }
    Some(SettingsVisibleSection { section, fields })
}

fn visible_fields_for_query<'a>(
    section: &'a SettingsSection,
    query: Option<&str>,
) -> Vec<&'a SettingsField> {
    if section_matches(section, query) {
        return section.fields.iter().collect();
    }
    section
        .fields
        .iter()
        .filter(|field| field.matches_query(query))
        .collect()
}

fn section_matches(section: &SettingsSection, query: Option<&str>) -> bool {
    query.is_none_or(|it| {
        let needle = it.to_lowercase();
        section.label.to_lowercase().contains(&needle)
            || section
                .description
                .as_ref()
                .is_some_and(|description| description.to_lowercase().contains(&needle))
    })
}
