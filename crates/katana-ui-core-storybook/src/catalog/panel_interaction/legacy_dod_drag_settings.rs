use super::{BeforeAfterReport, SettingsMutationReport, TypedOptionMutationReport};
use crate::catalog::StoryExample;

const PAGE: &str = "drag-and-drop";
const MARKER: &str = "catalog-drag-and-drop";

pub(super) fn drag_and_drop_settings_mutations(
    examples: &[StoryExample],
) -> Vec<SettingsMutationReport> {
    let Some(example) = examples.iter().find(|it| it.page == PAGE) else {
        return Vec::new();
    };
    drag_settings()
        .iter()
        .map(|setting| setting.report(example))
        .collect()
}

fn drag_settings() -> [DragSetting; 3] {
    [
        DragSetting {
            name: "drag.accept_policy",
            value_type: "DropAcceptance",
            before: "Reject",
            after: "Accept(move, indicator=inside)",
        },
        DragSetting {
            name: "drag.autoscroll",
            value_type: "AutoScrollPolicy",
            before: "disabled",
            after: "edge=24 acceleration=linear",
        },
        DragSetting {
            name: "drag.keyboard_draggable",
            value_type: "bool",
            before: "false",
            after: "true",
        },
    ]
}

struct DragSetting {
    name: &'static str,
    value_type: &'static str,
    before: &'static str,
    after: &'static str,
}

impl DragSetting {
    fn report(&self, example: &StoryExample) -> SettingsMutationReport {
        SettingsMutationReport {
            page: PAGE.to_string(),
            ui_marker: MARKER.to_string(),
            action: format!("set_{}", self.name),
            event: "drag_and_drop_settings_changed".to_string(),
            target_state_id: example.tree.root().props().state_id.as_str().to_string(),
            option: TypedOptionMutationReport {
                name: self.name.to_string(),
                value_type: self.value_type.to_string(),
                before_value: self.before.to_string(),
                after_value: self.after.to_string(),
            },
            state: BeforeAfterReport {
                before: format!("{MARKER} option:{}={}", self.name, self.before),
                after: format!("{MARKER} option:{}={}", self.name, self.after),
            },
            preview: BeforeAfterReport {
                before: format!("{MARKER}:preview:{}={}", self.name, self.before),
                after: format!("{MARKER}:preview:{}={}", self.name, self.after),
            },
        }
    }
}
