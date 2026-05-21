use super::{
    ComponentAction, StoryCatalog, StoryExample, UiAction, UiCallbackLog, UiStateId, atom, molecule,
};

pub(super) fn modal_story() -> StoryExample {
    let mut modal = molecule::Modal::new("Modal")
        .open(true)
        .native_window_mode(true)
        .title("Preferences")
        .panel_size("medium")
        .footer("Cancel / Save")
        .escape_dismiss(true)
        .focus_return("trigger:open-modal")
        .dismiss_policy("parent_interaction=Block")
        .backdrop("native-window")
        .child(atom::Button::new(
            "native window preset=native window action=open_native_window",
        ))
        .child(atom::Text::new(
            "escape close option=escape_dismiss=true event=ModalEscaped",
        ))
        .child(atom::Text::new(
            "focus return state=modal -> trigger focused preset=focus return",
        ))
        .child(atom::Text::new(
            "parent block parent_interaction=Block event=ParentInteractionBlocked",
        ))
        .child(atom::Text::new("Body"))
        .child(atom::Button::new(
            "Close title footer size title=Preferences footer=Cancel / Save size=medium",
        ));
    let target = modal.state_id().clone();
    let result = modal.apply_action(&UiAction::modal_escape(target));
    StoryCatalog::interactive_story(
        "modal",
        modal,
        modal_logs(result.target, result.callback_log),
    )
}

pub(super) fn modal_overlay_story() -> StoryExample {
    let mut overlay = molecule::ModalOverlay::new("Modal overlay")
        .open(true)
        .backdrop("dimmer")
        .escape_dismiss(true)
        .focus_trap(true)
        .focus_return("trigger:open-overlay")
        .outside_click_dismiss(true)
        .dismiss_policy("backdrop=true escape=true dismiss_disabled=false")
        .child(
            molecule::Modal::new("Overlay dialog")
                .open(true)
                .title("Confirm")
                .panel_size("small")
                .footer("Cancel / Continue")
                .child(atom::Text::new("same_window_overlay=true")),
        )
        .child(atom::Text::new(
            "overlay dialog preset=overlay dialog same_window_overlay=true",
        ))
        .child(atom::Button::new(
            "backdrop close action=modal_backdrop_click backdrop_close=true",
        ))
        .child(atom::Text::new(
            "escape close action=modal_escape escape_close=true",
        ))
        .child(atom::Text::new(
            "focus trap option=focus_trap=true event=FocusTrapCycled",
        ))
        .child(atom::Text::new(
            "focus return state=overlay -> trigger focused preset=focus return",
        ))
        .child(atom::Text::new(
            "dismiss disabled preset=dismiss disabled backdrop=false",
        ));
    let target = overlay.state_id().clone();
    let result = overlay.apply_action(&UiAction::modal_backdrop_click(target));
    StoryCatalog::interactive_story(
        "modal-overlay",
        overlay,
        modal_overlay_logs(result.target, result.callback_log),
    )
}

fn modal_logs(target: UiStateId, mut handled_logs: Vec<UiCallbackLog>) -> Vec<UiCallbackLog> {
    let mut logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "modal_escape",
            "state=open native_window_mode=true",
            "state=closed event=ModalEscaped",
        ),
        UiCallbackLog::new(
            target.clone(),
            "modal_focus_return",
            "focus=modal.close_button",
            "focus=trigger:open-modal event=FocusReturned",
        ),
        UiCallbackLog::new(
            target.clone(),
            "modal_parent_block",
            "parent_interaction=Block parent_click=requested",
            "parent_interaction=Block blocked=true event=ParentInteractionBlocked",
        ),
        UiCallbackLog::new(
            target,
            "modal_native_window_open",
            "window=closed parent=storybook",
            "window=opened same_display=true frontmost=true event=NativeWindowOpened",
        ),
    ];
    logs.append(&mut handled_logs);
    logs
}

fn modal_overlay_logs(
    target: UiStateId,
    mut handled_logs: Vec<UiCallbackLog>,
) -> Vec<UiCallbackLog> {
    let mut logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "modal_backdrop_click",
            "state=open backdrop_close=true",
            "state=closed event=OverlayBackdropClosed",
        ),
        UiCallbackLog::new(
            target.clone(),
            "modal_escape",
            "state=open escape_close=true",
            "state=closed event=OverlayEscaped",
        ),
        UiCallbackLog::new(
            target.clone(),
            "modal_focus_trap",
            "focus=first-field tab=forward",
            "focus=primary-action event=FocusTrapCycled",
        ),
        UiCallbackLog::new(
            target.clone(),
            "modal_focus_return",
            "focus=overlay.primary-action",
            "focus=trigger:open-overlay event=FocusReturned",
        ),
        UiCallbackLog::new(
            target,
            "modal_dismiss_disabled",
            "dismiss_disabled=true backdrop_click=requested",
            "dismiss_disabled=true state=open event=DismissBlocked",
        ),
    ];
    logs.append(&mut handled_logs);
    logs
}
