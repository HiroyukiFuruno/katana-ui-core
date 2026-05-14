mod headless;
mod interaction;
mod modal_state;
mod pages;
mod sidebar;

use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, button, h_stack, label, scroll, v_stack};
use floem::window::WindowId;
use floem::{Application, IntoView, View};
use katana_ui_widget::theme::Theme;
use pages::accordion::accordion_page;
use pages::align_center::align_center_page;
use pages::badge::badge_page;
use pages::breadcrumb::breadcrumb_page;
use pages::card::card_page;
use pages::code_diff::code_diff_page;
use pages::color_picker_rgba::color_picker_rgba_page;
use pages::color_swatch::color_swatch_page;
use pages::combo_box::combo_box_page;
use pages::command_palette::command_palette_page;
use pages::dynamic_array_editor::dynamic_array_editor_page;
use pages::icon::icon_page;
use pages::icon_text_button::icon_text_button_page;
use pages::key_cap::key_cap_page;
use pages::loading_dots::loading_dots_page;
use pages::menu_button::menu_button_page;
use pages::modal_overlay::modal_page;
use pages::notification_toast::notification_toast_page;
use pages::popover::popover_page;
use pages::progress_bar::progress_bar_page;
use pages::search_box::search_box_page;
use pages::segmented_toggle::segmented_toggle_page;
use pages::select_box::select_box_page;
use pages::selection_list::selection_list_page;
use pages::side_menu::side_menu_page;
use pages::slide_control::slide_control_page;
use pages::spinner::spinner_page;
use pages::split_pane::split_pane_page;
use pages::status_bar::status_bar_page;
use pages::svg_button::svg_button_page;
use pages::tabs::tabs_page;
use pages::text::text_page;
use pages::text_button::text_button_page;
use pages::text_input::text_input_page;
use pages::theme_tokens::theme_tokens_page;
use pages::toggle::toggle_page;
use pages::toolbar::toolbar_page;
use pages::tooltip::tooltip_page;
use pages::tree_view::tree_view_page;
use pages::welcome::welcome_page;
use sidebar::sidebar_tree;
use std::time::Duration;

const DEFER_APP_STATE_MS: u64 = 1;
const THEME_STABLE_MARKER_MS: u64 = 100;

fn set_theme_deferred(is_dark: floem::reactive::RwSignal<bool>, next: bool) {
    floem::action::exec_after(Duration::from_millis(DEFER_APP_STATE_MS), move |_| {
        is_dark.set(next);
    });
}

fn set_page_deferred(current_page: floem::reactive::RwSignal<Page>, next: Page) {
    floem::action::exec_after(Duration::from_millis(DEFER_APP_STATE_MS), move |_| {
        current_page.set(next);
    });
}

