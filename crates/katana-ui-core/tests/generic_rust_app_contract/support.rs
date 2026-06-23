use katana_ui_core::render_model::{
    UiIconProps, UiNode, UiNodeKind, UiSvgPaintPolicy, UiTextAreaWrapPolicy,
};
use katana_ui_core::widget::atoms::{Input, TextArea};
use katana_ui_core::widget::molecules::{CloseableTab, CloseableTabGroup, CloseableTabStrip};

const CALLER_SEARCH_SVG: &str =
    "<svg viewBox=\"0 0 16 16\"><circle cx=\"7\" cy=\"7\" r=\"4\"/><path d=\"M10 10l4 4\"/></svg>";
const CALLER_CLOSE_SVG: &str = "<svg viewBox=\"0 0 16 16\"><path d=\"M4 4l8 8M12 4l-8 8\"/></svg>";
const CALLER_TAB_SVG: &str = "<svg viewBox=\"0 0 16 16\"><path d=\"M3 3h10v10H3z\"/></svg>";

pub fn generic_tabs() -> CloseableTabStrip {
    CloseableTabStrip::new("workspace")
        .group(CloseableTabGroup::new("docs", "Docs"))
        .tab(
            CloseableTab::new("home", "Home")
                .pinned(true)
                .svg_icon(tab_icon()),
        )
        .tab(CloseableTab::new("editor", "Editor").group_id("docs"))
        .tab(CloseableTab::new("preview", "Preview"))
        .active_tab_id("editor")
}

pub fn search_input() -> Input {
    Input::new("Search")
        .placeholder("Search files")
        .value("src")
        .leading_icon_slot("Search icon", search_icon())
        .trailing_svg_icon_button("Clear", CALLER_CLOSE_SVG, "generic.search.clear")
        .trailing_svg_icon_button("Match case", CALLER_CLOSE_SVG, "generic.search.case")
}

pub fn notes_text_area() -> TextArea {
    TextArea::new("Notes")
        .placeholder("Write notes")
        .value("line 1\nline 2")
        .leading_icon_slot("Notes search", search_icon())
        .trailing_svg_icon_button("Clear", CALLER_CLOSE_SVG, "generic.notes.clear")
        .trailing_svg_icon_button("Format", CALLER_CLOSE_SVG, "generic.notes.format")
        .clear_action("Clear notes")
        .wrap_policy(UiTextAreaWrapPolicy::Soft)
        .resize_enabled(true)
        .vertical_scroll_enabled(true)
        .horizontal_scroll_enabled(true)
        .vertical_scrollbar_visible(true)
        .horizontal_scrollbar_visible(true)
}

pub fn assert_search_input_contract(node: &UiNode) {
    let text_entry = &node.props().text_entry;
    let leading_icon = text_entry
        .leading_slot
        .as_ref()
        .and_then(|slot| slot.icon.as_ref());

    assert_eq!("Search files", node.props().placeholder);
    assert_eq!("src", node.props().interaction.value);
    assert!(leading_icon.is_some());
    if let Some(icon) = leading_icon {
        assert_eq!(CALLER_SEARCH_SVG, icon.svg_source);
    }
    assert_eq!(2, text_entry.trailing_icon_buttons.len());
    assert_eq!(
        Some("generic.search.clear"),
        text_entry
            .trailing_icon_buttons
            .first()
            .and_then(|slot| slot.action.as_ref())
            .map(|action| action.callback.as_str())
    );
}

pub fn assert_text_area_contract(node: &UiNode) {
    let props = &node.props().text_area;
    let text_entry = &node.props().text_entry;

    assert_eq!("Write notes", node.props().placeholder);
    assert_eq!("line 1\nline 2", node.props().interaction.value);
    assert_eq!(
        Some(CALLER_SEARCH_SVG),
        text_entry
            .leading_slot
            .as_ref()
            .and_then(|slot| slot.icon.as_ref())
            .map(|icon| icon.svg_source.as_str())
    );
    assert_eq!(2, text_entry.trailing_icon_buttons.len());
    assert_eq!(
        Some("generic.notes.clear"),
        text_entry
            .trailing_icon_buttons
            .first()
            .and_then(|slot| slot.action.as_ref())
            .map(|action| action.callback.as_str())
    );
    assert_eq!(
        Some("Clear notes"),
        text_entry
            .clear_action
            .as_ref()
            .map(|action| action.label.as_str())
    );
    assert_eq!(UiTextAreaWrapPolicy::Soft, props.wrap_policy);
    assert!(props.resize_enabled);
    assert!(props.vertical_scroll_enabled);
    assert!(props.horizontal_scroll_enabled);
    assert!(props.vertical_scrollbar_visible);
    assert!(props.horizontal_scrollbar_visible);
}

pub fn assert_workspace_tab_contract(node: &UiNode) {
    let group = node
        .children()
        .iter()
        .find(|child| child.kind() == UiNodeKind::CloseableTabGroupHeader);
    let pinned = node.children().iter().find(|child| {
        child
            .props()
            .style_classes
            .iter()
            .any(|class| class == "closeable-tab-pinned")
    });

    assert!(
        group.is_some(),
        "closeable tab group header must be rendered"
    );
    assert!(pinned.is_some(), "pinned closeable tab must be rendered");
    let Some(group) = group else {
        return;
    };
    let Some(pinned) = pinned else {
        return;
    };

    assert_eq!(4, node.children().len());
    assert_eq!(UiNodeKind::CloseableTabGroupHeader, group.kind());
    assert_eq!(CALLER_TAB_SVG, pinned.props().icon.svg_source);
    assert_eq!("Docs group expanded", group.props().accessibility_label);
    assert!(
        pinned
            .props()
            .style_classes
            .iter()
            .any(|class| class == "closeable-tab-pinned")
    );
}

fn search_icon() -> UiIconProps {
    UiIconProps::new(CALLER_SEARCH_SVG)
        .role("search")
        .paint_policy(UiSvgPaintPolicy::CurrentColor)
}

fn tab_icon() -> UiIconProps {
    UiIconProps::new(CALLER_TAB_SVG)
        .role("workspace-tab")
        .paint_policy(UiSvgPaintPolicy::CurrentColor)
}
