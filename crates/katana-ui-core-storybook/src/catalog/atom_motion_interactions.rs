use super::{StoryCatalog, StoryExample};
use katana_ui_core::atom;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{RgbaActionValue, UiAction};
use katana_ui_core::render_model::{UiAnimationState, UiVisualRole};

const PROGRESS_PERCENT: u8 = 64;
const SLIDE_VALUE: &str = "42";
const COLOR_SWATCH_RED: u8 = 64;
const COLOR_SWATCH_GREEN: u8 = 128;
const COLOR_SWATCH_BLUE: u8 = 255;
const COLOR_SWATCH_ALPHA: u8 = 255;
const COLOR_SWATCH_HUE: u16 = 210;
const LOADING_PHASE: u16 = 2;
const SPINNER_PHASE: u16 = 3;

pub(super) fn loading_dots() -> StoryExample {
    let mut loading = atom::LoadingDots::new("Loading dots")
        .visual_role(UiVisualRole::Loading)
        .loading(true)
        .animation_state(UiAnimationState::Running);
    let target = loading.state_id().clone();
    let result = loading.apply_action(&UiAction::animation_tick(target, LOADING_PHASE));
    StoryCatalog::interactive_story("loading-dots", loading, result.callback_log)
}

pub(super) fn spinner() -> StoryExample {
    let mut spinner = atom::Spinner::new("Spinner")
        .accessibility_label("Loading")
        .visual_role(UiVisualRole::Loading)
        .loading(true);
    let target = spinner.state_id().clone();
    let result = spinner.apply_action(&UiAction::animation_tick(target, SPINNER_PHASE));
    StoryCatalog::interactive_story("spinner", spinner, result.callback_log)
}

pub(super) fn progress_bar() -> StoryExample {
    let mut progress = atom::ProgressBar::new("Progress bar").visual_role(UiVisualRole::Progress);
    let target = progress.state_id().clone();
    let result = progress.apply_action(&UiAction::progress_changed(target, true, PROGRESS_PERCENT));
    StoryCatalog::interactive_story("progress-bar", progress, result.callback_log)
}

pub(super) fn color_swatch() -> StoryExample {
    let mut color = atom::ColorSwatch::new("Color swatch").visual_role(UiVisualRole::Control);
    let target = color.state_id().clone();
    let result = color.apply_action(&UiAction::color_drag(
        target,
        RgbaActionValue::new(
            COLOR_SWATCH_RED,
            COLOR_SWATCH_GREEN,
            COLOR_SWATCH_BLUE,
            COLOR_SWATCH_ALPHA,
        ),
        COLOR_SWATCH_HUE,
        false,
    ));
    StoryCatalog::interactive_story("color-swatch", color, result.callback_log)
}

pub(super) fn toggle() -> StoryExample {
    let mut toggle = atom::Toggle::new("Toggle")
        .visual_role(UiVisualRole::Control)
        .selected(false);
    let target = toggle.state_id().clone();
    let result = toggle.apply_action(&UiAction::toggle_checked(target, true));
    StoryCatalog::interactive_story("toggle", toggle, result.callback_log)
}

pub(super) fn slide_control() -> StoryExample {
    let mut slide = atom::SlideControl::new("Slide control").visual_role(UiVisualRole::Control);
    let target = slide.state_id().clone();
    let result = slide.apply_action(&UiAction::slide_changed(target, SLIDE_VALUE));
    StoryCatalog::interactive_story("slide-control", slide, result.callback_log)
}
