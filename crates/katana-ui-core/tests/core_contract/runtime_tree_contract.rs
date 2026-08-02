use katana_ui_core::atom::{Button, IconTextButton, SvgButton, Text, TextButton};
use katana_ui_core::layout::Row;
use katana_ui_core::molecule::Toolbar;
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::{
    UiCommonProps, UiDimension, UiNode, UiNodeId, UiNodeKind, UiStateId, UiTree,
};
use katana_ui_core::runtime::{
    AppConfig, AppHandle, AppLifecycle, Application, RuntimeAdapter, RuntimeRunReport,
};
use katana_ui_core::surface::{PaintRequest, SurfaceMetrics};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core::window::{WindowConfig, WindowEvent};

#[derive(Default)]
struct NoopRuntime {
    events: Vec<AppLifecycle>,
}

impl RuntimeAdapter for NoopRuntime {
    fn run(&mut self, config: AppConfig, windows: Vec<WindowConfig>) -> AppHandle {
        self.events.push(AppLifecycle::Started);
        let window_ids = windows
            .into_iter()
            .map(WindowConfig::into_id)
            .collect::<Vec<_>>();
        let first_window = window_ids[0].clone();
        let report = RuntimeRunReport::new()
            .lifecycle(AppLifecycle::Created)
            .lifecycle(AppLifecycle::Started)
            .window_event(WindowEvent::Created(first_window.clone()))
            .window_event(WindowEvent::Focused(first_window.clone()))
            .paint_request(PaintRequest::new(
                first_window,
                SurfaceMetrics::new(1024.0, 768.0, 2.0, 220.0),
            ))
            .request_redraw()
            .request_shutdown()
            .lifecycle(AppLifecycle::ShuttingDown)
            .lifecycle(AppLifecycle::Stopped);
        AppHandle::new(config.app_id, window_ids).with_runtime_report(report)
    }
}

#[test]
fn application_builds_without_framework_types() {
    let runtime = NoopRuntime::default();
    let handle = Application::new(AppConfig::new("katana-ui-core"))
        .window(WindowConfig::new("Main"))
        .run_with(runtime);

    assert_eq!("katana-ui-core", handle.app_id());
    assert_eq!(1, handle.window_ids().len());
}

#[test]
fn runtime_handle_reports_event_loop_redraw_and_shutdown_contract() {
    let handle = Application::new(AppConfig::new("katana-ui-core"))
        .window(WindowConfig::new("Main"))
        .run_with(NoopRuntime::default());
    let report = handle.runtime_report();

    assert_eq!(
        &[
            AppLifecycle::Created,
            AppLifecycle::Started,
            AppLifecycle::ShuttingDown,
            AppLifecycle::Stopped
        ],
        report.lifecycle_events()
    );
    assert!(matches!(
        report.window_events(),
        [WindowEvent::Created(_), WindowEvent::Focused(_)]
    ));
    assert_eq!(1, report.paint_requests().len());
    assert_eq!(2.0, report.paint_requests()[0].metrics().scale_factor);
    assert!(report.redraw_requested());
    assert!(report.shutdown_requested());
}

#[test]
fn application_builder_and_empty_handle_keep_neutral_runtime_contract() {
    let handle = Application::builder(AppConfig::new("builder-app"))
        .window(WindowConfig::new("Builder"))
        .run_with(NoopRuntime::default());
    assert_eq!("builder-app", handle.app_id());
    assert_eq!(1, handle.window_ids().len());

    let empty = AppHandle::new("empty-app", Vec::new());
    assert_eq!("empty-app", empty.app_id());
    assert!(empty.window_ids().is_empty());
    assert!(empty.runtime_report().lifecycle_events().is_empty());
}

#[test]
fn neutral_tree_can_represent_atoms_and_layout() {
    let button = Button::new("Save").disabled(false).focusable(true);
    let tree = UiTree::new(Row::new().child(Text::new("Title")).child(button));

    assert_eq!(UiNodeKind::Row, tree.root().kind());
    assert_eq!(2, tree.root().children().len());
}

#[test]
fn common_props_are_available_to_atoms_molecules_and_panels() {
    let common = UiCommonProps::default()
        .width(UiDimension::percent(100))
        .height(UiDimension::px(48))
        .accessibility_label("Shared surface");
    let tree = UiTree::new(
        Panel::new("Root", PanelRegion::Root, ThemeSnapshot::dark())
            .common(common.clone())
            .child(Toolbar::new("Actions").common(common.clone()))
            .child(Button::new("Save").common(common.clone())),
    );
    let toolbar = &tree.root().children()[0];
    let button = &tree.root().children()[1];

    assert_eq!(UiDimension::percent(100), tree.root().props().common.width);
    assert_eq!(UiDimension::percent(100), toolbar.props().common.width);
    assert_eq!(UiDimension::percent(100), button.props().common.width);
    assert_eq!("Shared surface", button.props().accessibility_label);
}

#[test]
fn duplicate_ui_instances_have_unique_state_identity() {
    let tree = UiTree::new(
        Row::new()
            .child(Button::new("Save"))
            .child(Button::new("Save")),
    );
    let first = &tree.root().children()[0];
    let second = &tree.root().children()[1];

    assert_ne!(first.id(), second.id());
    assert_ne!(first.props().state_id, second.props().state_id);
}

#[test]
fn button_variants_keep_unique_state_identity() {
    let tree = UiTree::new(
        Row::new()
            .child(SvgButton::new("Action"))
            .child(TextButton::new("Action"))
            .child(IconTextButton::new("Action")),
    );
    let first = &tree.root().children()[0];
    let second = &tree.root().children()[1];
    let third = &tree.root().children()[2];

    assert_ne!(first.props().state_id, second.props().state_id);
    assert_ne!(second.props().state_id, third.props().state_id);
    assert_ne!(first.props().state_id, third.props().state_id);
}

#[test]
fn owned_identity_values_and_state_id_alias_preserve_exact_values() {
    let node_id = UiNodeId::from("owned-node".to_string());
    let state_id = UiStateId::from("owned-state".to_string());
    let node = UiNode::from(Text::new("Identity")).state_id(state_id);

    assert_eq!("owned-node", node_id.as_str());
    assert_eq!("owned-state", node.props().state_id.as_str());
}
