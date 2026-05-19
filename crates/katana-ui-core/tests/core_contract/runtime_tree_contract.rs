use katana_ui_core::atom::{Button, IconTextButton, SvgButton, Text, TextButton};
use katana_ui_core::layout::Row;
use katana_ui_core::molecule::Toolbar;
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::{UiCommonProps, UiDimension, UiNodeKind, UiTree};
use katana_ui_core::runtime::{AppConfig, AppHandle, AppLifecycle, Application, RuntimeAdapter};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core::window::WindowConfig;

#[derive(Default)]
struct NoopRuntime {
    events: Vec<AppLifecycle>,
}

impl RuntimeAdapter for NoopRuntime {
    fn run(&mut self, config: AppConfig, windows: Vec<WindowConfig>) -> AppHandle {
        self.events.push(AppLifecycle::Started);
        AppHandle::new(
            config.app_id,
            windows.into_iter().map(WindowConfig::into_id).collect(),
        )
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
