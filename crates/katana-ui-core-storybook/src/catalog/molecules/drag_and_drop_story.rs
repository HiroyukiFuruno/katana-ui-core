use crate::catalog::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, molecule};

const DRAG_AND_DROP_PRESET_COUNT: usize = 5;

pub(super) fn story() -> StoryExample {
    let mut harness = DragAndDropHarness::new();
    let logs = drag_and_drop_presets()
        .iter()
        .flat_map(|preset| harness.apply_preset(preset))
        .collect();
    let root = drag_and_drop_presets()
        .iter()
        .fold(molecule::List::new("DragAndDrop"), |root, preset| {
            root.child(atom::Badge::new(preset.preview_label()))
        });
    StoryCatalog::interactive_story("drag-and-drop", root, logs)
}

struct DragAndDropHarness {
    source: atom::Badge,
    target: atom::Badge,
}

impl DragAndDropHarness {
    fn new() -> Self {
        Self {
            source: atom::Badge::new("drag source"),
            target: atom::Badge::new("drop target"),
        }
    }

    fn apply_preset(&mut self, preset: &DragAndDropPreset) -> Vec<UiCallbackLog> {
        let mut logs = Vec::new();
        logs.extend(self.apply_action(UiAction::dragging(self.source.state_id().clone(), true)));
        logs.push(preset.event_log(
            self.source.state_id(),
            "drag_source_started",
            DragAndDropEvent::DragStart,
        ));
        logs.extend(self.apply_action(UiAction::hover(
            self.target.state_id().clone(),
            preset.target_enabled,
        )));
        logs.push(preset.event_log(
            self.target.state_id(),
            "drop_target_entered",
            DragAndDropEvent::DragEnter,
        ));
        logs.push(preset.commit_or_cancel_log(self.target.state_id()));
        logs.extend(self.apply_action(UiAction::dragging(self.source.state_id().clone(), false)));
        logs
    }

    fn apply_action(&mut self, action: UiAction) -> Vec<UiCallbackLog> {
        if action.target() == self.source.state_id() {
            return self.source.apply_action(&action).callback_log;
        }
        self.target.apply_action(&action).callback_log
    }
}

fn drag_and_drop_presets() -> [DragAndDropPreset; DRAG_AND_DROP_PRESET_COUNT] {
    [
        DragAndDropPreset {
            name: "reorder list",
            payload: "katana-ui-core/list-row:item-02",
            target: "list:item-04",
            policy: DropPolicy::MoveAfter,
            autoscroll: "edge=24 speed=medium",
            keyboard_draggable: true,
            target_enabled: true,
            outcome: DragAndDropOutcome::Drop("reorder_list_drop"),
        },
        DragAndDropPreset {
            name: "file drop",
            payload: "os/file-list:3 files",
            target: "drop-zone:imports",
            policy: DropPolicy::CopyOutline,
            autoscroll: "off",
            keyboard_draggable: false,
            target_enabled: true,
            outcome: DragAndDropOutcome::Drop("file_drop_accept"),
        },
        DragAndDropPreset {
            name: "tab reorder",
            payload: molecule::CLOSEABLE_TAB_DRAG_TAG,
            target: "tab:settings",
            policy: DropPolicy::MoveBefore,
            autoscroll: "edge=16 speed=slow",
            keyboard_draggable: true,
            target_enabled: true,
            outcome: DragAndDropOutcome::Drop("tab_reorder_drop"),
        },
        DragAndDropPreset {
            name: "attachment drop",
            payload: "consumer/chat-attachment:image.png",
            target: "composer:attachments",
            policy: DropPolicy::CopyInside,
            autoscroll: "edge=32 speed=fast",
            keyboard_draggable: false,
            target_enabled: true,
            outcome: DragAndDropOutcome::Drop("attachment_drop_accept"),
        },
        DragAndDropPreset {
            name: "keyboard drag",
            payload: "katana-ui-core/list-row:item-01",
            target: "list:item-03",
            policy: DropPolicy::MoveAfter,
            autoscroll: "edge=24 speed=keyboard",
            keyboard_draggable: true,
            target_enabled: false,
            outcome: DragAndDropOutcome::Cancel("keyboard_drag_cancel"),
        },
    ]
}

