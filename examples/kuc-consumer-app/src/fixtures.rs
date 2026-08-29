use katana_ui_core::layout::{Column, Length, Row, ScrollArea, ScrollAxis, ScrollbarVisibility};
use katana_ui_core::render_model::{
    UiIconProps, UiImageSurfaceFit, UiImageSurfaceProps, UiImageSurfaceTransform, UiSvgPaintPolicy,
};
use katana_ui_core::widget::atoms::{
    Button, Icon, IconTextButton, ImageSurface, Input, SvgButton, Text, TextArea, TextButton,
};
use katana_ui_core::widget::molecules::{
    Accordion, ChoiceItem, CloseableTab, CloseableTabGroup, CloseableTabStrip, ComboBox,
    ContextMenu, ContextMenuItem, GenericGrid, GridCoordinate, GridTrackSizeProvider, GridViewport,
    MenuButton, SearchBox, SelectBox, SelectionList, Toolbar,
};

const SEARCH_SVG: &str =
    "<svg viewBox=\"0 0 16 16\"><circle cx=\"7\" cy=\"7\" r=\"4\"/><path d=\"M10 10l4 4\"/></svg>";
const CLOSE_SVG: &str = "<svg viewBox=\"0 0 16 16\"><path d=\"M4 4l8 8M12 4l-8 8\"/></svg>";
const DOC_SVG: &str = "<svg viewBox=\"0 0 16 16\"><path d=\"M4 2h7l2 2v10H4z\"/></svg>";

pub(crate) fn search_input() -> Input {
    Input::new("Search")
        .placeholder("Search files")
        .leading_icon_slot("Search icon", search_icon())
        .trailing_svg_icon_button("Clear", CLOSE_SVG, "consumer.search.clear")
}

pub(crate) fn quick_search_box() -> SearchBox {
    SearchBox::new("Quick search")
        .placeholder("Search symbols")
        .value("main")
        .clear_action("Clear")
        .submit_on_enter(true)
}

pub(crate) fn workspace_select() -> SelectBox {
    SelectBox::new("Workspace")
        .placeholder("Workspace")
        .item(ChoiceItem::new("source", "Source"))
        .item(ChoiceItem::new("tests", "Tests"))
        .item(ChoiceItem::new("docs", "Docs"))
        .selected_index(0)
}

pub(crate) fn symbol_combo() -> ComboBox {
    ComboBox::new("Symbol")
        .placeholder("Symbol")
        .item(ChoiceItem::new("main", "main"))
        .item(ChoiceItem::new("render", "render"))
        .item(ChoiceItem::new("tests", "tests"))
        .input_value("ma")
        .free_input(true)
}

pub(crate) fn notes_area() -> TextArea {
    TextArea::new("Notes")
        .placeholder("Write notes")
        .value("first line")
        .resize_enabled(true)
        .vertical_scroll_enabled(true)
        .vertical_scrollbar_visible(true)
}

pub(crate) fn preview_surface() -> ImageSurface {
    ImageSurface::new(
        "Preview image",
        UiImageSurfaceProps {
            fingerprint: "consumer-preview-surface".to_string(),
            width: 1,
            height: 1,
            display_width: 1,
            display_height: 1,
            display_width_milli: 1000,
            display_height_milli: 1000,
            rgba: vec![24, 48, 96, 255],
            content_scale: 100,
            fit: UiImageSurfaceFit::Contain,
            accessibility_label: "Consumer preview image".to_string(),
            selection_text: String::new(),
            highlight_rects: Vec::new(),
            transform: UiImageSurfaceTransform::default(),
        },
    )
}

pub(crate) fn workspace_tabs() -> CloseableTabStrip {
    CloseableTabStrip::new("Documents")
        .group(CloseableTabGroup::new("source", "Source"))
        .tab(
            CloseableTab::new("home", "Home")
                .pinned(true)
                .svg_icon(doc_icon()),
        )
        .tab(
            CloseableTab::new("editor", "Editor")
                .group_id("source")
                .svg_icon(doc_icon()),
        )
        .active_tab_id("editor")
}

pub(crate) fn navigation_list() -> SelectionList {
    SelectionList::new("Navigation")
        .item(ChoiceItem::new("home", "Home"))
        .item(ChoiceItem::new("editor", "Editor"))
        .item(ChoiceItem::new("settings", "Settings"))
        .selected_index(0)
}

pub(crate) fn navigation_scroll() -> ScrollArea {
    ScrollArea::new()
        .axis(ScrollAxis::Vertical)
        .viewport(220, 480)
        .content_extent(220, 960)
        .scrollbar_visibility(ScrollbarVisibility::Auto)
}

pub(crate) fn data_grid() -> GenericGrid {
    GenericGrid::new("Large data grid", 1_000, 100)
        .row_tracks(GridTrackSizeProvider::fixed(24))
        .column_tracks(GridTrackSizeProvider::fixed(96))
        .viewport(GridViewport::new(480, 240).scroll(192, 240))
        .overscan(2, 1)
        .frozen(1, 1)
        .active_cell(GridCoordinate::new(10, 2))
}

pub(crate) fn main_panel(
    tabs: CloseableTabStrip,
    search: Input,
    quick_search: SearchBox,
    workspace_select: SelectBox,
    symbol_combo: ComboBox,
    notes: TextArea,
    preview: ImageSurface,
    data_grid: GenericGrid,
) -> Column {
    Column::new()
        .gap(Length::px(8.0))
        .child(tabs)
        .child(
            Toolbar::new("Document toolbar")
                .child(Icon::new("File icon").svg_icon(doc_icon()))
                .child(search)
                .child(quick_search)
                .child(workspace_select)
                .child(symbol_combo)
                .child(SvgButton::new("Find").svg_icon(search_icon()))
                .child(TextButton::new("Preview").command("consumer.preview.open"))
                .child(IconTextButton::new("Open").command("consumer.file.open"))
                .child(MenuButton::new("More").item(ChoiceItem::new("copy", "Copy path")))
                .child(Button::new("Run"))
                .child(Button::new("Save")),
        )
        .child(
            Row::new()
                .gap(Length::px(6.0))
                .child(Text::new("Status: ready")),
        )
        .child(notes)
        .child(preview)
        .child(data_grid)
        .child(Accordion::new("Details").child(Text::new("Open document metadata")))
        .child(ContextMenu::new("Tab context").item(ContextMenuItem::action("close", "Close")))
}

pub(crate) fn doc_icon() -> UiIconProps {
    UiIconProps::new(DOC_SVG)
        .role("document")
        .paint_policy(UiSvgPaintPolicy::CurrentColor)
}

fn search_icon() -> UiIconProps {
    UiIconProps::new(SEARCH_SVG)
        .role("search")
        .paint_policy(UiSvgPaintPolicy::CurrentColor)
}
