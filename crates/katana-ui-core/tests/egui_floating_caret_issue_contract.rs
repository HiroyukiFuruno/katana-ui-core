#![cfg(feature = "egui")]

#[path = "floating_caret_fixtures.rs"]
mod fixtures;

use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurface, EguiTextCommandSurfaceRoot, KucInteractionActionClass,
    KucInteractionSelector, TextCommandSurfaceStyle,
};
use katana_ui_core::molecule::command_chrome::FloatingCommandToolbarVisibility;

fn render(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceRoot,
    events: Vec<egui::Event>,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceRootOutput {
    let mut output = None;
    let mut full = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(fixtures::FRAME_WIDTH, fixtures::FRAME_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            output = Some(root.show(ui, &TextCommandSurfaceStyle::standard().expect("style")));
        },
    );
    full.textures_delta.clear();
    output.expect("root frame").expect("root render")
}

#[test]
fn visible_at_caret_requires_focus_and_nonempty_selection_exposes_floating_locator() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture())
        .with_floating_toolbar(
            fixtures::floating_toolbar_fixture(),
            FloatingCommandToolbarVisibility::VisibleAtCaret,
        );
    let mut root = EguiTextCommandSurfaceRoot::with_identity("floating-caret-contract", surface)
        .expect("root");
    let initial = render(&context, &mut root, Vec::new());
    assert!(matches!(
        initial
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "floating-bold",
                KucInteractionActionClass::FloatingToolbar,
            )),
        Err(katana_ui_core::egui::text_command_surface::KucInteractionLocatorError::Missing)
            | Err(katana_ui_core::egui::text_command_surface::KucInteractionLocatorError::Hidden)
    ));
}

#[test]
fn visible_selection_floating_toolbar_is_locator_addressable() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut surface = fixtures::text_surface_fixture();
    let _ = surface.apply_action(katana_ui_core::text_surface::TextSurfaceAction::TextArea(
        katana_ui_core::atom::TextAreaAction::Select(katana_ui_core::atom::TextAreaSelection {
            start: 0,
            end: 3,
        }),
    ));
    let surface = EguiTextCommandSurface::new(surface).with_floating_toolbar(
        fixtures::floating_toolbar_fixture(),
        FloatingCommandToolbarVisibility::Visible,
    );
    let mut root =
        EguiTextCommandSurfaceRoot::with_identity("floating-selection-contract", surface)
            .expect("root");
    let frame = render(&context, &mut root, Vec::new());
    assert!(
        frame
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "floating-bold",
                KucInteractionActionClass::FloatingToolbar,
            ))
            .is_ok()
    );
}

#[test]
fn focused_collapsed_caret_exposes_visible_at_caret_toolbar_and_click_continuation() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut text = fixtures::text_surface_fixture();
    let mut presentation =
        katana_ui_core::text_surface::TextSurfacePresentation::from_props(text.props());
    presentation.focus_request = Some(katana_ui_core::text_surface::TextSurfaceFocusRequest::new(
        katana_ui_core::text_surface::TextSurfaceFocusRequestToken::new("caret-focus"),
        true,
    ));
    let _ = text.synchronize_presentation(presentation);
    let surface = EguiTextCommandSurface::new(text).with_floating_toolbar(
        fixtures::floating_toolbar_fixture(),
        FloatingCommandToolbarVisibility::VisibleAtCaret,
    );
    let mut root =
        EguiTextCommandSurfaceRoot::with_identity("floating-caret-focused", surface).expect("root");
    let mut frame = render(&context, &mut root, Vec::new());
    for _ in 0..2 {
        frame = render(&context, &mut root, Vec::new());
    }
    let selector =
        KucInteractionSelector::new("floating-bold", KucInteractionActionClass::FloatingToolbar);
    let mut click = frame
        .interaction_locator()
        .begin_click(selector)
        .expect("focused caret floating toolbar target");
    let mut input = egui::RawInput::default();
    let mut activated = false;
    for _ in 0..3 {
        click
            .apply_to_raw_input_once(&mut input)
            .expect("click input");
        let next = render(&context, &mut root, std::mem::take(&mut input.events));
        match click
            .advance(next.interaction_locator())
            .expect("click continuation")
        {
            Some(next) => click = next,
            None => {
                activated = true;
                break;
            }
        }
    }
    assert!(
        activated,
        "floating toolbar click must complete exactly once"
    );
}

