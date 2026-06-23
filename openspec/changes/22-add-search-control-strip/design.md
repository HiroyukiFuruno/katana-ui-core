# Design — 22-add-search-control-strip

## 方針

`SearchControlStrip` は、検索操作の見た目と状態だけを持つ molecule とする。
検索 engine、workspace traversal、editor buffer 操作、viewer hit-test は持たない。

consumer は次を自前で持つ。

- 検索対象
- 検索実行
- 検索結果生成
- replace 実行
- command / history / provider domain

KUC は次を持つ。

- query input
- typed search options
- navigation buttons
- result count / active index
- optional replace row
- action / event
- shortcut 表示
- disabled state

## Model

```rust
pub struct SearchControlStrip {
    pub query: String,
    pub options: SearchOptions,
    pub result_count: Option<usize>,
    pub active_index: Option<usize>,
    pub replace_mode: ReplaceMode,
    pub replace_value: String,
}

pub struct SearchOptions {
    pub match_case: bool,
    pub whole_word: bool,
    pub use_regex: bool,
}

pub enum ReplaceMode {
    Hidden,
    Visible,
    Disabled,
}
```

## Layout

- compact mode: query input + option toggle icons + previous / next + result count。
- expanded mode: compact row + replace input + replace / replace all。
- option toggle は icon button を基本にし、tooltip と accessibility label を必須にする。
- result count は `current / total` または `0 results` を domain-free に表す。

## Boundary

| UI | KUC の扱い |
| --- | --- |
| query input only | `SearchBox` |
| search options + navigation | `SearchControlStrip` |
| result rows | `CommandPalette` / `CommandResultRow` |
| workspace / editor / viewer search engine | consumer |

## Non-goals

- regex validation の engine は持たない。consumer が validation result を渡す。
- file include / exclude pattern の意味は持たない。必要なら consumer が custom action slot を足す。
- editor replace の副作用は持たない。
