const CHIP_GROUP_CHIP_WIDTH: u16 = 40;
const CHIP_GROUP_GAP: u16 = 5;
const CHIP_GROUP_AVAILABLE_WIDTH: u16 = 95;
const CHIP_GROUP_OVERFLOW_TRIGGER_WIDTH: u16 = 20;
const MOTION_SPEC_DURATION_MS: u16 = 180;
const MOTION_SPEC_DELAY_MS: u16 = 12;
const MOTION_KEYBOARD_TICK_PHASE: u16 = 3;
const MOTION_OPTION_PHASE: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::visual) struct RuntimeStructuredScreenState {
    pub(in crate::visual) shortcut_combo: ShortcutComboRuntimeState,
    pub(in crate::visual) skeleton_cluster: SkeletonClusterRuntimeState,
    pub(in crate::visual) motion: MotionRuntimeState,
    pub(in crate::visual) window_control: WindowControlRuntimeState,
    pub(in crate::visual) startup_state: StartupStateRuntimeState,
    pub(in crate::visual) attachment_chip: AttachmentChipRuntimeState,
    pub(in crate::visual) chip_group: ChipGroupRuntimeState,
    pub(in crate::visual) accordion: AccordionRuntimeState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::visual) struct ShortcutComboRuntimeState {
    pub(in crate::visual) platform_display_macos: bool,
    pub(in crate::visual) separator_none: bool,
    pub(in crate::visual) size_large: bool,
    pub(in crate::visual) tone_accent: bool,
    pub(in crate::visual) a11y_custom: bool,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) hovered: bool,
    pub(in crate::visual) platform_preview_macos: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::visual) struct SkeletonClusterRuntimeState {
    pub(in crate::visual) preset_card: bool,
    pub(in crate::visual) children_three: bool,
    pub(in crate::visual) live_region_card: bool,
    pub(in crate::visual) reduced_motion: bool,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) hovered: bool,
    pub(in crate::visual) keyboard_reduced_motion: bool,
    pub(in crate::visual) preview_child_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::visual) struct WindowControlRuntimeState {
    pub(in crate::visual) position_trailing: bool,
    pub(in crate::visual) size_tall: bool,
    pub(in crate::visual) controls_close_only: bool,
    pub(in crate::visual) visibility_hover: bool,
    pub(in crate::visual) pressed_close: bool,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) hover_visible: bool,
    pub(in crate::visual) keyboard_restore: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::visual) struct StartupStateRuntimeState {
    pub(in crate::visual) error: bool,
    pub(in crate::visual) retried: bool,
    pub(in crate::visual) canceled: bool,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) hovered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::visual) struct AttachmentChipRuntimeState {
    pub(in crate::visual) kind_image: bool,
    pub(in crate::visual) name_changed: bool,
    pub(in crate::visual) meta_visible: bool,
    pub(in crate::visual) thumbnail_visible: bool,
    pub(in crate::visual) status_error: bool,
    pub(in crate::visual) progress_uploading: bool,
    pub(in crate::visual) retry_visible: bool,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) hovered: bool,
    pub(in crate::visual) retried: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::visual) struct ChipGroupRuntimeState {
    pub(in crate::visual) label_changed: bool,
    pub(in crate::visual) chip_count_five: bool,
    pub(in crate::visual) wrap_enabled: bool,
    pub(in crate::visual) overflow_menu: bool,
    pub(in crate::visual) reorder_enabled: bool,
    pub(in crate::visual) gap_eight: bool,
    pub(in crate::visual) width_expanded: bool,
    pub(in crate::visual) trigger_wide: bool,
    pub(in crate::visual) hidden_count_two: bool,
    pub(in crate::visual) overflow_open: bool,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) hovered: bool,
    pub(in crate::visual) keyboard_dismissed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::visual) struct MotionRuntimeState {
    pub(in crate::visual) reduced: bool,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) hovered: bool,
    pub(in crate::visual) keyboard_phase: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::visual) struct AccordionRuntimeState {
    pub(in crate::visual) expanded: bool,
    pub(in crate::visual) disabled: bool,
    pub(in crate::visual) controlled: bool,
    pub(in crate::visual) trigger_area_full_row: bool,
    pub(in crate::visual) reduced_motion: bool,
}

#[path = "runtime_structured_state_attachment_chip_group.rs"]
mod attachment_chip_group_impl;
#[path = "runtime_structured_state_motion.rs"]
mod motion_impl;
#[path = "runtime_structured_state_shortcut_skeleton.rs"]
mod shortcut_skeleton_impl;
#[path = "runtime_structured_state_window_startup.rs"]
mod window_startup_impl;

impl RuntimeStructuredScreenState {
    pub(in crate::visual) fn apply_option(&mut self, page: &str, setting: &str) {
        match page {
            "shortcut-combo" => self.shortcut_combo.apply_option(setting),
            "skeleton-cluster" => self.skeleton_cluster.apply_option(setting),
            "motion" => self.motion.apply_option(setting),
            "window-control-button-group" => self.window_control.apply_option(setting),
            "startup-state-panel" => self.startup_state.apply_option(setting),
            "attachment-chip" => self.attachment_chip.apply_option(setting),
            "chip-group" => self.chip_group.apply_option(setting),
            "accordion" => self.accordion.apply_option(setting),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct RuntimeStructuredUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
}

impl RuntimeStructuredUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}

fn shortcut_combo_visual_text() -> String {
    use katana_ui_core::atom::{
        KeyCombo, KeyKind, KeyModifiers, RuntimePlatform, ShortcutCombo, ShortcutPlatform,
    };
    ShortcutCombo::new(
        "Open command palette",
        KeyCombo::new(
            KeyModifiers {
                command: true,
                control: false,
                alt: false,
                shift: false,
                meta: false,
            },
            KeyKind::Char('k'),
        ),
    )
    .platform_display(ShortcutPlatform::MacOS)
    .visual_text(RuntimePlatform::MacOS)
}
