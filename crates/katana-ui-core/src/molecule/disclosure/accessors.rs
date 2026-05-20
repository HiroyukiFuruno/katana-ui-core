use super::modal_types::ModalParentInteraction;
use super::model::{Accordion, Modal, NotificationToast, Popover, SlideControl, Tooltip};
use crate::molecule::DisclosureTriggerArea;

macro_rules! disclosure_accessors {
    ($name:ident) => {
        impl $name {
            #[must_use]
            pub fn placement_model(&self) -> &str {
                &self.model.placement
            }

            #[must_use]
            pub fn offset_model(&self) -> (i16, i16) {
                self.model.offset
            }

            #[must_use]
            pub fn dismisses_on_outside_click(&self) -> bool {
                self.model.outside_click_dismiss
            }

            #[must_use]
            pub fn dismisses_on_escape(&self) -> bool {
                self.model.escape_dismiss
            }

            #[must_use]
            pub fn anchor_model(&self) -> &str {
                &self.model.anchor_summary
            }

            #[must_use]
            pub fn backdrop_model(&self) -> &str {
                &self.model.backdrop
            }

            #[must_use]
            pub fn focus_return_model(&self) -> &str {
                &self.model.focus_return
            }

            #[must_use]
            pub fn dismiss_policy_model(&self) -> &str {
                &self.model.dismiss_policy
            }

            #[must_use]
            pub fn title_model(&self) -> &str {
                &self.model.title
            }

            #[must_use]
            pub fn panel_size_model(&self) -> &str {
                &self.model.size
            }

            #[must_use]
            pub fn footer_model(&self) -> &str {
                &self.model.footer
            }

            #[must_use]
            pub fn uses_native_window_mode(&self) -> bool {
                self.model.native_window_mode
            }

            #[must_use]
            pub fn parent_interaction_model(&self) -> ModalParentInteraction {
                self.model.parent_interaction
            }

            #[must_use]
            pub fn width_model(&self) -> &str {
                &self.model.width
            }

            #[must_use]
            pub fn focus_handling_model(&self) -> &str {
                &self.model.focus_handling
            }

            #[must_use]
            pub fn delay_ms_model(&self) -> u16 {
                self.model.delay_ms
            }

            #[must_use]
            pub fn max_width_model(&self) -> u16 {
                self.model.max_width
            }

            #[must_use]
            pub fn opens_on_hover(&self) -> bool {
                self.model.hover_trigger
            }

            #[must_use]
            pub fn opens_on_focus(&self) -> bool {
                self.model.focus_trigger
            }

            #[must_use]
            pub fn timer_summary_model(&self) -> &str {
                &self.model.timer_summary
            }

            #[must_use]
            pub fn is_controlled(&self) -> bool {
                self.model.controlled
            }

            #[must_use]
            pub fn allows_multiple(&self) -> bool {
                self.model.multiple
            }

            #[must_use]
            pub fn indicator_position_model(&self) -> &str {
                &self.model.indicator_position
            }

            #[must_use]
            pub fn trigger_area_model(&self) -> DisclosureTriggerArea {
                self.model.trigger_area
            }

            #[must_use]
            pub fn toggle_icon_model(&self) -> &str {
                &self.model.toggle_icon
            }

            #[must_use]
            pub fn is_tree_mode(&self) -> bool {
                self.model.tree_mode
            }

            #[must_use]
            pub fn uses_reduced_motion(&self) -> bool {
                self.model.reduced_motion
            }

            #[must_use]
            pub fn has_body_border(&self) -> bool {
                self.model.body_border
            }

            #[must_use]
            pub fn is_selected(&self) -> bool {
                self.model.selected
            }

            #[must_use]
            pub fn depth_model(&self) -> u8 {
                self.model.depth
            }

            #[must_use]
            pub fn shows_lines(&self) -> bool {
                self.model.show_lines
            }

            #[must_use]
            pub fn range_model(&self) -> (i32, i32, i32) {
                (self.model.minimum, self.model.maximum, self.model.step)
            }

            #[must_use]
            pub fn binding_model(&self) -> &str {
                &self.model.binding
            }
        }
    };
}

disclosure_accessors!(Accordion);
disclosure_accessors!(Modal);
disclosure_accessors!(NotificationToast);
disclosure_accessors!(Popover);
disclosure_accessors!(SlideControl);
disclosure_accessors!(Tooltip);
