use katana_ui_core::atom::{Button, Text};
use katana_ui_core::component::ComponentTree;
use katana_ui_core::facade::{UiCoreFacade, UiGlobalState};
use katana_ui_core::render_model::{UiNodeId, UiTree};
use katana_ui_core::style::{StyleDeclaration, StyleProperty, StyleRule, StyleSheet, StyleValue};
use katana_ui_core::theme::{FontFamily, ThemeSnapshot};

#[test]
fn facade_configures_theme_style_font_and_global_state() {
    let focus_target = UiNodeId::new("button-save");
    let style_sheet = StyleSheet::new().rule(StyleRule::class(
        "primary",
        vec![StyleDeclaration::new(
            StyleProperty::Background,
            StyleValue::ColorToken("accent".to_string()),
        )],
    ));
    let global_state =
        UiGlobalState::new(ThemeSnapshot::dark().id).focus_target(focus_target.clone());
    let facade = UiCoreFacade::default()
        .with_style_sheet(style_sheet)
        .with_global_state(global_state)
        .with_default_font_role("code");
    let context = facade.render_context(800.0, 600.0);

    assert_eq!("dark", context.theme_id.as_str());
    assert_eq!(
        Some(&focus_target),
        facade.global_state().focus_target.as_ref()
    );
    assert_eq!(1, facade.style_sheet().rule_count());
    assert_eq!("code", facade.default_font_role());
    assert_eq!(Some(FontFamily::Monospace), facade.font_family("missing"));
}

#[test]
fn font_roles_stay_on_the_core_node_model() {
    let tree = UiTree::new(Text::new("code").font_role("code"));

    assert_eq!("code", tree.root().props().font_role);
}

#[test]
fn facade_global_state_does_not_replace_component_owned_state() {
    let facade = UiCoreFacade::default();
    let tree = ComponentTree::new(Button::new("Save").disabled(true)).into_tree();

    assert!(tree.root().props().disabled);
    assert_eq!("dark", facade.global_state().active_theme_id.as_str());
    assert_ne!(tree.root().props().state_id.as_str(), "global");
}
