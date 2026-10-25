use katana_ui_core::atom::TextArea;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeDropdown, CommandChromeDropdownItem,
    CommandChromeDropdownTrigger, CommandChromeFamilyId, CommandChromeToolbar,
};
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_selection::UiTextSelectionRange;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAnnotation, TextSurfaceAnnotationStyle, TextSurfaceGutter,
    TextSurfaceGutterRow, TextSurfaceProps, TextSurfaceViewport,
};

pub const FRAME_WIDTH: f32 = 1280.0;
pub const FRAME_HEIGHT: f32 = 720.0;
const GUTTER_WIDTH: u32 = 40;

pub fn floating_toolbar_fixture() -> CommandChromeToolbar {
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
        .command_family(CommandChromeFamilyId::new("fixture-floating"))
        .action(
            action("floating-code", "選択コード", false)
                .tooltip("選択コード ⭐️")
                .dropdown(dropdown),
        )
        .action(action("floating-bold", "選択ツール", false).tooltip("選択ツール ⭐️"))
        .action(action("floating-link", "リンク", false).tooltip("リンクを挿入"))
}

pub fn toolbar_fixture() -> CommandChromeToolbar {
    CommandChromeToolbar::new()
        .command_family(CommandChromeFamilyId::new("fixture-primary"))
        .action(action("heading-1", "見出し 1", false))
        .action(action("disabled", "利用不可", true))
}

pub fn text_surface_fixture() -> TextSurface {
    let text = "一行目: 日本語 ⭐️\n二行目: IME と selection\n三行目: gutter と annotation\n四行目: scroll target";
    TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("floating-caret.text-surface")
                .stable_state_id("floating-caret.text-surface")
                .value(text),
            UiTextSpan::emoji_marked_spans(text, Default::default()),
            TextSurfaceViewport::new(0, 0, 960, 288),
        )
        .accessibility_label("TextSurface story")
        .annotation(TextSurfaceAnnotation::new(
            "floating-caret-annotation",
            UiTextSelectionRange::new(8, 11),
            "floating-caret-annotation",
            TextSurfaceAnnotationStyle::Underline,
        ))
        .gutter(gutter()),
    )
}

fn gutter() -> TextSurfaceGutter {
    let mut gutter = TextSurfaceGutter::new(GUTTER_WIDTH);
    for row in 0..4 {
        gutter
            .rows
            .push(TextSurfaceGutterRow::new(row, (row + 1).to_string()));
    }
    gutter
}

fn action(id: &str, label: &str, disabled: bool) -> CommandChromeAction {
    CommandChromeAction::new(id, label)
        .accessibility_label(format!("{label} ⭐️"))
        .disabled(disabled)
}
