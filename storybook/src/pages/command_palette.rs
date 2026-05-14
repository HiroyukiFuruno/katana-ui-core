use floem::IntoView;
use floem::peniko::Color;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, container, label, scroll, v_stack};
use katana_ui_widget::composite::command_palette::{
    CallbackCommandPaletteProvider, CommandPalette, CommandPaletteItem,
};
use katana_ui_widget::primitive::icon::IconSource;
use katana_ui_widget::theme::Theme;

const FILE_ICON: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path fill='currentColor' d='M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z'/><polyline fill='none' stroke='currentColor' stroke-width='2' points='14 2 14 8 20 8'/></svg>";
const CMD_ICON: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path fill='currentColor' d='M12 2l3.5 7.5L23 10l-6 3.5L14 21l-2-6.5L7 21l.9-7.5L2 10l7.5-0.5L12 2z'/></svg>";

fn file_items(query: &str) -> Vec<CommandPaletteItem<String>> {
    let files = [
        "src/main.rs",
        "src/lib.rs",
        "README.md",
        "Cargo.toml",
        "crates/katana-ui-widget/src/composite/command_palette/view.rs",
        "storybook/src/main.rs",
        "storybook/src/main.rs",
    ];
    let term = query.to_lowercase();
    files
        .into_iter()
        .map(|path| {
            let path_lower = path.to_lowercase();
            let score = if term.is_empty() || path_lower.contains(&term) {
                100 - ((path_lower.len() as i32 - term.len() as i32).max(0))
            } else {
                0
            };
            CommandPaletteItem::new(path, path.to_string())
                .icon(IconSource::SvgBytes(FILE_ICON))
                .shortcut("Ctrl+P")
                .score(score)
        })
        .filter(|item| query.is_empty() || item.label.to_lowercase().contains(&term))
        .collect()
}

fn command_items(query: &str) -> Vec<CommandPaletteItem<String>> {
    let list = [
        ("Open file", "⌘O"),
        ("Save file", "⌘S"),
        ("Close tab", "⌘W"),
        ("Run build", "⌘B"),
        ("Run tests", "⌘T"),
    ];
    let term = query.to_lowercase();
    let lowered = term.as_str();
    list.into_iter()
        .map(|(label, key)| {
            let label_lower = label.to_lowercase();
            let score = if term.is_empty() || label_lower.contains(lowered) {
                80
            } else {
                10
            };
            CommandPaletteItem::new(label, label.to_string())
                .icon(IconSource::SvgBytes(CMD_ICON))
                .shortcut(key)
                .score(score)
        })
        .collect()
}

fn log_block(
    query: RwSignal<String>,
    selected: RwSignal<String>,
    execute: RwSignal<String>,
    close_count: RwSignal<u32>,
) -> impl IntoView {
    v_stack((
        label(move || format!("query: {}", query.get())),
        label(move || format!("selection: {}", selected.get())),
        label(move || format!("execute: {}", execute.get())),
        label(move || format!("close: {}", close_count.get())),
    ))
    .style(|style| style.gap(4.0))
}

fn section(
    theme: Theme,
    title: &'static str,
    placeholder: &'static str,
    query_provider: impl Fn(&str) -> Vec<CommandPaletteItem<String>> + 'static,
) -> impl IntoView {
    let query_signal = create_rw_signal(String::new());
    let selected_signal = create_rw_signal("未選択".to_string());
    let execute_signal = create_rw_signal("未実行".to_string());
    let close_signal = create_rw_signal(0_u32);

    if title == "ファイル検索風" {
        crate::interaction::replay("query-command", "command-palette", "query-main", {
            let query_signal = query_signal;
            let selected_signal = selected_signal;
            move || {
                query_signal.set("main".to_string());
                selected_signal.set("main / 0".to_string());
            }
        });
    }

    let palette = CommandPalette::new(CallbackCommandPaletteProvider::new(query_provider))
        .placeholder(placeholder)
        .on_query({
            let query_signal = query_signal;
            move |query| query_signal.set(query)
        })
        .on_selection_change({
            let selected_signal = selected_signal;
            move |query, index| selected_signal.set(format!("{query} / {index}"))
        })
        .on_execute({
            let execute_signal = execute_signal;
            move |query, index, payload| {
                execute_signal.set(format!("{query} / {index} / {payload}"))
            }
        })
        .on_close({
            let close_signal = close_signal;
            move || close_signal.update(|value| *value += 1)
        });
    container(v_stack((
        label(move || title.to_string()).style(|style| style.font_size(18.0)),
        palette.view(theme.clone()).into_any(),
        log_block(query_signal, selected_signal, execute_signal, close_signal),
    )))
    .style(|style| style.gap(10.0))
}

pub fn command_palette_page(theme: Theme) -> impl IntoView {
    let bg = Color::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = Color::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);
    let disabled_close = create_rw_signal(0_u32);
    let disabled_palette = CommandPalette::new(CallbackCommandPaletteProvider::new(file_items))
        .placeholder("disabled")
        .disabled(true)
        .on_close({
            let disabled_close = disabled_close;
            move || disabled_close.update(|value| *value += 1)
        });

    scroll(
        v_stack((
            section(
                theme.clone(),
                "ファイル検索風",
                "ファイル名で検索",
                file_items,
            ),
            section(
                theme.clone(),
                "コマンド検索風",
                "コマンドを検索",
                command_items,
            ),
            v_stack((
                label(|| "disabled 状態").style(|style| style.font_size(18.0)),
                disabled_palette.view(theme.clone()),
                label(move || format!("disabled close callback: {}", disabled_close.get())),
            ))
            .style(|style| style.gap(8.0)),
        ))
        .style(move |style| style.gap(24.0).padding(16.0).background(bg).color(text)),
    )
}
