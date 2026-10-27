impl std::fmt::Debug for TabStripSwatchDescriptor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.target.payload.len();
        let _ = self.display_color;
        let _ = self
            .accessibility_label
            .as_ref()
            .map(|value| value.value.len());
        formatter.write_str("TabStripSwatchDescriptor(..)")
    }
}

impl TabStripContextMenuPresentation {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    #[must_use]
    pub fn entry(mut self, value: TabStripMenuEntry) -> Self {
        self.entries.push(value);
        self
    }
}

impl std::fmt::Debug for TabStripContextMenuPresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.entries.len();
        formatter.write_str("TabStripContextMenuPresentation(..)")
    }
}

impl Default for TabStripGroupPopupPresentation {
    fn default() -> Self {
        Self::new()
    }
}

impl TabStripGroupPopupPresentation {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            rename_placeholder: None,
            request_inline_rename: false,
            rename_policy: TabStripRenamePolicy::OnSubmit,
            entries: Vec::new(),
        }
    }

    #[must_use]
    pub fn rename_placeholder(mut self, value: TabStripText) -> Self {
        self.rename_placeholder = Some(value);
        self
    }

    /// Requests an immediate inline rename after the host accepts this group.
    #[must_use]
    pub const fn request_inline_rename(mut self, value: bool) -> Self {
        self.request_inline_rename = value;
        self
    }

    #[must_use]
    pub const fn rename_policy(mut self, value: TabStripRenamePolicy) -> Self {
        self.rename_policy = value;
        self
    }

    #[must_use]
    pub fn entry(mut self, value: TabStripMenuEntry) -> Self {
        self.entries.push(value);
        self
    }
}

impl std::fmt::Debug for TabStripGroupPopupPresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self
            .rename_placeholder
            .as_ref()
            .map(|value| value.value.len());
        let _ = self.entries.len();
        formatter.write_str("TabStripGroupPopupPresentation(..)")
    }
}
