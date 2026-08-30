use super::*;
use crate::text_surface::{TextSurfacePaintStyle, TextSurfaceRasterStyle};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeCapability, SearchControlCapabilities, SearchControlStrings,
    SearchResultSummaryTemplate,
};
use katana_ui_core::molecule::structured::SearchControlStrip;
use katana_ui_core::theme::{FontFamily, FontToken};

#[test]
fn synchronization_rebuilds_surfaces_for_identity_presentation_and_size_changes() {
    let initial_style = style(160, 24);
    let initial = strip("search-a", "Search", "Replace", false);
    let mut state = SearchSurfaceState::new(&initial, &initial_style);

    let replacement = strip("search-b", "Find", "Substitute", true);
    let replacement_style = style(240, 32);
    state.synchronize(&replacement, &replacement_style);

    assert_eq!(state.strip_state_id, "search-b");
    assert_eq!(state.query_presentation.visible, "Find");
    assert_eq!(state.replace_presentation.visible, "Substitute");
    assert_eq!(state.query.props().viewport.width, 240);
    assert_eq!(state.query.props().viewport.height, 32);
    assert_eq!(
        state.replace.props().disabled_reason.as_deref(),
        Some("read only")
    );

    let changed_presentation = strip("search-b", "Locate", "Rewrite", false);
    let changed_style = style(320, 40);
    state.synchronize(&changed_presentation, &changed_style);

    assert_eq!(state.query_presentation.visible, "Locate");
    assert_eq!(state.replace_presentation.visible, "Rewrite");
    assert_eq!(state.query.props().viewport.width, 320);
    assert_eq!(state.query.props().viewport.height, 40);
    assert_eq!(state.replace.props().viewport.width, 320);
    assert_eq!(state.replace.props().viewport.height, 40);
    assert!(!state.replace.state().text_area.disabled);
    assert_eq!(state.replace.props().disabled_reason, None);
}

fn strip(
    state_id: &str,
    query_label: &str,
    replace_label: &str,
    replace_unavailable: bool,
) -> CommandChromeSearchStrip {
    let replace = if replace_unavailable {
        CommandChromeCapability::unavailable("read only")
    } else {
        CommandChromeCapability::available()
    };
    CommandChromeSearchStrip::new(
        SearchControlStrip::new("Search")
            .stable_state_id(state_id)
            .query("needle")
            .replace_mode(ReplaceMode::Visible)
            .replace_value("replacement"),
        strings(query_label, replace_label),
    )
    .capabilities(SearchControlCapabilities {
        regex: CommandChromeCapability::available(),
        replace,
        navigation: CommandChromeCapability::available(),
        close: CommandChromeCapability::available(),
    })
}

fn strings(query: &str, replace: &str) -> SearchControlStrings {
    SearchControlStrings {
        strip: text("Search"),
        query: text(query),
        replace: text(replace),
        match_case: text("Match case"),
        whole_word: text("Whole word"),
        use_regex: text("Regex"),
        previous: text("Previous"),
        next: text("Next"),
        replace_one: text("Replace"),
        replace_all: text("Replace all"),
        close: text("Close"),
        result_summary: SearchResultSummaryTemplate {
            empty: String::new(),
            zero_results: "0".to_string(),
            single_result: "1".to_string(),
            indexed_result: "{active}/{count}".to_string(),
            count_results: "{count}".to_string(),
        },
    }
}

fn text(value: &str) -> CommandChromeText {
    CommandChromeText::new(value, value, value)
}

fn style(width: u32, height: u32) -> EguiCommandChromeSearchStyle {
    EguiCommandChromeSearchStyle {
        input_raster: TextSurfaceRasterStyle::new(
            FontToken {
                name: "search".to_string(),
                family: FontFamily::Monospace,
                size: 14.0,
                weight: 400,
            },
            [230, 230, 230, 255],
            20.0,
        ),
        input_paint: TextSurfacePaintStyle {
            background_rgba: [20, 20, 20, 255],
            gutter_background_rgba: [20, 20, 20, 255],
            gutter_paints: Vec::new(),
            selection_rgba: [64, 96, 160, 180],
            preedit_rgba: [255, 196, 64, 255],
            caret_rgba: [255, 255, 255, 255],
            annotation_paints: Vec::new(),
        },
        input_width_px: width,
        input_height_px: height,
        gap_px: 8,
        control_padding_px: 6,
        active_control_rgba: [64, 96, 160, 255],
    }
}
