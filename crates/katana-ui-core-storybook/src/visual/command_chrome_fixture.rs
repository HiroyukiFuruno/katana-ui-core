use katana_ui_core::egui::command_chrome::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeSearchStyle,
};
use katana_ui_core::egui::text_surface::{TextSurfacePaintStyle, TextSurfaceRasterStyle};
use katana_ui_core::interaction::placement::Rect;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeCapability, CommandChromeDropdown, CommandChromeDropdownItem,
    CommandChromeDropdownTrigger, CommandChromeSearchPresentation, CommandChromeSearchStrip,
    CommandChromeText, CommandChromeToolbar, CommandChromeToolbarPresentation,
    FloatingCommandToolbar, SearchControlCapabilities, SearchControlStrings,
    SearchResultSummaryTemplate,
};
use katana_ui_core::molecule::structured::{ReplaceMode, SearchControlStrip, SearchOptions};
use katana_ui_core::render_model::UiNodeId;
use katana_ui_core::theme::{FontFamily, FontToken};

pub(super) const FRAME_WIDTH: f32 = 1280.0;
pub(super) const FRAME_HEIGHT: f32 = 720.0;
const CODE_DROPDOWN_ITEM_COUNT: usize = 17;
const FLOATING_ANCHOR_X: i32 = 400;
const FLOATING_ANCHOR_Y: i32 = 260;
const FLOATING_ANCHOR_WIDTH: u32 = 80;
const FLOATING_ANCHOR_HEIGHT: u32 = 24;
const SEARCH_RESULT_COUNT: usize = 3;
const SEARCH_ACTIVE_RESULT: usize = 1;
const FONT_SIZE_PX: f32 = 15.0;
const FONT_WEIGHT_NORMAL: u16 = 400;
const TEXT_LINE_HEIGHT_PX: f32 = 22.0;
const ICON_SIZE_PX: u32 = 16;
const INPUT_WIDTH_PX: u32 = 190;
const INPUT_HEIGHT_PX: u32 = 30;
const INPUT_GAP_PX: u32 = 6;
const CONTROL_PADDING_PX: u32 = 6;
const TEXT_RGBA: [u8; 4] = [235, 235, 235, 255];
const ACTION_RGBA: [u8; 4] = [36, 36, 38, 255];
const HOVERED_ACTION_RGBA: [u8; 4] = [62, 82, 108, 255];
const DISABLED_ACTION_RGBA: [u8; 4] = [48, 48, 50, 255];
const INPUT_BACKGROUND_RGBA: [u8; 4] = [28, 28, 30, 255];
const SELECTION_RGBA: [u8; 4] = [63, 91, 135, 255];
const PREEDIT_RGBA: [u8; 4] = [255, 196, 64, 255];
const CARET_RGBA: [u8; 4] = [255, 255, 255, 255];
const ACTIVE_CONTROL_RGBA: [u8; 4] = [58, 86, 128, 255];

pub(super) fn toolbar_fixture() -> CommandChromeToolbar {
    let mut dropdown = CommandChromeDropdown::new(CommandChromeDropdownTrigger::SplitSecondary);
    for index in 1..=CODE_DROPDOWN_ITEM_COUNT {
        dropdown = dropdown.item(CommandChromeDropdownItem::new(
            format!("code-{index:02}"),
            format!("候補 {index:02} ⭐️"),
        ));
    }
    CommandChromeToolbar::new()
        .action(action("inline-bold", "太字", false))
        .action(action("inline-italic", "斜体", false))
        .action(action("inline-strike", "取り消し", false))
        .action(action("inline-code", "コード", false))
        .action(action("heading-1", "見出し 1", false))
        .action(action("heading-2", "見出し 2", false))
        .action(action("heading-3", "見出し 3", false))
        .action(action("list-bullet", "箇条書き", false))
        .action(action("list-numbered", "番号付き", false))
        .action(action("quote", "引用", false))
        .action(
            action("code-block", "コード候補", false)
                .split(katana_ui_core::molecule::toolbar::SplitAction::new(
                    katana_ui_core::molecule::toolbar::SplitActionPart::new(),
                    katana_ui_core::molecule::toolbar::SplitActionPart::new(),
                ))
                .dropdown(dropdown),
        )
        .action(action("image-like", "画像を追加", false))
        .action(action("disabled", "利用不可", true))
}

pub(super) fn floating_toolbar_fixture() -> CommandChromeToolbar {
    let dropdown = CommandChromeDropdown::new(CommandChromeDropdownTrigger::SplitSecondary)
        .item(CommandChromeDropdownItem::new(
            "floating-markdown",
            "選択コード ⭐️",
        ))
        .item(CommandChromeDropdownItem::new(
            "floating-plain",
            "floating Plain",
        ));
    CommandChromeToolbar::new()
        .action(
            action("floating-code", "選択コード", false)
                .tooltip("選択コード ⭐️")
                .split(katana_ui_core::molecule::toolbar::SplitAction::new(
                    katana_ui_core::molecule::toolbar::SplitActionPart::new(),
                    katana_ui_core::molecule::toolbar::SplitActionPart::new(),
                ))
                .dropdown(dropdown),
        )
        .action(action("floating-bold", "選択ツール", false).tooltip("選択ツール ⭐️"))
        .action(action("floating-link", "リンク", false).tooltip("リンクを挿入"))
}

