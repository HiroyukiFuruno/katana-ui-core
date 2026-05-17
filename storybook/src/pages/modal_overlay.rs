use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, button, h_stack, label, scroll, v_stack};
use floem::window::WindowId;
use katana_ui_core::layout::modal::{Modal, ModalParentInteraction, ModalSize};
use katana_ui_core::theme::Theme;

use crate::modal_state::{
    FooterSample, ModalOpenSnapshot, ModalSettingAction, ModalStateSignals, apply_modal_setting,
    bool_label, modal_open_matches_action, modal_open_summary, modal_setting_button_label,
    open_snapshot_from_signals, parent_interaction_label, size_label, state_signals,
};

fn setting_button(action: ModalSettingAction, state: ModalStateSignals) -> impl IntoView {
    button(label(move || modal_setting_button_label(action, state))).action(move || {
        apply_modal_setting(action, state);
    })
}

fn setting_buttons(state: ModalStateSignals) -> (impl IntoView, impl IntoView, impl IntoView) {
    let size_buttons = h_stack((
        setting_button(ModalSettingAction::SizeSm, state),
        setting_button(ModalSettingAction::SizeLg, state),
        setting_button(ModalSettingAction::SizeCustom, state),
    ))
    .style(|style| style.gap(8.0).flex_wrap(floem::style::FlexWrap::Wrap));

    let behavior_buttons = h_stack((
        setting_button(ModalSettingAction::EscEnabled, state),
        setting_button(ModalSettingAction::EscDisabled, state),
        setting_button(ModalSettingAction::ParentBlock, state),
        setting_button(ModalSettingAction::ParentAllow, state),
    ))
    .style(|style| style.gap(8.0).flex_wrap(floem::style::FlexWrap::Wrap));

    let footer_buttons = h_stack((
        setting_button(ModalSettingAction::FooterConfirm, state),
        setting_button(ModalSettingAction::FooterForm, state),
        setting_button(ModalSettingAction::FooterDetail, state),
    ))
    .style(|style| style.gap(8.0).flex_wrap(floem::style::FlexWrap::Wrap));

    (size_buttons, behavior_buttons, footer_buttons)
}

fn schedule_requested_setting_replay(
    theme: Theme,
    state: ModalStateSignals,
    native_log: RwSignal<String>,
    focus_log: RwSignal<String>,
) {
    for action in ModalSettingAction::ALL {
        if !crate::interaction::requested(action.interaction()) {
            continue;
        }

        crate::interaction::mark_supported("modal", action.interaction());
        let replay_theme = theme.clone();
        crate::interaction::schedule_replay(move || {
            apply_modal_setting(action, state);
            let verify_log = native_log;
            if open_native_modal(
                replay_theme.clone(),
                state,
                native_log,
                focus_log,
                move |opened| {
                    if modal_open_matches_action(action, &opened) {
                        crate::interaction::mark_exercised(
                            "modal",
                            action.interaction(),
                            action.detail(),
                        );
                    } else {
                        verify_log.set(format!(
                            "Modal native window: setting mismatch {}",
                            modal_open_summary(&opened)
                        ));
                    }
                },
            ) {
                native_log.set("Modal native window: setting open requested".to_string());
            }
        });
    }
}

fn open_native_modal(
    theme: Theme,
    state: ModalStateSignals,
    native_log: RwSignal<String>,
    focus_log: RwSignal<String>,
    on_window_created: impl Fn(ModalOpenSnapshot) + 'static,
) -> bool {
    let open_log = native_log;
    let close_log = native_log;
    let focus_return_log = focus_log;
    let snapshot = open_snapshot_from_signals(state);
    let footer_body = snapshot.footer_body.clone();
    let modal = Modal::new()
        .open(true)
        .size(snapshot.size.clone())
        .title(snapshot.title.clone())
        .children(snapshot.body.clone())
        .footer(footer_body.clone())
        .parent_interaction(snapshot.parent_interaction.clone())
        .dismiss_on_backdrop(snapshot.dismiss_on_backdrop)
        .dismiss_on_esc(snapshot.dismiss_on_esc);
    let open_result = modal
        .on_open(move || {
            open_log.set(format!("Modal native window: created footer={footer_body}"));
            on_window_created(snapshot.clone());
        })
        .on_close(move || {
            close_log.set("Modal native window: on_close()".to_string());
        })
        .on_focus_return(move || {
            focus_return_log.set("Modal native window: on_focus_return()".to_string());
        })
        .open_window(theme);

    let opened = match open_result {
        Ok(opened) => opened,
        Err(error) => {
            native_log.set(format!("Modal native window error: {error}"));
            false
        }
    };

    if opened {
        native_log.set("Modal native window: open requested".to_string());
    }
    opened
}

