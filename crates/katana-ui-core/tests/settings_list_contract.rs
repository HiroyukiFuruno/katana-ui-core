use katana_ui_core::atom::Text;
use katana_ui_core::molecule::{
    SettingsControl, SettingsControlOption, SettingsDirtyVisualization, SettingsField,
    SettingsKeyboardInput, SettingsList, SettingsListAction, SettingsListDensity,
    SettingsListEvent, SettingsSection, SettingsValue,
};
use katana_ui_core::render_model::{UiNode, UiNodeKind, UiSize, UiTree, UiVariant};

#[test]
fn typed_controls_render_with_distinct_child_state_ids() {
    let list = SettingsList::new("Settings").section(
        SettingsSection::new("general", "General")
            .field(field("toggle", SettingsControl::Toggle { checked: true }))
            .field(field(
                "select",
                SettingsControl::Select {
                    options: options(),
                    selected: "dark".to_string(),
                },
            ))
            .field(field(
                "combo",
                SettingsControl::Combo {
                    options: options(),
                    query: "da".to_string(),
                    selected: Some("dark".to_string()),
                },
            ))
            .field(field(
                "input",
                SettingsControl::Input {
                    value: "katana".to_string(),
                },
            ))
            .field(field(
                "text-area",
                SettingsControl::TextArea {
                    value: "日本語\nEnglish".to_string(),
                },
            ))
            .field(field(
                "number",
                SettingsControl::Number {
                    value: 24,
                    min: 0,
                    max: 100,
                },
            ))
            .field(field(
                "chips",
                SettingsControl::Chips {
                    values: vec!["lint".to_string(), "format".to_string()],
                },
            ))
            .field(field(
                "radio",
                SettingsControl::Radio {
                    options: options(),
                    selected: "light".to_string(),
                },
            ))
            .field(field(
                "color",
                SettingsControl::ColorPicker {
                    color: SettingsValue::Color {
                        red: 64,
                        green: 128,
                        blue: 255,
                        alpha: 255,
                    },
                },
            ))
            .field(field(
                "custom",
                SettingsControl::custom(Text::new("custom subtree")),
            )),
    );

    let tree = UiTree::new(list);
    assert_eq!(UiNodeKind::SettingsList, tree.root().kind());
    assert!(
        tree.root()
            .children()
            .iter()
            .any(|it| it.kind() == UiNodeKind::SearchBox)
    );
    assert!(
        tree.root()
            .children()
            .iter()
            .any(|it| it.kind() == UiNodeKind::FormField)
    );
    assert!(all_state_ids_are_distinct(&tree));
}

#[test]
fn query_matches_section_label_and_field_description() {
    let section = SettingsSection::new("editor", "Editor")
        .description("Text behavior")
        .field(
            field(
                "font",
                SettingsControl::Input {
                    value: "body".to_string(),
                },
            )
            .description("Typography role"),
        )
        .field(field("wrap", SettingsControl::Toggle { checked: true }));
    let by_section = SettingsList::new("Settings")
        .query("editor")
        .section(section.clone());
    let by_description = SettingsList::new("Settings")
        .query("typography")
        .section(section);

    assert_eq!(2, by_section.visible_fields().len());
    assert_eq!(1, by_description.visible_fields().len());
    assert_eq!("font", by_description.visible_fields()[0].id);
}

#[test]
fn collapse_reset_and_dirty_state_emit_typed_events() {
    let field = field(
        "theme",
        SettingsControl::Select {
            options: options(),
            selected: "dark".to_string(),
        },
    )
    .reset_to_default(SettingsValue::Text("light".to_string()));
    let mut list = SettingsList::new("Settings").section(
        SettingsSection::new("appearance", "Appearance")
            .collapsible(true)
            .default_collapsed(true)
            .field(field),
    );

    assert!(list.collapsed_section_ids().contains("appearance"));
    let open = list.apply_settings_action(SettingsListAction::KeyboardSection {
        section_id: "appearance".to_string(),
        input: SettingsKeyboardInput::Enter,
    });
    let reset = list.apply_settings_action(SettingsListAction::ResetField {
        field_id: "theme".to_string(),
    });

    assert!(matches!(
        open.as_slice(),
        [SettingsListEvent::SectionCollapsed { section_id, collapsed }]
            if section_id == "appearance" && !collapsed
    ));
    assert!(matches!(
        reset.as_slice(),
        [SettingsListEvent::FieldReset { field_id }] if field_id == "theme"
    ));
    assert!(!list.dirty_field_ids().contains("theme"));
}

#[test]
fn zero_query_result_renders_empty_state_with_distinct_state() {
    let list = SettingsList::new("Settings").query("missing").section(
        SettingsSection::new("general", "General")
            .field(field("theme", SettingsControl::Toggle { checked: true })),
    );
    let tree = UiTree::new(list);
    let empty = tree
        .root()
        .children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::EmptyState);

    assert!(empty.is_some());
    assert!(empty.is_some_and(|it| it.props().state_id != tree.root().props().state_id));
}

