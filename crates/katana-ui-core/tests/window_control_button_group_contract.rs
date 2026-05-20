use katana_ui_core::adapter_contract::{AdapterExtension, WindowControlDispatchRequest};
use katana_ui_core::molecule::selection::window_control_button_group::{
    WindowControlButtonGroup, WindowControlButtonGroupAction, WindowControlButtonGroupEvent,
    WindowControlButtonGroupOptions, WindowControlKind, WindowControlSize, WindowControlVisibility,
    WindowControlsPosition,
};
use katana_ui_core::render_model::{UiDimension, UiJustifyContent, UiNode, UiNodeKind};
use katana_ui_core::window::{WindowCommand, WindowId};

#[test]
fn options_expose_controls_position_size_and_visibility_only() {
    let options = WindowControlButtonGroupOptions {
        controls: vec![WindowControlKind::Close, WindowControlKind::Restore],
        position: WindowControlsPosition::Leading,
        visibility: WindowControlVisibility::Always,
        size: WindowControlSize::Compact,
    };
    let group = WindowControlButtonGroup::new("window-controls").options(options);
    let node = UiNode::from(group);

    assert_eq!(UiNodeKind::WindowControlButtonGroup, node.kind());
    assert_eq!("window-controls", node.props().label);
    assert_eq!(UiJustifyContent::Start, node.props().common.justify_content);
    assert_eq!(2, node.children().len());
    assert!(
        node.children()
            .iter()
            .all(|it| it.kind() == UiNodeKind::Button)
    );
}

#[test]
fn public_contract_excludes_title_bar_and_draggable_regions() {
    let public_surface =
        include_str!("../src/molecule/selection/window_control_button_group/options.rs");

    assert!(!public_surface.contains("draggable"));
    assert!(!public_surface.contains("title"));
    assert!(!public_surface.contains("TitleBar"));
}

#[test]
fn position_changes_layout_contract_without_title_bar_ownership() {
    let trailing = UiNode::from(WindowControlButtonGroup::new("trailing").options(
        WindowControlButtonGroupOptions {
            position: WindowControlsPosition::Trailing,
            ..WindowControlButtonGroupOptions::default()
        },
    ));
    let auto = UiNode::from(WindowControlButtonGroup::new("auto").options(
        WindowControlButtonGroupOptions {
            position: WindowControlsPosition::Auto,
            ..WindowControlButtonGroupOptions::default()
        },
    ));

    assert_eq!(
        UiJustifyContent::End,
        trailing.props().common.justify_content
    );
    assert_eq!(UiJustifyContent::Start, auto.props().common.justify_content);
    assert!(
        trailing
            .children()
            .iter()
            .all(|it| it.children().is_empty())
    );
}

#[test]
fn pressing_controls_emits_typed_intent_events() {
    let mut group = WindowControlButtonGroup::new("window-controls");
    let events = group.apply_action(WindowControlButtonGroupAction::Press(
        WindowControlKind::Close,
    ));
    let window_id = WindowId::new("main");

    assert_eq!(
        vec![WindowControlButtonGroupEvent::ControlPressed {
            which: WindowControlKind::Close
        }],
        events
    );
    assert_eq!(
        Some(WindowCommand::Close {
            window_id: window_id.clone()
        }),
        events[0].window_command(window_id)
    );
    assert_eq!(events, group.state().events());
}

#[test]
fn adapter_dispatch_request_maps_all_control_events_to_window_commands() {
    let window_id = WindowId::new("main-window");

    let cases = [
        (
            WindowControlKind::Close,
            WindowCommand::Close {
                window_id: window_id.clone(),
            },
        ),
        (
            WindowControlKind::Minimize,
            WindowCommand::Minimize {
                window_id: window_id.clone(),
            },
        ),
        (
            WindowControlKind::Maximize,
            WindowCommand::Maximize {
                window_id: window_id.clone(),
            },
        ),
        (
            WindowControlKind::Restore,
            WindowCommand::Restore {
                window_id: window_id.clone(),
            },
        ),
    ];

    for (control, command) in cases {
        let request = WindowControlDispatchRequest::from_event(
            WindowControlButtonGroupEvent::ControlPressed { which: control },
            window_id.clone(),
        )
        .expect("control press must become adapter dispatch request");

        assert_eq!(window_id, request.window_id);
        assert_eq!(control, request.control);
        assert_eq!(command, request.command());
    }
}

#[test]
fn adapter_extension_carries_window_control_without_owning_title_bar_layout() {
    let request =
        WindowControlDispatchRequest::new(WindowId::new("main-window"), WindowControlKind::Close);
    let extension = AdapterExtension::WindowControl(request.clone());
    let adapter_contract_source = include_str!("../src/adapter_contract/mod.rs");

    assert_eq!(AdapterExtension::WindowControl(request), extension);
    assert!(!adapter_contract_source.contains("TitleBar"));
    assert!(!adapter_contract_source.contains("DraggableRegion"));
}

#[test]
fn hover_and_fullscreen_visibility_emit_state_events() {
    let mut group =
        WindowControlButtonGroup::new("window-controls").options(WindowControlButtonGroupOptions {
            visibility: WindowControlVisibility::FullscreenHover,
            ..WindowControlButtonGroupOptions::default()
        });

    let fullscreen_events = group.apply_action(WindowControlButtonGroupAction::SetFullscreen(true));
    assert_eq!(
        vec![
            WindowControlButtonGroupEvent::FullscreenChanged { fullscreen: true },
            WindowControlButtonGroupEvent::VisibilityChanged { visible: false },
        ],
        fullscreen_events
    );

    let hover_events = group.apply_action(WindowControlButtonGroupAction::SetHover(true));
    assert_eq!(
        vec![WindowControlButtonGroupEvent::VisibilityChanged { visible: true }],
        hover_events
    );
    assert!(group.state().visible());
}

#[test]
fn size_tokens_map_to_stable_pixel_contracts() {
    let compact = render_with_size(WindowControlSize::Compact);
    let default = render_with_size(WindowControlSize::Default);
    let tall = render_with_size(WindowControlSize::Tall);

    assert_eq!(
        UiDimension::px(WindowControlSize::Compact.pixels()),
        compact.children()[0].props().common.width
    );
    assert_eq!(
        UiDimension::px(WindowControlSize::Default.pixels()),
        default.children()[0].props().common.width
    );
    assert_eq!(
        UiDimension::px(WindowControlSize::Tall.pixels()),
        tall.children()[0].props().common.width
    );
}

fn render_with_size(size: WindowControlSize) -> UiNode {
    UiNode::from(WindowControlButtonGroup::new("window-controls").options(
        WindowControlButtonGroupOptions {
            controls: vec![WindowControlKind::Close],
            size,
            ..WindowControlButtonGroupOptions::default()
        },
    ))
}
