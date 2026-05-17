use super::CodeDiff;
use super::model::CodeDiffModelBuilder;
use super::row::CodeDiffRowView;
use super::split_model::{CodeDiffVisibleRow, CodeDiffVisibleRows};
use super::style::CodeDiffStyle;
use super::types::{CodeDiffAlignedRow, CodeDiffLineKind, CodeDiffMode, CodeDiffSplitOrientation};
use crate::theme::Theme;
use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{
    Decorators, button, dyn_container, h_stack, label, scroll, v_stack, v_stack_from_iter,
};

const PANEL_GAP: f32 = 10.0;
const HEADER_SIZE: f32 = 12.0;
const TITLE_SIZE: f32 = 15.0;
const ERROR_FONT_SIZE: f32 = 13.0;
const PANEL_PADDING: f32 = 12.0;
const HEADER_GAP: f32 = 8.0;
const BUTTON_PADDING_X: f32 = 8.0;
const BUTTON_PADDING_Y: f32 = 4.0;
const SIDE_SPLIT_MIN_HEIGHT: f32 = 260.0;
const STACK_GAP: f32 = 4.0;
const SIDE_PANEL_MIN_WIDTH: f32 = 520.0;
const INLINE_PANEL_MIN_WIDTH: f32 = 620.0;

impl CodeDiff {
    pub(crate) fn build_view(self, theme: Theme) -> impl IntoView {
        match CodeDiffModelBuilder::build_model(&self.props.before, &self.props.after) {
            Ok(model) => diff_view(self, theme, model).into_any(),
            Err(error) => label(move || error.to_string())
                .style(|style| style.font_size(ERROR_FONT_SIZE).padding(PANEL_PADDING))
                .into_any(),
        }
    }
}

fn diff_view(diff: CodeDiff, theme: Theme, model: super::types::CodeDiffModel) -> impl IntoView {
    let mode = create_rw_signal(diff.props.mode);
    let orientation = create_rw_signal(diff.props.split_orientation);
    let expanded_blocks = create_rw_signal(if diff.props.collapse.initially_expanded {
        (0..model.rows.len()).collect::<Vec<_>>()
    } else {
        Vec::new()
    });
    let style = CodeDiffStyle::from_theme(&theme);
    let collapse = diff.props.collapse;
    let show_header = diff.props.show_header;

    dyn_container(
        move || (mode.get(), orientation.get(), expanded_blocks.get()),
        move |(current_mode, current_orientation, expanded)| {
            let rows = CodeDiffVisibleRows::visible_rows(
                &model,
                collapse.enabled,
                collapse.context_lines,
                &expanded,
            );
            let content = match current_mode {
                CodeDiffMode::Split => {
                    split_view(rows, current_orientation, expanded_blocks, style).into_any()
                }
                CodeDiffMode::Inline => inline_view(rows, expanded_blocks, style).into_any(),
            };
            let diff_style = style;

            v_stack((
                header_view(HeaderViewContext {
                    show_header,
                    added_count: model.added_count,
                    removed_count: model.removed_count,
                    changed_block_count: model.changed_block_count,
                    mode: current_mode,
                    orientation: current_orientation,
                    mode_signal: mode,
                    orientation_signal: orientation,
                    style,
                }),
                content,
            ))
            .style(move |s| {
                s.width_full()
                    .gap(PANEL_GAP)
                    .padding(PANEL_PADDING)
                    .border(1.0)
                    .border_color(diff_style.border)
                    .background(diff_style.surface)
            })
            .into_any()
        },
    )
}

struct HeaderViewContext {
    show_header: bool,
    added_count: usize,
    removed_count: usize,
    changed_block_count: usize,
    mode: CodeDiffMode,
    orientation: CodeDiffSplitOrientation,
    mode_signal: floem::reactive::RwSignal<CodeDiffMode>,
    orientation_signal: floem::reactive::RwSignal<CodeDiffSplitOrientation>,
    style: CodeDiffStyle,
}

fn header_view(context: HeaderViewContext) -> impl IntoView {
    let HeaderViewContext {
        show_header,
        added_count,
        removed_count,
        changed_block_count,
        mode,
        orientation,
        mode_signal,
        orientation_signal,
        style,
    } = context;
    let title = if show_header {
        format!("CodeDiff  +{added_count} / −{removed_count} / 変更 {changed_block_count} ブロック")
    } else {
        String::new()
    };
    let split_disabled = mode == CodeDiffMode::Inline;

    h_stack((
        label(move || title.clone()).style(move |s| s.font_size(TITLE_SIZE).color(style.text)),
        toolbar_button(
            "左右",
            mode == CodeDiffMode::Split && orientation == CodeDiffSplitOrientation::Horizontal,
            split_disabled,
            style,
            move || {
                mode_signal.set(CodeDiffMode::Split);
                orientation_signal.set(CodeDiffSplitOrientation::Horizontal);
            },
        ),
        toolbar_button(
            "上下",
            mode == CodeDiffMode::Split && orientation == CodeDiffSplitOrientation::Vertical,
            split_disabled,
            style,
            move || {
                mode_signal.set(CodeDiffMode::Split);
                orientation_signal.set(CodeDiffSplitOrientation::Vertical);
            },
        ),
        toolbar_button(
            "内包",
            mode == CodeDiffMode::Inline,
            false,
            style,
            move || {
                mode_signal.set(CodeDiffMode::Inline);
            },
        ),
    ))
    .style(|s| s.gap(HEADER_GAP).items_center().justify_between())
}

