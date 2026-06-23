use super::{
    RuntimeStructuredUpdate, ShortcutComboRuntimeState, SkeletonClusterRuntimeState,
    shortcut_combo_visual_text,
};

impl ShortcutComboRuntimeState {
    pub(in crate::visual) fn preview_platform(&mut self) -> RuntimeStructuredUpdate {
        let visual_text = shortcut_combo_visual_text();
        self.platform_preview_macos = visual_text == "⌘K";
        RuntimeStructuredUpdate::new(
            "shortcut_platform_preview",
            "shortcut_display_changed",
            if self.platform_preview_macos {
                "combo=Command+K"
            } else {
                "combo=unknown"
            },
        )
    }

    pub(in crate::visual) fn focus(&mut self) -> RuntimeStructuredUpdate {
        self.focused = true;
        RuntimeStructuredUpdate::new("shortcut_combo_focus", "focus", "focus=true")
    }

    pub(in crate::visual) fn hover(&mut self) -> RuntimeStructuredUpdate {
        self.hovered = true;
        RuntimeStructuredUpdate::new("shortcut_combo_hover", "hover_start", "hover=true")
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "shortcut_combo.platform_display" => self.platform_display_macos = true,
            "shortcut_combo.separator" => self.separator_none = true,
            "shortcut_combo.size" => self.size_large = true,
            "shortcut_combo.tone" => self.tone_accent = true,
            "shortcut_combo.a11y_label" => self.a11y_custom = true,
            _ => {}
        }
    }
}

impl SkeletonClusterRuntimeState {
    pub(in crate::visual) fn preview_card(&mut self) -> RuntimeStructuredUpdate {
        self.preset_card = true;
        self.preview_child_count = skeleton_cluster_child_count();
        RuntimeStructuredUpdate::new(
            "skeleton_cluster_preset_apply",
            "skeleton_cluster_changed",
            if self.preview_child_count == 2 {
                "items=2"
            } else {
                "items=unknown"
            },
        )
    }

    pub(in crate::visual) fn focus(&mut self) -> RuntimeStructuredUpdate {
        self.focused = true;
        let live_region = skeleton_cluster_live_region();
        RuntimeStructuredUpdate::new(
            "skeleton_cluster_focus",
            "focus",
            if live_region == "Loading card loading" {
                "focus=cluster"
            } else {
                "focus=unknown"
            },
        )
    }

    pub(in crate::visual) fn hover(&mut self) -> RuntimeStructuredUpdate {
        self.hovered = true;
        RuntimeStructuredUpdate::new("skeleton_cluster_hover", "hover_start", "hover=cluster")
    }

    pub(in crate::visual) fn keyboard_reduce_motion(&mut self) -> RuntimeStructuredUpdate {
        self.keyboard_reduced_motion = skeleton_reduced_motion_action_handled();
        RuntimeStructuredUpdate::new(
            "skeleton_cluster_keyboard_reduce_motion",
            "skeleton_reduced_motion_changed",
            if self.keyboard_reduced_motion {
                "reduced_motion=true"
            } else {
                "reduced_motion=ignored"
            },
        )
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "skeleton_cluster.preset" => self.preset_card = true,
            "skeleton_cluster.children" => self.children_three = true,
            "skeleton_cluster.live_region" => self.live_region_card = true,
            "skeleton_cluster.reduced_motion" => self.reduced_motion = true,
            _ => {}
        }
    }
}

fn skeleton_cluster_child_count() -> usize {
    use katana_ui_core::molecule::{SkeletonCluster, SkeletonClusterPreset};
    use katana_ui_core::render_model::UiTree;

    UiTree::new(SkeletonCluster::new("card loading").preset(SkeletonClusterPreset::Card))
        .root()
        .children()
        .len()
}

fn skeleton_cluster_live_region() -> String {
    use katana_ui_core::molecule::{SkeletonCluster, SkeletonClusterPreset};
    use katana_ui_core::render_model::UiTree;

    UiTree::new(SkeletonCluster::new("card loading").preset(SkeletonClusterPreset::Card))
        .root()
        .props()
        .accessibility_label
        .clone()
}

fn skeleton_reduced_motion_action_handled() -> bool {
    use katana_ui_core::atom::{Skeleton, SkeletonAnimation, SkeletonShape};
    use katana_ui_core::component::ComponentAction;
    use katana_ui_core::interaction::UiAction;

    let mut skeleton =
        Skeleton::new("line", SkeletonShape::Rect).animation(SkeletonAnimation::Shimmer);
    let action = UiAction::reduced_motion(skeleton.state_id().clone(), true);
    skeleton.apply_action(&action).handled
}
