use katana_ui_core::atom::Text;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::SettingsListLayoutMetrics;
use katana_ui_core::molecule::{
    SettingsControl, SettingsControlOption, SettingsDirtyVisualization, SettingsField,
    SettingsKeyboardInput, SettingsList, SettingsListAction, SettingsListDensity,
    SettingsListEvent, SettingsListHitTestInput, SettingsListHitTestResult, SettingsSection,
    SettingsValue,
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
fn settings_render_and_missing_updates_cover_optional_control_boundaries() {
    let mut list = SettingsList::new("Settings").section(
        SettingsSection::new("general", "General")
            .field(field(
                "combo-none",
                SettingsControl::Combo {
                    options: options(),
                    query: String::new(),
                    selected: None,
                },
            ))
            .field(field(
                "radio-match",
                SettingsControl::Radio {
                    options: options(),
                    selected: "dark".to_string(),
                },
            ))
            .field(field(
                "radio-missing",
                SettingsControl::Radio {
                    options: options(),
                    selected: "missing".to_string(),
                },
            ))
            .field(field(
                "no-default",
                SettingsControl::Input {
                    value: "value".to_string(),
                },
            )),
    );
    let node = UiNode::from(list.clone());
    assert_eq!(UiNodeKind::SettingsList, node.kind());

    assert_eq!(
        vec![SettingsListEvent::FieldChanged {
            field_id: "missing".to_string(),
        }],
        list.apply_settings_action(SettingsListAction::UpdateField {
            field_id: "missing".to_string(),
            value: SettingsValue::Text("ignored".to_string()),
        })
    );
    assert_eq!(
        vec![SettingsListEvent::FieldReset {
            field_id: "missing".to_string(),
        }],
        list.apply_settings_action(SettingsListAction::ResetField {
            field_id: "missing".to_string(),
        })
    );
    assert_eq!(
        vec![SettingsListEvent::FieldReset {
            field_id: "no-default".to_string(),
        }],
        list.apply_settings_action(SettingsListAction::ResetField {
            field_id: "no-default".to_string(),
        })
    );
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
fn settings_keyboard_hover_focus_and_empty_navigation_are_explicit() {
    let mut list = SettingsList::new("Settings").section(
        SettingsSection::new("appearance", "Appearance")
            .collapsible(true)
            .field(field("theme", SettingsControl::Toggle { checked: true }))
            .field(field(
                "font",
                SettingsControl::Input {
                    value: "Inter".to_string(),
                },
            )),
    );

    assert!(
        list.apply_settings_action(SettingsListAction::KeyboardSection {
            section_id: "appearance".to_string(),
            input: SettingsKeyboardInput::Tab,
        })
        .is_empty()
    );
    assert!(matches!(
        list.apply_settings_action(SettingsListAction::HoverSection {
            section_id: "appearance".to_string(),
            hovered: true,
        })
        .as_slice(),
        [SettingsListEvent::SectionHovered {
            section_id,
            hovered: true
        }] if section_id == "appearance"
    ));
    assert!(matches!(
        list.apply_settings_action(SettingsListAction::HoverField {
            field_id: "theme".to_string(),
            hovered: true,
        })
        .as_slice(),
        [SettingsListEvent::FieldHovered {
            field_id,
            hovered: true
        }] if field_id == "theme"
    ));
    assert!(matches!(
        list.apply_settings_action(SettingsListAction::FocusField {
            field_id: Some("missing".to_string()),
        })
        .as_slice(),
        [SettingsListEvent::FieldFocused { field_id: None }]
    ));
    assert!(matches!(
        list.apply_settings_action(SettingsListAction::KeyboardField {
            field_id: "theme".to_string(),
            input: SettingsKeyboardInput::Tab,
        })
        .as_slice(),
        [SettingsListEvent::FieldFocused {
            field_id: Some(field_id)
        }] if field_id == "font"
    ));

    let mut empty = SettingsList::new("Empty");
    assert!(matches!(
        empty
            .apply_settings_action(SettingsListAction::KeyboardField {
                field_id: "missing".to_string(),
                input: SettingsKeyboardInput::Tab,
            })
            .as_slice(),
        [SettingsListEvent::FieldFocused { field_id: None }]
    ));
    assert!(
        empty
            .apply_settings_action(SettingsListAction::KeyboardField {
                field_id: "missing".to_string(),
                input: SettingsKeyboardInput::Enter,
            })
            .is_empty()
    );
}

#[test]
fn settings_layout_metrics_cover_footer_choice_toggle_and_custom_controls() {
    let metrics = SettingsListLayoutMetrics::default();
    let radio = SettingsControl::Radio {
        options: options(),
        selected: "dark".to_string(),
    };
    let toggle = SettingsControl::Toggle { checked: true };
    let custom = SettingsControl::custom(Text::new("Custom"));

    assert_eq!(22, metrics.footer_height());
    assert_eq!(8, metrics.child_indent());
    assert_eq!(112, metrics.field_label_width());
    assert_eq!(132, metrics.choice_control_width());
    assert_eq!(48, metrics.control_width(&toggle));
    assert_eq!(132, metrics.control_width(&radio));
    assert_eq!(132, metrics.control_width(&custom));
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
                    .description("Applied to all windows")
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
    assert!(contains_node(
        &tree,
        UiNodeKind::Text,
        "Applied to all windows"
    ));
    assert!(contains_node(&tree, UiNodeKind::Button, "Reset"));
}

#[test]
fn footer_outside_hit_and_non_color_picker_value_are_safe() {
    let list = SettingsList::new("Settings").section(
        SettingsSection::new("appearance", "Appearance")
            .footer("Restart required")
            .field(field(
                "color",
                SettingsControl::ColorPicker {
                    color: SettingsValue::Text("invalid".to_string()),
                },
            )),
    );

    assert_eq!(110, list.content_height());
    assert_eq!(
        SettingsListHitTestResult::None,
        list.hit_test(SettingsListHitTestInput {
            pointer_x: 10,
            pointer_y: 100,
            scroll_offset_y: 0,
        })
    );
    let outside = list.interaction_for_hit(
        SettingsListHitTestInput {
            pointer_x: 10,
            pointer_y: 500,
            scroll_offset_y: 0,
        },
        320,
    );
    assert_eq!(SettingsListHitTestResult::None, outside.result);
    assert!(outside.action.is_none());
    assert!(outside.target.is_none());
    assert!(list.hit_target_for_field("missing", 320).is_none());

    let tree = UiTree::new(list);
    assert!(contains_node(&tree, UiNodeKind::Text, "rgba(0, 0, 0, 0)"));
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

#[test]
fn field_update_convenience_api_emits_the_same_typed_event() {
    let mut list = SettingsList::new("Settings").section(
        SettingsSection::new("general", "General")
            .field(field("toggle", SettingsControl::Toggle { checked: false })),
    );

    let events = list.apply_field_update("toggle", SettingsValue::Bool(true));

    assert!(matches!(
        events.as_slice(),
        [SettingsListEvent::FieldChanged { field_id }] if field_id == "toggle"
    ));
}

#[test]
fn settings_query_child_event_dirty_reset_and_selected_index_are_stateful() {
    let mut list = SettingsList::new("Settings").section(
        SettingsSection::new("general", "General").field(
            field("toggle", SettingsControl::Toggle { checked: false })
                .reset_to_default(SettingsValue::Bool(false)),
        ),
    );

    assert!(matches!(
        list.apply_settings_action(SettingsListAction::SetQuery(Some("toggle".to_string())))
            .as_slice(),
        [SettingsListEvent::QueryChanged(Some(query))] if query == "toggle"
    ));
    assert!(matches!(
        list.apply_settings_action(SettingsListAction::RouteChildEvent {
            field_id: "toggle".to_string(),
            event: "pressed".to_string(),
        })
        .as_slice(),
        [SettingsListEvent::ChildEventRouted { field_id, event }]
            if field_id == "toggle" && event == "pressed"
    ));

    list.apply_settings_action(SettingsListAction::UpdateField {
        field_id: "toggle".to_string(),
        value: SettingsValue::Bool(true),
    });
    assert!(list.dirty_field_ids().contains("toggle"));
    list.apply_settings_action(SettingsListAction::FocusField {
        field_id: Some("toggle".to_string()),
    });
    let dirty_node = UiNode::from(list.clone());
    assert!(dirty_node.props().interaction.has_selection);
    assert_eq!(0, dirty_node.props().interaction.selected_index);

    list.apply_settings_action(SettingsListAction::ResetField {
        field_id: "toggle".to_string(),
    });
    assert!(!list.dirty_field_ids().contains("toggle"));
    assert!(matches!(
        list.apply_settings_action(SettingsListAction::SetQuery(Some(String::new())))
            .as_slice(),
        [SettingsListEvent::QueryChanged(None)]
    ));
}

#[test]
fn settings_component_action_handles_query_and_rejects_invalid_routes() {
    let mut list = SettingsList::new("Settings").section(
        SettingsSection::new("general", "General")
            .field(field("toggle", SettingsControl::Toggle { checked: false })),
    );
    let target = list.state_id().clone();

    assert!(
        ComponentAction::apply_action(&mut list, &UiAction::set_value(target.clone(), "toggle"))
            .handled
    );
    assert!(
        !ComponentAction::apply_action(
            &mut list,
            &UiAction::set_value(
                katana_ui_core::render_model::UiStateId::new("other-settings"),
                "ignored",
            )
        )
        .handled
    );
    assert!(
        !ComponentAction::apply_action(
            &mut list,
            &UiAction::set_selected_index(target.clone(), 99)
        )
        .handled
    );
    assert!(
        !ComponentAction::apply_action(&mut list, &UiAction::clear_value(target.clone())).handled
    );
    assert!(!ComponentAction::apply_action(&mut list, &UiAction::focus(target)).handled);
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
