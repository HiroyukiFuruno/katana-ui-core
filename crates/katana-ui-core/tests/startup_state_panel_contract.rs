use katana_ui_core::interaction::{MotionPrimitiveKind, MotionSpec, ReducedMotionPolicy};
use katana_ui_core::molecule::structured::startup_state_panel::{
    StartupState, StartupStatePanel, StartupStatePanelAction, StartupStatePanelEvent,
    StartupStatePanelOptions,
};
use katana_ui_core::render_model::{
    UiAnimationState, UiNodeKind, UiProgressMode, UiTree, UiVisualRole,
};

const PROGRESS_OVER_MAX: u8 = 250;
const PROGRESS_CLAMPED_MAX: u8 = 100;

#[test]
fn idle_and_loading_use_status_role_and_live_region_label() {
    let idle = StartupStatePanel::new("Startup").live_region_label("App startup status");
    let loading =
        StartupStatePanel::new("Startup").state(StartupState::loading(None, Some("Booting")));

    assert_eq!("status", idle.accessibility_role());
    assert_eq!("App startup status", idle.live_region_label_model());
    assert_eq!("status", loading.accessibility_role());

    let tree = UiTree::new(idle);
    assert_eq!(UiNodeKind::StartupStatePanel, tree.root().kind());
    assert_eq!(
        "status: App startup status",
        tree.root().props().accessibility_label
    );
    assert_eq!(UiVisualRole::Status, tree.root().props().visual_role);
}

#[test]
fn public_contract_excludes_splash_template_layout_fields() {
    let public_surface = include_str!("../src/molecule/structured/startup_state_panel/types.rs");

    assert!(!public_surface.contains("SplashScreen"));
    assert!(!public_surface.contains("full_screen"));
    assert!(!public_surface.contains("background"));
    assert!(!public_surface.contains("centered"));
}

#[test]
fn loading_progress_none_is_indeterminate_and_uses_loading_dots() {
    let tree = UiTree::new(
        StartupStatePanel::new("Startup")
            .state(StartupState::loading(None, Some("Preparing")))
            .option(StartupStatePanelOptions::default().reduced_motion(false)),
    );
    let root = tree.root();

    assert!(root.props().loading);
    assert!(!root.props().determinate);
    assert_eq!(
        UiProgressMode::Indeterminate,
        root.props().loading_indicator.mode
    );
    assert!(
        root.children()
            .iter()
            .any(|it| it.kind() == UiNodeKind::LoadingDots)
    );
    assert!(
        root.children()
            .iter()
            .any(|it| it.kind() == UiNodeKind::ProgressBar)
    );
}

#[test]
fn loading_progress_some_is_determinate_and_clamped() {
    let tree = UiTree::new(
        StartupStatePanel::new("Startup").state(StartupState::loading(
            Some(PROGRESS_OVER_MAX),
            Some("Downloading"),
        )),
    );
    let root = tree.root();
    let progress_nodes = root
        .children()
        .iter()
        .filter(|it| it.kind() == UiNodeKind::ProgressBar)
        .collect::<Vec<_>>();

    assert!(root.props().determinate);
    assert_eq!(1, progress_nodes.len(), "progress bar is rendered");
    assert_eq!(PROGRESS_CLAMPED_MAX, root.props().progress_percent);
    assert_eq!(
        UiProgressMode::Determinate,
        progress_nodes[0].props().loading_indicator.mode
    );
    assert_eq!(
        PROGRESS_CLAMPED_MAX,
        progress_nodes[0].props().progress_percent
    );
}

#[test]
fn reduced_motion_downgrades_loading_animation() {
    let tree = UiTree::new(
        StartupStatePanel::new("Startup")
            .state(StartupState::loading(None, Some("Preparing")))
            .option(StartupStatePanelOptions::default().reduced_motion(true)),
    );
    let loading_dots_nodes = tree
        .root()
        .children()
        .iter()
        .filter(|it| it.kind() == UiNodeKind::LoadingDots)
        .collect::<Vec<_>>();

    assert_eq!(1, loading_dots_nodes.len(), "loading dots are rendered");
    assert!(
        loading_dots_nodes[0]
            .props()
            .loading_indicator
            .reduced_motion
    );
    assert_eq!(
        UiAnimationState::Idle,
        loading_dots_nodes[0]
            .props()
            .loading_indicator
            .animation_state
    );
}

#[test]
fn set_state_retry_and_cancel_emit_typed_events() {
    let mut panel = StartupStatePanel::new("Startup");
    let loading = panel.apply_action(StartupStatePanelAction::SetState(StartupState::loading(
        Some(42),
        Some("Loading workspace"),
    )));
    let error = panel.apply_action(StartupStatePanelAction::SetState(StartupState::error(
        "Could not open workspace",
        true,
        true,
    )));
    let idle = panel.apply_action(StartupStatePanelAction::SetState(StartupState::Idle));
    let retried = panel.apply_action(StartupStatePanelAction::Retry);
    let canceled = panel.apply_action(StartupStatePanelAction::Cancel);

    assert!(matches!(
        loading.as_slice(),
        [StartupStatePanelEvent::StartupStateChanged {
            from: StartupState::Idle,
            to: StartupState::Loading { .. }
        }]
    ));
    assert!(matches!(
        error.as_slice(),
        [StartupStatePanelEvent::StartupStateChanged {
            from: StartupState::Loading { .. },
            to: StartupState::Error { .. }
        }]
    ));
    assert_eq!(
        idle.as_slice(),
        &[StartupStatePanelEvent::StartupStateChanged {
            from: StartupState::error("Could not open workspace", true, true),
            to: StartupState::Idle
        }]
    );
    assert_eq!(
        retried.as_slice(),
        &[StartupStatePanelEvent::StartupRetried]
    );
    assert_eq!(
        canceled.as_slice(),
        &[StartupStatePanelEvent::StartupCanceled]
    );
    assert!(
        panel
            .apply_action(StartupStatePanelAction::SetState(StartupState::Idle))
            .is_empty()
    );
    assert!(
        panel
            .apply_action(StartupStatePanelAction::SetReducedMotion(true))
            .is_empty()
    );

    let custom_motion = MotionSpec::new(
        MotionPrimitiveKind::Fade,
        50,
        0,
        ReducedMotionPolicy::Respect,
    );
    let options = StartupStatePanelOptions::default().motion(custom_motion.clone());
    assert_eq!(custom_motion, options.motion);
}

#[test]
fn error_uses_alert_role_and_optional_retry_cancel_buttons() {
    let tree = UiTree::new(StartupStatePanel::new("Startup").state(StartupState::error(
        "Could not open workspace",
        true,
        true,
    )));
    let root = tree.root();

    assert_eq!("alert", root.props().accessibility_label);
    assert_eq!(UiVisualRole::Status, root.props().visual_role);
    assert_eq!(
        2,
        root.children()
            .iter()
            .filter(|it| it.kind() == UiNodeKind::Button)
            .count()
    );
}
