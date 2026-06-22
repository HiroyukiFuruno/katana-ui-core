use super::{
    AccordionRuntimeState, MOTION_KEYBOARD_TICK_PHASE, MOTION_OPTION_PHASE, MOTION_SPEC_DELAY_MS,
    MOTION_SPEC_DURATION_MS, MotionRuntimeState, RuntimeStructuredUpdate,
};

impl MotionRuntimeState {
    pub(in crate::visual) fn preview_reduce(&mut self) -> RuntimeStructuredUpdate {
        self.reduced = motion_reduced_action_is_instant();
        RuntimeStructuredUpdate::new(
            "motion_reduce",
            "motion_snapshot_changed",
            if self.reduced {
                "instant=true"
            } else {
                "instant=false"
            },
        )
    }

    pub(in crate::visual) fn focus(&mut self) -> RuntimeStructuredUpdate {
        self.focused = true;
        RuntimeStructuredUpdate::new("motion_focus", "focus", "focus=motion")
    }

    pub(in crate::visual) fn hover(&mut self) -> RuntimeStructuredUpdate {
        self.hovered = true;
        RuntimeStructuredUpdate::new("motion_hover", "hover_start", "hover=motion")
    }

    pub(in crate::visual) fn keyboard_tick(&mut self) -> RuntimeStructuredUpdate {
        self.keyboard_phase = motion_animation_tick_phase();
        RuntimeStructuredUpdate::new("motion_keyboard_tick", "motion_phase_changed", "phase=3")
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "motion.reduced_policy" | "motion.disable_context" => self.reduced = true,
            "motion.duration" | "motion.easing" => self.keyboard_phase = MOTION_OPTION_PHASE,
            _ => {}
        }
    }
}

fn motion_reduced_action_is_instant() -> bool {
    use katana_ui_core::component::ComponentAction;
    use katana_ui_core::interaction::{
        MotionPrimitiveKind, MotionSpec, ReducedMotionPolicy, UiAction,
    };
    use katana_ui_core::molecule::MotionPrimitive;

    let spec = MotionSpec::new(
        MotionPrimitiveKind::Slide,
        MOTION_SPEC_DURATION_MS,
        MOTION_SPEC_DELAY_MS,
        ReducedMotionPolicy::Respect,
    );
    let mut motion = MotionPrimitive::new("Panel motion", spec);
    let action = UiAction::reduced_motion(motion.state_id().clone(), true);
    motion.apply_action(&action);
    motion.motion_snapshot().instant
}

fn motion_animation_tick_phase() -> u16 {
    use katana_ui_core::component::ComponentAction;
    use katana_ui_core::interaction::{
        MotionPrimitiveKind, MotionSpec, ReducedMotionPolicy, UiAction,
    };
    use katana_ui_core::molecule::MotionPrimitive;
    use katana_ui_core::render_model::UiTree;

    let spec = MotionSpec::new(
        MotionPrimitiveKind::Slide,
        MOTION_SPEC_DURATION_MS,
        MOTION_SPEC_DELAY_MS,
        ReducedMotionPolicy::Respect,
    );
    let mut motion = MotionPrimitive::new("Panel motion", spec);
    let action = UiAction::animation_tick(motion.state_id().clone(), MOTION_KEYBOARD_TICK_PHASE);
    motion.apply_action(&action);
    UiTree::new(motion)
        .root()
        .props()
        .interaction
        .animation_phase
}

impl AccordionRuntimeState {
    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "accordion.expanded" => self.expanded = true,
            "accordion.disabled" => self.disabled = true,
            "accordion.controlled" => self.controlled = true,
            "accordion.trigger_area" => self.trigger_area_full_row = true,
            "accordion.reduced_motion" => self.reduced_motion = true,
            _ => {}
        }
    }
}
