## Why

`katana` の explorer 空表示（`explorer/empty.rs`）、search no result、tab bar が空のとき、chat history が空のとき、DiagnosticsList が clean の場合、command palette の検索結果空など、「空表示（empty state）」は consumer のあらゆる場所に存在する。

現状 KUC には empty state 用 molecule がなく、consumer ごとに「アイコン + 見出し + 補足 + アクション」を ad hoc に組んでいる。揃った余白・揃ったタイポグラフィ・action button の有無で揺れが出ており、画像回帰の対象にもなっていない。

## What Changes

- `widget::molecules` に `EmptyState` molecule を追加する:
  - option:
    - `icon` (Optional SvgIcon or atom Icon)
    - `illustration` (Optional 大きな SVG / Image)
    - `heading: String`
    - `body: Option<String>`
    - `primary_action: Option<Button>`
    - `secondary_action: Option<Button>`
    - `tone: Neutral | Subtle | Accent | Warning | Danger`
    - `size: Compact | Default | Large`
    - `alignment: Center | Leading`
  - action: `PressPrimary` / `PressSecondary`
  - event: `EmptyStateActioned`
  - state: callback log、focus

## Capabilities

### New Capabilities

- `kuc-empty-state`: EmptyState molecule の option / action / event / state / preset / preview / settings / 自動テスト / 画像回帰 / Storybook ページの完了条件を定義する。

## Impact

- `crates/katana-ui-core/src/molecule/basic.rs`（または `molecule/empty_state.rs`）に追加する。
- DiagnosticsList、SelectionList、TreeView、SearchBox、CommandPalette に empty 表示を embed できる前提として整理する。
- consumer (`katana` explorer empty、`katana-chat-ui` history empty 等) は KUC molecule に置き換え可能になる。
