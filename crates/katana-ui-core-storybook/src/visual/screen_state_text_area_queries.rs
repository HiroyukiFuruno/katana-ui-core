use super::screen_state::StorybookScreenState;
use super::text_area_screen_state::DEFAULT_TEXT_AREA_INSTANCE;
use super::text_area_screen_state::TextAreaRuntimeState;

impl StorybookScreenState {
    pub(in crate::visual) fn text_area_value(&self) -> &str {
        self.text_area_value_for(DEFAULT_TEXT_AREA_INSTANCE)
    }

    pub(in crate::visual) fn text_area_value_for(&self, instance: &'static str) -> &str {
        self.text_area_runtime_for(instance).value()
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_area_focused(&self) -> bool {
        self.text_area_focused_for(DEFAULT_TEXT_AREA_INSTANCE)
    }

    pub(in crate::visual) fn text_area_focused_for(&self, instance: &'static str) -> bool {
        self.text_area_runtime_for(instance).focused()
    }

    pub(in crate::visual) fn text_area_uses_live_value_for(&self, instance: &'static str) -> bool {
        self.text_area_runtime_for(instance).uses_live_value()
    }

    pub(in crate::visual) fn text_area_caret_visible_for(&self, instance: &'static str) -> bool {
        self.text_area_runtime_for(instance).caret_visible()
    }

    pub(in crate::visual) fn text_area_readonly_for(&self, instance: &'static str) -> bool {
        self.text_area_runtime_for(instance).readonly()
    }

    pub(in crate::visual) fn text_area_disabled_for(&self, instance: &'static str) -> bool {
        self.text_area_runtime_for(instance).disabled()
    }

    pub(in crate::visual) fn text_area_wrap_enabled_for(&self, instance: &'static str) -> bool {
        self.text_area_runtime_for(instance).wrap_enabled()
    }

    pub(in crate::visual) fn text_area_resize_enabled_for(&self, instance: &'static str) -> bool {
        self.text_area_runtime_for(instance).resize_enabled()
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_area_vertical_scroll_enabled(&self) -> bool {
        self.text_area_vertical_scroll_enabled_for(DEFAULT_TEXT_AREA_INSTANCE)
    }

    pub(in crate::visual) fn text_area_vertical_scroll_enabled_for(
        &self,
        instance: &'static str,
    ) -> bool {
        self.text_area_runtime_for(instance)
            .vertical_scroll_enabled()
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_area_horizontal_scroll_enabled(&self) -> bool {
        self.text_area_horizontal_scroll_enabled_for(DEFAULT_TEXT_AREA_INSTANCE)
    }

    pub(in crate::visual) fn text_area_horizontal_scroll_enabled_for(
        &self,
        instance: &'static str,
    ) -> bool {
        self.text_area_runtime_for(instance)
            .horizontal_scroll_enabled()
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_area_vertical_scrollbar_visible(&self) -> bool {
        self.text_area_vertical_scrollbar_visible_for(DEFAULT_TEXT_AREA_INSTANCE)
    }

    pub(in crate::visual) fn text_area_vertical_scrollbar_visible_for(
        &self,
        instance: &'static str,
    ) -> bool {
        self.text_area_runtime_for(instance)
            .vertical_scrollbar_visible()
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_area_horizontal_scrollbar_visible(&self) -> bool {
        self.text_area_horizontal_scrollbar_visible_for(DEFAULT_TEXT_AREA_INSTANCE)
    }

    pub(in crate::visual) fn text_area_horizontal_scrollbar_visible_for(
        &self,
        instance: &'static str,
    ) -> bool {
        self.text_area_runtime_for(instance)
            .horizontal_scrollbar_visible()
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_area_scroll_offset(&self) -> usize {
        self.text_area_scroll_offset_for(DEFAULT_TEXT_AREA_INSTANCE)
    }

    pub(in crate::visual) fn text_area_scroll_offset_for(&self, instance: &'static str) -> usize {
        self.text_area_runtime_for(instance).scroll_offset()
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_area_scroll_x_offset(&self) -> usize {
        self.text_area_scroll_x_offset_for(DEFAULT_TEXT_AREA_INSTANCE)
    }

    pub(in crate::visual) fn text_area_scroll_x_offset_for(&self, instance: &'static str) -> usize {
        self.text_area_runtime_for(instance).scroll_x_offset()
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_area_resize_width_delta(&self) -> usize {
        self.text_area_resize_width_delta_for(DEFAULT_TEXT_AREA_INSTANCE)
    }

    pub(in crate::visual) fn text_area_resize_width_delta_for(
        &self,
        instance: &'static str,
    ) -> usize {
        self.text_area_runtime_for(instance).resize_width_delta()
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_area_resize_height_delta(&self) -> usize {
        self.text_area_resize_height_delta_for(DEFAULT_TEXT_AREA_INSTANCE)
    }

    pub(in crate::visual) fn text_area_resize_height_delta_for(
        &self,
        instance: &'static str,
    ) -> usize {
        self.text_area_runtime_for(instance).resize_height_delta()
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_area_runtime(&self) -> &TextAreaRuntimeState {
        self.text_area_runtime_for(DEFAULT_TEXT_AREA_INSTANCE)
    }

    pub(in crate::visual) fn text_area_runtime_for(
        &self,
        instance: &'static str,
    ) -> &TextAreaRuntimeState {
        self.text_areas.runtime(instance)
    }

    pub(in crate::visual) fn text_area_runtime_mut_for(
        &mut self,
        instance: &'static str,
    ) -> &mut TextAreaRuntimeState {
        self.text_areas.runtime_mut(instance)
    }
}
