use katana_ui_core::molecule::structured::startup_state_panel::{
    StartupState, StartupStatePanel, StartupStatePanelOptions,
};
use katana_ui_core::render_model::{UiNodeKind, UiTree};

#[test]
fn version_label_option_renders_as_child_text_without_template_ownership() {
    let options = StartupStatePanelOptions::default().version_label(Some("v0.1.0"));
    let tree = UiTree::new(
        StartupStatePanel::new("Startup")
            .state(StartupState::loading(Some(64), Some("Loading workspace")))
            .option(options),
    );
    let public_surface = include_str!("../src/molecule/structured/startup_state_panel/types.rs");

    assert!(
        tree.root()
            .children()
            .iter()
            .any(|it| { it.kind() == UiNodeKind::Text && it.props().label == "v0.1.0" })
    );
    assert!(!public_surface.contains("SplashScreen"));
    assert!(!public_surface.contains("background_image"));
    assert!(!public_surface.contains("full_screen"));
}

#[test]
fn retry_and_cancel_visibility_follow_error_state_flags() {
    let retry_only = render_error(true, false);
    let cancel_only = render_error(false, true);
    let no_action = render_error(false, false);

    assert_eq!(vec!["Retry"], button_labels(&retry_only));
    assert_eq!(vec!["Cancel"], button_labels(&cancel_only));
    assert!(button_labels(&no_action).is_empty());
}

fn render_error(retry: bool, cancel: bool) -> UiTree {
    UiTree::new(StartupStatePanel::new("Startup").state(StartupState::error(
        "Startup failed",
        retry,
        cancel,
    )))
}

fn button_labels(tree: &UiTree) -> Vec<&str> {
    tree.root()
        .children()
        .iter()
        .filter(|it| it.kind() == UiNodeKind::Button)
        .map(|it| it.props().label.as_str())
        .collect()
}
