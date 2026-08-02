use katana_ui_core::interaction::placement::{AnchorKind, Placement, PlacementRequest, Rect, Size};
use katana_ui_core::molecule::toolbar::{
    KeyCombo, KeyModifier, MeasuredToolbarAction, SplitAction, SplitActionPart, ToolbarAction,
    ToolbarContractViolation, ToolbarDensity, ToolbarDisplayMode, ToolbarEvent, ToolbarFocusState,
    ToolbarGroup, ToolbarGroupId, ToolbarGroupLayout, ToolbarInteractionAction, ToolbarKeyInput,
    ToolbarKeyboardInput, ToolbarKeyboardNavigator, ToolbarOptions, ToolbarOverflowInput,
    ToolbarOverflowPlanner, ToolbarPlacementRequest, ToolbarPriority, ToolbarState,
    ToolbarStrategy,
};
use katana_ui_core::molecule::{ContextMenuAnchor, ContextMenuRect};

#[test]
fn overflow_hides_lowest_priority_and_trailing_ties_first() {
    let actions = vec![
        measured("save", 40, 100),
        measured("search", 40, 10),
        measured("export", 40, 10),
        measured("settings", 40, 50),
    ];
    let input = ToolbarOverflowInput::new(110, 10, ToolbarStrategy::Menu, actions);

    let first_plan = ToolbarOverflowPlanner::plan(&input);
    let second_plan = ToolbarOverflowPlanner::plan(&input);

    assert_eq!(first_plan, second_plan);
    assert_eq!(vec!["save", "settings"], first_plan.visible_action_ids());
    assert_eq!(vec!["export", "search"], first_plan.hidden_action_ids());
    assert!(first_plan.overflow_trigger_visible());
}

#[test]
fn overflow_handles_fit_non_menu_strategies_and_exhausted_actions() {
    let fitting = ToolbarOverflowPlanner::plan(&ToolbarOverflowInput::new(
        100,
        10,
        ToolbarStrategy::Menu,
        vec![measured("save", 40, 10)],
    ));
    assert_eq!(vec!["save"], fitting.visible_action_ids());
    assert!(fitting.hidden_action_ids().is_empty());
    assert!(!fitting.overflow_trigger_visible());

    for strategy in [ToolbarStrategy::Hide, ToolbarStrategy::Custom] {
        let plan = ToolbarOverflowPlanner::plan(&ToolbarOverflowInput::new(
            39,
            10,
            strategy,
            vec![measured("save", 40, 10)],
        ));
        assert!(plan.visible_action_ids().is_empty());
        assert_eq!(vec!["save"], plan.hidden_action_ids());
        assert!(!plan.overflow_trigger_visible());
    }

    let exhausted = ToolbarOverflowPlanner::plan(&ToolbarOverflowInput::new(
        0,
        10,
        ToolbarStrategy::Menu,
        vec![measured("save", 40, 10)],
    ));
    assert!(exhausted.visible_action_ids().is_empty());
    assert_eq!(vec!["save"], exhausted.hidden_action_ids());
    assert!(exhausted.overflow_trigger_visible());
}

#[test]
fn icon_only_requires_accessibility_label_or_tooltip() {
    let invalid = ToolbarOptions::new()
        .display_mode(ToolbarDisplayMode::IconOnly)
        .action(ToolbarAction::new("run", "Run"));
    let valid = ToolbarOptions::new()
        .display_mode(ToolbarDisplayMode::IconOnly)
        .action(ToolbarAction::new("save", "Save").tooltip("Save file"));

    assert_eq!(
        vec![ToolbarContractViolation::MissingIconOnlyAccessibleName {
            action_id: "run".into()
        }],
        invalid.validate()
    );
    assert!(valid.validate().is_empty());
}

#[test]
fn display_mode_change_invalidates_measured_widths_before_recompute() {
    let mut state = ToolbarState::new(ToolbarDisplayMode::IconLeading).with_measured_width(
        MeasuredToolbarAction::new("save", 72, ToolbarPriority::new(10)),
    );

    let invalidated = state.set_display_mode(ToolbarDisplayMode::IconOnly);

    assert!(invalidated);
    assert!(state.measured_widths().is_empty());
    assert_eq!(ToolbarDisplayMode::IconOnly, state.display_mode());
}

#[test]
fn split_action_keeps_primary_and_secondary_disabled_state_independent() {
    let action = ToolbarAction::new("save-as", "Save As").split(SplitAction::new(
        SplitActionPart::new().disabled(true),
        SplitActionPart::new()
            .disabled(false)
            .tooltip("More save options"),
    ));
    let mut state = ToolbarState::new(ToolbarDisplayMode::IconLeading);

    let events = state.apply_action(
        &ToolbarInteractionAction::open_split_dropdown("save-as"),
        std::slice::from_ref(&action),
    );

    assert!(action.split_state().primary_disabled());
    assert!(!action.split_state().secondary_disabled());
    assert_eq!(Some(&"save-as".into()), state.split_open());
    assert_eq!(
        vec![ToolbarEvent::SplitDropdownOpened {
            action_id: "save-as".into(),
            placement: ToolbarPlacementRequest::Menu,
        }],
        events
    );
}

