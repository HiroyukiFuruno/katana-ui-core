use katana_ui_core::interaction::placement::{Rect, Size};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeContractViolation, CommandChromeDisplayMode,
    CommandChromeDropdown, CommandChromeDropdownItem, CommandChromeDropdownLayout,
    CommandChromeDropdownTrigger, CommandChromeMeasuredAction, CommandChromeToolbar,
    CommandChromeToolbarAction, CommandChromeToolbarEvent, CommandChromeToolbarPresentation,
};
use katana_ui_core::molecule::toolbar::{
    KeyCombo, SplitAction, SplitActionPart, ToolbarKeyboardInput, ToolbarPlacementRequest,
    ToolbarPriority, ToolbarStrategy,
};
use katana_ui_core::render_model::UiIconProps;

#[test]
fn icon_only_rejects_missing_svg_and_accessible_name_without_label_fallback() {
    let mut toolbar = CommandChromeToolbar::new()
        .display_mode(CommandChromeDisplayMode::IconOnly)
        .action(CommandChromeAction::new("bold", "Bold"))
        .action(CommandChromeAction::new("italic", "Italic").icon(empty_icon()));

    assert_eq!(
        vec![
            CommandChromeContractViolation::MissingIconOnlyIcon {
                action_id: "bold".into(),
            },
            CommandChromeContractViolation::MissingIconOnlyAccessibleName {
                action_id: "bold".into(),
            },
            CommandChromeContractViolation::MissingIconOnlyIcon {
                action_id: "italic".into(),
            },
            CommandChromeContractViolation::MissingIconOnlyAccessibleName {
                action_id: "italic".into(),
            },
        ],
        toolbar.validate()
    );
    assert!(
        toolbar
            .apply_action(CommandChromeToolbarAction::activate("bold"))
            .is_empty()
    );
}

#[test]
fn host_icon_is_preserved_and_activation_emits_only_generic_action_id() {
    let icon = icon();
    let mut toolbar = icon_toolbar().action(
        CommandChromeAction::new("bold", "Bold")
            .icon(icon.clone())
            .tooltip("Bold"),
    );

    assert_eq!(Some(&icon), toolbar.actions()[0].icon_model());
    assert_eq!(
        vec![CommandChromeToolbarEvent::CommandActivated {
            action_id: "bold".into(),
        }],
        toolbar.apply_action(CommandChromeToolbarAction::activate("bold"))
    );
}

#[test]
fn renderer_accessors_expose_injected_chrome_data_without_host_semantics() {
    let split = SplitAction::new(SplitActionPart::new(), SplitActionPart::new());
    let action = CommandChromeAction::new("command-id", "ホスト注入ラベル")
        .icon(icon())
        .tooltip("注入 tooltip")
        .accessibility_label("注入 accessibility")
        .split(split.clone());
    let mut toolbar = icon_toolbar().action(action);

    let action = &toolbar.actions()[0];
    assert_eq!("ホスト注入ラベル", action.label_model());
    assert_eq!(Some(&"注入 tooltip".to_string()), action.tooltip_model());
    assert_eq!(
        Some(&"注入 accessibility".to_string()),
        action.accessibility_label_model()
    );
    assert_eq!(Some(&split), action.split_model());
    assert_eq!(
        vec![CommandChromeToolbarEvent::FocusChanged {
            action_id: "command-id".into(),
        }],
        toolbar.apply_action(CommandChromeToolbarAction::Keyboard {
            input: ToolbarKeyboardInput::Home,
        })
    );
    assert_eq!(Some("command-id".into()), toolbar.focused_action_id_model());
}

