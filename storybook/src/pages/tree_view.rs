use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, label, scroll, v_stack};
use katana_ui_core::composite::tree_view::{
    TreeView, TreeViewExpandTrigger, TreeViewLineKind, TreeViewLineStyle, TreeViewNode,
};
use katana_ui_core::primitive::icon::IconSource;
use katana_ui_core::theme::Theme;

const ICON_FOLDER: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><path d='M2 3h5l2 2h6a1 1 0 0 1 1 1v7a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1z' fill='none' stroke='currentColor' stroke-width='1.2'/><path d='M6 3v2h6' fill='none' stroke='currentColor' stroke-width='1.2'/></svg>";
const ICON_FILE: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><path d='M3 2h6l3 3v7a1 1 0 0 1-1 1H3z' fill='none' stroke='currentColor' stroke-width='1.2'/><path d='M9 2v3h3' fill='none' stroke='currentColor' stroke-width='1.2'/></svg>";
const ICON_SETTINGS: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><circle cx='8' cy='8' r='6' fill='none' stroke='currentColor' stroke-width='1.2'/><path d='M7.2 8h1.6m-0.8-0.8v-3.2m0 8v0.4' fill='none' stroke='currentColor' stroke-width='1.2'/><circle cx='8' cy='8' r='1.4' fill='none' stroke='currentColor' stroke-width='1.2'/></svg>";

fn event_log(log: RwSignal<String>, value: impl Into<String>) -> impl Fn() + 'static {
    let message = value.into();
    move || {
        log.set(message.clone());
    }
}

fn file_tree_nodes(log: RwSignal<String>) -> Vec<TreeViewNode> {
    vec![
        TreeViewNode::new("workspace", "workspace")
            .icon(IconSource::SvgBytes(ICON_FOLDER))
            .expanded(true)
            .on_expand(event_log(log, "parent expand: workspace"))
            .on_collapse(event_log(log, "parent collapse: workspace"))
            .children(vec![
                TreeViewNode::new("src", "src")
                    .icon(IconSource::SvgBytes(ICON_FOLDER))
                    .expanded(true)
                    .on_context(event_log(log, "context: src"))
                    .children(vec![
                        TreeViewNode::new("main.rs", "main.rs")
                            .icon(IconSource::SvgBytes(ICON_FILE))
                            .on_select(event_log(log, "leaf selected: main.rs")),
                        TreeViewNode::new("tree_view.rs", "tree_view.rs")
                            .icon(IconSource::SvgBytes(ICON_FILE))
                            .active(true)
                            .on_select(event_log(log, "leaf selected: tree_view.rs")),
                    ]),
                TreeViewNode::new("README.md", "README.md")
                    .icon(IconSource::SvgBytes(ICON_FILE))
                    .on_select(event_log(log, "leaf selected: README.md")),
            ]),
    ]
}

fn toc_tree_nodes(log: RwSignal<String>) -> Vec<TreeViewNode> {
    vec![
        TreeViewNode::new("section-1", "セクション 1")
            .expanded(true)
            .children(vec![
                TreeViewNode::new("section-1-ready", "Definition of Ready")
                    .on_select(event_log(log, "leaf selected: DoR")),
                TreeViewNode::new("section-1-done", "Definition of Done")
                    .on_select(event_log(log, "leaf selected: DoD")),
            ]),
        TreeViewNode::new("section-2", "セクション 2").children(vec![
            TreeViewNode::new("section-2-design", "設計")
                .on_select(event_log(log, "leaf selected: 設計")),
            TreeViewNode::new("section-2-task", "Tasks")
                .on_select(event_log(log, "leaf selected: Tasks")),
        ]),
    ]
}

