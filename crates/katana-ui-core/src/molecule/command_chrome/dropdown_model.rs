use crate::interaction::placement::{
    AnchorKind, Placement, PlacementEngine, PlacementRequest, PlacementResult, Rect, Size,
};
use crate::render_model::UiIconProps;
use serde::{Deserialize, Serialize};

const DEFAULT_DROPDOWN_OFFSET: i32 = 4;
const DEFAULT_DROPDOWN_CLAMP_MARGIN: i32 = 4;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandChromeDropdownItemId(String);

impl CommandChromeDropdownItemId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for CommandChromeDropdownItemId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CommandChromeDropdownItemId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandChromeDropdownTrigger {
    Primary,
    SplitSecondary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromeDropdownItem {
    id: CommandChromeDropdownItemId,
    label: String,
    accessibility_label: Option<String>,
    tooltip: Option<String>,
    icon: Option<UiIconProps>,
    disabled: bool,
    selected: bool,
}

impl CommandChromeDropdownItem {
    #[must_use]
    pub fn new(id: impl Into<CommandChromeDropdownItemId>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            accessibility_label: None,
            tooltip: None,
            icon: None,
            disabled: false,
            selected: false,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &CommandChromeDropdownItemId {
        &self.id
    }

    #[must_use]
    pub fn label_model(&self) -> &str {
        self.label.as_str()
    }

    #[must_use]
    pub const fn accessibility_label_model(&self) -> Option<&String> {
        self.accessibility_label.as_ref()
    }

    #[must_use]
    pub const fn tooltip_model(&self) -> Option<&String> {
        self.tooltip.as_ref()
    }

    #[must_use]
    pub const fn icon_model(&self) -> Option<&UiIconProps> {
        self.icon.as_ref()
    }

    #[must_use]
    pub const fn disabled_model(&self) -> bool {
        self.disabled
    }

    #[must_use]
    pub const fn selected_model(&self) -> bool {
        self.selected
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = Some(value.into());
        self
    }

    #[must_use]
    pub fn tooltip(mut self, value: impl Into<String>) -> Self {
        self.tooltip = Some(value.into());
        self
    }

    #[must_use]
    pub fn icon(mut self, value: UiIconProps) -> Self {
        self.icon = Some(value);
        self
    }

    #[must_use]
    pub const fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }

    #[must_use]
    pub const fn selected(mut self, value: bool) -> Self {
        self.selected = value;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromeDropdown {
    trigger: CommandChromeDropdownTrigger,
    items: Vec<CommandChromeDropdownItem>,
}

impl CommandChromeDropdown {
    #[must_use]
    pub const fn new(trigger: CommandChromeDropdownTrigger) -> Self {
        Self {
            trigger,
            items: Vec::new(),
        }
    }

    #[must_use]
    pub const fn trigger_model(&self) -> CommandChromeDropdownTrigger {
        self.trigger
    }

    #[must_use]
    pub fn items(&self) -> &[CommandChromeDropdownItem] {
        self.items.as_slice()
    }

    #[must_use]
    pub fn item(mut self, value: CommandChromeDropdownItem) -> Self {
        self.items.push(value);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromeDropdownLayout {
    trigger_bounds: Rect,
    viewport: Rect,
    panel_size: Size,
}

impl CommandChromeDropdownLayout {
    #[must_use]
    pub const fn new(trigger_bounds: Rect, viewport: Rect, panel_size: Size) -> Self {
        Self {
            trigger_bounds,
            viewport,
            panel_size,
        }
    }

    #[must_use]
    pub const fn trigger_bounds(&self) -> Rect {
        self.trigger_bounds
    }

    #[must_use]
    pub const fn viewport(&self) -> Rect {
        self.viewport
    }

    #[must_use]
    pub const fn panel_size(&self) -> Size {
        self.panel_size
    }

    #[must_use]
    pub(crate) fn resolve(&self) -> PlacementResult {
        PlacementEngine::resolve(
            &PlacementRequest::new(
                AnchorKind::virtual_rect(self.trigger_bounds),
                Placement::BottomStart,
                self.panel_size,
                self.viewport,
            )
            .priority([Placement::BottomStart, Placement::TopStart])
            .offset(DEFAULT_DROPDOWN_OFFSET)
            .clamp_margin(DEFAULT_DROPDOWN_CLAMP_MARGIN),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandChromeDropdownCloseReason {
    OutsideClick,
    Escape,
    Explicit,
    ItemActivated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandChromeDropdownKey {
    ArrowUp,
    ArrowDown,
    Home,
    End,
    Enter,
    Space,
    Escape,
}
