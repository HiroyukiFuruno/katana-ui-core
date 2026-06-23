## Why

`katana` の problems panel（`views/panels/problems`）と editor inline diagnostics（`widgets/editor/diagnostics_*`）、`katana-chat-ui` の tool result / permission request output、`katana-markdown-linter` の lint 結果表示はいずれも「重大度（severity）+ メッセージ + 位置 + 修正候補 + bulk fix」の構造を持つ問題リストを表示する。

現状 KUC には汎用 `SelectionList` / `List` molecule はあるが、severity / fix preview / code action / grouping / counter / location（file + line / scope）を typed に持つ molecule がない。consumer 毎に reimpl されており、数値化された描画契約 / 入力回帰の対象から外れている。

## What Changes

- `widget::molecules` に `DiagnosticsList` molecule を追加する:
  - option:
    - `group_by`: Severity / Source / Location / None
    - `items`: 各 diagnostic（severity / code / message / location / source / fix_preview / actions / source_doc）
    - `severity_filter`: 表示する severity 集合
    - `sort_by`: Severity / Location / Source / Order
    - `wrap_error_navigation`: F8 / Shift+F8 の wrap 制御
    - `empty_slot` / `loading_slot` / `bulk_preview`
  - action: `SetGroupBy` / `SetSortBy` / `SetSeverityFilter` / `Select` / `ToggleFixPreview` / `ApplyFix` / `OpenBulkPreview` / `ConfirmBulkApply` / `Keyboard`
  - event: `DiagnosticSelected` / `DiagnosticFixPreviewToggled` / `DiagnosticFixApplied` / `BulkFixPreviewOpened` / `BulkFixApplied` / `NavigateRequested` / `FilterChanged`
  - state: 親 state（selected_id, expanded_ids, loading, bulk_preview_open）、子 fix_preview / slot の `UiStateId` 分離
- `DiagnosticsItem` の `fix_preview` は `CodeDiff` molecule を embed できる typed slot。
- empty / loading は child slot として契約に含める。`EmptyState` や `Skeleton` を直接所有せず、consumer または後続 change が組み合わせる。

## Capabilities

### New Capabilities

- `kuc-diagnostics-list`: DiagnosticsList molecule の option / action / event / state / preset / preview / settings / 自動テスト / 数値化された描画契約 / Storybook ページの完了条件を定義する。

## Impact

- `crates/katana-ui-core/src/molecule/structured/diagnostics_list/` に型、option、action、event、state、render、planner を追加する。
- `CodeDiff` molecule（既存）を fix_preview slot に embed する。
- empty state は別 change `09-add-empty-state`、loading は `17-add-skeleton-loader` を slot として活用できる。ただし本 change の必須依存にはしない。
- consumer (`katana` problems panel、`katana` editor diagnostics、`katana-chat-ui` tool result) は KUC molecule に置き換え可能になる。
