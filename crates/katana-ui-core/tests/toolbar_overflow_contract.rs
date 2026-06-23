use katana_ui_core::interaction::placement::{AnchorKind, Placement, PlacementRequest, Rect, Size};
use katana_ui_core::molecule::toolbar::{
    KeyCombo, MeasuredToolbarAction, SplitAction, SplitActionPart, ToolbarAction,
    ToolbarContractViolation, ToolbarDisplayMode, ToolbarEvent, ToolbarFocusState, ToolbarGroup,
    ToolbarGroupId, ToolbarGroupLayout, ToolbarInteractionAction, ToolbarKeyInput,
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
}

#[test]
fn keyboard_navigation_moves_and_activates_without_layout_state() {
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
        ToolbarGroup::new(build_group.clone()),
    ];
    let actions = vec![
        ToolbarAction::new("cut", "Cut").group_id(edit_group.clone()),
        ToolbarAction::new("copy", "Copy").group_id(edit_group),
        ToolbarAction::new("preview", "Preview").group_id(view_group),
        ToolbarAction::new("run", "Run").group_id(build_group),
    ];

    let dividers = ToolbarGroupLayout::visible_group_dividers(&actions, &groups);
    let sections = ToolbarOverflowPlanner::overflow_menu_sections(&actions[1..], &groups);

    assert_eq!(vec!["preview", "run"], divider_targets(&dividers));
    assert_eq!(3, sections.len());
    assert_eq!(Some("Edit"), sections[0].label());
    assert_eq!(Some("View"), sections[1].label());
    assert_eq!(None, sections[2].label());
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
