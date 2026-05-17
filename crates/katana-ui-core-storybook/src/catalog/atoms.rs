use super::{StoryCatalog, StoryExample};
use katana_ui_core::atom;
use katana_ui_core::component::Component;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        StoryCatalog::story("text", atom::Text::new("Text").accessibility_label("Text")),
        StoryCatalog::story("icon", atom::Icon::new("Icon").accessibility_label("Icon")),
        StoryCatalog::story(
            "button",
            atom::Button::new("Button").focusable(true).class("control"),
        ),
        StoryCatalog::story("text-button", atom::TextButton::new("Text button")),
        StoryCatalog::story("svg-button", atom::SvgButton::new("Svg button")),
        StoryCatalog::story("icon-text-button", atom::IconTextButton::new("Icon text")),
        StoryCatalog::story(
            "text-input",
            atom::Input::new("Text input")
                .focusable(true)
                .value("typed")
                .class("field"),
        ),
        StoryCatalog::story("checkbox", atom::Checkbox::new("Checkbox")),
        StoryCatalog::story("radio", atom::Radio::new("Radio")),
        StoryCatalog::story(
            "badge",
            atom::Badge::new("Badge").accessibility_label("Status badge"),
        ),
        StoryCatalog::story("divider", atom::Divider::new("Divider")),
        StoryCatalog::story("spacer", atom::Spacer::new("Spacer")),
        StoryCatalog::story(
            "key-cap",
            atom::KeyCap::new("Key cap").accessibility_label("Shortcut"),
        ),
        StoryCatalog::story("loading-dots", atom::LoadingDots::new("Loading dots")),
        StoryCatalog::story(
            "spinner",
            atom::Spinner::new("Spinner").accessibility_label("Loading"),
        ),
        StoryCatalog::story("progress-bar", atom::ProgressBar::new("Progress bar")),
        StoryCatalog::story(
            "color-swatch",
            atom::ColorSwatch::new("Color swatch").value("rgba(64, 128, 255, 1)"),
        ),
        StoryCatalog::story("toggle", atom::Toggle::new("Toggle").selected(true)),
        StoryCatalog::story("slide-control", atom::SlideControl::new("Slide control")),
    ]
}