fn theme_toggle_button(is_dark: floem::reactive::RwSignal<bool>) -> impl IntoView {
    button(label(move || if is_dark.get() { "Dark" } else { "Light" }))
        .action(move || {
            let next = !is_dark.get();
            set_theme_deferred(is_dark, next);
        })
        .style(|style| style.min_width(72.0))
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Page {
    Overview,
    ThemeTokens,
    Text,
    Icon,
    Spinner,
    SvgButton,
    TextButton,
    IconTextButton,
    Toggle,
    SegmentedToggle,
    SelectBox,
    ComboBox,
    ColorSwatch,
    ColorPickerRgba,
    TextInput,
    SearchBox,
    Tooltip,
    Badge,
    KeyCap,
    Card,
    Accordion,
    MenuButton,
    SideMenu,
    CommandPalette,
    SplitPane,
    Modal,
    Popover,
    AlignCenter,
    LoadingDots,
    Toolbar,
    ProgressBar,
    StatusBar,
    SelectionList,
    NotificationToast,
    SlideControl,
    Breadcrumb,
    DynamicArrayEditor,
    Tabs,
    TreeView,
    CodeDiff,
}

impl Page {
    fn initial() -> Self {
        std::env::var("KATANA_UI_WIDGET_STORYBOOK_PAGE")
            .ok()
            .and_then(|key| Self::from_key(&key))
            .unwrap_or(Self::Overview)
    }

    fn from_key(key: &str) -> Option<Self> {
        match key {
            "overview" => Some(Self::Overview),
            "theme-tokens" => Some(Self::ThemeTokens),
            "text" => Some(Self::Text),
            "icon" => Some(Self::Icon),
            "spinner" => Some(Self::Spinner),
            "svg-button" => Some(Self::SvgButton),
            "text-button" => Some(Self::TextButton),
            "icon-text-button" => Some(Self::IconTextButton),
            "toggle" => Some(Self::Toggle),
            "segmented-toggle" => Some(Self::SegmentedToggle),
            "select-box" => Some(Self::SelectBox),
            "combo-box" => Some(Self::ComboBox),
            "color-swatch" => Some(Self::ColorSwatch),
            "color-picker-rgba" => Some(Self::ColorPickerRgba),
            "text-input" => Some(Self::TextInput),
            "search-box" => Some(Self::SearchBox),
            "tooltip" => Some(Self::Tooltip),
            "badge" => Some(Self::Badge),
            "key-cap" => Some(Self::KeyCap),
            "card" => Some(Self::Card),
            "accordion" => Some(Self::Accordion),
            "menu-button" => Some(Self::MenuButton),
            "side-menu" => Some(Self::SideMenu),
            "command-palette" => Some(Self::CommandPalette),
            "split-pane" => Some(Self::SplitPane),
            "modal" | "modal-overlay" => Some(Self::Modal),
            "popover" => Some(Self::Popover),
            "align-center" => Some(Self::AlignCenter),
            "loading-dots" => Some(Self::LoadingDots),
            "toolbar" => Some(Self::Toolbar),
            "progress-bar" => Some(Self::ProgressBar),
            "status-bar" => Some(Self::StatusBar),
            "selection-list" => Some(Self::SelectionList),
            "notification-toast" => Some(Self::NotificationToast),
            "slide-control" => Some(Self::SlideControl),
            "breadcrumb" => Some(Self::Breadcrumb),
            "dynamic-array-editor" => Some(Self::DynamicArrayEditor),
            "tabs" => Some(Self::Tabs),
            "tree-view" => Some(Self::TreeView),
            "code-diff" => Some(Self::CodeDiff),
            _ => None,
        }
    }
}

fn app_view(main_window_id: WindowId) -> impl IntoView {
    let current_page = create_rw_signal(Page::initial());
    let is_dark = create_rw_signal(false);
    if interaction::requested("theme-toggle") {
        interaction::mark_supported("overview", "theme-toggle");
        interaction::schedule_replay(move || {
            set_theme_deferred(is_dark, true);
            floem::action::exec_after(Duration::from_millis(THEME_STABLE_MARKER_MS), move |_| {
                interaction::mark_exercised("overview", "theme-toggle", "dark-true");
            });
        });
    }
    if interaction::requested("select-page") {
        interaction::mark_supported("overview", "select-page");
        interaction::schedule_replay(move || {
            set_page_deferred(current_page, Page::SvgButton);
            floem::action::exec_after(Duration::from_millis(THEME_STABLE_MARKER_MS), move |_| {
                interaction::mark_exercised("overview", "select-page", "svg-button-selected");
            });
        });
    }

    let sidebar_nav = sidebar_tree(current_page, is_dark);

    let theme_switch = h_stack((
        label(|| "Theme").style(|s| s.font_size(13.0).margin_left(4.0)),
        theme_toggle_button(is_dark),
    ))
    .style(|s| s.gap(8.0).items_center().padding(8.0));

    let sidebar = scroll(v_stack((
        theme_switch,
        label(|| "Components").style(|s| {
            s.font_size(13.0)
                .margin_left(8.0)
                .margin_top(8.0)
                .margin_bottom(4.0)
        }),
        sidebar_nav.style(|s| {
            s.width_full()
                .min_width(0.0)
                .padding(4.0)
                .padding_right(18.0)
                .gap(4.0)
        }),
    )))
    .style(|s| s.width(230.0).min_width(230.0).height_full());

    let content = floem::views::dyn_container(
        move || (current_page.get(), is_dark.get()),
        move |(page, dark)| page_view(page, dark, Some(main_window_id)),
    )
    .style(|s| s.flex_grow(1.0).width_full().height_full().min_width(0.0));

    h_stack((sidebar, content)).style(|s| s.width_full().height_full())
}

fn page_view(page: Page, dark: bool, main_window_id: Option<WindowId>) -> Box<dyn View> {
    let theme = if dark {
        Theme::default_dark()
    } else {
        Theme::default_light()
    };
    theme.clone().provide();

    match page {
        Page::Overview => welcome_page().into_any(),
        Page::ThemeTokens => theme_tokens_page(theme).into_any(),
        Page::Text => text_page(theme).into_any(),
        Page::Icon => icon_page(theme).into_any(),
        Page::Spinner => spinner_page(theme).into_any(),
        Page::SvgButton => svg_button_page(theme).into_any(),
        Page::TextButton => text_button_page(theme).into_any(),
        Page::IconTextButton => icon_text_button_page(theme).into_any(),
        Page::Toggle => toggle_page(theme).into_any(),
        Page::SegmentedToggle => segmented_toggle_page(theme).into_any(),
        Page::SelectBox => select_box_page(theme).into_any(),
        Page::ComboBox => combo_box_page(theme).into_any(),
        Page::ColorSwatch => color_swatch_page(theme).into_any(),
        Page::ColorPickerRgba => color_picker_rgba_page(theme).into_any(),
        Page::TextInput => text_input_page(theme).into_any(),
        Page::SearchBox => search_box_page(theme).into_any(),
        Page::Tooltip => tooltip_page(theme).into_any(),
        Page::Badge => badge_page(theme).into_any(),
        Page::KeyCap => key_cap_page(theme).into_any(),
        Page::Card => card_page(theme).into_any(),
        Page::Accordion => accordion_page(theme).into_any(),
        Page::MenuButton => menu_button_page(theme).into_any(),
        Page::SideMenu => side_menu_page(theme).into_any(),
        Page::CommandPalette => command_palette_page(theme).into_any(),
        Page::SplitPane => split_pane_page(theme).into_any(),
        Page::Modal => modal_page(theme, main_window_id).into_any(),
        Page::Popover => popover_page(theme).into_any(),
        Page::AlignCenter => align_center_page(theme).into_any(),
        Page::LoadingDots => loading_dots_page(theme).into_any(),
        Page::Toolbar => toolbar_page(theme).into_any(),
        Page::ProgressBar => progress_bar_page(theme).into_any(),
        Page::StatusBar => status_bar_page(theme).into_any(),
        Page::SelectionList => selection_list_page(theme).into_any(),
        Page::NotificationToast => notification_toast_page(theme).into_any(),
        Page::SlideControl => slide_control_page(theme).into_any(),
        Page::Breadcrumb => breadcrumb_page(theme).into_any(),
        Page::DynamicArrayEditor => dynamic_array_editor_page(theme).into_any(),
        Page::Tabs => tabs_page(theme).into_any(),
        Page::TreeView => tree_view_page(theme).into_any(),
        Page::CodeDiff => code_diff_page(theme).into_any(),
    }
}

fn main() {
    headless::exit_if_requested();
    Application::new()
        .window(|window_id| app_view(window_id), None)
        .run();
}
