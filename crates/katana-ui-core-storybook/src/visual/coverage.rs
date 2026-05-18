use crate::catalog::StoryExample;
use crate::requirements::StoryRequirements;
use katana_ui_core::render_model::UiNodeKind;
use serde::{Deserialize, Serialize};

use super::palette;
use super::render;

const INITIAL_VISIBLE_STORIES: usize = 24;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorybookVisualCoverageReport {
    pub required_ui: usize,
    pub dedicated_ui: usize,
    pub required_ui_fallbacks: usize,
    pub initial_visible_fallbacks: usize,
    pub modal_required: bool,
    pub non_empty_pixels: usize,
    pub theme_difference_pixels: usize,
    pub operation_difference_pixels: usize,
    pub missing_pages: Vec<String>,
}

impl StorybookVisualCoverageReport {
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "required_ui={} dedicated_ui={} required_ui_fallbacks={} initial_visible_fallbacks={} modal_required={} non_empty_pixels={} theme_difference_pixels={} operation_difference_pixels={}",
            self.required_ui,
            self.dedicated_ui,
            self.required_ui_fallbacks,
            self.initial_visible_fallbacks,
            self.modal_required,
            self.non_empty_pixels,
            self.theme_difference_pixels,
            self.operation_difference_pixels
        )
    }
}

pub(super) fn visual_coverage_report(examples: &[StoryExample]) -> StorybookVisualCoverageReport {
    let mut dedicated_ui = 0;
    let mut missing_pages = Vec::new();

    for page in StoryRequirements::required_pages() {
        let example = examples.iter().find(|it| it.page == *page);
        match example {
            Some(it) if has_dedicated_renderer(it.tree.root().kind()) => {
                dedicated_ui += 1;
            }
            Some(_) | None => missing_pages.push((*page).to_string()),
        }
    }

    let dark = render::render_storybook_canvas_for("dark", "button", false);
    let light = render::render_storybook_canvas_for("light", "button", false);
    let operation_after = render::render_storybook_canvas_for("dark", "button", true);

    StorybookVisualCoverageReport {
        required_ui: StoryRequirements::required_pages().len(),
        dedicated_ui,
        required_ui_fallbacks: missing_pages.len(),
        initial_visible_fallbacks: initial_visible_fallbacks(examples),
        modal_required: StoryRequirements::required_pages().contains(&"modal"),
        non_empty_pixels: dark.non_background_pixels(palette::DEFAULT_BACKGROUND),
        theme_difference_pixels: pixel_difference(dark.pixels(), light.pixels()),
        operation_difference_pixels: pixel_difference(dark.pixels(), operation_after.pixels()),
        missing_pages,
    }
}

fn initial_visible_fallbacks(examples: &[StoryExample]) -> usize {
    examples
        .iter()
        .take(INITIAL_VISIBLE_STORIES)
        .filter(|it| !has_dedicated_renderer(it.tree.root().kind()))
        .count()
}

fn pixel_difference(left: &[u32], right: &[u32]) -> usize {
    left.iter()
        .zip(right.iter())
        .filter(|(left, right)| left != right)
        .count()
}

pub(super) fn has_dedicated_renderer(kind: UiNodeKind) -> bool {
    matches!(
        kind,
        UiNodeKind::Button
            | UiNodeKind::Text
            | UiNodeKind::Icon
            | UiNodeKind::Input
            | UiNodeKind::Checkbox
            | UiNodeKind::Radio
            | UiNodeKind::SelectBox
            | UiNodeKind::Toggle
            | UiNodeKind::Divider
            | UiNodeKind::Spacer
            | UiNodeKind::KeyCap
            | UiNodeKind::LoadingDots
            | UiNodeKind::Spinner
            | UiNodeKind::Badge
            | UiNodeKind::ProgressBar
            | UiNodeKind::ColorSwatch
            | UiNodeKind::SlideControl
            | UiNodeKind::NotificationToast
            | UiNodeKind::Popover
            | UiNodeKind::Tooltip
            | UiNodeKind::Modal
            | UiNodeKind::ModalOverlay
            | UiNodeKind::CodeDiff
            | UiNodeKind::ColorPicker
            | UiNodeKind::TreeView
            | UiNodeKind::CommandPalette
            | UiNodeKind::DynamicArrayEditor
            | UiNodeKind::SvgButton
            | UiNodeKind::TextButton
            | UiNodeKind::IconTextButton
            | UiNodeKind::Card
            | UiNodeKind::List
            | UiNodeKind::Menu
            | UiNodeKind::Tabs
            | UiNodeKind::Toolbar
            | UiNodeKind::FormField
            | UiNodeKind::Breadcrumb
            | UiNodeKind::Accordion
            | UiNodeKind::ComboBox
            | UiNodeKind::MenuButton
            | UiNodeKind::SearchBox
            | UiNodeKind::SegmentedToggle
            | UiNodeKind::SelectionList
            | UiNodeKind::SideMenu
            | UiNodeKind::StatusBar
            | UiNodeKind::Row
            | UiNodeKind::Column
            | UiNodeKind::Stack
            | UiNodeKind::Grid
            | UiNodeKind::ScrollArea
            | UiNodeKind::SplitPane
            | UiNodeKind::AlignCenter
    )
}
