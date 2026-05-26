use super::canvas::Canvas;
use super::coverage;
use super::dedicated_atoms;
use super::dedicated_attachment_chip;
use super::dedicated_banner;
use super::dedicated_basic;
use super::dedicated_breadcrumb;
use super::dedicated_chip;
use super::dedicated_chip_group;
use super::dedicated_closeable_tab_strip;
use super::dedicated_collapsible_panel;
use super::dedicated_command_palette;
use super::dedicated_complex;
use super::dedicated_context_menu;
use super::dedicated_diagnostics_list;
use super::dedicated_dod_atom_divider;
use super::dedicated_dod_atom_spacer;
use super::dedicated_dod_atoms;
use super::dedicated_dod_forms;
use super::dedicated_dod_layout_align_center;
use super::dedicated_dod_layout_column;
use super::dedicated_dod_layout_grid;
use super::dedicated_dod_layout_scroll_area;
use super::dedicated_dod_layout_stack;
use super::dedicated_dod_layouts;
use super::dedicated_dod_molecule_menu;
use super::dedicated_dod_molecules;
use super::dedicated_dod_runtime_motion;
use super::dedicated_drag_and_drop;
use super::dedicated_dynamic_array_editor;
use super::dedicated_empty_state;
use super::dedicated_feedback;
use super::dedicated_foundation_panel;
use super::dedicated_hover_card;
use super::dedicated_list;
use super::dedicated_menu_button;
use super::dedicated_modal;
use super::dedicated_node_labels;
use super::dedicated_notification_toast;
use super::dedicated_search_control_strip;
use super::dedicated_settings_list;
use super::dedicated_shortcut_cheatsheet;
use super::dedicated_shortcut_combo;
use super::dedicated_side_menu;
use super::dedicated_skeleton_cluster;
use super::dedicated_startup_state_panel;
use super::dedicated_status_bar;
use super::dedicated_tabs;
use super::dedicated_toast_stack_manager;
use super::dedicated_toolbar;
use super::dedicated_tooltip;
use super::dedicated_virtualization;
use super::dedicated_window_control_button_group;
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
        "panel" => dedicated_foundation_panel::draw(canvas, text, node, palette, scenario, x, y),
        "theme-tokens" => dedicated_dod_atoms::theme(canvas, text, palette, scenario, x, y),
        "text" => dedicated_dod_atoms::text_grid(canvas, text, palette, scenario, x, y),
        "icon" => dedicated_dod_atoms::icon_grid(canvas, text, palette, scenario, x, y),
        "divider" => dedicated_dod_atom_divider::divider(canvas, text, palette, scenario, x, y),
        "spacer" => dedicated_dod_atom_spacer::spacer(canvas, text, palette, scenario, x, y),
        "loading-dots" => dedicated_dod_atoms::loading_dots(canvas, text, palette, scenario, x, y),
        "spinner" => dedicated_dod_atoms::spinner(canvas, text, palette, scenario, x, y),
        "progress-bar" => dedicated_dod_atoms::progress(canvas, text, palette, scenario, x, y),
        "skeleton" => dedicated_dod_atoms::skeleton(canvas, text, palette, scenario, x, y),
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
        "combo-box" => dedicated_dod_forms::combo_box(canvas, text, palette, scenario, x, y),
        "color-swatch" => dedicated_dod_atoms::swatch(canvas, text, palette, scenario, x, y),
        "slide-control" => {
            dedicated_dod_atoms::slide_control(canvas, text, palette, scenario, x, y)
        }
        "text-input" => dedicated_dod_forms::input(canvas, text, palette, scenario, x, y),
        "text-area" => dedicated_dod_forms::text_area(canvas, text, palette, scenario, x, y),
        "search-box" => dedicated_dod_forms::search(canvas, text, palette, scenario, x, y),
        "search-control-strip" => {
            dedicated_search_control_strip::search_control_strip(
                canvas, text, palette, scenario, x, y,
            );
        }
        "form-field" => dedicated_dod_forms::form_field(canvas, text, palette, scenario, x, y),
        "selection-list" => {
            dedicated_dod_forms::selection_list(canvas, text, palette, scenario, x, y)
        }
        "checkbox" => dedicated_dod_forms::checkbox(canvas, text, palette, scenario, x, y),
        "radio" => dedicated_dod_forms::radio(canvas, text, palette, scenario, x, y),
        "tooltip" => dedicated_tooltip::tooltip(canvas, text, palette, scenario, x, y),
        "badge" => dedicated_dod_molecules::badge(canvas, text, palette, scenario, x, y),
        "chip" => dedicated_chip::chip(canvas, text, palette, scenario, x, y),
        "attachment-chip" => {
            dedicated_attachment_chip::attachment_chip(canvas, text, palette, scenario, x, y);
        }
        "chip-group" => {
            dedicated_chip_group::chip_group(canvas, text, palette, scenario, x, y);
        }
        "empty-state" => {
            dedicated_empty_state::empty_state(canvas, text, palette, scenario, x, y);
        }
        "key-cap" => dedicated_dod_molecules::key_cap(canvas, text, palette, scenario, x, y),
        "card" => dedicated_dod_molecules::card(canvas, text, palette, scenario, x, y),
        "diagnostics-list" => {
            dedicated_diagnostics_list::diagnostics_list(canvas, text, palette, scenario, x, y);
        }
        "list" => dedicated_list::list(canvas, text, palette, scenario, x, y),
        "settings-list" => {
            dedicated_settings_list::settings_list(canvas, text, palette, scenario, x, y);
        }
        "accordion" => dedicated_dod_molecules::accordion(canvas, text, palette, scenario, x, y),
        "tree-view" => dedicated_dod_molecules::tree_view(canvas, text, node, palette, x, y),
        "context-menu" => {
            dedicated_context_menu::context_menu(canvas, text, palette, scenario, x, y);
        }
        "split-pane" => dedicated_dod_molecules::split_pane(canvas, text, palette, scenario, x, y),
        "modal" | "modal-overlay" => {
            dedicated_modal::modal(canvas, text, palette, scenario, x, y);
        }
        "notification-toast" => {
            dedicated_notification_toast::notification_toast(canvas, text, palette, scenario, x, y);
        }
        "banner" => dedicated_banner::banner(canvas, text, palette, scenario, x, y),
        "toast-stack-manager" => {
            dedicated_toast_stack_manager::toast_stack_manager(
                canvas, text, palette, scenario, x, y,
            );
        }
        "popover" => dedicated_dod_forms::popover(canvas, text, palette, scenario, x, y),
        "hover-card" => dedicated_hover_card::hover_card(canvas, text, palette, scenario, x, y),
        "tabs" => dedicated_tabs::tabs(canvas, text, palette, scenario, x, y),
        "toolbar" => dedicated_toolbar::toolbar(canvas, text, palette, scenario, x, y),
        "breadcrumb" => dedicated_breadcrumb::breadcrumb(canvas, text, palette, scenario, x, y),
        "status-bar" => {
            dedicated_status_bar::status_bar(canvas, text, palette, scenario, x, y);
        }
        "shortcut-combo" => {
            dedicated_shortcut_combo::shortcut_combo(canvas, text, palette, scenario, x, y);
        }
        "shortcut-cheatsheet" => {
            dedicated_shortcut_cheatsheet::shortcut_cheatsheet(
                canvas, text, palette, scenario, x, y,
            );
        }
        "closeable-tab-strip" => {
            dedicated_closeable_tab_strip::closeable_tab_strip(
                canvas, text, palette, scenario, x, y,
            );
        }
        "collapsible-panel" => {
            dedicated_collapsible_panel::collapsible_panel(canvas, text, palette, scenario, x, y);
        }
        "color-picker-rgba" => {
            dedicated_dod_molecules::color_picker(canvas, text, palette, scenario, x, y);
        }
        "code-diff" => dedicated_dod_molecules::code_diff(canvas, text, palette, scenario, x, y),
        "dynamic-array-editor" => {
            dedicated_dynamic_array_editor::dynamic_array_editor(
                canvas, text, palette, scenario, x, y,
            );
        }
        "motion" => dedicated_dod_runtime_motion::motion(canvas, text, palette, scenario, x, y),
        "virtualization" => {
            dedicated_virtualization::virtualization(canvas, text, palette, scenario, x, y);
        }
        "drag-and-drop" => {
            dedicated_drag_and_drop::drag_and_drop(canvas, text, palette, scenario, x, y);
        }
        "skeleton-cluster" => {
            dedicated_skeleton_cluster::skeleton_cluster(canvas, text, palette, scenario, x, y);
        }
        "window-control-button-group" => {
            dedicated_window_control_button_group::window_control_button_group(
                canvas, text, palette, scenario, x, y,
            );
        }
        "startup-state-panel" => {
            dedicated_startup_state_panel::startup_state_panel(
                canvas, text, palette, scenario, x, y,
            );
        }
        "menu" => dedicated_dod_molecule_menu::menu(canvas, text, palette, scenario, x, y),
        "command-palette" => {
            dedicated_command_palette::command_palette(canvas, text, palette, scenario, x, y);
        }
        "menu-button" => {
            dedicated_menu_button::menu_button(canvas, text, palette, scenario, x, y);
        }
        "side-menu" => dedicated_side_menu::side_menu(canvas, text, palette, scenario, x, y),
        "row" => dedicated_dod_layouts::row(canvas, text, palette, scenario, x, y),
        "column" => dedicated_dod_layout_column::column(canvas, text, palette, scenario, x, y),
        "stack" => dedicated_dod_layout_stack::stack(canvas, text, palette, scenario, x, y),
        "grid" => dedicated_dod_layout_grid::grid(canvas, text, palette, scenario, x, y),
        "align-center" => {
            dedicated_dod_layout_align_center::align_center(canvas, text, palette, scenario, x, y);
        }
        "scroll-area" => {
            dedicated_dod_layout_scroll_area::scroll_area(canvas, text, palette, scenario, x, y);
        }
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
    let label = dedicated_node_labels::label_for(node.kind());
    match node.kind() {
        UiNodeKind::Button | UiNodeKind::TextButton | UiNodeKind::IconTextButton => {
            dedicated_basic::button(canvas, text, palette, x, y, label);
        }
        UiNodeKind::SvgButton => dedicated_atoms::icon_button(canvas, text, palette, x, y, label),
        UiNodeKind::Badge | UiNodeKind::Chip | UiNodeKind::AttachmentChip => {
            dedicated_feedback::badge(canvas, text, palette, x, y, label);
        }
        UiNodeKind::Input | UiNodeKind::TextArea | UiNodeKind::SelectBox => {
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