fn open_modal_button(
    theme: Theme,
    state: ModalStateSignals,
    native_log: RwSignal<String>,
    focus_log: RwSignal<String>,
) -> impl IntoView {
    button(label(|| "別ウィンドウでModalを開く")).action(move || {
        let theme = theme.clone();
        let _ = open_native_modal(
            theme,
            state,
            native_log,
            focus_log,
            |_| {},
        );
    })
}

fn native_status(
    state: ModalStateSignals,
    log: RwSignal<String>,
    focus_log: RwSignal<String>,
) -> impl IntoView {
    v_stack((
        label(move || format!("title: {}", state.title.get())),
        label(move || {
            format!(
                "size={} footer={} esc={} parent={}",
                size_label(&state.size.get()),
                state.footer.get().label(),
                bool_label(state.dismiss_on_esc.get()),
                parent_interaction_label(&state.parent_interaction.get()),
            )
        }),
        label(move || format!("native log: {}", log.get())),
        label(move || format!("focus log: {}", focus_log.get())),
    ))
    .style(|style| style.gap(4.0))
}

pub fn modal_page(theme: Theme, _parent_window_id: Option<WindowId>) -> impl IntoView {
    let selected_size = create_rw_signal(ModalSize::Md);
    let selected_title = create_rw_signal("確認Modal".to_string());
    let selected_body = create_rw_signal("別ウィンドウとして開くModalです。".to_string());
    let selected_footer = create_rw_signal(FooterSample::Confirm);
    let dismiss_on_backdrop = create_rw_signal(true);
    let dismiss_on_esc = create_rw_signal(true);
    let parent_interaction = create_rw_signal(ModalParentInteraction::Block);
    let native_log = create_rw_signal("not opened".to_string());
    let native_focus_log = create_rw_signal("not returned".to_string());
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);

    if crate::interaction::requested("open") {
        crate::interaction::mark_supported("modal", "open");
        let replay_state = state_signals(
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
            parent_interaction,
        );
        let replay_theme = theme.clone();
        crate::interaction::schedule_replay(move || {
            if open_native_modal(
                replay_theme.clone(),
                replay_state,
                native_log,
                native_focus_log,
                |_| {
                    crate::interaction::mark_exercised("modal", "open", "native-window-created");
                },
            ) {
                native_log.set("Modal native window: open requested".to_string());
            }
        });
    }

    schedule_requested_setting_replay(
        theme.clone(),
        state_signals(
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
            parent_interaction,
        ),
        native_log,
        native_focus_log,
    );

    let state = state_signals(
        selected_size,
        selected_title,
        selected_body,
        selected_footer,
        dismiss_on_backdrop,
        dismiss_on_esc,
        parent_interaction,
    );
    let (size_buttons, behavior_buttons, footer_buttons) = setting_buttons(state_signals(
        selected_size,
        selected_title,
        selected_body,
        selected_footer,
        dismiss_on_backdrop,
        dismiss_on_esc,
        parent_interaction,
    ));
    let open_button = open_modal_button(
        theme.clone(),
        state_signals(
            selected_size,
            selected_title,
            selected_body,
            selected_footer,
            dismiss_on_backdrop,
            dismiss_on_esc,
            parent_interaction,
        ),
        native_log,
        native_focus_log,
    );
    scroll(
        v_stack((
            label(|| "Modal Samples").style(|style| style.font_size(16.0).margin_bottom(8.0)),
            label(|| "Live widget: ボタンを押すと別ウィンドウが開きます。")
                .style(|style| style.font_size(13.0)),
            open_button,
            native_status(state, native_log, native_focus_log),
            label(|| "Modal の設定").style(|style| style.font_size(13.0).margin_top(12.0)),
            size_buttons,
            behavior_buttons,
            footer_buttons,
        ))
        .style(move |style| style.gap(8.0).padding(16.0).background(bg).min_width_full()),
    )
    .style(|style| style.width_full().height_full())
}
