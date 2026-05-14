use floem::IntoView;
use floem::peniko::Color as PenikoColor;
use floem::views::{Decorators, label, scroll, v_stack};
use katana_ui_widget::composite::code_diff::{
    CodeDiff, CodeDiffCollapseOptions, CodeDiffMode, CodeDiffSource, CodeDiffSplitOrientation,
};
use katana_ui_widget::theme::Theme;

fn source(text: &'static str, first_line_number: usize) -> CodeDiffSource {
    CodeDiffSource::new(text, first_line_number, line_count(text))
}

fn line_count(text: &str) -> usize {
    if text.is_empty() {
        0
    } else {
        text.split('\n').count()
    }
}

fn section(title: &'static str, body: impl IntoView + 'static) -> impl IntoView {
    v_stack((
        label(move || title).style(|s| s.font_size(16.0).margin_bottom(4.0)),
        body,
    ))
    .style(|s| s.gap(6.0).width_full())
}

fn collapsed_options() -> CodeDiffCollapseOptions {
    CodeDiffCollapseOptions {
        enabled: true,
        initially_expanded: false,
        context_lines: 1,
    }
}

fn horizontal_sample(theme: Theme) -> impl IntoView {
    CodeDiff::new(
        source("fn main() {\n    println!(\"old\");\n}", 1),
        source("fn main() {\n    println!(\"new\");\n}", 1),
    )
    .view(theme)
}

fn vertical_sample(theme: Theme) -> impl IntoView {
    CodeDiff::new(
        source("alpha\nbeta\ngamma", 10),
        source("alpha\nbeta!\ngamma", 10),
    )
    .split_orientation(CodeDiffSplitOrientation::Vertical)
    .view(theme)
}

fn inline_sample(theme: Theme) -> impl IntoView {
    CodeDiff::new(
        source("one\ntwo\nthree", 1),
        source("one\nTWO\nthree\nfour", 1),
    )
    .mode(CodeDiffMode::Inline)
    .view(theme)
}

fn add_delete_sample(theme: Theme) -> impl IntoView {
    CodeDiff::new(source("remove me\nkeep", 1), source("keep\nadd me", 1)).view(theme)
}

fn whitespace_sample(theme: Theme) -> impl IntoView {
    CodeDiff::new(
        source("let\tname = \"katana\";", 1),
        source("let name  = \"katana\";", 1),
    )
    .view(theme)
}

fn trailing_newline_sample(theme: Theme) -> impl IntoView {
    CodeDiff::new(source("return value;", 40), source("return value;\n", 40)).view(theme)
}

fn multibyte_sample(theme: Theme) -> impl IntoView {
    CodeDiff::new(source("名前 = \"太郎\"", 1), source("名前 = \"花子\"", 1)).view(theme)
}

fn collapse_sample(theme: Theme) -> impl IntoView {
    let before = "same 1\nsame 2\nsame 3\nold value\nsame 4\nsame 5\nsame 6";
    let after = "same 1\nsame 2\nsame 3\nnew value\nsame 4\nsame 5\nsame 6";
    CodeDiff::new(source(before, 1), source(after, 1))
        .collapse(collapsed_options())
        .view(theme)
}

fn long_line_sample(theme: Theme) -> impl IntoView {
    let before = "let message = \"this is a very long line that should be read horizontally before the changed tail appears\";";
    let after = "let message = \"this is a very long line that should be read horizontally after the changed tail appears\";";
    CodeDiff::new(source(before, 1), source(after, 1)).view(theme)
}

fn no_diff_sample(theme: Theme) -> impl IntoView {
    CodeDiff::new(
        source("no change\nsecond line", 1),
        source("no change\nsecond line", 1),
    )
    .view(theme)
}

fn page_content(theme: Theme) -> impl IntoView {
    let bg = PenikoColor::rgb8(theme.color.bg.r, theme.color.bg.g, theme.color.bg.b);
    let text = PenikoColor::rgb8(theme.color.text.r, theme.color.text.g, theme.color.text.b);

    scroll(
        v_stack((
            label(|| "CodeDiff Samples").style(|s| s.font_size(18.0).margin_bottom(8.0)),
            section("左右分割", horizontal_sample(theme.clone())),
            section("上下分割", vertical_sample(theme.clone())),
            section("内包表示", inline_sample(theme.clone())),
            section("追加と削除", add_delete_sample(theme.clone())),
            section("空白とタブ", whitespace_sample(theme.clone())),
            section("末尾改行", trailing_newline_sample(theme.clone())),
            section("日本語など複数バイト文字", multibyte_sample(theme.clone())),
            section("省略と展開", collapse_sample(theme.clone())),
            section("長い行と横スクロール", long_line_sample(theme.clone())),
            section("差分なし", no_diff_sample(theme)),
        ))
        .style(move |s| {
            s.gap(18.0)
                .padding(16.0)
                .background(bg)
                .color(text)
                .min_width_full()
        }),
    )
}

pub fn code_diff_page(theme: Theme) -> impl IntoView {
    page_content(theme)
}
