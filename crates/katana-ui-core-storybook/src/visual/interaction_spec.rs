#[path = "interaction_spec_atoms.rs"]
mod interaction_spec_atoms;
#[path = "interaction_spec_layouts.rs"]
mod interaction_spec_layouts;
#[path = "interaction_spec_molecules.rs"]
mod interaction_spec_molecules;
#[path = "interaction_spec_runtime.rs"]
mod interaction_spec_runtime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct StorybookInteractionSpec {
    pub(super) action: &'static str,
    pub(super) event: &'static str,
    pub(super) option: &'static str,
    pub(super) after: &'static str,
    pub(super) state: &'static str,
}

impl StorybookInteractionSpec {
    pub(super) fn for_page(page: &str) -> Self {
        interaction_spec_atoms::for_page(page)
            .or_else(|| interaction_spec_molecules::for_page(page))
            .or_else(|| interaction_spec_runtime::for_page(page))
            .or_else(|| interaction_spec_layouts::for_page(page))
            .unwrap_or_else(|| {
                spec(
                    "component_action",
                    "component_event",
                    "option",
                    "changed",
                    "changed=true",
                )
            })
    }
}

pub(super) const fn spec(
    action: &'static str,
    event: &'static str,
    option: &'static str,
    after: &'static str,
    state: &'static str,
) -> StorybookInteractionSpec {
    StorybookInteractionSpec {
        action,
        event,
        option,
        after,
        state,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        interaction_spec_atoms, interaction_spec_layouts, interaction_spec_molecules,
        interaction_spec_runtime,
    };
    use crate::requirements::StoryRequirements;

    #[test]
    fn required_storybook_pages_have_explicit_interaction_specs() {
        for page in StoryRequirements::required_pages() {
            assert!(
                has_explicit_spec(page),
                "{page} must not fall back to generic interaction spec"
            );
        }
    }

    fn has_explicit_spec(page: &str) -> bool {
        interaction_spec_atoms::for_page(page).is_some()
            || interaction_spec_molecules::for_page(page).is_some()
            || interaction_spec_runtime::for_page(page).is_some()
            || interaction_spec_layouts::for_page(page).is_some()
    }
}
