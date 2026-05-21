use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::{
    inspector_rows, layout_metrics, navigation_tree, palette, panel_scroll_state, panel_scrollbars,
    preview_contract, preview_contract_rows, preview_detail, render, scrollbar,
};
use crate::catalog::StoryExample;
use std::collections::BTreeMap;

const MIN_NAV_COLLAPSE_DIFF: usize = 1_000;
const PREVIEW_SIGNATURE_SEED: u64 = 17;
const PREVIEW_SIGNATURE_PRIME: u64 = 1_099_511_628_211;
const LEGACY_DETAIL_TABLE_Y: usize = 398;
const LEGACY_DETAIL_TABLE_SAMPLE_OFFSET: usize = 10;
const LEGACY_DOD_PREVIEW_PAGES: &[&str] = &[
    "theme-tokens",
    "text",
    "icon",
    "chip",
    "loading-dots",
    "spinner",
    "button",
    "text-button",
    "svg-button",
    "icon-text-button",
    "toggle",
    "segmented-toggle",
    "select-box",
    "color-swatch",
    "text-input",
    "search-box",
    "tooltip",
    "badge",
    "key-cap",
    "card",
    "accordion",
    "split-pane",
    "modal",
    "popover",
    "color-picker-rgba",
    "code-diff",
    "attachment-chip",
    "chip-group",
];

pub(super) struct CoverageMarkers {
    pub(super) selected_preview_visible: bool,
    pub(super) selected_preview_interaction_visible: bool,
    pub(super) detail_tables_hidden: bool,
    pub(super) scrollbar_thumb_bottom: bool,
    pub(super) contract_rows_fit: bool,
    pub(super) inspector_rows_fit: bool,
    pub(super) tree_view_selected: bool,
    pub(super) tree_view_settings_visible: bool,
    pub(super) tree_view_line_option_visible: bool,
    pub(super) tree_view_icon_option_visible: bool,
    pub(super) tree_view_trigger_option_visible: bool,
    pub(super) tree_view_action_logged: bool,
    pub(super) panel_scrollbars_visible: bool,
    pub(super) navigation_collapsed_pixels_changed: usize,
    pub(super) legacy_preview_signatures: usize,
    pub(super) legacy_preview_signature_collisions: usize,
}

pub(super) fn build(examples: &[StoryExample]) -> CoverageMarkers {
    let tree_view = examples.iter().find(|it| it.page == "tree-view");
    let legacy_preview = legacy_preview_signature_stats();
    CoverageMarkers {
        selected_preview_visible: selected_preview_visible(),
        selected_preview_interaction_visible: selected_preview_interaction_visible(),
        detail_tables_hidden: detail_tables_hidden(),
        scrollbar_thumb_bottom: scrollbar_thumb_bottom(),
        contract_rows_fit: preview_contract_rows::rows_fit(examples),
        inspector_rows_fit: inspector_rows::rows_fit(examples),
        tree_view_selected: tree_view.is_some(),
        tree_view_settings_visible: tree_view_option_visible(tree_view, "context menu: enabled"),
        tree_view_line_option_visible: tree_view_option_visible(
            tree_view,
            "line: solid 1px enabled",
        ),
        tree_view_icon_option_visible: tree_view_option_visible(
            tree_view,
            "icons: folder/file visible",
        ),
        tree_view_trigger_option_visible: tree_view_option_visible(
            tree_view,
            "trigger: icon+text chevron",
        ),
        tree_view_action_logged: tree_view.is_some_and(|it| !it.callback_logs.is_empty()),
        panel_scrollbars_visible: panel_scrollbars_visible(),
        navigation_collapsed_pixels_changed: navigation_collapsed_pixels_changed(),
        legacy_preview_signatures: legacy_preview.signatures,
        legacy_preview_signature_collisions: legacy_preview.collisions,
    }
}

fn panel_scrollbars_visible() -> bool {
    let canvas = render::render_storybook_canvas_for("dark", "panel", false);
    let accent =
        palette::VisualPalette::from_theme(&katana_ui_core::theme::ThemeSnapshot::dark()).accent;
    panel_scrollbar_center_is_accent(
        &canvas,
        accent,
        panel_scroll_state::PanelScrollRegion::Navigation,
    ) && panel_scrollbar_center_is_accent(
        &canvas,
        accent,
        panel_scroll_state::PanelScrollRegion::Preview,
    ) && panel_scrollbar_center_is_accent(
        &canvas,
        accent,
        panel_scroll_state::PanelScrollRegion::Inspector,
    )
}

