use super::canvas::Canvas;
use super::coverage;
use super::dedicated_atoms;
use super::dedicated_basic;
use super::dedicated_complex;
use super::dedicated_context_menu;
use super::dedicated_dod_atoms;
use super::dedicated_dod_forms;
use super::dedicated_dod_molecules;
use super::dedicated_feedback;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;
use katana_ui_core::render_model::{UiNode, UiNodeKind};

pub(super) struct DedicatedPageRequest<'a> {
    pub(super) text: &'a TextRenderer,
    pub(super) page: &'a str,
    pub(super) node: &'a UiNode,
    pub(super) palette: &'a VisualPalette,
    pub(super) scenario: ScenarioContext<'a>,
    pub(super) x: usize,
    pub(super) y: usize,
}

pub(super) fn draw_page(canvas: &mut Canvas, request: DedicatedPageRequest<'_>) {
    let DedicatedPageRequest {
        text,
        page,
        node,
        palette,
        scenario,
        x,
        y,
    } = request;
    match page {
        "theme-tokens" => dedicated_dod_atoms::theme(canvas, text, palette, scenario, x, y),
        "text" => dedicated_dod_atoms::text_grid(canvas, text, palette, scenario, x, y),
        "icon" => dedicated_dod_atoms::icon_grid(canvas, text, palette, scenario, x, y),
        "loading-dots" => dedicated_dod_atoms::loading_dots(canvas, text, palette, scenario, x, y),
        "spinner" => dedicated_dod_atoms::spinner(canvas, text, palette, scenario, x, y),
        "progress-bar" => dedicated_dod_atoms::progress(canvas, text, palette, scenario, x, y),
        "button" => {
            dedicated_dod_atoms::button_matrix(canvas, text, palette, scenario, x, y, "Button");
        }
        "text-button" => {
            dedicated_dod_atoms::button_matrix(canvas, text, palette, scenario, x, y, "TextButton");
        }
        "svg-button" => {
            dedicated_dod_atoms::button_matrix(canvas, text, palette, scenario, x, y, "SvgButton");
        }
        "icon-text-button" => {
            dedicated_dod_atoms::button_matrix(
                canvas,
                text,
                palette,
                scenario,
                x,
                y,
                "IconTextButton",
            );
        }
        "toggle" => dedicated_dod_atoms::toggle(canvas, text, palette, scenario, x, y),
        "segmented-toggle" => dedicated_dod_forms::segmented(canvas, text, palette, scenario, x, y),
        "select-box" => dedicated_dod_forms::select_box(canvas, text, palette, scenario, x, y),
        "color-swatch" => dedicated_dod_atoms::swatch(canvas, text, palette, scenario, x, y),
        "text-input" => dedicated_dod_forms::input(canvas, text, palette, scenario, x, y),
        "search-box" => dedicated_dod_forms::search(canvas, text, palette, scenario, x, y),
        "checkbox" => dedicated_dod_forms::checkbox(canvas, text, palette, scenario, x, y),
        "radio" => dedicated_dod_forms::radio(canvas, text, palette, scenario, x, y),
        "tooltip" => dedicated_dod_forms::tooltip(canvas, text, palette, scenario, x, y),
        "badge" => dedicated_dod_molecules::badge(canvas, text, palette, scenario, x, y),
        "key-cap" => dedicated_dod_molecules::key_cap(canvas, text, palette, scenario, x, y),
        "card" => dedicated_dod_molecules::card(canvas, text, palette, scenario, x, y),
        "accordion" => dedicated_dod_molecules::accordion(canvas, text, palette, scenario, x, y),
        "tree-view" => dedicated_dod_molecules::tree_view(canvas, text, node, palette, x, y),
        "context-menu" => {
            dedicated_context_menu::context_menu(canvas, text, palette, scenario, x, y);
        }
        "split-pane" => dedicated_dod_molecules::split_pane(canvas, text, palette, scenario, x, y),
        "modal" | "modal-overlay" => {
            dedicated_dod_molecules::modal(canvas, text, palette, scenario, x, y);
        }
        "popover" => dedicated_dod_forms::popover(canvas, text, palette, scenario, x, y),
        "color-picker-rgba" => {
            dedicated_dod_molecules::color_picker(canvas, text, palette, scenario, x, y);
        }
        "code-diff" => dedicated_dod_molecules::code_diff(canvas, text, palette, scenario, x, y),
        _ => draw(canvas, text, node, palette, x, y),
    }
}

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    let label = label_for(node.kind());
    match node.kind() {
        UiNodeKind::Button | UiNodeKind::TextButton | UiNodeKind::IconTextButton => {
            dedicated_basic::button(canvas, text, palette, x, y, label);
        }
        UiNodeKind::SvgButton => dedicated_atoms::icon_button(canvas, text, palette, x, y, label),
        UiNodeKind::Badge => dedicated_feedback::badge(canvas, text, palette, x, y, label),
        UiNodeKind::Input | UiNodeKind::SelectBox => {
            dedicated_basic::outlined_control(canvas, text, palette, x, y, label);
        }
        UiNodeKind::Checkbox | UiNodeKind::Radio => {
            dedicated_atoms::selection_control(canvas, text, palette, x, y, label);
        }
        UiNodeKind::Toggle => dedicated_basic::toggle(canvas, text, palette, x, y, label),
        UiNodeKind::Divider => dedicated_atoms::divider(canvas, text, palette, x, y, label),
        UiNodeKind::Spacer => dedicated_atoms::spacer(canvas, text, palette, x, y, label),
        UiNodeKind::KeyCap => dedicated_atoms::key_cap(canvas, text, palette, x, y, label),
        UiNodeKind::LoadingDots => {
            dedicated_atoms::loading_dots(canvas, text, palette, x, y, label)
        }
        UiNodeKind::Spinner => dedicated_atoms::spinner(canvas, text, palette, x, y, label),
        UiNodeKind::ProgressBar => {
            dedicated_feedback::progress(canvas, text, node, palette, x, y, label);
        }
        UiNodeKind::ColorSwatch => {
            dedicated_atoms::color_swatch(canvas, text, palette, x, y, label);
        }
        UiNodeKind::SlideControl => {
            dedicated_atoms::slide_control(canvas, text, palette, x, y, label)
        }
        UiNodeKind::CodeDiff => dedicated_complex::diff(canvas, text, palette, x, y, label),
        UiNodeKind::ColorPicker => {
            dedicated_complex::color_picker(canvas, text, palette, x, y, label);
        }
        UiNodeKind::Modal | UiNodeKind::ModalOverlay => {
            dedicated_feedback::overlay(canvas, text, palette, x, y, label);
        }
        kind if coverage::has_dedicated_renderer(kind) => {
            dedicated_basic::structured(canvas, text, palette, x, y, label);
        }
        _ => dedicated_basic::fallback(canvas, text, palette, x, y),
    }
}

