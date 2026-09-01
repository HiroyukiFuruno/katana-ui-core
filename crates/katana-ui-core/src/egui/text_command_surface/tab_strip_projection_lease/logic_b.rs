impl TabStripGroupCapabilities {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            collapsed: false,
            collapsible: false,
            menu_available: false,
            renamable: false,
            recolorable: false,
            closeable: false,
            ungroupable: false,
            draggable: false,
            accepts_tab_drop: false,
        }
    }

    #[must_use]
    pub const fn collapsed(mut self, value: bool) -> Self {
        self.collapsed = value;
        self
    }

    #[must_use]
    pub const fn collapsible(mut self, value: bool) -> Self {
        self.collapsible = value;
        self
    }

    #[must_use]
    pub const fn menu_available(mut self, value: bool) -> Self {
        self.menu_available = value;
        self
    }

    #[must_use]
    pub const fn renamable(mut self, value: bool) -> Self {
        self.renamable = value;
        self
    }

    #[must_use]
    pub const fn recolorable(mut self, value: bool) -> Self {
        self.recolorable = value;
        self
    }

    #[must_use]
    pub const fn closeable(mut self, value: bool) -> Self {
        self.closeable = value;
        self
    }

    #[must_use]
    pub const fn ungroupable(mut self, value: bool) -> Self {
        self.ungroupable = value;
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
}

impl TabStripSurfaceCapabilities {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            previous_available: false,
            next_available: false,
            overflow_available: false,
            restore_available: false,
            create_group_available: false,
            tab_drop_at_end_available: false,
        }
    }

    #[must_use]
    pub const fn previous_available(mut self, value: bool) -> Self {
        self.previous_available = value;
        self
    }

    #[must_use]
    pub const fn next_available(mut self, value: bool) -> Self {
        self.next_available = value;
        self
    }

    #[must_use]
    pub const fn overflow_available(mut self, value: bool) -> Self {
        self.overflow_available = value;
        self
    }

    #[must_use]
    pub const fn restore_available(mut self, value: bool) -> Self {
        self.restore_available = value;
        self
    }

    #[must_use]
    pub const fn create_group_available(mut self, value: bool) -> Self {
        self.create_group_available = value;
        self
    }

    #[must_use]
    pub const fn tab_drop_at_end_available(mut self, value: bool) -> Self {
        self.tab_drop_at_end_available = value;
        self
    }
}

impl TabStripSwatchDescriptor {
    #[must_use]
    pub const fn new(target: TabStripSwatchTarget, display_color: RgbaColor) -> Self {
        Self {
            target,
            display_color,
            selected: false,
            accessibility_label: None,
        }
    }

    #[must_use]
    pub const fn selected(mut self, value: bool) -> Self {
        self.selected = value;
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: TabStripText) -> Self {
        self.accessibility_label = Some(value);
        self
    }
}

impl std::fmt::Debug for TabStripMenuOperation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("TabStripMenuOperation(..)")
    }
}

impl TabStripMenuEntry {
    #[must_use]
    pub fn action(
        label: TabStripText,
        accessibility_label: TabStripText,
        operation: TabStripMenuOperation,
    ) -> Self {
        Self {
            label,
            accessibility_label,
            separator: false,
            enabled: true,
            checked: false,
            operation: Some(operation),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn submenu(label: TabStripText, accessibility_label: TabStripText) -> Self {
        Self {
            label,
            accessibility_label,
            separator: false,
            enabled: true,
            checked: false,
            operation: None,
            children: Vec::new(),
        }
    }

    /// A host-projected visual separation with no route, target, or action.
    #[must_use]
    pub fn separator() -> Self {
        Self {
            label: TabStripText::new(""),
            accessibility_label: TabStripText::new(""),
            separator: true,
            enabled: false,
            checked: false,
            operation: None,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub const fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    #[must_use]
    pub const fn checked(mut self, value: bool) -> Self {
        self.checked = value;
        self
    }

    #[must_use]
    pub fn child(mut self, value: Self) -> Self {
        self.children.push(value);
        self
    }
}

impl std::fmt::Debug for TabStripMenuEntry {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.label.value.len();
        let _ = self.accessibility_label.value.len();
        let _ = self.operation.as_ref();
        formatter.write_str("TabStripMenuEntry(..)")
    }
}
