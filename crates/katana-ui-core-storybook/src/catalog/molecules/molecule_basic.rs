use super::super::{StoryCatalog, StoryExample};
#[path = "breadcrumb_story.rs"]
mod breadcrumb_story;
#[path = "form_field_story.rs"]
mod form_field_story;
#[path = "side_menu_story.rs"]
mod side_menu_story;
#[path = "tabs_story.rs"]
mod tabs_story;
#[path = "toolbar_story.rs"]
mod toolbar_story;
use super::molecule_virtualization;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, molecule};
use molecule::{
    AttachmentChipAction, AttachmentKind, AttachmentProgress, AttachmentStatus, ChipGroupAction,
    ChipGroupOverflow, EmptyStateAction, EmptyStateActionId, EmptyStateAlignment, EmptyStateSize,
    EmptyStateTone,
};

const CHIP_GROUP_AVAILABLE_WIDTH: u16 = 88;
const CHIP_GROUP_TRIGGER_WIDTH: u16 = 24;
const CHIP_GROUP_CHIP_WIDTH: u16 = 42;
const ATTACHMENT_UPLOAD_PROGRESS: u16 = 4_200;
const LIST_FOCUSED_INDEX: usize = 18;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        card_story(),
        list_story(),
        tabs_story::story(),
        toolbar_story::story(),
        attachment_chip_story(),
        chip_group_story(),
        empty_state_story(),
        form_field_story::story(),
        breadcrumb_story::story(),
        side_menu_story::story(),
    ]
}

fn list_story() -> StoryExample {
    let config = molecule_virtualization::fixed_config(
        molecule_virtualization::LIST_TOTAL_COUNT,
        Some(LIST_FOCUSED_INDEX),
    );
    let list = molecule::List::new("List")
        .child(atom::Text::new("Row 1"))
        .child(atom::Text::new("Row 2"))
        .child(atom::Badge::new(molecule_virtualization::compact_label(
            &config,
        )))
        .child(molecule::VirtualizedList::new(
            "List virtualization",
            config.clone(),
        ));
    StoryCatalog::interactive_story(
        "list",
        list,
        vec![molecule_virtualization::log(
            UiStateId::new("state:List:virtualization"),
            "list_virtualization_range",
            &config,
        )],
    )
}

fn attachment_chip_story() -> StoryExample {
    let mut attachment = molecule::AttachmentChip::new(AttachmentKind::Image, "screenshot.png")
        .progress(AttachmentProgress::from_basis_points(
            ATTACHMENT_UPLOAD_PROGRESS,
        ))
        .status(AttachmentStatus::Uploading);
    let target = attachment.chip().state_id().clone();
    let events = attachment.apply_action(AttachmentChipAction::TransitionStatus(
        AttachmentStatus::Error,
    ));
    let log = UiCallbackLog::new(
        target,
        "attachment_status",
        "status=uploading",
        format!("events={events:?}"),
    );
    StoryCatalog::interactive_story("attachment-chip", attachment, vec![log])
}

fn chip_group_story() -> StoryExample {
    let first = atom::Chip::new("lint").dismissible(true);
    let first_id = first.state_id().clone();
    let second = atom::Chip::new("format").dismissible(true);
    let third = atom::Chip::new("docs").dismissible(true);
    let mut group = molecule::ChipGroup::new("Chip group")
        .chip(first, CHIP_GROUP_CHIP_WIDTH)
        .chip(second, CHIP_GROUP_CHIP_WIDTH)
        .chip(third, CHIP_GROUP_CHIP_WIDTH)
        .wrap(false)
        .available_width(CHIP_GROUP_AVAILABLE_WIDTH)
        .overflow_trigger_width(CHIP_GROUP_TRIGGER_WIDTH)
        .overflow(ChipGroupOverflow::Menu)
        .reorder(true);
    let events = group.apply_action(ChipGroupAction::OpenOverflow);
    let log = UiCallbackLog::new(
        first_id,
        "chip_group_overflow",
        "hidden=0",
        format!("events={events:?}"),
    );
    StoryCatalog::interactive_story("chip-group", group, vec![log])
}

fn empty_state_story() -> StoryExample {
    let empty = molecule::EmptyState::new("No diagnostics")
        .body("日本語 mixed text is centered.")
        .tone(EmptyStateTone::Accent)
        .size(EmptyStateSize::Default)
        .alignment(EmptyStateAlignment::Center)
        .primary_action(EmptyStateAction::new("reload", "Reload"))
        .secondary_action(EmptyStateAction::new("docs", "Open docs"));
    let target = empty.state_id().clone();
    let primary = empty.apply_action(EmptyStateActionId::Primary);
    let secondary = empty.apply_action(EmptyStateActionId::Secondary);
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "empty_state_primary",
            "action=none",
            format!("event={primary:?}"),
        ),
        UiCallbackLog::new(
            target,
            "empty_state_secondary",
            "action=primary",
            format!("event={secondary:?}"),
        ),
    ];
    StoryCatalog::interactive_story("empty-state", empty, logs)
}

fn card_story() -> StoryExample {
    let mut card = molecule::Card::new("Card")
        .interactive(true)
        .header(atom::Text::new("Header"))
        .child(atom::Text::new("Body"))
        .footer(atom::Button::new("Open"));
    let target = card.state_id().clone();
    let result = card.apply_action(&UiAction::click(target));
    StoryCatalog::interactive_story("card", card, result.callback_log)
}
