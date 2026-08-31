impl SanitizedTabTarget {
    /// Creates a target without assigning meaning to the supplied data.
    #[must_use]
    pub fn from_opaque_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            opaque: bytes.into().into_boxed_slice(),
        }
    }
}

impl SanitizedTabCapabilities {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            active: false,
            dirty: false,
            pinned: false,
            close: false,
        }
    }

    #[must_use]
    pub const fn active_state(mut self, value: bool) -> Self {
        self.active = value;
        self
    }

    #[must_use]
    pub const fn dirty_state(mut self, value: bool) -> Self {
        self.dirty = value;
        self
    }

    #[must_use]
    pub const fn pinned_state(mut self, value: bool) -> Self {
        self.pinned = value;
        self
    }

    #[must_use]
    pub const fn close_state(mut self, value: bool) -> Self {
        self.close = value;
        self
    }
}

impl SanitizedTabClosePresentation {
    #[must_use]
    pub fn new(
        visible_label: impl Into<String>,
        tooltip: impl Into<String>,
        accessibility_label: impl Into<String>,
    ) -> Self {
        Self {
            visible_label: visible_label.into(),
            tooltip: tooltip.into(),
            accessibility_label: accessibility_label.into(),
        }
    }
}

impl SanitizedTabGroupTarget {
    /// Creates a target without assigning meaning to the supplied data.
    #[must_use]
    pub fn from_opaque_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            opaque: bytes.into().into_boxed_slice(),
        }
    }
}

impl SanitizedTabGroupCapabilities {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            collapse: false,
            menu: false,
            rename: false,
            recolor: false,
            close: false,
            ungroup: false,
            drag: false,
        }
    }

    #[must_use]
    pub const fn collapse_state(mut self, value: bool) -> Self {
        self.collapse = value;
        self
    }

    #[must_use]
    pub const fn menu_state(mut self, value: bool) -> Self {
        self.menu = value;
        self
    }

    #[must_use]
    pub const fn rename_state(mut self, value: bool) -> Self {
        self.rename = value;
        self
    }

    #[must_use]
    pub const fn recolor_state(mut self, value: bool) -> Self {
        self.recolor = value;
        self
    }

    #[must_use]
    pub const fn close_state(mut self, value: bool) -> Self {
        self.close = value;
        self
    }

    #[must_use]
    pub const fn ungroup_state(mut self, value: bool) -> Self {
        self.ungroup = value;
        self
    }

    #[must_use]
    pub const fn drag_state(mut self, value: bool) -> Self {
        self.drag = value;
        self
    }
}

impl SanitizedTab {
    #[must_use]
    pub fn new(target: SanitizedTabTarget, order: u32, label: impl Into<String>) -> Self {
        Self {
            target,
            order,
            label: label.into(),
            icon: None,
            capabilities: SanitizedTabCapabilities::new(),
            close_presentation: None,
        }
    }

    #[must_use]
    pub fn with_icon(mut self, value: UiIconProps) -> Self {
        self.icon = Some(value);
        self
    }

    #[must_use]
    pub fn with_capabilities(mut self, value: SanitizedTabCapabilities) -> Self {
        self.capabilities = value;
        self
    }

    #[must_use]
    pub fn with_close_presentation(mut self, value: SanitizedTabClosePresentation) -> Self {
        self.close_presentation = Some(value);
        self
    }
}

impl SanitizedTabGroup {
    #[must_use]
    pub fn new(target: SanitizedTabGroupTarget, order: u32, label: impl Into<String>) -> Self {
        Self {
            target,
            order,
            label: label.into(),
            icon: None,
            capabilities: SanitizedTabGroupCapabilities::new(),
            tabs: Vec::new(),
            groups: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_icon(mut self, value: UiIconProps) -> Self {
        self.icon = Some(value);
        self
    }

    #[must_use]
    pub fn with_capabilities(mut self, value: SanitizedTabGroupCapabilities) -> Self {
        self.capabilities = value;
        self
    }

    #[must_use]
    pub fn tab(mut self, value: SanitizedTab) -> Self {
        self.tabs.push(value);
        self
    }

    #[must_use]
    pub fn group(mut self, value: SanitizedTabGroup) -> Self {
        self.groups.push(value);
        self
    }
}