fn label_for(kind: UiNodeKind) -> &'static str {
    match kind {
        UiNodeKind::Button | UiNodeKind::TextButton | UiNodeKind::IconTextButton => "button action",
        UiNodeKind::SvgButton => "svg icon action",
        UiNodeKind::Input => "input value",
        UiNodeKind::SelectBox => "select option",
        UiNodeKind::Checkbox => "checkbox state",
        UiNodeKind::Radio => "radio choice",
        UiNodeKind::Toggle => "toggle state",
        UiNodeKind::Badge => "badge status",
        UiNodeKind::Divider => "divider line",
        UiNodeKind::Spacer => "layout space",
        UiNodeKind::KeyCap => "shortcut key",
        UiNodeKind::LoadingDots => "loading dots",
        UiNodeKind::Spinner => "spinner loading",
        UiNodeKind::ProgressBar => "progress",
        UiNodeKind::ColorSwatch => "color swatch",
        UiNodeKind::SlideControl => "slider value",
        UiNodeKind::NotificationToast => "toast dismiss",
        UiNodeKind::Popover => "popover layer",
        UiNodeKind::Tooltip => "tooltip layer",
        UiNodeKind::Modal => "modal layer",
        UiNodeKind::ModalOverlay => "modal overlay",
        UiNodeKind::CodeDiff => "+/- diff",
        UiNodeKind::ColorPicker => "rgba picker",
        UiNodeKind::TreeView => "tree nodes",
        UiNodeKind::ContextMenu => "context menu",
        UiNodeKind::CommandPalette => "command list",
        UiNodeKind::DynamicArrayEditor => "array edit",
        UiNodeKind::Text => "text content",
        UiNodeKind::Icon => "icon glyph",
        UiNodeKind::Card => "card surface",
        UiNodeKind::List => "list rows",
        UiNodeKind::Menu => "menu items",
        UiNodeKind::Tabs => "tabs switch",
        UiNodeKind::Toolbar => "toolbar tools",
        UiNodeKind::FormField => "field group",
        UiNodeKind::Breadcrumb => "breadcrumb path",
        UiNodeKind::Accordion => "accordion panel",
        UiNodeKind::ComboBox => "combo input",
        UiNodeKind::MenuButton => "menu trigger",
        UiNodeKind::SearchBox => "search query",
        UiNodeKind::SegmentedToggle => "segments",
        UiNodeKind::SelectionList => "select list",
        UiNodeKind::SideMenu => "side menu",
        UiNodeKind::StatusBar => "status line",
        UiNodeKind::Row => "row layout",
        UiNodeKind::Column => "column layout",
        UiNodeKind::Stack => "stack layout",
        UiNodeKind::Grid => "grid layout",
        UiNodeKind::ScrollArea => "scroll area",
        UiNodeKind::SplitPane => "split pane",
        UiNodeKind::AlignCenter => "align center",
        _ => "node",
    }
}