#[test]
fn disabled_split_primary_never_emits_command_while_enabled_dropdown_emits_only_dropdown() {
    let action = CommandChromeAction::new("format", "Format")
        .icon(icon())
        .tooltip("Format")
        .split(SplitAction::new(
            SplitActionPart::new().disabled(true),
            SplitActionPart::new(),
        ));
    let mut toolbar = icon_toolbar().action(action);

    assert!(
        toolbar
            .apply_action(CommandChromeToolbarAction::activate("format"))
            .is_empty()
    );
    assert_eq!(
        vec![CommandChromeToolbarEvent::SplitDropdownOpened {
            action_id: "format".into(),
            placement: ToolbarPlacementRequest::Menu,
        }],
        toolbar.apply_action(CommandChromeToolbarAction::open_split_dropdown("format"))
    );
}

#[test]
fn command_chrome_delegates_overflow_priority_to_existing_toolbar_planner() {
    let toolbar = icon_toolbar()
        .overflow_strategy(ToolbarStrategy::Menu)
        .action(action("save", 100))
        .action(action("search", 10))
        .action(action("export", 10));
    let plan = toolbar.plan_overflow(
        90,
        10,
        &[
            CommandChromeMeasuredAction::new("save", 40),
            CommandChromeMeasuredAction::new("search", 40),
            CommandChromeMeasuredAction::new("export", 40),
        ],
    );

    assert_eq!(vec!["save", "search"], plan.visible_action_ids());
    assert_eq!(vec!["export"], plan.hidden_action_ids());
    assert!(plan.overflow_trigger_visible());
}

#[test]
fn keyboard_navigation_uses_existing_toolbar_state_and_never_activates_disabled_action() {
    let mut toolbar = icon_toolbar()
        .action(action("bold", 10))
        .action(action("italic", 10).disabled(true));

    assert_eq!(
        vec![CommandChromeToolbarEvent::FocusChanged {
            action_id: "bold".into(),
        }],
        toolbar.apply_action(CommandChromeToolbarAction::Keyboard {
            input: ToolbarKeyboardInput::Home,
        })
    );
    assert_eq!(
        vec![
            CommandChromeToolbarEvent::FocusChanged {
                action_id: "bold".into(),
            },
            CommandChromeToolbarEvent::CommandActivated {
                action_id: "bold".into(),
            },
        ],
        toolbar.apply_action(CommandChromeToolbarAction::Keyboard {
            input: ToolbarKeyboardInput::Enter,
        })
    );
    assert_eq!(
        vec![CommandChromeToolbarEvent::FocusChanged {
            action_id: "italic".into(),
        }],
        toolbar.apply_action(CommandChromeToolbarAction::Keyboard {
            input: ToolbarKeyboardInput::ArrowRight,
        })
    );
    assert_eq!(
        vec![CommandChromeToolbarEvent::FocusChanged {
            action_id: "italic".into(),
        }],
        toolbar.apply_action(CommandChromeToolbarAction::Keyboard {
            input: ToolbarKeyboardInput::Space,
        })
    );
}

#[test]
fn accelerator_composes_existing_toolbar_event_without_host_command() {
    let mut toolbar = icon_toolbar().action(
        CommandChromeAction::new("bold", "Bold")
            .icon(icon())
            .tooltip("Bold")
            .accelerator(KeyCombo::command_or_control("b")),
    );

    assert_eq!(
        vec![
            CommandChromeToolbarEvent::AcceleratorTriggered {
                action_id: "bold".into(),
                combo: KeyCombo::command_or_control("b"),
            },
            CommandChromeToolbarEvent::CommandActivated {
                action_id: "bold".into(),
            },
        ],
        toolbar.apply_action(CommandChromeToolbarAction::TriggerAccelerator {
            input: katana_ui_core::molecule::toolbar::ToolbarKeyInput::new("b")
                .command_or_control(),
            focus: katana_ui_core::molecule::toolbar::ToolbarFocusState::new("surface"),
        })
    );
}

