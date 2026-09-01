mod body_ime_search;
mod context_menu;
mod context_menu_overflow;
mod floating_dropdown;

pub(crate) fn actual_egui_text_command_surface_composes_full_interaction_sequence()
-> Result<(), Box<dyn std::error::Error>> {
    body_ime_search::run()
}

pub(crate) fn actual_egui_text_command_surface_escapes_floating_toolbar_with_raw_input()
-> Result<(), Box<dyn std::error::Error>> {
    floating_dropdown::run()
}

pub(crate) fn actual_egui_text_command_surface_activates_last_floating_dropdown_item()
-> Result<(), Box<dyn std::error::Error>> {
    floating_dropdown::last_item_run()
}

pub(crate) fn actual_egui_text_command_surface_owns_context_menu_from_actual_input()
-> Result<(), Box<dyn std::error::Error>> {
    context_menu::run()
}

pub(crate) fn actual_egui_text_command_surface_scrolls_context_menu_overflow_from_actual_input()
-> Result<(), Box<dyn std::error::Error>> {
    context_menu_overflow::run()
}
