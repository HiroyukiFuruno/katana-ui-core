use super::lines::horizontal_line;
use super::ops;
use super::types::{TreeViewExpandTrigger, TreeViewLineStyle};
use crate::floem_view::FloemColor;
use crate::layout::align_center::AlignCenterWrapper;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::View;
use floem::action::exec_after;
use floem::reactive::{RwSignal, SignalUpdate};
use floem::views::{Decorators, container, h_stack_from_iter, label, v_stack};
use std::rc::Rc;
use std::time::Duration;

const ITEM_LEFT_PADDING: f32 = 6.0;
const LABEL_SIZE: f32 = 14.0;
const ROW_HOVER_ALPHA: u8 = 20;
const ROW_CONTENT_CAPACITY: usize = 3;
const ROW_CONTENT_GAP: f32 = 6.0;
const DEFER_TREE_MUTATION_MS: u64 = 1;

pub(super) struct RenderRowArgs {
    pub(super) item: ops::FlattenedTreeItem,
    pub(super) theme: Theme,
    pub(super) tree_items: RwSignal<Vec<super::types::TreeViewItem>>,
    pub(super) render_version: RwSignal<u64>,
    pub(super) show_indent_lines: bool,
    pub(super) show_horizontal_lines: bool,
    pub(super) horizontal_line_style: TreeViewLineStyle,
    pub(super) expand_trigger: TreeViewExpandTrigger,
    pub(super) row_height: f32,
}

fn to_floem_color(token: Color) -> floem::peniko::Color {
    FloemColor::from_token(token)
}

fn row_text_color(theme: &Theme, item: &ops::FlattenedTreeItem) -> Color {
    if item.disabled {
        theme.color.text_disabled
    } else if item.active {
        theme.color.accent
    } else {
        theme.color.text
    }
}

fn row_hover_color(theme: &Theme) -> Color {
    Color {
        r: theme.color.accent.r,
        g: theme.color.accent.g,
        b: theme.color.accent.b,
        a: ROW_HOVER_ALPHA,
    }
}

fn primary_pointer(event: &floem::event::Event) -> bool {
    matches!(
        event,
        floem::event::Event::PointerDown(pointer_event) if pointer_event.button.is_primary()
    )
}

fn secondary_pointer(event: &floem::event::Event) -> bool {
    matches!(
        event,
        floem::event::Event::PointerDown(pointer_event) if pointer_event.button.is_secondary()
    )
}

fn trigger_uses_icon(trigger: TreeViewExpandTrigger) -> bool {
    matches!(
        trigger,
        TreeViewExpandTrigger::IconOnly | TreeViewExpandTrigger::IconAndLabel
    )
}

fn trigger_uses_label(trigger: TreeViewExpandTrigger) -> bool {
    matches!(
        trigger,
        TreeViewExpandTrigger::LabelOnly | TreeViewExpandTrigger::IconAndLabel
    )
}

fn toggle_item(
    tree_items: RwSignal<Vec<super::types::TreeViewItem>>,
    render_version: RwSignal<u64>,
    path: &[usize],
    on_expand: &Rc<dyn Fn()>,
    on_collapse: &Rc<dyn Fn()>,
) {
    let path = path.to_vec();
    let on_expand = Rc::clone(on_expand);
    let on_collapse = Rc::clone(on_collapse);
    exec_after(Duration::from_millis(DEFER_TREE_MUTATION_MS), move |_| {
        let mut next_state = None;
        tree_items.update(|nodes| {
            next_state = ops::TreeViewOps::toggle_expand(nodes, &path);
        });
        render_version.update(|version| *version += 1);
        match next_state {
            Some(true) => {
                exec_after(Duration::from_millis(DEFER_TREE_MUTATION_MS), move |_| {
                    on_expand()
                });
            }
            Some(false) => {
                exec_after(Duration::from_millis(DEFER_TREE_MUTATION_MS), move |_| {
                    on_collapse()
                });
            }
            None => {}
        }
    });
}

fn select_item(
    tree_items: RwSignal<Vec<super::types::TreeViewItem>>,
    render_version: RwSignal<u64>,
    path: &[usize],
    on_select: &Rc<dyn Fn()>,
) {
    let path = path.to_vec();
    let on_select = Rc::clone(on_select);
    exec_after(Duration::from_millis(DEFER_TREE_MUTATION_MS), move |_| {
        let mut selected = false;
        tree_items.update(|nodes| {
            selected = ops::TreeViewOps::set_active(nodes, &path);
        });
        render_version.update(|version| *version += 1);
        if selected {
            exec_after(Duration::from_millis(DEFER_TREE_MUTATION_MS), move |_| {
                on_select()
            });
        }
    });
}

