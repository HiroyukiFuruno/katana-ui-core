impl TabStripTabDescriptor {
    #[must_use]
    pub fn new(target: TabStripTabTarget, label: TabStripText) -> Self {
        Self {
            target,
            label,
            tooltip: None,
            accessibility_label: None,
            capabilities: TabStripTabCapabilities::new(),
            trailing_control: None,
            context_menu: None,
        }
    }

    #[must_use]
    pub fn tooltip(mut self, value: TabStripText) -> Self {
        self.tooltip = Some(value);
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: TabStripText) -> Self {
        self.accessibility_label = Some(value);
        self
    }

    #[must_use]
    pub const fn capabilities(mut self, value: TabStripTabCapabilities) -> Self {
        self.capabilities = value;
        self
    }

    #[must_use]
    pub fn trailing_control(mut self, value: TabStripControlPresentation) -> Self {
        self.trailing_control = Some(value);
        self
    }

    #[must_use]
    pub fn context_menu(mut self, value: TabStripContextMenuPresentation) -> Self {
        self.context_menu = Some(value);
        self
    }
}

impl std::fmt::Debug for TabStripTabDescriptor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.label.value.len();
        let _ = self.tooltip.as_ref().map(|value| value.value.len());
        let _ = self
            .accessibility_label
            .as_ref()
            .map(|value| value.value.len());
        let _ = self
            .trailing_control
            .as_ref()
            .map(|value| value.tooltip.value.len() + value.accessibility_label.value.len());
        let _ = self.context_menu.as_ref().map(|value| value.entries.len());
        formatter
            .debug_struct("TabStripTabDescriptor")
            .field("target", &self.target)
            .field("label", &"<opaque>")
            .field("tooltip", &self.tooltip.as_ref().map(|_| "<opaque>"))
            .field(
                "accessibility_label",
                &self.accessibility_label.as_ref().map(|_| "<opaque>"),
            )
            .field("capabilities", &self.capabilities)
            .finish()
    }
}

impl TabStripGroupDescriptor {
    #[must_use]
    pub fn new(target: TabStripGroupTarget, label: TabStripText) -> Self {
        Self {
            target,
            label,
            accessibility_label: None,
            capabilities: TabStripGroupCapabilities::new(),
            swatches: Vec::new(),
            tabs: Vec::new(),
            groups: Vec::new(),
            popup: None,
        }
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: TabStripText) -> Self {
        self.accessibility_label = Some(value);
        self
    }

    #[must_use]
    pub const fn capabilities(mut self, value: TabStripGroupCapabilities) -> Self {
        self.capabilities = value;
        self
    }

    #[must_use]
    pub fn swatch(mut self, value: TabStripSwatchDescriptor) -> Self {
        self.swatches.push(value);
        self
    }

    #[must_use]
    pub fn tab(mut self, value: TabStripTabDescriptor) -> Self {
        self.tabs.push(value);
        self
    }

    #[must_use]
    pub fn group(mut self, value: TabStripGroupDescriptor) -> Self {
        self.groups.push(value);
        self
    }

    #[must_use]
    pub fn popup(mut self, value: TabStripGroupPopupPresentation) -> Self {
        self.popup = Some(value);
        self
    }
}

impl std::fmt::Debug for TabStripGroupDescriptor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.label.value.len();
        let _ = self
            .accessibility_label
            .as_ref()
            .map(|value| value.value.len());
        let _ = self.tabs.len();
        let _ = self.groups.len();
        let _ = self.swatches.len();
        let _ = self.popup.as_ref().map(|value| value.entries.len());
        formatter
            .debug_struct("TabStripGroupDescriptor")
            .field("target", &self.target)
            .field("label", &"<opaque>")
            .field(
                "accessibility_label",
                &self.accessibility_label.as_ref().map(|_| "<opaque>"),
            )
            .field("capabilities", &self.capabilities)
            .field("tabs", &"<opaque>")
            .field("groups", &"<opaque>")
            .field("swatches", &"<opaque>")
            .finish()
    }
}

impl TabStripProjection {
    #[must_use]
    pub fn new(revision: u64, correlation: TabStripCorrelation) -> Self {
        Self {
            revision,
            correlation,
            groups: Vec::new(),
            tabs: Vec::new(),
            capabilities: TabStripSurfaceCapabilities::new(),
            navigation: None,
            scroll_presentation: TabStripScrollPresentation::new(),
        }
    }

    #[must_use]
    pub fn tab(mut self, value: TabStripTabDescriptor) -> Self {
        self.tabs.push(value);
        self
    }

    #[must_use]
    pub fn group(mut self, value: TabStripGroupDescriptor) -> Self {
        self.groups.push(value);
        self
    }

    #[must_use]
    pub const fn capabilities(mut self, value: TabStripSurfaceCapabilities) -> Self {
        self.capabilities = value;
        self
    }

    #[must_use]
    pub fn navigation(mut self, value: TabStripNavigationPresentation) -> Self {
        self.navigation = Some(value);
        self
    }

    #[must_use]
    pub const fn scroll_presentation(mut self, value: TabStripScrollPresentation) -> Self {
        self.scroll_presentation = value;
        self
    }
}

impl std::fmt::Debug for TabStripProjection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.revision;
        let _ = self.groups.len();
        let _ = self.tabs.len();
        let _ = self.navigation.is_some();
        let _ = self.scroll_presentation;
        formatter
            .debug_struct("TabStripProjection")
            .field("revision", &"<opaque>")
            .field("correlation", &self.correlation)
            .field("groups", &"<opaque>")
            .field("tabs", &"<opaque>")
            .field("capabilities", &self.capabilities)
            .field("navigation", &self.navigation.as_ref().map(|_| "<opaque>"))
            .finish()
    }
}

impl TabStripCorrelation {
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

impl std::fmt::Debug for TabStripCorrelation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.payload.len();
        formatter.write_str("TabStripCorrelation(..)")
    }
}

impl TabStripProjectionLease {
    #[must_use]
    pub fn new(projection: TabStripProjection) -> Self {
        Self {
            projection,
            proposal_port: None,
        }
    }

    pub fn with_proposal_port<P>(mut self, port: P) -> Self
    where
        P: super::super::tab_strip_proposal_port::TabStripProposalPort + 'static,
    {
        self.proposal_port =
            Some(super::super::tab_strip_proposal_port::TabStripProposalPortHandle::new(port));
        self
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        TabStripProjection,
        Option<super::super::tab_strip_proposal_port::TabStripProposalPortHandle>,
    ) {
        (self.projection, self.proposal_port)
    }
}

impl std::fmt::Debug for TabStripProjectionLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.projection.revision;
        formatter.write_str("TabStripProjectionLease(..)")
    }
}