#[test]
fn legacy_visible_does_not_expose_focused_collapsed_caret_toolbar() {
    let context = egui::Context::default();
    let mut text = fixtures::text_surface_fixture();
    let mut presentation =
        katana_ui_core::text_surface::TextSurfacePresentation::from_props(text.props());
    presentation.focus_request = Some(katana_ui_core::text_surface::TextSurfaceFocusRequest::new(
        katana_ui_core::text_surface::TextSurfaceFocusRequestToken::new("legacy-focus"),
        true,
    ));
    let _ = text.synchronize_presentation(presentation);
    let surface = EguiTextCommandSurface::new(text).with_floating_toolbar(
        fixtures::floating_toolbar_fixture(),
        FloatingCommandToolbarVisibility::Visible,
    );
    let mut root =
        EguiTextCommandSurfaceRoot::with_identity("floating-caret-legacy", surface).expect("root");
    let frame = render(&context, &mut root, Vec::new());
    assert!(matches!(
        frame
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "floating-bold",
                KucInteractionActionClass::FloatingToolbar,
            )),
        Err(katana_ui_core::egui::text_command_surface::KucInteractionLocatorError::Missing)
            | Err(katana_ui_core::egui::text_command_surface::KucInteractionLocatorError::Hidden)
    ));
}

#[test]
fn inline_disabled_action_is_distinct_from_enabled_heading_action() {
    let context = egui::Context::default();
    let surface = EguiTextCommandSurface::new(fixtures::text_surface_fixture())
        .with_toolbar(fixtures::toolbar_fixture());
    let mut root =
        EguiTextCommandSurfaceRoot::with_identity("inline-disabled", surface).expect("root");
    let frame = render(&context, &mut root, Vec::new());
    assert!(
        frame
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "heading-1",
                KucInteractionActionClass::Toolbar,
            ))
            .is_ok()
    );
    assert!(matches!(
        frame
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "disabled",
                KucInteractionActionClass::Toolbar,
            )),
        Err(katana_ui_core::egui::text_command_surface::KucInteractionLocatorError::Disabled)
    ));
}

fn focused_caret_root(
    identity: &str,
    readonly: bool,
    disabled: bool,
) -> EguiTextCommandSurfaceRoot {
    let mut text = fixtures::text_surface_fixture();
    let mut presentation =
        katana_ui_core::text_surface::TextSurfacePresentation::from_props(text.props());
    presentation.readonly = readonly;
    presentation.disabled = disabled;
    presentation.focus_request = Some(katana_ui_core::text_surface::TextSurfaceFocusRequest::new(
        katana_ui_core::text_surface::TextSurfaceFocusRequestToken::new(identity),
        true,
    ));
    let _ = text.synchronize_presentation(presentation);
    let surface = EguiTextCommandSurface::new(text).with_floating_toolbar(
        fixtures::floating_toolbar_fixture(),
        FloatingCommandToolbarVisibility::VisibleAtCaret,
    );
    EguiTextCommandSurfaceRoot::with_identity(identity, surface).expect("root")
}

fn assert_floating_hidden(
    frame: &katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceRootOutput,
) {
    assert!(matches!(
        frame
            .interaction_locator()
            .request(KucInteractionSelector::new(
                "floating-bold",
                KucInteractionActionClass::FloatingToolbar,
            )),
        Err(katana_ui_core::egui::text_command_surface::KucInteractionLocatorError::Missing)
            | Err(katana_ui_core::egui::text_command_surface::KucInteractionLocatorError::Hidden)
    ));
}

#[test]
fn readonly_and_disabled_focused_collapsed_carets_hide_floating_toolbar() {
    let context = egui::Context::default();
    for (identity, readonly, disabled) in [
        ("floating-readonly", true, false),
        ("floating-disabled", false, true),
    ] {
        let mut root = focused_caret_root(identity, readonly, disabled);
        let frame = render(&context, &mut root, Vec::new());
        assert_floating_hidden(&frame);
    }
}

#[test]
fn focus_loss_and_escape_dismiss_focused_collapsed_caret_toolbar() {
    let context = egui::Context::default();
    let mut root = focused_caret_root("floating-focus-loss", false, false);
    let _ = render(&context, &mut root, Vec::new());
    let mut presentation = katana_ui_core::text_surface::TextSurfacePresentation::from_props(
        fixtures::text_surface_fixture().props(),
    );
    presentation.focus_request = Some(katana_ui_core::text_surface::TextSurfaceFocusRequest::new(
        katana_ui_core::text_surface::TextSurfaceFocusRequestToken::new("focus-loss"),
        false,
    ));
    let _ = root.synchronize_presentation(
        katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentation {
            text_state_id: None,
            text: presentation,
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
    );
    let frame = render(&context, &mut root, Vec::new());
    assert_floating_hidden(&frame);

    let mut escaped_root = focused_caret_root("floating-escape", false, false);
    let _ = render(&context, &mut escaped_root, Vec::new());
    let frame = render(
        &context,
        &mut escaped_root,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    assert_floating_hidden(&frame);
}