pub(super) fn floating_fixture() -> FloatingCommandToolbar {
    FloatingCommandToolbar::new_adapter_measured(
        floating_toolbar_fixture(),
        Rect::new(
            FLOATING_ANCHOR_X,
            FLOATING_ANCHOR_Y,
            FLOATING_ANCHOR_WIDTH,
            FLOATING_ANCHOR_HEIGHT,
        ),
        Rect::new(0, 0, FRAME_WIDTH as u32, FRAME_HEIGHT as u32),
    )
    .initial_visibility(
        katana_ui_core::molecule::command_chrome::FloatingCommandToolbarVisibility::Visible,
    )
    .focus_return_target(UiNodeId::new("storybook-return-target"))
}

pub(super) fn search_fixture(disable_regex: bool) -> CommandChromeSearchStrip {
    let strip = SearchControlStrip::new("検索と置換")
        .stable_state_id("storybook.command-chrome.search")
        .query("日本語 ⭐️")
        .replace_mode(ReplaceMode::Visible)
        .replace_value("置換後 ⭐️")
        .options(SearchOptions::default())
        .result_position(SEARCH_RESULT_COUNT, Some(SEARCH_ACTIVE_RESULT));
    let regex = if disable_regex {
        CommandChromeCapability::unavailable("regex capability unavailable")
    } else {
        CommandChromeCapability::available()
    };
    CommandChromeSearchStrip::new(strip, strings()).capabilities(SearchControlCapabilities {
        regex,
        replace: CommandChromeCapability::available(),
        navigation: CommandChromeCapability::available(),
        close: CommandChromeCapability::available(),
    })
}

pub(super) fn toolbar_presentation() -> CommandChromeToolbarPresentation {
    let toolbar = toolbar_fixture();
    CommandChromeToolbarPresentation {
        actions: toolbar.actions().to_vec(),
        groups: Vec::new(),
        display_mode: toolbar.display_mode_model(),
        density: Default::default(),
        overflow_strategy: Default::default(),
    }
}

pub(super) fn floating_toolbar_presentation() -> CommandChromeToolbarPresentation {
    let toolbar = floating_toolbar_fixture();
    CommandChromeToolbarPresentation {
        actions: toolbar.actions().to_vec(),
        groups: Vec::new(),
        display_mode: toolbar.display_mode_model(),
        density: Default::default(),
        overflow_strategy: Default::default(),
    }
}

pub(super) fn search_presentation() -> CommandChromeSearchPresentation {
    CommandChromeSearchPresentation {
        query: "日本語 ⭐️".to_string(),
        options: SearchOptions::default(),
        result_count: Some(SEARCH_RESULT_COUNT),
        active_index: Some(SEARCH_ACTIVE_RESULT),
        replace_mode: ReplaceMode::Visible,
        replace_value: "置換後 ⭐️".to_string(),
        strings: strings(),
        capabilities: SearchControlCapabilities::all_available(),
        icons: Default::default(),
    }
}

pub(super) fn raster_style() -> CommandChromeRasterStyle {
    let [red, green, blue, alpha] = TEXT_RGBA;
    CommandChromeRasterStyle {
        font: FontToken {
            name: "storybook-command-chrome".to_string(),
            family: FontFamily::Proportional,
            size: FONT_SIZE_PX,
            weight: FONT_WEIGHT_NORMAL,
        },
        text_color_rgba: TEXT_RGBA,
        icon_color: katana_ui_core::molecule::RgbaColor::new(red, green, blue, alpha),
        line_height_px: TEXT_LINE_HEIGHT_PX,
        icon_size_px: ICON_SIZE_PX,
    }
}

pub(super) const fn paint_style() -> CommandChromePaintStyle {
    CommandChromePaintStyle {
        action_rgba: ACTION_RGBA,
        hovered_action_rgba: HOVERED_ACTION_RGBA,
        disabled_action_rgba: DISABLED_ACTION_RGBA,
    }
}

pub(super) fn search_style() -> EguiCommandChromeSearchStyle {
    let input_raster =
        TextSurfaceRasterStyle::new(raster_style().font, TEXT_RGBA, TEXT_LINE_HEIGHT_PX);
    EguiCommandChromeSearchStyle {
        input_raster,
        input_paint: TextSurfacePaintStyle {
            background_rgba: INPUT_BACKGROUND_RGBA,
            gutter_background_rgba: INPUT_BACKGROUND_RGBA,
            gutter_paints: Vec::new(),
            selection_rgba: SELECTION_RGBA,
            preedit_rgba: PREEDIT_RGBA,
            caret_rgba: CARET_RGBA,
            annotation_paints: Vec::new(),
        },
        input_width_px: INPUT_WIDTH_PX,
        input_height_px: INPUT_HEIGHT_PX,
        gap_px: INPUT_GAP_PX,
        control_padding_px: CONTROL_PADDING_PX,
        active_control_rgba: ACTIVE_CONTROL_RGBA,
    }
}

fn action(id: &str, label: &str, disabled: bool) -> CommandChromeAction {
    CommandChromeAction::new(id, label)
        .accessibility_label(format!("{label} ⭐️"))
        .disabled(disabled)
}

fn strings() -> SearchControlStrings {
    let text = |visible: &str| CommandChromeText::new(visible, visible, format!("{visible} ⭐️"));
    SearchControlStrings {
        strip: text("検索と置換"),
        query: text("検索"),
        replace: text("置換"),
        match_case: text("大文字小文字"),
        whole_word: text("単語全体"),
        use_regex: text("正規表現"),
        previous: text("前へ"),
        next: text("次へ"),
        replace_one: text("置換"),
        replace_all: text("すべて置換"),
        close: text("閉じる"),
        result_summary: SearchResultSummaryTemplate {
            empty: String::new(),
            zero_results: "0".to_string(),
            single_result: "1 / 1".to_string(),
            indexed_result: "{active} / {count}".to_string(),
            count_results: "{count}".to_string(),
        },
    }
}