#[test]
fn density_section_icon_footer_and_dirty_visualization_render_contracts() {
    let list = SettingsList::new("Settings")
        .density(SettingsListDensity::Compact)
        .dirty_visualization(SettingsDirtyVisualization::Marker)
        .section(
            SettingsSection::new("appearance", "Appearance")
                .icon("settings")
                .footer("Restart required")
                .field(
                    field(
                        "theme",
                        SettingsControl::Select {
                            options: options(),
                            selected: "dark".to_string(),
                        },
                    )
                    .reset_to_default(SettingsValue::Text("light".to_string())),
                ),
        );

    let tree = UiTree::new(list);

    assert_eq!(UiSize::Small, tree.root().props().size);
    assert_eq!(UiVariant::Outline, tree.root().props().variant);
    assert!(
        tree.root()
            .props()
            .style_classes
            .contains(&"settings-density-compact".to_string())
    );
    assert!(contains_node(&tree, UiNodeKind::Icon, "settings"));
    assert!(contains_node(&tree, UiNodeKind::Text, "Restart required"));
    assert!(contains_node(&tree, UiNodeKind::Button, "Reset"));
}

#[test]
fn density_and_dirty_visualization_variants_have_numeric_rendering_contracts() {
    let density_cases = [
        (
            SettingsListDensity::Compact,
            UiSize::Small,
            "settings-density-compact",
        ),
        (
            SettingsListDensity::Default,
            UiSize::Medium,
            "settings-density-default",
        ),
        (
            SettingsListDensity::Spacious,
            UiSize::Large,
            "settings-density-spacious",
        ),
    ];
    let dirty_cases = [
        (SettingsDirtyVisualization::None, UiVariant::Plain),
        (SettingsDirtyVisualization::Marker, UiVariant::Outline),
        (SettingsDirtyVisualization::Highlight, UiVariant::Filled),
    ];

    for (density, size, class_name) in density_cases {
        let tree = UiTree::new(
            SettingsList::new("Settings")
                .density(density)
                .section(SettingsSection::new("general", "General")),
        );
        assert_eq!(size, tree.root().props().size);
        assert!(
            tree.root()
                .props()
                .style_classes
                .contains(&class_name.to_string())
        );
    }
    for (dirty_visualization, variant) in dirty_cases {
        let tree = UiTree::new(
            SettingsList::new("Settings")
                .dirty_visualization(dirty_visualization)
                .section(SettingsSection::new("general", "General")),
        );
        assert_eq!(variant, tree.root().props().variant);
    }
}

#[test]
fn update_focus_and_callback_log_are_stateful_and_typed() {
    let mut list = SettingsList::new("Settings").section(
        SettingsSection::new("general", "General")
            .field(field("toggle", SettingsControl::Toggle { checked: false }))
            .field(field(
                "input",
                SettingsControl::Input {
                    value: "old".to_string(),
                },
            )),
    );

    let changed = list.apply_settings_action(SettingsListAction::UpdateField {
        field_id: "toggle".to_string(),
        value: SettingsValue::Bool(true),
    });
    let focused = list.apply_settings_action(SettingsListAction::FocusField {
        field_id: Some("input".to_string()),
    });
    let next = list.apply_settings_action(SettingsListAction::KeyboardField {
        field_id: "toggle".to_string(),
        input: SettingsKeyboardInput::Tab,
    });

    assert!(matches!(
        changed.as_slice(),
        [SettingsListEvent::FieldChanged { field_id }] if field_id == "toggle"
    ));
    assert!(matches!(
        focused.as_slice(),
        [SettingsListEvent::FieldFocused { field_id }] if field_id.as_deref() == Some("input")
    ));
    assert!(matches!(
        next.as_slice(),
        [SettingsListEvent::FieldFocused { field_id }] if field_id.as_deref() == Some("input")
    ));
    assert_eq!(Some("input"), list.focused_field_id());
    assert_eq!(3, list.callback_log().len());
}

fn field(id: &str, control: SettingsControl) -> SettingsField {
    SettingsField::new(id, id, control)
}

fn options() -> Vec<SettingsControlOption> {
    vec![
        SettingsControlOption::new("light", "Light"),
        SettingsControlOption::new("dark", "Dark"),
    ]
}

fn all_state_ids_are_distinct(tree: &UiTree) -> bool {
    let mut ids = Vec::new();
    collect_ids(tree.root(), &mut ids);
    let unique: std::collections::BTreeSet<&str> = ids
        .iter()
        .map(katana_ui_core::render_model::UiStateId::as_str)
        .collect();
    ids.len() == unique.len()
}

fn collect_ids(
    node: &katana_ui_core::render_model::UiNode,
    ids: &mut Vec<katana_ui_core::render_model::UiStateId>,
) {
    ids.push(node.props().state_id.clone());
    for child in node.children() {
        collect_ids(child, ids);
    }
}

fn contains_node(tree: &UiTree, kind: UiNodeKind, label: &str) -> bool {
    contains_node_inner(tree.root(), kind, label)
}

fn contains_node_inner(node: &UiNode, kind: UiNodeKind, label: &str) -> bool {
    node.kind() == kind && node.props().label == label
        || node
            .children()
            .iter()
            .any(|child| contains_node_inner(child, kind, label))
}
