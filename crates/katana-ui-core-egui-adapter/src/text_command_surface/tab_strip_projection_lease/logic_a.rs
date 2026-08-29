impl TabStripTabTarget {
    #[must_use]
    pub fn from_opaque_bytes(payload: impl Into<Vec<u8>>) -> Self {
        Self {
            payload: payload.into().into_boxed_slice(),
        }
    }

    pub(crate) fn copy_for_route(&self) -> Self {
        Self::from_opaque_bytes(self.payload.to_vec())
    }

    pub(crate) fn same_target(&self, other: &Self) -> bool {
        self.payload == other.payload
    }
}

impl TabStripGroupTarget {
    #[must_use]
    pub fn from_opaque_bytes(payload: impl Into<Vec<u8>>) -> Self {
        Self {
            payload: payload.into().into_boxed_slice(),
        }
    }

    pub(crate) fn copy_for_route(&self) -> Self {
        Self::from_opaque_bytes(self.payload.to_vec())
    }
}

impl TabStripSwatchTarget {
    #[must_use]
    pub fn from_opaque_bytes(payload: impl Into<Vec<u8>>) -> Self {
        Self {
            payload: payload.into().into_boxed_slice(),
        }
    }

    pub(crate) fn copy_for_route(&self) -> Self {
        Self::from_opaque_bytes(self.payload.to_vec())
    }
}

impl TabStripText {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl TabStripControlPresentation {
    #[must_use]
    pub fn new(tooltip: TabStripText, accessibility_label: TabStripText) -> Self {
        Self {
            tooltip,
            accessibility_label,
        }
    }
}

impl TabStripNavigationPresentation {
    #[must_use]
    pub fn new(previous: TabStripControlPresentation, next: TabStripControlPresentation) -> Self {
        Self {
            previous,
            next,
            overflow: None,
        }
    }

    #[must_use]
    pub fn overflow(mut self, value: TabStripControlPresentation) -> Self {
        self.overflow = Some(value);
        self
    }
}

impl TabStripScrollPresentation {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            request_active_reveal: false,
        }
    }

    #[must_use]
    pub const fn request_active_reveal(mut self, value: bool) -> Self {
        self.request_active_reveal = value;
        self
    }
}

impl TabStripTabCapabilities {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            active: false,
            dirty: false,
            pinned: false,
            selectable: false,
            closeable: false,
            draggable: false,
            accepts_tab_drop: false,
            groupable: false,
            virtual_tab: false,
        }
    }
    #[must_use]
    pub const fn active(mut self, value: bool) -> Self {
        self.active = value;
        self
    }

    #[must_use]
    pub const fn dirty(mut self, value: bool) -> Self {
        self.dirty = value;
        self
    }

    #[must_use]
    pub const fn pinned(mut self, value: bool) -> Self {
        self.pinned = value;
        self
    }

    #[must_use]
    pub const fn selectable(mut self, value: bool) -> Self {
        self.selectable = value;
        self
    }

    #[must_use]
    pub const fn closeable(mut self, value: bool) -> Self {
        self.closeable = value;
        self
    }

    #[must_use]
    pub const fn draggable(mut self, value: bool) -> Self {
        self.draggable = value;
        self
    }

    #[must_use]
    pub const fn accepts_tab_drop(mut self, value: bool) -> Self {
        self.accepts_tab_drop = value;
        self
    }

    #[must_use]
    pub const fn groupable(mut self, value: bool) -> Self {
        self.groupable = value;
        self
    }

    #[must_use]
    pub const fn virtual_tab(mut self, value: bool) -> Self {
        self.virtual_tab = value;
        self
    }
}

impl std::fmt::Debug for TabStripTabTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.payload.len();
        formatter.write_str("TabStripTabTarget(..)")
    }
}

impl std::fmt::Debug for TabStripGroupTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.payload.len();
        formatter.write_str("TabStripGroupTarget(..)")
    }
}

impl std::fmt::Debug for TabStripSwatchTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.payload.len();
        formatter.write_str("TabStripSwatchTarget(..)")
    }
}

impl std::fmt::Debug for TabStripText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.value.len();
        formatter.write_str("TabStripText(..)")
    }
}

impl std::fmt::Debug for TabStripControlPresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.tooltip.value.len();
        let _ = self.accessibility_label.value.len();
        formatter.write_str("TabStripControlPresentation(..)")
    }
}

impl std::fmt::Debug for TabStripNavigationPresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.previous.tooltip.value.len();
        let _ = self.next.tooltip.value.len();
        let _ = self.overflow.is_some();
        formatter.write_str("TabStripNavigationPresentation(..)")
    }
}
