mod molecule_app_primitives;
mod molecule_basic;
mod molecule_color_diff;
mod molecule_command_search;
mod molecule_context_menu;
mod molecule_heavy;
mod molecule_interaction;
mod molecule_platform_primitives;
mod molecule_runtime_primitives;
mod molecule_virtualization;
mod molecule_workspace;

use super::StoryExample;

pub(super) fn examples() -> Vec<StoryExample> {
    let mut examples = Vec::new();
    examples.extend(molecule_basic::examples());
    examples.extend(molecule_app_primitives::examples());
    examples.extend(molecule_color_diff::examples());
    examples.extend(molecule_command_search::examples());
    examples.extend(molecule_context_menu::examples());
    examples.extend(molecule_heavy::examples());
    examples.extend(molecule_interaction::examples());
    examples.extend(molecule_platform_primitives::examples());
    examples.extend(molecule_runtime_primitives::examples());
    examples.extend(molecule_workspace::examples());
    examples
}
