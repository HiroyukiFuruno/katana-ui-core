mod molecule_basic;
mod molecule_heavy;
mod molecule_interaction;
mod molecule_workspace;

use super::StoryExample;

pub(super) fn examples() -> Vec<StoryExample> {
    let mut examples = Vec::new();
    examples.extend(molecule_basic::examples());
    examples.extend(molecule_heavy::examples());
    examples.extend(molecule_interaction::examples());
    examples.extend(molecule_workspace::examples());
    examples
}