fn settings_tree_nodes(log: RwSignal<String>) -> Vec<TreeViewNode> {
    vec![
        TreeViewNode::new("root", "全体")
            .icon(IconSource::SvgBytes(ICON_SETTINGS))
            .expanded(true)
            .children(vec![
                TreeViewNode::new("appearance", "表示")
                    .expanded(true)
                    .children(vec![
                        TreeViewNode::new("theme", "テーマ")
                            .on_select(event_log(log, "leaf selected: 表示 > テーマ")),
                        TreeViewNode::new("language", "言語")
                            .on_select(event_log(log, "leaf selected: 表示 > 言語")),
                    ]),
                TreeViewNode::new("shortcut", "ショートカット").children(vec![
                    TreeViewNode::new("save", "保存")
                        .on_select(event_log(log, "leaf selected: ショートカット > 保存")),
                    TreeViewNode::new("search", "検索")
                        .disabled(true)
                        .on_select(event_log(log, "leaf selected: ショートカット > 検索")),
                ]),
            ])
            .on_expand(event_log(log, "parent expand: 全体"))
            .on_collapse(event_log(log, "parent collapse: 全体")),
    ]
}

fn virtualized_nodes(log: RwSignal<String>) -> Vec<TreeViewNode> {
    let children = (1..=30)
        .map(|index| {
            TreeViewNode::new(format!("item-{index}"), format!("仮想行 {index}"))
                .on_select(event_log(log, format!("virtual selected: {index}")))
        })
        .collect();

    vec![
        TreeViewNode::new("virtual-root", "Virtualized parent")
            .expanded(true)
            .children(children),
    ]
}

fn page_block(
    theme: Theme,
    title: &'static str,
    content: impl IntoView + 'static,
    log: RwSignal<String>,
) -> impl IntoView {
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    v_stack((
        label(move || title).style(|style| style.font_size(16.0).margin_bottom(6.0)),
        content,
        label(move || format!("ログ: {}", log.get())),
    ))
    .style(move |style| {
        style
            .gap(8.0)
            .padding_left(4.0)
            .padding_right(4.0)
            .color(text)
    })
}

pub fn tree_view_page(theme: Theme) -> impl IntoView {
    let file_log = create_rw_signal("未選択".to_string());
    let toc_log = create_rw_signal("未選択".to_string());
    let settings_log = create_rw_signal("未選択".to_string());
    let virtual_log = create_rw_signal("未選択".to_string());
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);

    crate::interaction::replay("select-leaf", "tree-view", "leaf-tree-view", {
        let file_log = file_log;
        move || {
            file_log.set("leaf selected: tree_view.rs".to_string());
        }
    });

    let horizontal = TreeViewLineStyle::new(TreeViewLineKind::Solid, 1.0, theme.color.border);
    let file_view = TreeView::from_nodes(file_tree_nodes(file_log))
        .expand_trigger(TreeViewExpandTrigger::IconAndLabel)
        .show_indent_lines(true)
        .show_horizontal_lines(true)
        .horizontal_line_style(horizontal)
        .show_expand_controls(true)
        .row_height(30.0)
        .view(theme.clone());
    let toc_view = TreeView::from_nodes(toc_tree_nodes(toc_log))
        .expand_trigger(TreeViewExpandTrigger::IconOnly)
        .show_indent_lines(true)
        .show_expand_controls(true)
        .row_height(24.0)
        .view(theme.clone());
    let settings_view = TreeView::from_nodes(settings_tree_nodes(settings_log))
        .expand_trigger(TreeViewExpandTrigger::LabelOnly)
        .show_indent_lines(false)
        .show_expand_controls(true)
        .view(theme.clone());
    let virtualized_view = TreeView::from_nodes(virtualized_nodes(virtual_log))
        .virtualized(true)
        .show_indent_lines(true)
        .row_height(26.0)
        .view(theme.clone());

    scroll(
        v_stack((
            page_block(theme.clone(), "ファイルツリー", file_view, file_log),
            page_block(theme.clone(), "TOC", toc_view, toc_log),
            page_block(theme.clone(), "設定ツリー", settings_view, settings_log),
            page_block(
                theme.clone(),
                "大量行向け表示",
                virtualized_view,
                virtual_log,
            ),
        ))
        .style(move |style| {
            style
                .gap(20.0)
                .padding(16.0)
                .background(bg)
                .min_width_full()
        }),
    )
    .style(|style| style.width_full().height_full())
}
