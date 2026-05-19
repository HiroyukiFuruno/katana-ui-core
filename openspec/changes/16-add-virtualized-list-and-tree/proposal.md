## Why

`katana` explorer の大量ファイルツリー、command palette の検索結果（数百〜数千件）、`katana-chat-ui` の long history、`katana-markdown-linter` の diagnostics 多数件など、大量項目を扱う場面で virtualization（仮想化）が必要。現状 `List` / `SelectionList` / `TreeView` / `CommandPalette` / `DiagnosticsList` は全件描画前提で、項目数が増えると描画コスト / フレーム時間 / accessibility 計算が爆発する。

## What Changes

- `List` / `SelectionList` / `TreeView` / `CommandPalette` / `DiagnosticsList` の各 molecule に共通の virtualization API を追加する:
  - option: `virtualization: VirtualizationConfig`
    - `enabled: bool`
    - `estimated_row_height: f32`
    - `overscan: usize`（可視 viewport 外を何 row 余分に描画するか）
    - `row_height_provider: RowHeightProvider`（Fixed / Variable<height_fn>）
    - `keep_focused_in_window: bool`（focused row が常に描画される）
- molecule 共通の `VirtualRange { start, end, total }` を state に持たせる。
- scroll / focus / selection の event は仮想行に依存せず項目 id ベースで安定。
- a11y: 全件分の `aria-setsize` / `aria-posinset` を仮想化中でも報告（virtualization-aware accessibility）

## Capabilities

### New Capabilities

- `kuc-virtualization`: virtualization config と VirtualRange / row_height_provider の契約を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `List` / `SelectionList` / `TreeView` / `CommandPalette` / `DiagnosticsList` が共通の virtualization config を受け付け、accessibility と event ルーティングを保つことを明記する。

## Impact

- `crates/katana-ui-core/src/interaction/virtualization.rs` 新設（共通 logic）。
- 各 molecule に virtualization 受け入れの option / state を追加する。
- adapter 側で測定 callback の責務を明確化（実 row height の測定）。
- Storybook に 10k 行の List / 10k node の TreeView preset を追加。