#[test]
fn toolbar_menu_placement_uses_shared_placement_engine() {
    let request = PlacementRequest::new(
        AnchorKind::virtual_rect(Rect::new(120, 190, 40, 24)),
        Placement::BottomStart,
        Size::new(120, 80),
        Rect::new(0, 0, 320, 220),
    )
    .clamp_margin(8);

    let result = ToolbarPlacementRequest::Menu.resolve(&request);

    assert_eq!(Placement::TopStart, result.placement_used);
}

#[test]
fn toolbar_options_expose_context_menu_hook_anchor() {
    let anchor = ContextMenuAnchor::VirtualRect(ContextMenuRect::new(12, 24, 80, 32));
    let options = ToolbarOptions::new().context_menu_anchor(anchor.clone());

    assert_eq!(Some(&anchor), options.context_menu_anchor_model());
}

#[test]
fn toolbar_options_preserve_density_overflow_groups_and_default_contract() {
    let group = ToolbarGroup::new("editing");
    let configured = ToolbarOptions::default()
        .density(ToolbarDensity::Spacious)
        .overflow_strategy(ToolbarStrategy::Custom)
        .group(group);

    assert_ne!(configured, ToolbarOptions::new());
    assert!(configured.validate().is_empty());
}

#[test]
fn toolbar_identifiers_accept_owned_values_and_expose_priority() {
    let action_id = katana_ui_core::molecule::toolbar::ToolbarActionId::from("save".to_string());
    let group_id = ToolbarGroupId::from("editing".to_string());
    let priority = ToolbarPriority::new(-4);

    assert_eq!("save", action_id.as_str());
    assert_eq!("editing", group_id.as_str());
    assert_eq!(-4, priority.value());
}

#[test]
fn toolbar_state_covers_noop_disabled_overflow_group_and_action_builder_paths() {
    let enabled = ToolbarAction::new("save", "Save");
    let primary_disabled = ToolbarAction::new("delete", "Delete").split(SplitAction::new(
        SplitActionPart::new().disabled(true),
        SplitActionPart::new(),
    ));
    let secondary_disabled = ToolbarAction::new("export", "Export").split(SplitAction::new(
        SplitActionPart::new(),
        SplitActionPart::new().disabled(true),
    ));
    let actions = vec![enabled, primary_disabled, secondary_disabled];
    let mut state = ToolbarState::new(ToolbarDisplayMode::IconLeading);

    assert!(!state.set_display_mode(ToolbarDisplayMode::IconLeading));
    assert!(
        state
            .apply_action(&ToolbarInteractionAction::press("missing"), &actions)
            .is_empty()
    );
    assert!(
        state
            .apply_action(&ToolbarInteractionAction::press("delete"), &actions)
            .is_empty()
    );
    assert_eq!(
        vec![ToolbarEvent::Command {
            action_id: "save".into()
        }],
        state.apply_action(&ToolbarInteractionAction::activate("save"), &actions)
    );
    assert_eq!(
        vec![ToolbarEvent::OverflowOpened],
        state.apply_action(&ToolbarInteractionAction::OpenOverflow, &actions)
    );
    assert_eq!(
        vec![ToolbarEvent::GroupCollapseToggled {
            group_id: "editing".into()
        }],
        state.apply_action(
            &ToolbarInteractionAction::toggle_group_collapse("editing"),
            &actions
        )
    );
    assert!(
        state
            .apply_action(
                &ToolbarInteractionAction::open_split_dropdown("missing"),
                &actions
            )
            .is_empty()
    );
    assert!(
        state
            .apply_action(
                &ToolbarInteractionAction::open_split_dropdown("export"),
                &actions
            )
            .is_empty()
    );
}

#[test]
fn accelerator_fires_command_without_moving_focus() {
    let actions = vec![
        ToolbarAction::new("save", "Save").accelerator(KeyCombo::command_or_control("s")),
        ToolbarAction::new("delete", "Delete")
            .accelerator(KeyCombo::command_or_control("backspace"))
            .disabled(true),
    ];
    let input = ToolbarKeyInput::new("s").command_or_control();
    let focus = ToolbarFocusState::new("editor-body");

    let result = ToolbarState::new(ToolbarDisplayMode::IconLeading).trigger_accelerator(
        &actions,
        &input,
        focus.clone(),
    );

    assert_eq!(&focus, result.focus_before());
    assert_eq!(&focus, result.focus_after());
    assert_eq!(
        vec![
            ToolbarEvent::AcceleratorTriggered {
                action_id: "save".into(),
                combo: KeyCombo::command_or_control("s"),
            },
            ToolbarEvent::Command {
                action_id: "save".into(),
            },
        ],
        result.events()
    );

    let disabled = ToolbarState::new(ToolbarDisplayMode::IconLeading).trigger_accelerator(
        &actions,
        &ToolbarKeyInput::new("backspace").command_or_control(),
        focus,
    );
    assert!(disabled.events().is_empty());
}

