use katana_ui_core::atom::{Chip, ChipAction, ChipEvent, ChipKeyboardInput, ChipTone, ChipVariant};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{
    AttachmentChip, AttachmentChipAction as AAction, AttachmentChipEvent as AEvent, AttachmentKind,
    AttachmentMeta, AttachmentProgress, AttachmentStatus as AStatus, AttachmentThumbnail,
    ChipGroup, ChipGroupAction, ChipGroupEvent, ChipGroupFocusTarget, ChipGroupOverflow,
};
use katana_ui_core::render_model::{UiNode, UiNodeKind, UiStateId, UiTone};

#[test]
fn chip_keeps_typed_options_and_keyboard_dismisses() {
    let mut backspace_chip = Chip::new("Filter")
        .leading_icon("filter")
        .tone(ChipTone::Danger)
        .variant(ChipVariant::Outline)
        .dismissible(true)
        .accessibility_label("Remove filter")
        .focused(true);
    let backspace_id = backspace_chip.state_id().clone();
    let mut delete_chip = Chip::new("Delete").dismissible(true).focused(true);
    let delete_id = delete_chip.state_id().clone();

    assert_eq!(Some("filter"), backspace_chip.leading_icon_value());
    assert_eq!("chip.outline.danger", backspace_chip.theme_token_key());
    assert_eq!(
        vec![ChipEvent::ChipDismissed { id: backspace_id }],
        backspace_chip.apply_action(ChipAction::Keyboard(ChipKeyboardInput::Backspace))
    );
    assert_eq!(
        vec![ChipEvent::ChipDismissed { id: delete_id }],
        delete_chip.apply_action(ChipAction::Keyboard(ChipKeyboardInput::Delete))
    );
}

#[test]
fn disabled_chip_suppresses_press_and_dismiss() {
    let mut chip = Chip::new("Disabled")
        .interactive(true)
        .dismissible(true)
        .disabled(true)
        .focused(true);

    let blocked = [
        chip.apply_action(ChipAction::Press).is_empty(),
        chip.apply_action(ChipAction::Dismiss).is_empty(),
        chip.apply_action(ChipAction::Keyboard(ChipKeyboardInput::Delete))
            .is_empty(),
    ];
    assert_eq!([true, true, true], blocked);
}

#[test]
fn chip_action_contract_covers_pointer_focus_and_component_dispatch() {
    let mut chip = Chip::new("Interactive").interactive(true).dismissible(true);
    let id = chip.state_id().clone();

    assert_eq!(
        vec![ChipEvent::ChipPressed { id: id.clone() }],
        chip.apply_action(ChipAction::Press)
    );
    assert_eq!(
        vec![ChipEvent::Focus { id: id.clone() }],
        chip.apply_action(ChipAction::Focus)
    );
    assert!(chip.apply_action(ChipAction::Focus).is_empty());
    assert!(
        chip.apply_action(ChipAction::Keyboard(ChipKeyboardInput::Other(
            "Escape".to_string(),
        )))
        .is_empty()
    );
    assert_eq!(
        vec![ChipEvent::Blur { id: id.clone() }],
        chip.apply_action(ChipAction::Blur)
    );
    assert!(chip.apply_action(ChipAction::Blur).is_empty());
    assert_eq!(
        vec![ChipEvent::ChipDismissed { id: id.clone() }],
        chip.apply_action(ChipAction::Dismiss)
    );
    assert_eq!(4, chip.callback_log().len());

    let wrong_target = UiAction::press(UiStateId::new("other"));
    assert!(!ComponentAction::apply_action(&mut chip, &wrong_target).handled);
    let press = UiAction::press(id.clone());
    assert!(ComponentAction::apply_action(&mut chip, &press).handled);
    let focus = UiAction::focus(id.clone());
    assert!(ComponentAction::apply_action(&mut chip, &focus).handled);
    let blur = UiAction::blur(id.clone());
    assert!(ComponentAction::apply_action(&mut chip, &blur).handled);
    let unsupported = UiAction::set_value(id, "ignored");
    assert!(!ComponentAction::apply_action(&mut chip, &unsupported).handled);

    let mut dismissible = Chip::new("Dismiss").dismissible(true);
    let dismiss = UiAction::dismiss(dismissible.state_id().clone());
    assert!(ComponentAction::apply_action(&mut dismissible, &dismiss).handled);
}

