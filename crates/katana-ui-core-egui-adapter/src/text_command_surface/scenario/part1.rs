use super::{
    EguiTextCommandSurfaceCommandFamilyProjection, EguiTextCommandSurfaceFloatingPresentation,
    EditorViewportProjectionLease,
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceSearchPresentation,
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucInteractionSelector, KucOpaqueClickContinuation, KucOpaqueClickContinuationError,
    KucOpaqueSearchTraceContinuation, KucOpaqueTextSelectionContinuation,
    KucSearchTraceContinuationError, KucTextSelectionContinuationError,
    SourceAddressProjectionLease, SourceAddressSubmissionPort, SourceAddressSubmissionPortError,
    StatusDiagnosticsProjectionLease, TabStripCorrelation, TabStripGroupCapabilities,
    TabStripGroupDescriptor, TabStripGroupTarget, TabStripProjection, TabStripProjectionLease,
    TabStripProposal, TabStripProposalPort, TabStripProposalPortError, TabStripSurfaceCapabilities,
    TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
    TextCommandSurfaceStyle,
};
use crate::context_menu::{ContextMenuPresentation, ContextMenuPresentationItem};
use crate::text_surface::TextSurfaceAnnotationPaint;
use katana_ui_core::atom::TextArea;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeCapability, CommandChromeDisplayMode, CommandChromeDropdown,
    CommandChromeDropdownItem, CommandChromeDropdownTrigger, CommandChromeIcon,
    CommandChromeSearchPresentation, CommandChromeText, CommandChromeToolbarPresentation,
    FloatingCommandToolbarVisibility, SearchControlCapabilities, SearchControlIcons,
    SearchControlStrings, SearchResultSummaryTemplate,
};
use katana_ui_core::molecule::selection::ContextMenuItemKind;
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressPresentation, SourceAddressStrip, SourceAddressSubmission,
};
use katana_ui_core::molecule::structured::{ReplaceMode, SearchOptions};
use katana_ui_core::molecule::{
    DiagnosticLocation, DiagnosticSeverity, DiagnosticsList, StatusBar, StatusBarDensity,
    StatusBarMode, StatusBarSegment,
};
use katana_ui_core::render_model::{
    UiImageSurfaceFit, UiImageSurfaceProps, UiImageSurfaceTransform, UiStateId,
};
use katana_ui_core::text_selection::UiTextSelectionRange;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAnnotation, TextSurfaceAnnotationStyle,
    TextSurfaceAutomaticGutterPresentation, TextSurfacePresentation, TextSurfaceProps,
    TextSurfaceViewport,
};

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;
const RESIZED_WIDTH: f32 = 900.0;
const RESIZED_HEIGHT: f32 = 520.0;
const IME_PIXELS_PER_POINT: f32 = 1.25;
const DIAGNOSTIC_LINE: u32 = 3;
const DIAGNOSTIC_COLUMN: u32 = 12;
const TAB_SOURCE_X: f32 = 42.0;
const TAB_TARGET_X: f32 = 420.0;
const TAB_Y: f32 = 18.0;
const FIND_ADVANCE_COUNT: usize = 7;
const SELECTION_ADVANCE_COUNT: usize = 5;
const FIND_FIXTURE_QUERY: &str = "日本語";
const NAVIGATION_INPUT_FIXTURE: &str = "入力 ⭐️";
const GENERIC_SEARCH_MATCH_ROLE: &str = "generic-search-match";
const GENERIC_SEARCH_CURRENT_ROLE: &str = "generic-search-current";
const GENERIC_LANGUAGE_CHOICE_LABELS: [&str; 17] = [
    "text",
    "markdown",
    "bash",
    "zsh",
    "mermaid",
    "drawio",
    "plantuml",
    "json",
    "yaml",
    "toml",
    "rust",
    "typescript",
    "javascript",
    "python",
    "html",
    "css",
    "sql",
];
const FIXTURE_TEXT: &str = "# Generic text surface

日本語と ASCII mixed content
exact ⭐️ VS16 and ☆ text
01 ordinary paragraph with stable wrapping
02 日本語の行番号と CJK glyphs
03 **strong** and _emphasis_ like syntax
04 `inline code` and [generic link](https://example.invalid)
05 > quoted content remains plain text
06 - unordered item one
07 - unordered item two
08 1. ordered item one
09 2. ordered item two
10 a horizontal rule-like marker follows
11 ---
12 a second paragraph keeps the surface dense
13 日本語入力と English input are adjacent
14 exact ⭐️ appears again with variation selector
15 code-like content starts below
16 ```
17 fn generic_example(value: &str) -> usize {
18     value.chars().count()
19 }
20 ```
21 code block closing line is visible
22 another paragraph after the code block
23 tabular text | left | center | right
24 ------------ | ---- | ------ | -----
25 日本語        | 文字 | ⭐️    | ☆
26 long content extends beyond the first viewport
27 scroll target alpha with CJK 日本語
28 scroll target beta with ASCII words
29 scroll target gamma with exact VS16 ⭐️
30 selection target starts on this line
31 selection target continues on this line
32 readonly and IME states use the same document
33 deterministic artifact input remains generic
34 focus and caret are retained between frames
35 context target is opened through pointer input
36 resize changes the actual screen rectangle
37 search-like text is content, not host semantics
38 final code-like marker: `value += 1`
39 final Japanese row: 終了ではなく次の状態へ
40 final ASCII row keeps the fixture above forty lines
41 trailing row preserves scrollable content
42 exact ⭐️ VS16 final control row";

/// Stable IDs for KUC-owned interaction states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FullTextCommandSurfaceScenarioId {
    Resting,
    Selection,
    Find,
    Context,
    Readonly,
    ResizeScrollIme,
    NavigationInput,
    WorkspaceTabs,
}

/// An opaque deterministic input stage. Coordinates and event payloads stay KUC-owned.
#[derive(Clone)]
pub struct FullTextCommandSurfaceRawInputStage {
    input: egui::RawInput,
}

impl FullTextCommandSurfaceRawInputStage {
    fn new(input: egui::RawInput) -> Self {
        Self { input }
    }

    /// Adds this stage to a consumer-owned egui frame input.
    pub fn apply_to(&self, input: &mut egui::RawInput) {
        input.events.extend(self.input.events.iter().cloned());
        input.screen_rect = self.input.screen_rect;
        input.viewports.extend(self.input.viewports.clone());
    }

    /// Returns the number of events in this deterministic stage.
    #[must_use]
    pub fn event_count(&self) -> usize {
        self.input.events.len()
    }
}

impl std::fmt::Debug for FullTextCommandSurfaceRawInputStage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("FullTextCommandSurfaceRawInputStage")
            .field("event_count", &self.event_count())
            .finish_non_exhaustive()
    }
}

/// A KUC-owned one-shot continuation whose phase remains opaque to consumers.
pub struct KucOpaqueMotionContinuation {
    state: KucOpaqueMotionContinuationState,
}

enum KucOpaqueMotionContinuationState {
    Selection(KucOpaqueTextSelectionContinuation),
    Search(KucOpaqueSearchTraceContinuation),
    Click(KucOpaqueClickContinuation),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucOpaqueMotionContinuationError {
    Selection(KucTextSelectionContinuationError),
    Search(KucSearchTraceContinuationError),
    Click(KucOpaqueClickContinuationError),
}

impl std::fmt::Display for KucOpaqueMotionContinuationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Selection(error) => write!(formatter, "selection continuation failed: {error}"),
            Self::Search(error) => write!(formatter, "search continuation failed: {error}"),
            Self::Click(error) => write!(formatter, "click continuation failed: {error}"),
        }
    }
}
