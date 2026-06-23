use super::{
    ACCORDION_TREE_DEPTH, ComponentAction, StoryCatalog, StoryExample, UiAction, UiCallbackLog,
    atom, molecule,
};

pub(super) fn accordion_story() -> StoryExample {
    let mut accordion = molecule::Accordion::new("Accordion")
        .open(false)
        .controlled(true)
        .multiple(true)
        .indicator_position("leading")
        .trigger_area(molecule::DisclosureTriggerArea::IconAndText)
        .toggle_icon("<svg data-icon=\"chevron\"/>")
        .tree_mode(true)
        .depth(ACCORDION_TREE_DEPTH)
        .selected(true)
        .show_lines(true)
        .reduced_motion(true)
        .body_border(true)
        .child(atom::Button::new("closed expanded=false disabled=false"))
        .child(atom::Text::new("open expanded=true body_border=true"))
        .child(atom::Text::new("disabled disabled=true block toggle"))
        .child(atom::Text::new("controlled controlled=true request"))
        .child(atom::Text::new("multiple multiple=true group item"))
        .child(atom::Text::new(
            "tree mode tree_mode=true depth=2 selected=true",
        ))
        .child(atom::Text::new("reduced motion reduced_motion=true"))
        .child(atom::Text::new(
            "trigger areas trigger_area=IconAndText toggle_icon=chevron",
        ));
    let target = accordion.state_id().clone();
    let result = accordion.apply_action(&UiAction::accordion_toggle(target));
    StoryCatalog::interactive_story("accordion", accordion, accordion_logs(result.callback_log))
}

fn accordion_logs(mut logs: Vec<UiCallbackLog>) -> Vec<UiCallbackLog> {
    let target = katana_ui_core::render_model::UiStateId::new("state:Accordion:storybook");
    logs.push(UiCallbackLog::new(
        target.clone(),
        "accordion_trigger_area",
        "trigger_area=IconOnly",
        "trigger_area=IconAndText event=TriggerAreaChanged",
    ));
    logs.push(UiCallbackLog::new(
        target.clone(),
        "accordion_controlled_request",
        "controlled=true expanded=false",
        "controlled=true request=Expand event=ControlledExpandRequested",
    ));
    logs.push(UiCallbackLog::new(
        target.clone(),
        "accordion_group_toggle",
        "multiple=true expanded=item-a",
        "multiple=true expanded=item-a,item-b event=GroupToggle",
    ));
    logs.push(UiCallbackLog::new(
        target,
        "accordion_disabled_block",
        "disabled=true expanded=false",
        "disabled=true blocked=true event=DisabledToggleBlocked",
    ));
    logs
}