#[test]
fn chip_builders_and_render_conversions_cover_all_visual_variants() {
    let accent = Chip::new("Accent")
        .trailing_icon("close")
        .tone(ChipTone::Accent)
        .variant(ChipVariant::Solid)
        .size(katana_ui_core::atom::ChipSize::Large)
        .interactive(true)
        .selected(true)
        .accessibility_label("Accent chip")
        .focused(true);
    assert_eq!("Accent", accent.label());
    assert_eq!(Some("close"), accent.trailing_icon_value());
    assert!(accent.interactive_value());
    assert!(accent.selected_value());
    assert!(accent.focused_value());
    assert_eq!("Accent chip", accent.accessibility_label_value());
    let accent_node = UiNode::from(accent);
    assert_eq!(UiTone::Accent, accent_node.props().tone);

    let muted_ghost = UiNode::from(
        Chip::new("Muted")
            .tone(ChipTone::Muted)
            .variant(ChipVariant::Ghost),
    );
    assert_eq!(UiTone::Neutral, muted_ghost.props().tone);
    assert_eq!(
        katana_ui_core::render_model::UiVariant::Text,
        muted_ghost.props().variant
    );

    let success_outline = UiNode::from(
        Chip::new("Success")
            .tone(ChipTone::Success)
            .variant(ChipVariant::Outline),
    );
    assert_eq!(UiTone::Success, success_outline.props().tone);
    assert_eq!(
        katana_ui_core::render_model::UiVariant::Outline,
        success_outline.props().variant
    );
}

#[test]
fn chip_tone_and_variant_token_names_are_total() {
    assert_eq!(
        ["neutral", "accent", "success", "warning", "danger", "muted"],
        [
            ChipTone::Neutral,
            ChipTone::Accent,
            ChipTone::Success,
            ChipTone::Warning,
            ChipTone::Danger,
            ChipTone::Muted,
        ]
        .map(ChipTone::token_name)
    );
    assert_eq!(
        ["solid", "soft", "outline", "ghost"],
        [
            ChipVariant::Solid,
            ChipVariant::Soft,
            ChipVariant::Outline,
            ChipVariant::Ghost,
        ]
        .map(ChipVariant::token_name)
    );
}

#[test]
fn attachment_chip_emits_status_transitions_and_retry_reset() {
    let mut attachment = AttachmentChip::new(AttachmentKind::File, "report.pdf")
        .progress(AttachmentProgress::from_basis_points(4_200));
    let id = attachment.chip().state_id().clone();

    let uploading = attachment.apply_action(AAction::TransitionStatus(AStatus::Uploading));
    let _ = attachment.apply_action(AAction::TransitionStatus(AStatus::Ready));
    let _ = attachment.apply_action(AAction::TransitionStatus(AStatus::Error));
    let retry = attachment.apply_action(AAction::Retry);

    assert_eq!(
        vec![status_changed(&id, AStatus::Pending, AStatus::Uploading)],
        uploading
    );
    assert_eq!(AStatus::Pending, attachment.status_value());
    assert_eq!(
        vec![
            AEvent::Retry { id: id.clone() },
            status_changed(&id, AStatus::Error, AStatus::Pending),
        ],
        retry
    );
    assert_eq!(None, attachment.progress_value());
    assert_eq!(5, attachment.callback_log().len());
}

#[test]
fn attachment_chip_exposes_kind_metadata_preview_and_error_rendering() {
    let kinds = [
        (AttachmentKind::File, "file"),
        (AttachmentKind::Image, "image"),
        (AttachmentKind::Url, "link"),
        (AttachmentKind::Paste, "clipboard"),
        (AttachmentKind::Resource, "resource"),
    ];
    for (kind, icon) in kinds {
        assert_eq!(icon, kind.default_icon());
    }

    let mut attachment = AttachmentChip::new(AttachmentKind::Image, "preview.png")
        .meta(AttachmentMeta::new("24 KB", "image/png", "screenshot"))
        .thumbnail(AttachmentThumbnail::new("memory://preview", 16, 9))
        .progress(AttachmentProgress::from_basis_points(12_000))
        .status(AStatus::Error)
        .retry_action_label("Upload again");
    let id = attachment.chip().state_id().clone();

    assert_eq!(AttachmentKind::Image, attachment.kind());
    let progress = attachment.progress_value();
    assert!(progress.is_some(), "progress");
    let Some(progress) = progress else {
        return;
    };
    assert_eq!(100, progress.percent());
    assert_eq!(
        10_000,
        AttachmentProgress::from_basis_points(12_000).basis_points()
    );
    assert!(attachment.retry_action_visible());
    let retry_button = attachment.retry_button().map(UiNode::from);
    assert_eq!(
        Some("Upload again"),
        retry_button
            .as_ref()
            .map(|button| button.props().label.as_str())
    );
    assert_eq!(
        "chip.soft.danger",
        attachment.effective_chip().theme_token_key()
    );
    assert_eq!(
        vec![AEvent::Opened { id: id.clone() }],
        attachment.apply_action(AAction::OpenPreview)
    );
    assert_eq!(
        vec![AEvent::Dismissed { id }],
        attachment.apply_action(AAction::Dismiss)
    );

    let node = UiNode::from(attachment);
    assert_eq!(UiNodeKind::AttachmentChip, node.kind());
    assert_eq!(2, node.children().len());
    assert_eq!(UiNodeKind::Button, node.children()[1].kind());
}