fn panel_scrollbar_center_is_accent(
    canvas: &super::Canvas,
    accent: u32,
    region: panel_scroll_state::PanelScrollRegion,
) -> bool {
    let thumb =
        panel_scrollbars::thumb_rect_for(region, panel_scroll_state::PanelScrollOffsets::default());
    pixel_at(
        canvas,
        thumb.x + thumb.width / 2,
        thumb.y + thumb.height / 2,
    ) == Some(accent)
}

fn pixel_at(canvas: &super::Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}

fn selected_preview_visible() -> bool {
    visible_in_initial_viewport(preview_detail::selected_hero_y())
}

fn selected_preview_interaction_visible() -> bool {
    visible_in_initial_viewport(preview_detail::component_action_hit_rect("button").bottom())
}

fn visible_in_initial_viewport(content_y: usize) -> bool {
    content_y < render::VIEWPORT_HEIGHT
}

fn detail_tables_hidden() -> bool {
    let canvas = render::render_storybook_canvas_for("dark", "button", false);
    let palette = palette::VisualPalette::from_theme(&katana_ui_core::theme::ThemeSnapshot::dark());
    let sample = pixel_at(
        &canvas,
        layout_metrics::PREVIEW_X + LEGACY_DETAIL_TABLE_SAMPLE_OFFSET,
        LEGACY_DETAIL_TABLE_Y + LEGACY_DETAIL_TABLE_SAMPLE_OFFSET,
    );
    sample != Some(palette.code_background) && sample != Some(palette.accent)
}

fn scrollbar_thumb_bottom() -> bool {
    let track = scrollbar::track_rect();
    let thumb = scrollbar::thumb_rect(super::layout_metrics::MAX_SCROLL_Y);
    thumb.bottom() == track.bottom()
}

fn tree_view_option_visible(tree_view: Option<&StoryExample>, expected: &str) -> bool {
    let scenario = ScenarioContext {
        selected_page: "tree-view",
        preset_index: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        screen_state: StorybookScreenState::default(),
    };
    tree_view.is_some_and(|example| {
        inspector_rows::settings_rows(example.tree.root(), example, scenario)
            .iter()
            .any(|it| it == expected)
    })
}

fn navigation_collapsed_pixels_changed() -> usize {
    let open = render::render_storybook_canvas_for("dark", "button", false);
    let mut collapsed = navigation_tree::TreeExpansionState::default();
    collapsed.toggle(navigation_tree::NavigationGroup::Atoms);
    let closed = render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: "dark",
        selected_page: "button",
        preset_index: 0,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: panel_scroll_state::PanelScrollOffsets::default(),
        tree_expansion: collapsed,
        screen_state: StorybookScreenState::default(),
    });
    let diff = pixel_difference(open.pixels(), closed.pixels());
    if diff > MIN_NAV_COLLAPSE_DIFF {
        return diff;
    }
    0
}

fn pixel_difference(left: &[u32], right: &[u32]) -> usize {
    left.iter()
        .zip(right.iter())
        .filter(|(left, right)| left != right)
        .count()
}

fn legacy_preview_signature_stats() -> LegacyPreviewSignatureStats {
    let mut signatures = BTreeMap::new();
    let mut collisions = 0;
    for page in LEGACY_DOD_PREVIEW_PAGES {
        let canvas = render::render_storybook_canvas_for("dark", page, false);
        let signature = hero_preview_signature(&canvas);
        if signatures.insert(signature, *page).is_some() {
            collisions += 1;
        }
    }
    LegacyPreviewSignatureStats {
        signatures: signatures.len(),
        collisions,
    }
}

fn hero_preview_signature(canvas: &super::Canvas) -> u64 {
    let (x, y, width, height) = preview_contract::selected_detail_rect();
    let mut signature = PREVIEW_SIGNATURE_SEED;
    for current_y in y..y + height {
        for current_x in x..x + width {
            let index = current_y * canvas.width() + current_x;
            let pixel = u64::from(canvas.pixels()[index]);
            signature ^= pixel.wrapping_add(index as u64);
            signature = signature.wrapping_mul(PREVIEW_SIGNATURE_PRIME);
        }
    }
    signature
}

struct LegacyPreviewSignatureStats {
    signatures: usize,
    collisions: usize,
}
