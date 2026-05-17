use super::{StoryCatalog, StoryExample};
use katana_ui_core::atom;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::render_model::{UiSize, UiTone, UiVariant, UiVisualRole};

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        StoryCatalog::story(
            "text",
            atom::Text::new("日本語 Text 🔷")
                .accessibility_label("Text")
                .visual_role(UiVisualRole::Content)
                .font_role("body"),
        ),
        StoryCatalog::story(
            "icon",
            atom::Icon::new("Icon")
                .accessibility_label("Icon")
                .visual_role(UiVisualRole::Icon)
                .variant(UiVariant::Icon),
        ),
        interactive_button_story(),
        StoryCatalog::story(
            "text-button",
            atom::TextButton::new("Text button")
                .visual_role(UiVisualRole::Control)
                .variant(UiVariant::Text),
        ),
        StoryCatalog::story(
            "svg-button",
            atom::SvgButton::new("Svg button")
                .visual_role(UiVisualRole::Control)
                .variant(UiVariant::Icon),
        ),
        StoryCatalog::story(
            "icon-text-button",
            atom::IconTextButton::new("Icon text")
                .visual_role(UiVisualRole::Control)
                .variant(UiVariant::IconText),
        ),
        interactive_input_story(),
        interactive_checkbox_story(),
        StoryCatalog::story(
            "radio",
            atom::Radio::new("Radio")
                .visual_role(UiVisualRole::Control)
                .selected(true),
        ),
        StoryCatalog::story(
            "badge",
            atom::Badge::new("Badge")
                .accessibility_label("Status badge")
                .visual_role(UiVisualRole::Status)
                .tone(UiTone::Success)
                .size(UiSize::Small),
        ),
        StoryCatalog::story(
            "divider",
            atom::Divider::new("Divider").visual_role(UiVisualRole::Separator),
        ),
        StoryCatalog::story(
            "spacer",
            atom::Spacer::new("Spacer")
                .visual_role(UiVisualRole::Separator)
                .size(UiSize::Large),
        ),
        StoryCatalog::story(
            "key-cap",
            atom::KeyCap::new("⌘ K")
                .accessibility_label("Shortcut")
                .visual_role(UiVisualRole::Shortcut)
                .font_role("code")
                .variant(UiVariant::Outline),
        ),
        StoryCatalog::story(
            "loading-dots",
            atom::LoadingDots::new("Loading dots")
                .visual_role(UiVisualRole::Loading)
                .loading(true),
        ),
        StoryCatalog::story(
            "spinner",
            atom::Spinner::new("Spinner")
                .accessibility_label("Loading")
                .visual_role(UiVisualRole::Loading)
                .loading(true),
        ),
        StoryCatalog::story(
            "progress-bar",
            atom::ProgressBar::new("Progress bar")
                .visual_role(UiVisualRole::Progress)
                .progress(true, 64),
        ),
        StoryCatalog::story(
            "color-swatch",
            atom::ColorSwatch::new("Color swatch")
                .visual_role(UiVisualRole::Status)
                .value("rgba(64, 128, 255, 1)"),
        ),
        interactive_toggle_story(),
        StoryCatalog::story(
            "slide-control",
            atom::SlideControl::new("Slide control")
                .visual_role(UiVisualRole::Progress)
                .value("42"),
        ),
    ]
}

fn interactive_button_story() -> StoryExample {
    let mut button = atom::Button::new("Button")
        .focusable(true)
        .visual_role(UiVisualRole::Control)
        .variant(UiVariant::Filled)
        .tone(UiTone::Accent);
    let target = button.state_id().clone();
    let result = button.apply_action(&UiAction::button_press(target));
    StoryCatalog::interactive_story("button", button, result.callback_log)
}

fn interactive_input_story() -> StoryExample {
    let mut input = atom::Input::new("Text input")
        .focusable(true)
        .placeholder("日本語 input")
        .visual_role(UiVisualRole::Input)
        .value("typed");
    let target = input.state_id().clone();
    let result = input.apply_action(&UiAction::input_value(target, "typed 日本語 🔷"));
    StoryCatalog::interactive_story("text-input", input, result.callback_log)
}

fn interactive_checkbox_story() -> StoryExample {
    let mut checkbox = atom::Checkbox::new("Checkbox")
        .visual_role(UiVisualRole::Control)
        .checked(false);
    let target = checkbox.state_id().clone();
    let result = checkbox.apply_action(&UiAction::checkbox_checked(target, true));
    StoryCatalog::interactive_story("checkbox", checkbox.checked(true), result.callback_log)
}

fn interactive_toggle_story() -> StoryExample {
    let mut toggle = atom::Toggle::new("Toggle")
        .visual_role(UiVisualRole::Control)
        .selected(false);
    let target = toggle.state_id().clone();
    let result = toggle.apply_action(&UiAction::toggle_checked(target, true));
    StoryCatalog::interactive_story("toggle", toggle.selected(true), result.callback_log)
}
