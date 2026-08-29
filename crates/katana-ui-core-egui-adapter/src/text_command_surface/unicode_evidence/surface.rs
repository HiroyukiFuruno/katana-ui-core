use super::constants::{
    ALPHA_CHANNEL_INDEX, INITIAL_TEXT, SEARCH_CONTROL_PADDING_PX, SEARCH_GAP_PX,
    SEARCH_INPUT_HEIGHT_PX, SEARCH_INPUT_WIDTH_PX, TRACE_BACKGROUND, TRACE_CARET, TRACE_FONT_SIZE,
    TRACE_FONT_WEIGHT, TRACE_GUTTER_BACKGROUND, TRACE_HEIGHT, TRACE_ICON_COLOR, TRACE_ICON_SIZE_PX,
    TRACE_LINE_HEIGHT, TRACE_PREEDIT, TRACE_SELECTION, TRACE_TEXT_COLOR, TRACE_WIDTH,
};
use crate::command_chrome::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeSearchStyle,
};
use crate::text_command_surface::TextCommandSurfaceStyle;
use crate::text_surface::{TextSurfacePaintStyle, TextSurfaceRasterStyle};
use katana_ui_core::atom::TextArea;
use katana_ui_core::molecule::RgbaColor;
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_surface::{TextSurface, TextSurfaceProps, TextSurfaceViewport};
use katana_ui_core::theme::{FontFamily, FontToken};
use katana_ui_core_text_raster::PlatformTextGraphemeRange;

pub(super) fn evidence_surface() -> TextSurface {
    let area = TextArea::new("KUC Unicode evidence")
        .stable_state_id("kuc.unicode-color-glyph-evidence.text")
        .value(INITIAL_TEXT)
        .ime_enabled(true);
    TextSurface::new(TextSurfaceProps::new(
        area,
        explicit_spans(INITIAL_TEXT),
        TextSurfaceViewport::new(0, 0, TRACE_WIDTH as u32, TRACE_HEIGHT as u32),
    ))
}

pub(super) fn explicit_spans(text: &str) -> Vec<UiTextSpan> {
    let mut spans = Vec::new();
    let mut cursor = 0;
    for range in PlatformTextGraphemeRange::ranges(text) {
        let Some(grapheme) = text.get(range.byte_start..range.byte_end) else {
            continue;
        };
        if range.byte_start > cursor {
            spans.push(UiTextSpan::plain(&text[cursor..range.byte_start]));
        }
        if grapheme == super::constants::STAR_TEXT || grapheme == super::constants::ZWJ_TEXT {
            spans.push(UiTextSpan::emoji(grapheme));
        } else {
            spans.push(UiTextSpan::plain(grapheme));
        }
        cursor = range.byte_end;
    }
    if cursor < text.len() {
        spans.push(UiTextSpan::plain(&text[cursor..]));
    }
    spans
}

pub(super) fn trace_style() -> TextCommandSurfaceStyle {
    let font = FontToken {
        name: "kuc-unicode-evidence".to_string(),
        family: FontFamily::Proportional,
        size: TRACE_FONT_SIZE,
        weight: TRACE_FONT_WEIGHT,
    };
    TextCommandSurfaceStyle {
        text_raster: TextSurfaceRasterStyle::new(font.clone(), TRACE_TEXT_COLOR, TRACE_LINE_HEIGHT),
        text_paint: TextSurfacePaintStyle {
            background_rgba: TRACE_BACKGROUND,
            gutter_background_rgba: TRACE_GUTTER_BACKGROUND,
            gutter_paints: Vec::new(),
            selection_rgba: TRACE_SELECTION,
            preedit_rgba: TRACE_PREEDIT,
            caret_rgba: TRACE_CARET,
            annotation_paints: Vec::new(),
        },
        chrome_raster: CommandChromeRasterStyle {
            font,
            text_color_rgba: TRACE_TEXT_COLOR,
            icon_color: RgbaColor::new(
                TRACE_ICON_COLOR[0],
                TRACE_ICON_COLOR[1],
                TRACE_ICON_COLOR[2],
                TRACE_ICON_COLOR[ALPHA_CHANNEL_INDEX],
            ),
            line_height_px: TRACE_LINE_HEIGHT,
            icon_size_px: TRACE_ICON_SIZE_PX,
        },
        chrome_paint: CommandChromePaintStyle {
            action_rgba: TRACE_BACKGROUND,
            hovered_action_rgba: TRACE_SELECTION,
            disabled_action_rgba: TRACE_GUTTER_BACKGROUND,
        },
        search: EguiCommandChromeSearchStyle {
            input_raster: TextSurfaceRasterStyle::new(
                FontToken {
                    name: "kuc-unicode-evidence-search".to_string(),
                    family: FontFamily::Proportional,
                    size: TRACE_FONT_SIZE,
                    weight: TRACE_FONT_WEIGHT,
                },
                TRACE_TEXT_COLOR,
                TRACE_LINE_HEIGHT,
            ),
            input_paint: TextSurfacePaintStyle {
                background_rgba: TRACE_BACKGROUND,
                gutter_background_rgba: TRACE_GUTTER_BACKGROUND,
                gutter_paints: Vec::new(),
                selection_rgba: TRACE_SELECTION,
                preedit_rgba: TRACE_PREEDIT,
                caret_rgba: TRACE_CARET,
                annotation_paints: Vec::new(),
            },
            input_width_px: SEARCH_INPUT_WIDTH_PX,
            input_height_px: SEARCH_INPUT_HEIGHT_PX,
            gap_px: SEARCH_GAP_PX,
            control_padding_px: SEARCH_CONTROL_PADDING_PX,
            active_control_rgba: TRACE_SELECTION,
        },
    }
}
