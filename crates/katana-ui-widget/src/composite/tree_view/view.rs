use super::{ops, render, types};
use crate::theme::Theme;
use floem::IntoView;
use floem::action::exec_after;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{
    Decorators, VirtualDirection, VirtualItemSize, VirtualVector, button, dyn_container, h_stack,
    label, v_stack, v_stack_from_iter, virtual_stack,
};
use std::ops::Range;
use std::time::Duration;

const EXPAND_CONTROLS_GAP: f32 = 4.0;
const TREE_CONTENT_GAP: f32 = 6.0;
const DEFER_TREE_MUTATION_MS: u64 = 1;

struct VisibleRows(Vec<ops::FlattenedTreeItem>);

impl VirtualVector<ops::FlattenedTreeItem> for VisibleRows {
    fn total_len(&self) -> usize {
        self.0.len()
    }

    fn slice(&mut self, range: Range<usize>) -> impl Iterator<Item = ops::FlattenedTreeItem> {
        let end = range.end.min(self.0.len());
        self.0[range.start.min(end)..end].iter().cloned()
    }
}

fn expand_controls(
    items: RwSignal<Vec<types::TreeViewItem>>,
    version: RwSignal<u64>,
) -> impl IntoView {
    h_stack((
        button(label(|| "すべて展開")).action(move || {
            exec_after(Duration::from_millis(DEFER_TREE_MUTATION_MS), move |_| {
                items.update(|nodes| ops::TreeViewOps::expand_all(nodes));
                version.update(|value| *value += 1);
            });
        }),
        button(label(|| "すべて折りたたむ")).action(move || {
            exec_after(Duration::from_millis(DEFER_TREE_MUTATION_MS), move |_| {
                items.update(|nodes| ops::TreeViewOps::collapse_all(nodes));
                version.update(|value| *value += 1);
            });
        }),
    ))
    .style(|style| style.gap(EXPAND_CONTROLS_GAP).width_full())
}

pub(super) fn build_view(props: types::TreeViewProps, theme: Theme) -> impl IntoView {
    let tree_items = create_rw_signal(props.items);
    let render_version = create_rw_signal(0_u64);
    let force_open = props.force_open;
    let show_indent_lines = props.show_indent_lines;
    let show_horizontal_lines = props.show_horizontal_lines;
    let horizontal_line_style = props.horizontal_line_style;
    let expand_trigger = props.expand_trigger;
    let show_expand_controls = props.show_expand_controls;
    let row_height = props.row_height;
    let virtualized = props.virtualized;

    let rows = dyn_container(
        move || render_version.get(),
        move |_| {
            let items = tree_items.get();
            let visible_rows = ops::TreeViewOps::flatten_visible_items(&items, force_open);
            if virtualized {
                let row_source = visible_rows.clone();
                let row_theme = theme.clone();
                virtual_stack(
                    VirtualDirection::Vertical,
                    VirtualItemSize::Fixed(Box::new(move || f64::from(row_height))),
                    move || VisibleRows(row_source.clone()),
                    |item| item.path.clone(),
                    move |item| {
                        render::render_row(render::RenderRowArgs {
                            item,
                            theme: row_theme.clone(),
                            tree_items,
                            render_version,
                            show_indent_lines,
                            show_horizontal_lines,
                            horizontal_line_style,
                            expand_trigger,
                            row_height,
                        })
                    },
                )
                .style(|style| style.width_full())
                .into_any()
            } else {
                let rows = visible_rows
                    .into_iter()
                    .map(|item| {
                        render::render_row(render::RenderRowArgs {
                            item,
                            theme: theme.clone(),
                            tree_items,
                            render_version,
                            show_indent_lines,
                            show_horizontal_lines,
                            horizontal_line_style,
                            expand_trigger,
                            row_height,
                        })
                    })
                    .collect::<Vec<_>>();

                v_stack_from_iter(rows)
                    .style(move |style| style.gap(2.0).width_full())
                    .into_any()
            }
        },
    )
    .style(|style| style.width_full());

    if show_expand_controls {
        v_stack((expand_controls(tree_items, render_version), rows))
            .style(|style| style.gap(TREE_CONTENT_GAP).width_full())
            .into_any()
    } else {
        rows.into_any()
    }
}