struct DragAndDropPreset {
    name: &'static str,
    payload: &'static str,
    target: &'static str,
    policy: DropPolicy,
    autoscroll: &'static str,
    keyboard_draggable: bool,
    target_enabled: bool,
    outcome: DragAndDropOutcome,
}

impl DragAndDropPreset {
    fn preview_label(&self) -> String {
        format!(
            "preset={} payload={} target={} accept={} autoscroll={} keyboard_draggable={} target_enabled={} events={}",
            self.name,
            self.payload,
            self.target,
            self.policy.label(),
            self.autoscroll,
            self.keyboard_draggable,
            self.target_enabled,
            self.outcome.event_sequence()
        )
    }

    fn event_log(
        &self,
        target: &UiStateId,
        action: &'static str,
        event: DragAndDropEvent,
    ) -> UiCallbackLog {
        log(
            target,
            action,
            self.before_state(),
            format!(
                "event={} preset={} payload={} target={} target_enabled={} accept={}",
                event.name(),
                self.name,
                self.payload,
                self.target,
                self.target_enabled,
                self.policy.label()
            ),
        )
    }

    fn commit_or_cancel_log(&self, target: &UiStateId) -> UiCallbackLog {
        log(
            target,
            self.outcome.action(),
            self.before_state(),
            format!(
                "event={} events={} preset={} payload={} target={} accept={} autoscroll={} keyboard_draggable={} target_enabled={} committed={}",
                self.outcome.final_event().name(),
                self.outcome.event_sequence(),
                self.name,
                self.payload,
                self.target,
                self.policy.label(),
                self.autoscroll,
                self.keyboard_draggable,
                self.target_enabled,
                self.outcome.committed()
            ),
        )
    }

    fn before_state(&self) -> String {
        format!(
            "preset={} accept=pending autoscroll={} keyboard_draggable={} target_enabled={}",
            self.name, self.autoscroll, self.keyboard_draggable, self.target_enabled
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DropPolicy {
    MoveAfter,
    MoveBefore,
    CopyOutline,
    CopyInside,
}

impl DropPolicy {
    const fn label(self) -> &'static str {
        match self {
            Self::MoveAfter => "Accept(move, indicator=after)",
            Self::MoveBefore => "Accept(move, indicator=before)",
            Self::CopyOutline => "Accept(copy, indicator=outline)",
            Self::CopyInside => "Accept(copy, indicator=inside)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DragAndDropOutcome {
    Drop(&'static str),
    Cancel(&'static str),
}

impl DragAndDropOutcome {
    const fn action(self) -> &'static str {
        match self {
            Self::Drop(action) | Self::Cancel(action) => action,
        }
    }

    const fn committed(self) -> bool {
        matches!(self, Self::Drop(_))
    }

    const fn final_event(self) -> DragAndDropEvent {
        match self {
            Self::Drop(_) => DragAndDropEvent::Drop,
            Self::Cancel(_) => DragAndDropEvent::DragCancel,
        }
    }

    const fn event_sequence(self) -> &'static str {
        match self {
            Self::Drop(_) => "DragStart > DragMove > DragEnter > Drop > DragEnd(committed=true)",
            Self::Cancel(_) => {
                "DragStart(Space) > DragMove(ArrowDown) > DragEnter > DragCancel(Esc) > DragEnd(committed=false)"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DragAndDropEvent {
    DragStart,
    DragEnter,
    Drop,
    DragCancel,
}

impl DragAndDropEvent {
    const fn name(self) -> &'static str {
        match self {
            Self::DragStart => "DragStart",
            Self::DragEnter => "DragEnter",
            Self::Drop => "Drop",
            Self::DragCancel => "DragCancel",
        }
    }
}

fn log(
    target: &UiStateId,
    action: &str,
    before: impl Into<String>,
    after: impl Into<String>,
) -> UiCallbackLog {
    UiCallbackLog::new(target.clone(), action, before, after)
}
