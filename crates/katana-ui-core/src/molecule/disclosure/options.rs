use super::model::{Accordion, Modal, NotificationToast, Popover, SlideControl, Tooltip};

macro_rules! disclosure_options {
    ($name:ident) => {
        impl $name {
            #[must_use]
            pub fn title(mut self, value: impl Into<String>) -> Self {
                self.model.title = value.into();
                self
            }

            #[must_use]
            pub fn panel_size(mut self, value: impl Into<String>) -> Self {
                self.model.size = value.into();
                self
            }

            #[must_use]
            pub fn footer(mut self, value: impl Into<String>) -> Self {
                self.model.footer = value.into();
                self
            }

            #[must_use]
            pub fn native_window_mode(mut self, value: bool) -> Self {
                self.model.native_window_mode = value;
                self
            }

            #[must_use]
            pub fn width(mut self, value: impl Into<String>) -> Self {
                self.model.width = value.into();
                self
            }

            #[must_use]
            pub fn focus_handling(mut self, value: impl Into<String>) -> Self {
                self.model.focus_handling = value.into();
                self
            }

            #[must_use]
            pub fn delay_ms(mut self, value: u16) -> Self {
                self.model.delay_ms = value;
                self
            }

            #[must_use]
            pub fn max_width(mut self, value: u16) -> Self {
                self.model.max_width = value;
                self
            }

            #[must_use]
            pub fn hover_trigger(mut self, value: bool) -> Self {
                self.model.hover_trigger = value;
                self
            }

            #[must_use]
            pub fn focus_trigger(mut self, value: bool) -> Self {
                self.model.focus_trigger = value;
                self
            }

            #[must_use]
            pub fn timer_summary(mut self, value: impl Into<String>) -> Self {
                self.model.timer_summary = value.into();
                self
            }

            #[must_use]
            pub fn reduced_motion(mut self, value: bool) -> Self {
                self.model.reduced_motion = value;
                self
            }

            #[must_use]
            pub fn body_border(mut self, value: bool) -> Self {
                self.model.body_border = value;
                self
            }

            #[must_use]
            pub fn selected(mut self, value: bool) -> Self {
                self.model.selected = value;
                self.state.has_selection = value;
                self
            }

            #[must_use]
            pub fn depth(mut self, value: u8) -> Self {
                self.model.depth = value;
                self
            }

            #[must_use]
            pub fn show_lines(mut self, value: bool) -> Self {
                self.model.show_lines = value;
                self
            }
        }
    };
}

disclosure_options!(Accordion);
disclosure_options!(Modal);
disclosure_options!(NotificationToast);
disclosure_options!(Popover);
disclosure_options!(SlideControl);
disclosure_options!(Tooltip);
