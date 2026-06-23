use std::collections::BTreeMap;

const DEFAULT_TEXT_AREA_VALUE: &str = "English\n日本語 🔷";
pub(super) const DEFAULT_TEXT_AREA_INSTANCE: &str = "text-area.preview";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TextAreaRuntimeState {
    pub(super) value: String,
    pub(super) focused: bool,
    pub(super) uses_live_value: bool,
    pub(super) caret_visible: bool,
    pub(super) readonly: bool,
    pub(super) disabled: bool,
    pub(super) wrap_enabled: bool,
    pub(super) resize_enabled: bool,
    pub(super) vertical_scroll_enabled: bool,
    pub(super) horizontal_scroll_enabled: bool,
    pub(super) vertical_scrollbar_visible: bool,
    pub(super) horizontal_scrollbar_visible: bool,
    pub(super) scroll_offset: usize,
    pub(super) scroll_x_offset: usize,
    pub(super) resize_width_delta: usize,
    pub(super) resize_height_delta: usize,
    pub(super) caret: usize,
    pub(super) selection_start: usize,
    pub(super) selection_end: usize,
}

impl Default for TextAreaRuntimeState {
    fn default() -> Self {
        Self {
            value: DEFAULT_TEXT_AREA_VALUE.to_string(),
            focused: false,
            uses_live_value: false,
            caret_visible: false,
            readonly: false,
            disabled: false,
            wrap_enabled: true,
            resize_enabled: false,
            vertical_scroll_enabled: false,
            horizontal_scroll_enabled: false,
            vertical_scrollbar_visible: false,
            horizontal_scrollbar_visible: false,
            scroll_offset: 0,
            scroll_x_offset: 0,
            resize_width_delta: 0,
            resize_height_delta: 0,
            caret: DEFAULT_TEXT_AREA_VALUE.chars().count(),
            selection_start: DEFAULT_TEXT_AREA_VALUE.chars().count(),
            selection_end: DEFAULT_TEXT_AREA_VALUE.chars().count(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(super) struct TextAreaStateStore {
    default_runtime: TextAreaRuntimeState,
    instances: BTreeMap<&'static str, TextAreaRuntimeState>,
}

impl TextAreaStateStore {
    pub(super) fn runtime(&self, instance: &'static str) -> &TextAreaRuntimeState {
        self.instances
            .get(instance)
            .unwrap_or(&self.default_runtime)
    }

    pub(super) fn runtime_mut(&mut self, instance: &'static str) -> &mut TextAreaRuntimeState {
        if instance == DEFAULT_TEXT_AREA_INSTANCE {
            return &mut self.default_runtime;
        }
        self.instances.entry(instance).or_default()
    }
}

impl TextAreaRuntimeState {
    pub(super) fn value(&self) -> &str {
        self.value.as_str()
    }

    pub(super) const fn focused(&self) -> bool {
        self.focused
    }

    pub(super) const fn uses_live_value(&self) -> bool {
        self.uses_live_value
    }

    pub(super) const fn caret_visible(&self) -> bool {
        self.caret_visible
    }

    pub(super) const fn readonly(&self) -> bool {
        self.readonly
    }

    pub(super) const fn disabled(&self) -> bool {
        self.disabled
    }

    pub(super) const fn wrap_enabled(&self) -> bool {
        self.wrap_enabled
    }

    pub(super) const fn resize_enabled(&self) -> bool {
        self.resize_enabled
    }

    pub(super) const fn vertical_scroll_enabled(&self) -> bool {
        self.vertical_scroll_enabled
    }

    pub(super) const fn horizontal_scroll_enabled(&self) -> bool {
        self.horizontal_scroll_enabled
    }

    pub(super) const fn vertical_scrollbar_visible(&self) -> bool {
        self.vertical_scrollbar_visible
    }

    pub(super) const fn horizontal_scrollbar_visible(&self) -> bool {
        self.horizontal_scrollbar_visible
    }

    pub(super) const fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub(super) const fn scroll_x_offset(&self) -> usize {
        self.scroll_x_offset
    }

    pub(super) const fn resize_width_delta(&self) -> usize {
        self.resize_width_delta
    }

    pub(super) const fn resize_height_delta(&self) -> usize {
        self.resize_height_delta
    }

    pub(super) const fn selection(&self) -> (usize, usize, usize) {
        (self.caret, self.selection_start, self.selection_end)
    }
}
