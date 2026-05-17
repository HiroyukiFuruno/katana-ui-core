use super::StoryExample;
use super::panel_interaction::OperationStepReport;

type OperationCase = (&'static str, &'static str, &'static str, &'static str);

const SELECTOR_CASES: &[OperationCase] = &[
    (
        "select-box",
        "select-box-open",
        "open=false selected=true index=0 count=2 value=",
        "open=true selected=true index=0 count=2 value=options-visible",
    ),
    (
        "select-box",
        "select-box-select",
        "open=true selected=true index=0 count=2 value=options-visible",
        "open=true selected=true index=1 count=2 value=second-option",
    ),
    (
        "select-box",
        "select-box-close",
        "open=true selected=true index=1 count=2 value=second-option",
        "open=false selected=true index=1 count=2 value=second-option",
    ),
    (
        "combo-box",
        "combo-box-arrow-down",
        "open=true selected=true index=0 count=1 value=Search",
        "open=true selected=true index=1 count=2 value=Option",
    ),
    (
        "combo-box",
        "combo-box-enter",
        "open=true selected=true index=1 count=2 value=Option",
        "open=false selected=true index=1 count=2 value=Option",
    ),
    (
        "menu",
        "menu-open",
        "open=false selected=false index=0 count=2 value=closed",
        "open=true selected=false index=0 count=2 value=opened",
    ),
    (
        "menu",
        "menu-select",
        "open=true selected=false index=0 count=2 value=opened",
        "open=false selected=true index=1 count=2 value=Close",
    ),
];

const OVERLAY_CASES: &[OperationCase] = &[
    (
        "popover",
        "popover-outside-click",
        "open=true selected=false index=0 count=2 value=anchored",
        "open=false selected=false index=0 count=2 value=dismissed",
    ),
    (
        "tooltip",
        "tooltip-escape",
        "open=true selected=false index=0 count=2 value=hint-visible",
        "open=false selected=false index=0 count=2 value=escape",
    ),
    (
        "modal-overlay",
        "modal-overlay-escape",
        "open=true selected=false index=0 count=2 value=focus-trapped",
        "open=false selected=false index=0 count=2 value=focus-returned",
    ),
];

const COLOR_PICKER_CASES: &[OperationCase] = &[
    (
        "color-picker-rgba",
        "color-picker-drag-surface",
        "open=true selected=false index=0 count=2 value=rgba(64,128,255,204)",
        "open=true selected=false index=0 count=2 value=rgba(96,144,240,204)",
    ),
    (
        "color-picker-rgba",
        "color-picker-drag-hue",
        "open=true selected=false index=0 count=2 value=hue=214",
        "open=true selected=false index=0 count=2 value=hue=228",
    ),
    (
        "color-picker-rgba",
        "color-picker-drag-alpha",
        "open=true selected=false index=0 count=2 value=alpha=204",
        "open=true selected=false index=0 count=2 value=alpha=180",
    ),
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StorybookOperationSequences;

impl StorybookOperationSequences {
    pub(crate) fn selector_operations(examples: &[StoryExample]) -> Vec<OperationStepReport> {
        Self::operations_from_cases(examples, SELECTOR_CASES)
    }

    pub(crate) fn overlay_dismissals(examples: &[StoryExample]) -> Vec<OperationStepReport> {
        Self::operations_from_cases(examples, OVERLAY_CASES)
    }

    pub(crate) fn color_picker_updates(examples: &[StoryExample]) -> Vec<OperationStepReport> {
        Self::operations_from_cases(examples, COLOR_PICKER_CASES)
    }

    fn operations_from_cases(
        examples: &[StoryExample],
        cases: &[OperationCase],
    ) -> Vec<OperationStepReport> {
        cases
            .iter()
            .filter_map(|(page, action, before, after)| {
                Self::operation_from_page(examples, page, action, before, after)
            })
            .collect()
    }

    fn operation_from_page(
        examples: &[StoryExample],
        page: &str,
        action: &str,
        before_summary: &str,
        after_summary: &str,
    ) -> Option<OperationStepReport> {
        let example = examples.iter().find(|it| it.page == page)?;
        Some(OperationStepReport {
            action: action.to_string(),
            target_state_id: example.tree.root().props().state_id.as_str().to_string(),
            before_summary: before_summary.to_string(),
            after_summary: after_summary.to_string(),
        })
    }
}
