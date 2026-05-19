use crate::catalog::StoryExample;
use crate::requirements::StoryRequirements;
use katana_ui_core::render_model::UiNodeKind;
use serde::{Deserialize, Serialize};

use super::coverage_markers;
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
    pub selected_preview_visible: bool,
    pub selected_preview_interaction_visible: bool,
    pub detail_tables_hidden: bool,
    pub scrollbar_thumb_bottom: bool,
    pub contract_rows_fit: bool,
    pub inspector_rows_fit: bool,
    pub tree_view_selected: bool,
    pub tree_view_settings_visible: bool,
    pub tree_view_line_option_visible: bool,
    pub tree_view_icon_option_visible: bool,
    pub tree_view_trigger_option_visible: bool,
    pub tree_view_action_logged: bool,
    pub panel_scrollbars_visible: bool,
    pub navigation_collapsed_pixels_changed: usize,
    pub legacy_preview_signatures: usize,
    pub legacy_preview_signature_collisions: usize,
    pub missing_pages: Vec<String>,
}

impl StorybookVisualCoverageReport {
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "required_ui={} dedicated_ui={} required_ui_fallbacks={} initial_visible_fallbacks={} modal_required={} non_empty_pixels={} theme_difference_pixels={} operation_difference_pixels={} selected_preview_visible={} selected_preview_interaction_visible={} detail_tables_hidden={} scrollbar_thumb_bottom={} contract_rows_fit={} inspector_rows_fit={} tree_view_selected={} tree_view_settings_visible={} tree_view_line_option_visible={} tree_view_icon_option_visible={} tree_view_trigger_option_visible={} tree_view_action_logged={} panel_scrollbars_visible={} navigation_collapsed_pixels_changed={} legacy_preview_signatures={} legacy_preview_signature_collisions={}",
            self.required_ui,
            self.dedicated_ui,
            self.required_ui_fallbacks,
            self.initial_visible_fallbacks,
            self.modal_required,
            self.non_empty_pixels,
            self.theme_difference_pixels,
            self.operation_difference_pixels,
            self.selected_preview_visible,
            self.selected_preview_interaction_visible,
            self.detail_tables_hidden,
            self.scrollbar_thumb_bottom,
            self.contract_rows_fit,
            self.inspector_rows_fit,
            self.tree_view_selected,
            self.tree_view_settings_visible,
            self.tree_view_line_option_visible,
            self.tree_view_icon_option_visible,
            self.tree_view_trigger_option_visible,
            self.tree_view_action_logged,
            self.panel_scrollbars_visible,
            self.navigation_collapsed_pixels_changed,
            self.legacy_preview_signatures,
            self.legacy_preview_signature_collisions
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
    let markers = coverage_markers::build(examples);

    StorybookVisualCoverageReport {
        required_ui: StoryRequirements::required_pages().len(),
        dedicated_ui,
        required_ui_fallbacks: missing_pages.len(),
        initial_visible_fallbacks: initial_visible_fallbacks(examples),
        modal_required: StoryRequirements::required_pages().contains(&"modal"),
        non_empty_pixels: dark.non_background_pixels(palette::DEFAULT_BACKGROUND),
        theme_difference_pixels: pixel_difference(dark.pixels(), light.pixels()),
        operation_difference_pixels: pixel_difference(dark.pixels(), operation_after.pixels()),
        selected_preview_visible: markers.selected_preview_visible,
        selected_preview_interaction_visible: markers.selected_preview_interaction_visible,
        detail_tables_hidden: markers.detail_tables_hidden,
        scrollbar_thumb_bottom: markers.scrollbar_thumb_bottom,
        contract_rows_fit: markers.contract_rows_fit,
        inspector_rows_fit: markers.inspector_rows_fit,
        tree_view_selected: markers.tree_view_selected,
        tree_view_settings_visible: markers.tree_view_settings_visible,
        tree_view_line_option_visible: markers.tree_view_line_option_visible,
        tree_view_icon_option_visible: markers.tree_view_icon_option_visible,
        tree_view_trigger_option_visible: markers.tree_view_trigger_option_visible,
        tree_view_action_logged: markers.tree_view_action_logged,
        panel_scrollbars_visible: markers.panel_scrollbars_visible,
        navigation_collapsed_pixels_changed: markers.navigation_collapsed_pixels_changed,
        legacy_preview_signatures: markers.legacy_preview_signatures,
        legacy_preview_signature_collisions: markers.legacy_preview_signature_collisions,
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
            | UiNodeKind::Chip
            | UiNodeKind::Input
            | UiNodeKind::TextArea
            | UiNodeKind::Checkbox
            | UiNodeKind::Radio
            | UiNodeKind::SelectBox
            | UiNodeKind::Toggle
            | UiNodeKind::Divider
            | UiNodeKind::Spacer
            | UiNodeKind::KeyCap
            | UiNodeKind::Skeleton
            | UiNodeKind::LoadingDots
            | UiNodeKind::Spinner
            | UiNodeKind::Badge
            | UiNodeKind::ProgressBar
            | UiNodeKind::ColorSwatch
            | UiNodeKind::SlideControl
            | UiNodeKind::NotificationToast
            | UiNodeKind::Banner
            | UiNodeKind::ToastStackManager
            | UiNodeKind::Popover
            | UiNodeKind::HoverCard
            | UiNodeKind::Tooltip
            | UiNodeKind::AttachmentChip
            | UiNodeKind::ChipGroup
            | UiNodeKind::DiagnosticsList
            | UiNodeKind::EmptyState
            | UiNodeKind::Modal
            | UiNodeKind::ModalOverlay
            | UiNodeKind::CodeDiff
            | UiNodeKind::ColorPicker
            | UiNodeKind::TreeView
            | UiNodeKind::ContextMenu
            | UiNodeKind::CommandPalette
            | UiNodeKind::CommandResultRow
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
            | UiNodeKind::SearchControlStrip
            | UiNodeKind::SegmentedToggle
            | UiNodeKind::SelectionList
            | UiNodeKind::SideMenu
            | UiNodeKind::StatusBar
            | UiNodeKind::ShortcutCombo
            | UiNodeKind::ShortcutCheatsheet
            | UiNodeKind::SettingsList
            | UiNodeKind::CollapsiblePanel
            | UiNodeKind::VirtualizedList
            | UiNodeKind::VirtualizedTree
            | UiNodeKind::SkeletonCluster
            | UiNodeKind::MotionPrimitive
            | UiNodeKind::WindowControlButtonGroup
            | UiNodeKind::StartupStatePanel
            | UiNodeKind::Row
            | UiNodeKind::Column
            | UiNodeKind::Stack
            | UiNodeKind::Grid
            | UiNodeKind::ScrollArea
            | UiNodeKind::SplitPane
            | UiNodeKind::AlignCenter
    )
}