pub(super) fn render_row(args: RenderRowArgs) -> Box<dyn View> {
    let RenderRowArgs {
        item,
        theme,
        tree_items,
        render_version,
        show_indent_lines,
        show_horizontal_lines,
        horizontal_line_style,
        expand_trigger,
        row_height,
    } = args;
    let row_color = row_text_color(&theme, &item);
    let text_color = to_floem_color(row_color);
    let base_color = to_floem_color(if item.active {
        theme.color.accent_muted
    } else {
        theme.color.bg
    });
    let border_color = to_floem_color(theme.color.border);
    let hover_color = to_floem_color(row_hover_color(&theme));
    let line_indent = if show_indent_lines {
        super::row_chrome::indent_blocks(item.indent, row_height, border_color)
    } else {
        Vec::new()
    };

    let path = item.path.clone();
    let has_children = item.has_children;
    let disabled = item.disabled;
    let expanded = item.expanded;
    let label_text = item.label.clone();
    let on_select = Rc::clone(&item.on_select);
    let on_context = Rc::clone(&item.on_context);
    let on_expand = Rc::clone(&item.on_expand);
    let on_collapse = Rc::clone(&item.on_collapse);

    let mut content = Vec::with_capacity(ROW_CONTENT_CAPACITY);
    if item.has_children {
        let disclosure =
            super::row_chrome::disclosure_icon(expanded, row_height, row_color, theme.clone());
        if !disabled && trigger_uses_icon(expand_trigger) {
            let trigger_path = path.clone();
            let trigger_expand = Rc::clone(&on_expand);
            let trigger_collapse = Rc::clone(&on_collapse);
            content.push(
                disclosure
                    .on_event_stop(floem::event::EventListener::PointerDown, move |event| {
                        if primary_pointer(event) {
                            toggle_item(
                                tree_items,
                                render_version,
                                &trigger_path,
                                &trigger_expand,
                                &trigger_collapse,
                            );
                        }
                    })
                    .into_any(),
            );
        } else {
            content.push(disclosure.into_any());
        }
    } else {
        content.push(super::row_chrome::disclosure_spacer(row_height).into_any());
    }

    if let Some(source) = item.icon {
        content.push(
            super::row_chrome::item_icon(source, row_height, row_color, theme.clone()).into_any(),
        );
    }

    let row_label = container(
        label(move || label_text.clone())
            .style(move |style| style.font_size(LABEL_SIZE).color(text_color)),
    )
    .style(move |style| style.height(row_height).items_center());
    if disabled {
        content.push(row_label.into_any());
    } else if has_children && trigger_uses_label(expand_trigger) {
        let trigger_path = path.clone();
        let trigger_expand = Rc::clone(&on_expand);
        let trigger_collapse = Rc::clone(&on_collapse);
        content.push(
            row_label
                .on_event_stop(floem::event::EventListener::PointerDown, move |event| {
                    if secondary_pointer(event) {
                        on_context();
                    } else if primary_pointer(event) {
                        toggle_item(
                            tree_items,
                            render_version,
                            &trigger_path,
                            &trigger_expand,
                            &trigger_collapse,
                        );
                    }
                })
                .into_any(),
        );
    } else {
        let select_path = path.clone();
        content.push(
            row_label
                .on_event_stop(floem::event::EventListener::PointerDown, move |event| {
                    if secondary_pointer(event) {
                        on_context();
                    } else if primary_pointer(event) {
                        select_item(tree_items, render_version, &select_path, &on_select);
                    }
                })
                .into_any(),
        );
    }

    let row_segments = vec![
        h_stack_from_iter(line_indent).into_any(),
        h_stack_from_iter(content)
            .style(move |style| style.items_center().height(row_height).gap(ROW_CONTENT_GAP))
            .into_any(),
    ];
    let row_content = h_stack_from_iter(row_segments).style(move |style| {
        style
            .items_center()
            .padding_left(ITEM_LEFT_PADDING)
            .width_full()
            .background(base_color)
            .color(text_color)
    });
    let row = AlignCenterWrapper::new()
        .height(row_height)
        .view(theme, row_content)
        .style(move |style| {
            if disabled {
                style
            } else {
                style.hover(move |hover| hover.background(hover_color))
            }
        });

    if show_horizontal_lines && has_children && expanded {
        v_stack((row, horizontal_line(horizontal_line_style))).into_any()
    } else {
        row.into_any()
    }
}
