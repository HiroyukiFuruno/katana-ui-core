use super::button_options;
use super::render::StorybookRenderOptions;

const CHECKBOX_PAGE: &str = "checkbox";
const CHECKBOX_CHECKED_PRESET_INDEX: usize = 1;
const CHECKBOX_DISABLED_PRESET_INDEX: usize = 2;
const CHECKBOX_FOCUS_PRESET_INDEX: usize = 3;
const TOGGLE_PAGE: &str = "toggle";
const TOGGLE_ON_PRESET_INDEX: usize = 1;

pub(super) fn apply_preset_default_screen_state(options: &mut StorybookRenderOptions<'_>) {
    if options.selected_page == CHECKBOX_PAGE {
        apply_checkbox_preset_default_screen_state(options);
        return;
    }
    if options.selected_page == TOGGLE_PAGE {
        apply_toggle_preset_default_screen_state(options);
        return;
    }
    if !button_options::is_button_page(options.selected_page) {
        return;
    }
    if !options.screen_state.uses_default_button_options() {
        return;
    }
    options.screen_state.button_options =
        button_options::preset_button_options(options.preset_index);
}

fn apply_checkbox_preset_default_screen_state(options: &mut StorybookRenderOptions<'_>) {
    if !options.screen_state.uses_default_checkbox_state() {
        return;
    }
    if options.preset_index == CHECKBOX_CHECKED_PRESET_INDEX {
        options.screen_state.apply_checkbox_checked_preset_default();
    }
    if options.preset_index == CHECKBOX_DISABLED_PRESET_INDEX {
        options
            .screen_state
            .apply_checkbox_disabled_preset_default();
    }
    if options.preset_index == CHECKBOX_FOCUS_PRESET_INDEX {
        options.screen_state.apply_checkbox_focus_preset_default();
    }
}

fn apply_toggle_preset_default_screen_state(options: &mut StorybookRenderOptions<'_>) {
    if !options.screen_state.uses_default_toggle_state() {
        return;
    }
    if options.preset_index == TOGGLE_ON_PRESET_INDEX {
        options.screen_state.apply_toggle_checked_preset_default();
    }
}