#[test]
fn attachment_chip_ignores_redundant_status_and_retry_outside_error() {
    let mut attachment = AttachmentChip::new(AttachmentKind::Url, "https://example.test");

    assert!(attachment.apply_action(AAction::Retry).is_empty());
    assert!(
        attachment
            .apply_action(AAction::TransitionStatus(AStatus::Pending))
            .is_empty()
    );

    let node = UiNode::from(attachment);
    assert_eq!(1, node.children().len());
    assert_eq!(UiTone::Neutral, node.children()[0].props().tone);
}

#[test]
fn chip_group_menu_overflow_calculates_hidden_chips() {
    let first = Chip::new("first");
    let second = Chip::new("second");
    let third = Chip::new("third");
    let second_id = second.state_id().clone();
    let third_id = third.state_id().clone();
    let mut group = ChipGroup::new("filters")
        .chip(first, 40)
        .chip(second, 40)
        .chip(third, 40)
        .gap(5)
        .available_width(95)
        .overflow_trigger_width(20)
        .wrap(false)
        .overflow(ChipGroupOverflow::Menu);

    let layout = group.layout();
    let events = group.apply_action(ChipGroupAction::OpenOverflow);
    let menu = group.overflow_menu();

    assert_eq!(
        [second_id.clone(), third_id.clone()],
        layout.hidden_chip_ids()
    );
    assert!(menu.as_ref().is_some_and(|it| it.children().len() == 2));
    assert_eq!(
        vec![ChipGroupEvent::OverflowOpened {
            hidden_chip_ids: vec![second_id, third_id],
        }],
        events
    );
}

#[test]
fn chip_group_scroll_state_and_reorder_are_explicit() {
    let first = Chip::new("first");
    let first_id = first.state_id().clone();
    let second = Chip::new("second");
    let second_id = second.state_id().clone();
    let mut static_group = ChipGroup::new("static")
        .chip(first.clone(), 40)
        .chip(second.clone(), 40);
    let mut group = ChipGroup::new("filters")
        .chip(first, 40)
        .chip(second, 40)
        .overflow(ChipGroupOverflow::ScrollHorizontal)
        .reorder(true);

    assert!(
        static_group
            .apply_action(ChipGroupAction::Reorder { from: 0, to: 1 })
            .is_empty()
    );
    assert_eq!(
        vec![ChipGroupEvent::Scrolled { offset: 24 }],
        group.apply_action(ChipGroupAction::ScrollHorizontal { offset: 24 })
    );
    assert_eq!(24, group.layout().scroll_offset());
    assert_eq!(
        vec![ChipGroupEvent::ChipReordered {
            chip_id: first_id,
            from: 0,
            to: 1,
        }],
        group.apply_action(ChipGroupAction::Reorder { from: 0, to: 1 })
    );
    assert_eq!(second_id, group.chips()[0].chip().state_id().clone());
}

#[test]
fn chip_group_keyboard_dismiss_moves_focus_to_previous_chip() {
    let first = Chip::new("first");
    let first_id = first.state_id().clone();
    let second = Chip::new("second").dismissible(true).focused(true);
    let second_id = second.state_id().clone();
    let mut group = ChipGroup::new("filters").chip(first, 40).chip(second, 40);

    let events = group.dismiss_focused_with_keyboard(ChipKeyboardInput::Backspace);

    assert_eq!(
        vec![ChipGroupEvent::ChipDismissed {
            chip_id: second_id,
            focus_target: ChipGroupFocusTarget::Chip(first_id),
        }],
        events
    );
    assert!(group.chips()[0].chip().focused_value());
}

fn status_changed(id: &UiStateId, previous: AStatus, current: AStatus) -> AEvent {
    AEvent::StatusChanged {
        id: id.clone(),
        previous,
        current,
    }
}
