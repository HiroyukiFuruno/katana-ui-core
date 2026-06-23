use super::LayoutStoryState;

impl Default for LayoutStoryState {
    fn default() -> Self {
        Self {
            page: "none",
            selected_index: 0,
            alignment: "start",
            callback: "callback=idle",
            hovered: false,
            focused: false,
            resized: false,
        }
    }
}

impl LayoutStoryState {
    pub(in crate::visual) fn is_page(&self, page: &str) -> bool {
        self.page == page
    }

    pub(in crate::visual) const fn callback(&self) -> &'static str {
        self.callback
    }

    pub(in crate::visual) const fn hovered(&self) -> bool {
        self.hovered
    }

    pub(in crate::visual) const fn focused(&self) -> bool {
        self.focused
    }

    pub(in crate::visual) const fn resized(&self) -> bool {
        self.resized
    }
}
