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
            shortcut_platform_label(self.platform_preview_macos),
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
            skeleton_child_count_label(self.preview_child_count),
        )
    }

    pub(in crate::visual) fn focus(&mut self) -> RuntimeStructuredUpdate {
        self.focused = true;
        let live_region = skeleton_cluster_live_region();
        RuntimeStructuredUpdate::new(
            "skeleton_cluster_focus",
            "focus",
            skeleton_focus_label(live_region.as_str()),
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
            skeleton_reduced_motion_label(self.keyboard_reduced_motion),
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

const fn shortcut_platform_label(macos: bool) -> &'static str {
    if macos {
        "combo=Command+K"
    } else {
        "combo=unknown"
    }
}

const fn skeleton_child_count_label(count: usize) -> &'static str {
    if count == 2 {
        "items=2"
    } else {
        "items=unknown"
    }
}

fn skeleton_focus_label(live_region: &str) -> &'static str {
    if live_region == "Loading card loading" {
        "focus=cluster"
    } else {
        "focus=unknown"
    }
}

const fn skeleton_reduced_motion_label(handled: bool) -> &'static str {
    if handled {
        "reduced_motion=true"
    } else {
        "reduced_motion=ignored"
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

#[cfg(test)]
mod tests {
    use super::{
        ShortcutComboRuntimeState, SkeletonClusterRuntimeState, shortcut_platform_label,
        skeleton_child_count_label, skeleton_focus_label, skeleton_reduced_motion_label,
    };

    #[test]
    fn shortcut_and_skeleton_labels_cover_fallbacks_and_unknown_options_are_noops() {
        assert_eq!("combo=Command+K", shortcut_platform_label(true));
        assert_eq!("combo=unknown", shortcut_platform_label(false));
        assert_eq!("items=2", skeleton_child_count_label(2));
        assert_eq!("items=unknown", skeleton_child_count_label(3));
        assert_eq!(
            "focus=cluster",
            skeleton_focus_label("Loading card loading")
        );
        assert_eq!("focus=unknown", skeleton_focus_label("unknown"));
        assert_eq!("reduced_motion=true", skeleton_reduced_motion_label(true));
        assert_eq!(
            "reduced_motion=ignored",
            skeleton_reduced_motion_label(false)
        );

        let mut shortcut = ShortcutComboRuntimeState::default();
        shortcut.apply_option("unknown.setting");
        assert_eq!(ShortcutComboRuntimeState::default(), shortcut);

        let mut skeleton = SkeletonClusterRuntimeState::default();
        skeleton.apply_option("unknown.setting");
        assert_eq!(SkeletonClusterRuntimeState::default(), skeleton);
    }
}