fn toolbar_button(
    text: &'static str,
    active: bool,
    disabled: bool,
    style: CodeDiffStyle,
    on_press: impl Fn() + 'static,
) -> impl IntoView {
    let color = if disabled { style.muted } else { style.text };
    let background = if active {
        style.omitted_bg
    } else {
        style.surface
    };
    button(label(move || text).style(move |s| s.font_size(HEADER_SIZE).color(color)))
        .action(move || {
            if !disabled {
                on_press();
            }
        })
        .style(move |s| {
            s.padding_horiz(BUTTON_PADDING_X)
                .padding_vert(BUTTON_PADDING_Y)
                .border(1.0)
                .border_color(style.border)
                .background(background)
        })
}

fn split_view(
    rows: Vec<CodeDiffVisibleRow>,
    orientation: CodeDiffSplitOrientation,
    expanded_blocks: floem::reactive::RwSignal<Vec<usize>>,
    style: CodeDiffStyle,
) -> impl IntoView {
    match orientation {
        CodeDiffSplitOrientation::Horizontal => scroll(h_stack((
            side_panel("変更前", true, rows.clone(), expanded_blocks, style),
            label(|| "").style(move |s| {
                s.width(1.0)
                    .min_height(SIDE_SPLIT_MIN_HEIGHT)
                    .background(style.border)
            }),
            side_panel("変更後", false, rows, expanded_blocks, style),
        )))
        .into_any(),
        CodeDiffSplitOrientation::Vertical => v_stack((
            scroll(side_panel(
                "変更前",
                true,
                rows.clone(),
                expanded_blocks,
                style,
            )),
            label(|| "").style(move |s| s.height(1.0).width_full().background(style.border)),
            scroll(side_panel("変更後", false, rows, expanded_blocks, style)),
        ))
        .style(|s| s.gap(STACK_GAP))
        .into_any(),
    }
}

fn side_panel(
    title: &'static str,
    before_side: bool,
    rows: Vec<CodeDiffVisibleRow>,
    expanded_blocks: floem::reactive::RwSignal<Vec<usize>>,
    style: CodeDiffStyle,
) -> impl IntoView {
    let line_views = rows.into_iter().map(move |row| match row {
        CodeDiffVisibleRow::Row(row) => {
            let line = if before_side { row.before } else { row.after };
            CodeDiffRowView::code_line(line, style).into_any()
        }
        CodeDiffVisibleRow::Omitted {
            block_index,
            hidden_count,
        } => CodeDiffRowView::omitted_row(block_index, hidden_count, false, style, move |index| {
            toggle_expanded(expanded_blocks, index)
        })
        .into_any(),
    });

    v_stack((
        label(move || title).style(move |s| s.font_size(HEADER_SIZE).color(style.muted)),
        v_stack_from_iter(line_views).style(move |s| {
            s.border(1.0)
                .border_color(style.border)
                .min_width(SIDE_PANEL_MIN_WIDTH)
        }),
    ))
    .style(|s| s.gap(STACK_GAP).min_width(SIDE_PANEL_MIN_WIDTH))
}

fn inline_view(
    rows: Vec<CodeDiffVisibleRow>,
    expanded_blocks: floem::reactive::RwSignal<Vec<usize>>,
    style: CodeDiffStyle,
) -> impl IntoView {
    let views = rows.into_iter().flat_map(move |row| match row {
        CodeDiffVisibleRow::Row(row) => inline_row_views(row, style),
        CodeDiffVisibleRow::Omitted {
            block_index,
            hidden_count,
        } => vec![
            CodeDiffRowView::omitted_row(block_index, hidden_count, false, style, move |index| {
                toggle_expanded(expanded_blocks, index)
            })
            .into_any(),
        ],
    });
    scroll(v_stack_from_iter(views).style(move |s| {
        s.border(1.0)
            .border_color(style.border)
            .min_width(INLINE_PANEL_MIN_WIDTH)
    }))
}

fn inline_row_views(row: CodeDiffAlignedRow, style: CodeDiffStyle) -> Vec<Box<dyn floem::View>> {
    if row.before.kind == CodeDiffLineKind::Equal {
        return vec![CodeDiffRowView::code_line(row.before, style).into_any()];
    }

    let mut rows = Vec::new();
    if row.before.kind != CodeDiffLineKind::Placeholder {
        rows.push(CodeDiffRowView::code_line(row.before, style).into_any());
    }
    if row.after.kind != CodeDiffLineKind::Placeholder {
        rows.push(CodeDiffRowView::code_line(row.after, style).into_any());
    }
    rows
}

fn toggle_expanded(signal: floem::reactive::RwSignal<Vec<usize>>, block_index: usize) {
    signal.update(|values| {
        if let Some(index) = values.iter().position(|value| *value == block_index) {
            values.remove(index);
        } else {
            values.push(block_index);
        }
    });
}
