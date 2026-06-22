use super::{
    SettingsControl, SettingsField, SettingsList, SettingsListHitTestInput,
    SettingsListHitTestResult, SettingsListLayoutMetrics, SettingsSection,
};
use crate::render_model::UiCursor;

#[test]
fn hit_test_returns_section_toggle_for_section_header() {
    let action = sample_settings().hit_test(SettingsListHitTestInput {
        pointer_x: 0,
        pointer_y: display_section_center_y(),
        scroll_offset_y: 0,
    });

    assert_eq!(
        SettingsListHitTestResult::ToggleSection {
            section_id: "display".to_string()
        },
        action
    );
}

#[test]
fn hit_test_returns_none_for_search_box_row() {
    let action = sample_settings().hit_test(SettingsListHitTestInput {
        pointer_x: 0,
        pointer_y: metrics().title_height() + metrics().search_box_height() / 2,
        scroll_offset_y: 0,
    });

    assert_eq!(SettingsListHitTestResult::None, action);
}

#[test]
fn hit_test_uses_rendered_search_section_and_field_y_edges() {
    let list = sample_settings();
    let search_bottom_y = metrics().title_height() + metrics().search_box_height() - 1;
    let section_top_y = metrics().title_height() + metrics().search_box_height();
    let field_top_y = section_top_y + metrics().section_height();

    assert_eq!(
        SettingsListHitTestResult::None,
        list.hit_test(SettingsListHitTestInput {
            pointer_x: 0,
            pointer_y: search_bottom_y,
            scroll_offset_y: 0,
        })
    );
    assert_eq!(
        SettingsListHitTestResult::ToggleSection {
            section_id: "display".to_string()
        },
        list.hit_test(SettingsListHitTestInput {
            pointer_x: 0,
            pointer_y: section_top_y,
            scroll_offset_y: 0,
        })
    );
    assert_eq!(
        SettingsListHitTestResult::Field {
            field_id: "dark".to_string()
        },
        list.hit_test(SettingsListHitTestInput {
            pointer_x: toggle_control_center_x(),
            pointer_y: field_top_y,
            scroll_offset_y: 0,
        })
    );
}

#[test]
fn hit_test_returns_field_for_visible_field_row() {
    let action = sample_settings().hit_test(SettingsListHitTestInput {
        pointer_x: toggle_control_center_x(),
        pointer_y: dark_field_center_y(),
        scroll_offset_y: 0,
    });

    assert_eq!(
        SettingsListHitTestResult::Field {
            field_id: "dark".to_string()
        },
        action
    );
}

#[test]
fn hit_test_uses_field_row_x_edges_for_interactive_field() {
    let list = sample_settings();

    assert_eq!(
        SettingsListHitTestResult::Field {
            field_id: "dark".to_string()
        },
        list.hit_test(SettingsListHitTestInput {
            pointer_x: 0,
            pointer_y: dark_field_center_y(),
            scroll_offset_y: 0,
        })
    );
    assert_eq!(
        SettingsListHitTestResult::Field {
            field_id: "dark".to_string()
        },
        list.hit_test(SettingsListHitTestInput {
            pointer_x: metrics().field_control_x() - 1,
            pointer_y: dark_field_center_y(),
            scroll_offset_y: 0,
        })
    );
}

#[test]
fn hit_test_returns_field_for_field_label_inside_row() {
    let action = sample_settings().hit_test(SettingsListHitTestInput {
        pointer_x: metrics().field_control_x() - 1,
        pointer_y: dark_field_center_y(),
        scroll_offset_y: 0,
    });

    assert_eq!(
        SettingsListHitTestResult::Field {
            field_id: "dark".to_string()
        },
        action
    );
}

#[test]
fn hit_test_uses_rendered_field_row_height_for_later_fields() {
    let action = sample_settings().hit_test(SettingsListHitTestInput {
        pointer_x: text_entry_control_center_x(),
        pointer_y: theme_field_center_y(),
        scroll_offset_y: 0,
    });

    assert_eq!(
        SettingsListHitTestResult::Field {
            field_id: "theme".to_string()
        },
        action
    );
}

#[test]
fn hit_test_accounts_for_scroll_offset() {
    let action = sample_settings().hit_test(SettingsListHitTestInput {
        pointer_x: toggle_control_center_x(),
        pointer_y: 1,
        scroll_offset_y: dark_field_center_y() - 1,
    });

    assert_eq!(
        SettingsListHitTestResult::Field {
            field_id: "dark".to_string()
        },
        action
    );
}

#[test]
fn cursor_for_hit_uses_settings_row_interactive_contract() {
    let cursor = sample_settings().cursor_for_hit(SettingsListHitTestInput {
        pointer_x: toggle_control_center_x(),
        pointer_y: dark_field_center_y(),
        scroll_offset_y: 0,
    });

    assert_eq!(UiCursor::Pointer, cursor);
}

#[test]
fn cursor_for_hit_uses_pointer_for_field_label_inside_row() {
    let cursor = sample_settings().cursor_for_hit(SettingsListHitTestInput {
        pointer_x: metrics().field_control_x() - 1,
        pointer_y: dark_field_center_y(),
        scroll_offset_y: 0,
    });

    assert_eq!(UiCursor::Pointer, cursor);
}

#[test]
fn cursor_for_hit_uses_default_for_readonly_field_row() {
    let cursor = sample_settings().cursor_for_hit(SettingsListHitTestInput {
        pointer_x: text_entry_control_center_x(),
        pointer_y: theme_field_center_y(),
        scroll_offset_y: 0,
    });

    assert_eq!(UiCursor::Default, cursor);
}

fn display_section_center_y() -> u32 {
    metrics().title_height() + metrics().search_box_height() + metrics().section_height() / 2
}

fn dark_field_center_y() -> u32 {
    metrics().title_height()
        + metrics().search_box_height()
        + metrics().section_height()
        + metrics().field_height() / 2
}

fn theme_field_center_y() -> u32 {
    dark_field_center_y() + metrics().field_height()
}

const fn toggle_control_center_x() -> u32 {
    SettingsListLayoutMetrics::DEFAULT.field_control_x()
        + SettingsListLayoutMetrics::DEFAULT.toggle_width() / 2
}

const fn text_entry_control_center_x() -> u32 {
    SettingsListLayoutMetrics::DEFAULT.field_control_x()
        + SettingsListLayoutMetrics::DEFAULT.text_entry_width() / 2
}

const fn metrics() -> SettingsListLayoutMetrics {
    SettingsListLayoutMetrics::DEFAULT
}

fn sample_settings() -> SettingsList {
    SettingsList::new("settings").section(
        SettingsSection::new("display", "Display")
            .field(SettingsField::new(
                "dark",
                "Dark",
                SettingsControl::Toggle { checked: true },
            ))
            .field(SettingsField::new(
                "theme",
                "Theme",
                SettingsControl::Input {
                    value: "dark".to_string(),
                },
            )),
    )
}
