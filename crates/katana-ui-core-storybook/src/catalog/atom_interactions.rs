use super::{StoryCatalog, StoryExample};
use katana_ui_core::atom;
use katana_ui_core::atom::{
    ChipAction, ChipKeyboardInput, ChipTone, ChipVariant, TextAreaAction, TextAreaCompositionPhase,
    TextAreaNewlineKey, TextAreaSubmitKey, TextAreaTabBehavior,
};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::interaction::UiCallbackLog;
use katana_ui_core::render_model::{UiDismissAction, UiSize, UiTone, UiVariant, UiVisualRole};

const TEXT_AREA_MIN_ROWS: u16 = 2;
const TEXT_AREA_MAX_ROWS: u16 = 4;

macro_rules! click_story {
    ($name:ident, $page:literal, $component:expr) => {
        pub(super) fn $name() -> StoryExample {
            let mut component = $component;
            let target = component.state_id().clone();
            let result = component.apply_action(&UiAction::click(target));
            StoryCatalog::interactive_story($page, component, result.callback_log)
        }
    };
}

macro_rules! button_story {
    ($name:ident, $page:literal, $component:expr) => {
        pub(super) fn $name() -> StoryExample {
            let mut component = $component;
            let target = component.state_id().clone();
            let result = component.apply_action(&UiAction::button_press(target));
            StoryCatalog::interactive_story($page, component, result.callback_log)
        }
    };
}

click_story!(
    text,
    "text",
    atom::Text::new("日本語 Text 🔷")
        .accessibility_label("Text")
        .visual_role(UiVisualRole::Content)
        .font_role("body")
);

click_story!(
    icon,
    "icon",
    atom::Icon::new("Icon")
        .accessibility_label("Icon")
        .visual_role(UiVisualRole::Icon)
        .variant(UiVariant::Icon)
);

pub(super) fn chip() -> StoryExample {
    let mut chip = atom::Chip::new("filter: 日本語")
        .leading_icon("filter")
        .tone(ChipTone::Accent)
        .variant(ChipVariant::Outline)
        .interactive(true)
        .selected(true)
        .dismissible(true)
        .focused(true);
    let target = chip.state_id().clone();
    let events = chip.apply_action(ChipAction::Keyboard(ChipKeyboardInput::Backspace));
    let log = UiCallbackLog::new(
        target,
        "chip_dismiss",
        "selected=true dismissed=false",
        format!("events={events:?}"),
    );
    StoryCatalog::interactive_story("chip", chip, vec![log])
}

button_story!(
    button,
    "button",
    atom::Button::new("Button")
        .focusable(true)
        .visual_role(UiVisualRole::Control)
        .variant(UiVariant::Filled)
        .tone(UiTone::Accent)
);

button_story!(
    text_button,
    "text-button",
    atom::TextButton::new("Text button")
        .visual_role(UiVisualRole::Control)
        .variant(UiVariant::Text)
);

button_story!(
    svg_button,
    "svg-button",
    atom::SvgButton::new("Svg button")
        .visual_role(UiVisualRole::Control)
        .variant(UiVariant::Icon)
);

button_story!(
    icon_text_button,
    "icon-text-button",
    atom::IconTextButton::new("Icon text")
        .visual_role(UiVisualRole::Control)
        .variant(UiVariant::IconText)
);

click_story!(
    key_cap,
    "key-cap",
    atom::KeyCap::new("⌘ K")
        .accessibility_label("Shortcut")
        .visual_role(UiVisualRole::Shortcut)
        .font_role("code")
        .variant(UiVariant::Outline)
);

pub(super) fn input() -> StoryExample {
    let mut input = atom::Input::new("Text input")
        .focusable(true)
        .placeholder("日本語 input")
        .visual_role(UiVisualRole::Input)
        .value("typed");
    let target = input.state_id().clone();
    let result = input.apply_action(&UiAction::input_value(target, "typed 日本語 🔷"));
    StoryCatalog::interactive_story("text-input", input, result.callback_log)
}

pub(super) fn text_area() -> StoryExample {
    let mut text_area = atom::TextArea::new("Text area")
        .value("English\n日本語 🔷")
        .placeholder("message")
        .min_rows(TEXT_AREA_MIN_ROWS)
        .max_rows(TEXT_AREA_MAX_ROWS)
        .auto_grow(true)
        .submit_key(TextAreaSubmitKey::Enter)
        .newline_key(TextAreaNewlineKey::ShiftEnter)
        .tab_behavior(TextAreaTabBehavior::MoveFocus);
    let target = text_area.state_id().clone();
    let typed = text_area.apply_text_area_action(TextAreaAction::Type("\nemoji 👩‍💻".to_string()));
    let ime = text_area.apply_text_area_action(TextAreaAction::composition(
        TextAreaCompositionPhase::Update,
        "かな",
        "かな".len(),
    ));
    let log = UiCallbackLog::new(
        target,
        "text_area_type",
        "rows=2 ime=none",
        format!("typed={} ime={}", typed.events.len(), ime.events.len()),
    );
    StoryCatalog::interactive_story("text-area", text_area, vec![log])
}

pub(super) fn checkbox() -> StoryExample {
    let mut checkbox = atom::Checkbox::new("Checkbox")
        .visual_role(UiVisualRole::Control)
        .checked(false);
    let target = checkbox.state_id().clone();
    let result = checkbox.apply_action(&UiAction::checkbox_checked(target, true));
    StoryCatalog::interactive_story("checkbox", checkbox, result.callback_log)
}

pub(super) fn radio() -> StoryExample {
    let mut radio = atom::Radio::new("Radio")
        .visual_role(UiVisualRole::Control)
        .selected(false);
    let target = radio.state_id().clone();
    let result = radio.apply_action(&UiAction::radio_selected(target));
    StoryCatalog::interactive_story("radio", radio, result.callback_log)
}

pub(super) fn badge() -> StoryExample {
    let mut badge = atom::Badge::new("Badge")
        .accessibility_label("Status badge")
        .visual_role(UiVisualRole::Status)
        .tone(UiTone::Success)
        .size(UiSize::Small)
        .dismiss_action(UiDismissAction::Available);
    let target = badge.state_id().clone();
    let result = badge.apply_action(&UiAction::dismiss(target));
    StoryCatalog::interactive_story("badge", badge, result.callback_log)
}
