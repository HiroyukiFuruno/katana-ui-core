use crate::command_chrome::EguiCommandChromeError;
use crate::context_menu::ContextMenuAdapterError;
use crate::text_surface::EguiTextSurfaceError;
use katana_ui_core::molecule::command_chrome::CommandChromeFamilyId;

#[derive(Debug)]
pub enum EguiTextCommandSurfaceError {
    DuplicateCommandFamilyMount {
        family: CommandChromeFamilyId,
    },
    MissingThemeColor {
        token: &'static str,
    },
    MissingThemeFont {
        token: &'static str,
    },
    MissingThemeSpacing {
        token: &'static str,
    },
    InvalidThemeFont {
        token: &'static str,
        reason: &'static str,
    },
    InvalidThemeSpacing {
        token: &'static str,
        reason: &'static str,
    },
    SourceAddress(crate::source_address_strip::EguiSourceAddressStripError),
    Diagnostics(crate::diagnostics_list::EguiDiagnosticsListError),
    StatusBar(crate::status_bar::EguiStatusBarError),
    Text(EguiTextSurfaceError),
    Chrome(EguiCommandChromeError),
    ContextMenu(ContextMenuAdapterError),
    TabStrip {
        message: String,
    },
}

impl From<EguiTextSurfaceError> for EguiTextCommandSurfaceError {
    fn from(value: EguiTextSurfaceError) -> Self {
        Self::Text(value)
    }
}

impl From<crate::source_address_strip::EguiSourceAddressStripError>
    for EguiTextCommandSurfaceError
{
    fn from(value: crate::source_address_strip::EguiSourceAddressStripError) -> Self {
        Self::SourceAddress(value)
    }
}

impl From<crate::diagnostics_list::EguiDiagnosticsListError> for EguiTextCommandSurfaceError {
    fn from(value: crate::diagnostics_list::EguiDiagnosticsListError) -> Self {
        Self::Diagnostics(value)
    }
}

impl From<crate::status_bar::EguiStatusBarError> for EguiTextCommandSurfaceError {
    fn from(value: crate::status_bar::EguiStatusBarError) -> Self {
        Self::StatusBar(value)
    }
}

impl From<EguiCommandChromeError> for EguiTextCommandSurfaceError {
    fn from(value: EguiCommandChromeError) -> Self {
        Self::Chrome(value)
    }
}

impl From<ContextMenuAdapterError> for EguiTextCommandSurfaceError {
    fn from(value: ContextMenuAdapterError) -> Self {
        Self::ContextMenu(value)
    }
}

impl From<super::super::tab_strip_retained::TabStripRetainedError> for EguiTextCommandSurfaceError {
    fn from(value: super::super::tab_strip_retained::TabStripRetainedError) -> Self {
        Self::TabStrip {
            message: value.to_string(),
        }
    }
}

impl std::fmt::Display for EguiTextCommandSurfaceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateCommandFamilyMount { .. } => {
                formatter.write_str("command family is mounted in both primary and floating slots")
            }
            Self::MissingThemeColor { token } => write!(
                formatter,
                "KUC theme is missing required color token: {token}"
            ),
            Self::MissingThemeFont { token } => write!(
                formatter,
                "KUC theme is missing required font token: {token}"
            ),
            Self::MissingThemeSpacing { token } => write!(
                formatter,
                "KUC theme is missing required spacing token: {token}"
            ),
            Self::InvalidThemeFont { token, reason } => write!(
                formatter,
                "KUC theme has invalid font token {token}: {reason}"
            ),
            Self::InvalidThemeSpacing { token, reason } => write!(
                formatter,
                "KUC theme has invalid spacing token {token}: {reason}"
            ),
            Self::SourceAddress(error) => {
                write!(formatter, "text-command source address failed: {error}")
            }
            Self::Diagnostics(error) => {
                write!(formatter, "text-command diagnostics failed: {error}")
            }
            Self::StatusBar(error) => write!(formatter, "text-command status bar failed: {error}"),
            Self::Text(error) => write!(formatter, "text-command text surface failed: {error}"),
            Self::Chrome(error) => write!(formatter, "text-command command chrome failed: {error}"),
            Self::ContextMenu(error) => {
                write!(formatter, "text-command context menu failed: {error}")
            }
            Self::TabStrip { message } => {
                write!(formatter, "text-command tab strip failed: {message}")
            }
        }
    }
}

impl std::error::Error for EguiTextCommandSurfaceError {}
