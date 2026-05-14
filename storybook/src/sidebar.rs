use floem::IntoView;
use floem::action::exec_after;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::views::Decorators;
use katana_ui_widget::composite::tree_view::{TreeView, TreeViewExpandTrigger, TreeViewItem};
use katana_ui_widget::primitive::icon::IconSource;
use katana_ui_widget::theme::Theme;
use std::time::Duration;

use crate::Page;

const SIDEBAR_ROW_HEIGHT: f32 = 28.0;
const DEFER_PAGE_SELECT_MS: u64 = 1;
const FOLDER_ICON: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><path d='M2.5 4.5h4l1.2 1.5h5.8v6.5h-11z' fill='none' stroke='currentColor' stroke-width='1.6' stroke-linejoin='round'/></svg>";

#[derive(Clone, Copy)]
struct SidebarEntry {
    label: &'static str,
    page: Page,
}

struct SidebarSection {
    label: &'static str,
    entries: &'static [SidebarEntry],
}

const fn entry(label: &'static str, page: Page) -> SidebarEntry {
    SidebarEntry { label, page }
}

const OVERVIEW_ENTRIES: &[SidebarEntry] = &[
    entry("Overview", Page::Overview),
    entry("Theme Tokens", Page::ThemeTokens),
];

const PRIMITIVE_ENTRIES: &[SidebarEntry] = &[
    entry("Text", Page::Text),
    entry("Icon", Page::Icon),
    entry("Spinner", Page::Spinner),
    entry("LoadingDots", Page::LoadingDots),
];

const BUTTON_ENTRIES: &[SidebarEntry] = &[
    entry("SvgButton", Page::SvgButton),
    entry("TextButton", Page::TextButton),
    entry("IconTextButton", Page::IconTextButton),
    entry("MenuButton", Page::MenuButton),
];

const INPUT_ENTRIES: &[SidebarEntry] = &[
    entry("Toggle", Page::Toggle),
    entry("SegmentedToggle", Page::SegmentedToggle),
    entry("SelectBox", Page::SelectBox),
    entry("ComboBox", Page::ComboBox),
    entry("TextInput", Page::TextInput),
    entry("SearchBox", Page::SearchBox),
    entry("SelectionList", Page::SelectionList),
    entry("SlideControl", Page::SlideControl),
    entry("DynamicArrayEditor", Page::DynamicArrayEditor),
];

const COLOR_ENTRIES: &[SidebarEntry] = &[
    entry("ColorSwatch", Page::ColorSwatch),
    entry("ColorPickerRgba", Page::ColorPickerRgba),
];

const FEEDBACK_ENTRIES: &[SidebarEntry] = &[
    entry("Tooltip", Page::Tooltip),
    entry("Badge", Page::Badge),
    entry("KeyCap", Page::KeyCap),
    entry("ProgressBar", Page::ProgressBar),
    entry("StatusBar", Page::StatusBar),
    entry("NotificationToast", Page::NotificationToast),
];

const LAYOUT_ENTRIES: &[SidebarEntry] = &[
    entry("Card", Page::Card),
    entry("Accordion", Page::Accordion),
    entry("SideMenu", Page::SideMenu),
    entry("CommandPalette", Page::CommandPalette),
    entry("SplitPane", Page::SplitPane),
    entry("Modal", Page::Modal),
    entry("Popover", Page::Popover),
    entry("AlignCenter", Page::AlignCenter),
    entry("Toolbar", Page::Toolbar),
    entry("Breadcrumb", Page::Breadcrumb),
    entry("Tabs", Page::Tabs),
    entry("TreeView", Page::TreeView),
    entry("CodeDiff", Page::CodeDiff),
];

const SECTIONS: &[SidebarSection] = &[
    SidebarSection {
        label: "Overview",
        entries: OVERVIEW_ENTRIES,
    },
    SidebarSection {
        label: "Primitive",
        entries: PRIMITIVE_ENTRIES,
    },
    SidebarSection {
        label: "Button",
        entries: BUTTON_ENTRIES,
    },
    SidebarSection {
        label: "Input",
        entries: INPUT_ENTRIES,
    },
    SidebarSection {
        label: "Color",
        entries: COLOR_ENTRIES,
    },
    SidebarSection {
        label: "Feedback",
        entries: FEEDBACK_ENTRIES,
    },
    SidebarSection {
        label: "Layout",
        entries: LAYOUT_ENTRIES,
    },
];

pub(crate) fn sidebar_tree(current_page: RwSignal<Page>, is_dark: RwSignal<bool>) -> impl IntoView {
    floem::views::dyn_container(
        move || (current_page.get(), is_dark.get()),
        move |(_, dark)| {
            let theme = if dark {
                Theme::default_dark()
            } else {
                Theme::default_light()
            };

            TreeView::new(sidebar_items(current_page))
                .expand_trigger(TreeViewExpandTrigger::IconAndLabel)
                .show_indent_lines(true)
                .row_height(SIDEBAR_ROW_HEIGHT)
                .view(theme)
                .style(|style| style.width_full())
                .into_any()
        },
    )
}

fn sidebar_items(current_page: RwSignal<Page>) -> Vec<TreeViewItem> {
    SECTIONS
        .iter()
        .map(|section| sidebar_section(section, current_page))
        .collect()
}

fn sidebar_section(section: &SidebarSection, current_page: RwSignal<Page>) -> TreeViewItem {
    TreeViewItem::new(section.label)
        .icon(IconSource::SvgBytes(FOLDER_ICON))
        .expanded(true)
        .children(sidebar_pages(section.entries, current_page))
}

fn sidebar_pages(entries: &[SidebarEntry], current_page: RwSignal<Page>) -> Vec<TreeViewItem> {
    entries
        .iter()
        .map(|entry| sidebar_page(*entry, current_page))
        .collect()
}

fn sidebar_page(entry: SidebarEntry, current_page: RwSignal<Page>) -> TreeViewItem {
    TreeViewItem::new(entry.label)
        .active(current_page.get() == entry.page)
        .on_select(move || {
            exec_after(Duration::from_millis(DEFER_PAGE_SELECT_MS), move |_| {
                current_page.set(entry.page);
            });
        })
}