#[test]
fn keyboard_navigation_moves_and_activates_without_layout_state() {
    assert_eq!(
        Some(0),
        ToolbarKeyboardNavigator::apply(Some(0), 4, ToolbarKeyboardInput::ArrowLeft)
            .focused_index()
    );
    assert_eq!(
        Some(2),
        ToolbarKeyboardNavigator::apply(Some(1), 4, ToolbarKeyboardInput::ArrowRight)
            .focused_index()
    );
    assert_eq!(
        Some(0),
        ToolbarKeyboardNavigator::apply(Some(2), 4, ToolbarKeyboardInput::Home).focused_index()
    );
    assert_eq!(
        Some(3),
        ToolbarKeyboardNavigator::apply(Some(0), 4, ToolbarKeyboardInput::End).focused_index()
    );
    assert_eq!(
        Some(3),
        ToolbarKeyboardNavigator::apply(Some(3), 4, ToolbarKeyboardInput::Space).activated_index()
    );
    assert_eq!(
        None,
        ToolbarKeyboardNavigator::apply(None, 0, ToolbarKeyboardInput::Enter).focused_index()
    );
}

#[test]
fn group_boundaries_create_dividers_and_overflow_sections() {
    let edit_group = ToolbarGroupId::new("edit");
    let view_group = ToolbarGroupId::new("view");
    let build_group = ToolbarGroupId::new("build");
    let groups = vec![
        ToolbarGroup::new(edit_group.clone()).label("Edit"),
        ToolbarGroup::new(view_group.clone()).label("View"),
        ToolbarGroup::new(build_group.clone()).divider(false),
    ];
    let actions = vec![
        ToolbarAction::new("cut", "Cut").group_id(edit_group.clone()),
        ToolbarAction::new("copy", "Copy").group_id(edit_group),
        ToolbarAction::new("preview", "Preview").group_id(view_group),
        ToolbarAction::new("run", "Run").group_id(build_group),
    ];

    let dividers = ToolbarGroupLayout::visible_group_dividers(&actions, &groups);
    let sections = ToolbarOverflowPlanner::overflow_menu_sections(&actions[1..], &groups);

    assert_eq!(vec!["preview"], divider_targets(&dividers));
    assert_eq!(3, sections.len());
    assert_eq!(Some("Edit"), sections[0].label());
    assert_eq!(Some("View"), sections[1].label());
    assert_eq!(None, sections[2].label());
    assert!(!groups[2].divider_model());
}

#[test]
fn toolbar_action_split_accelerator_and_grouped_sections_cover_public_boundaries() {
    let split = SplitAction::new(
        SplitActionPart::default().accessibility_label("Run"),
        SplitActionPart::default()
            .tooltip("More")
            .accessibility_label("More run actions"),
    );
    let action = ToolbarAction::new("run", "Run")
        .priority(ToolbarPriority::new(42))
        .split(split)
        .accessibility_label("Run command");
    assert!(action.has_accessible_name());
    assert!(!action.split_state().primary_disabled());
    assert!(!action.split_state().secondary_disabled());

    let combo = KeyCombo::command_or_control("K");
    assert!(combo.matches_input(&ToolbarKeyInput::new("k").with_modifier(KeyModifier::Command)));
    assert!(combo.matches_input(&ToolbarKeyInput::new("K").with_modifier(KeyModifier::Control)));
    assert!(!combo.matches_input(&ToolbarKeyInput::new("k").with_modifier(KeyModifier::Shift)));
    assert!(KeyCombo::new("F1", Vec::new()).matches_input(&ToolbarKeyInput::new("f1")));
    assert!(
        KeyCombo::new("K", vec![KeyModifier::Shift])
            .matches_input(&ToolbarKeyInput::new("k").with_modifier(KeyModifier::Shift))
    );

    let group = ToolbarGroupId::new("edit");
    let actions = vec![
        ToolbarAction::new("cut", "Cut").group_id(group.clone()),
        ToolbarAction::new("copy", "Copy").group_id(group.clone()),
        ToolbarAction::new("paste", "Paste").group_id(group),
    ];
    let sections = ToolbarOverflowPlanner::overflow_menu_sections(
        &actions,
        &[ToolbarGroup::new("edit").label("Edit")],
    );
    assert_eq!(1, sections.len());
    assert_eq!(Some("Edit"), sections[0].label());
}

fn measured(id: &str, width: u32, priority: i32) -> MeasuredToolbarAction {
    MeasuredToolbarAction::new(id, width, ToolbarPriority::new(priority))
}

fn divider_targets(
    dividers: &[katana_ui_core::molecule::toolbar::ToolbarGroupDivider],
) -> Vec<&str> {
    dividers
        .iter()
        .map(|divider| divider.before_action_id().as_str())
        .collect()
}