#[test]
fn controlled_presentation_updates_without_synthesizing_a_command_event() {
    let mut toolbar = CommandChromeToolbar::new().action(CommandChromeAction::new("one", "One"));

    assert!(
        toolbar.synchronize_presentation(CommandChromeToolbarPresentation {
            actions: vec![CommandChromeAction::new("one", "Uno")],
            groups: Vec::new(),
            display_mode: CommandChromeDisplayMode::IconLeading,
            density: Default::default(),
            overflow_strategy: Default::default(),
        })
    );
    assert_eq!(toolbar.actions()[0].label_model(), "Uno");
    assert!(
        !toolbar.synchronize_presentation(CommandChromeToolbarPresentation {
            actions: vec![CommandChromeAction::new("one", "Uno")],
            groups: Vec::new(),
            display_mode: CommandChromeDisplayMode::IconLeading,
            density: Default::default(),
            overflow_strategy: Default::default(),
        })
    );
}

#[test]
fn controlled_presentation_retains_focus_by_opaque_id_across_reordering() {
    let mut toolbar = CommandChromeToolbar::new()
        .action(CommandChromeAction::new("first", "First"))
        .action(CommandChromeAction::new("second", "Second"));
    let _ = toolbar.apply_action(CommandChromeToolbarAction::Keyboard {
        input: ToolbarKeyboardInput::Home,
    });
    assert_eq!(
        toolbar
            .focused_action_id_model()
            .as_ref()
            .map(|id| id.as_str()),
        Some("first")
    );
    let _ = toolbar.apply_action(CommandChromeToolbarAction::Keyboard {
        input: ToolbarKeyboardInput::ArrowRight,
    });
    assert_eq!(
        toolbar
            .focused_action_id_model()
            .as_ref()
            .map(|id| id.as_str()),
        Some("second")
    );
    assert!(
        toolbar.synchronize_presentation(CommandChromeToolbarPresentation {
            actions: vec![
                CommandChromeAction::new("second", "Second"),
                CommandChromeAction::new("first", "First")
            ],
            groups: Vec::new(),
            display_mode: CommandChromeDisplayMode::IconLeading,
            density: Default::default(),
            overflow_strategy: Default::default(),
        })
    );
    assert_eq!(
        toolbar
            .focused_action_id_model()
            .as_ref()
            .map(|id| id.as_str()),
        Some("second")
    );
}

#[test]
fn controlled_presentation_keeps_an_open_dropdown_when_its_opaque_action_id_survives() {
    let dropdown = CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary)
        .item(CommandChromeDropdownItem::new("one", "One"));
    let mut toolbar = CommandChromeToolbar::new()
        .action(CommandChromeAction::new("format", "Format").dropdown(dropdown.clone()));
    let _ = toolbar.apply_action(CommandChromeToolbarAction::update_dropdown_layout(
        "format",
        CommandChromeDropdownLayout::new(
            Rect::new(0, 0, 24, 20),
            Rect::new(0, 0, 160, 120),
            Size::new(120, 64),
        ),
    ));
    let _ = toolbar.apply_action(CommandChromeToolbarAction::activate("format"));
    assert!(toolbar.open_dropdown_model().is_some());
    assert!(
        toolbar.synchronize_presentation(CommandChromeToolbarPresentation {
            actions: vec![CommandChromeAction::new("format", "Format updated").dropdown(dropdown)],
            groups: Vec::new(),
            display_mode: CommandChromeDisplayMode::IconLeading,
            density: Default::default(),
            overflow_strategy: Default::default(),
        })
    );
    assert_eq!(
        toolbar
            .open_dropdown_model()
            .map(|dropdown| dropdown.action_id().as_str()),
        Some("format")
    );
}

fn icon_toolbar() -> CommandChromeToolbar {
    CommandChromeToolbar::new().display_mode(CommandChromeDisplayMode::IconOnly)
}

fn action(id: &str, priority: i32) -> CommandChromeAction {
    CommandChromeAction::new(id, id)
        .icon(icon())
        .tooltip(id)
        .priority(ToolbarPriority::new(priority))
}

fn icon() -> UiIconProps {
    UiIconProps::new("<svg viewBox=\"0 0 16 16\"><path d=\"M1 1h14v14H1z\"/></svg>")
}

fn empty_icon() -> UiIconProps {
    UiIconProps::new("  ")
}
