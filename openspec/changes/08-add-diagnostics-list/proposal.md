## Why

`katana` の problems panel（`views/panels/problems`）と editor inline diagnostics（`widgets/editor/diagnostics_*`）、`katana-chat-ui` の tool result / permission request output、`katana-markdown-linter` の lint 結果表示はいずれも「重大度（severity）+ メッセージ + 位置 + 修正候補 + bulk fix」の構造を持つ問題リストを表示する。

現状 KUC には汎用 `SelectionList` / `List` molecule はあるが、severity / fix preview / code action / grouping / collapse / counter / location（file + line / scope）を typed に持つ molecule がない。consumer 毎に reimpl されており、画像 / 入力回帰の対象から外れている。

## What Changes

- `widget::molecules` に `DiagnosticsList` molecule を追加する:
  - option:
    - `groups`: severity または scope での group 化
    - `items`: 各 diagnostic（severity / code / message / location / source / fix_preview / actions / source_doc）
    - `severity_filter`: 表示する severity 集合
    - `sort_by`: Severity / Location / Source / Order
    - `selected_id`, `expanded_ids`, `collapsed_group_ids`
    - `bulk_action`: 表示と起動
  - action: `SelectItem` / `ExpandItem` / `CollapseItem` / `ToggleGroup` / `ApplyFix` / `ApplyBulkFix` / `Navigate` / `ToggleSeverityFilter`
  - event: `DiagnosticSelected` / `DiagnosticFixApplied` / `BulkFixApplied` / `NavigateRequested` / `FilterChanged`
  - state: 親 state（selected_id, expanded_ids, applied_fixes, callback_log）、子 fix_preview の `UiStateId` 分離
- `DiagnosticsItem` の `fix_preview` は `CodeDiff` molecule を embed できる typed slot。
- empty state（lint cleanは empty state を持つ）と loading state を契約に含める。

## Capabilities

### New Capabilities

- `kuc-diagnostics-list`: DiagnosticsList molecule の option / action / event / state / preset / preview / settings / 自動テスト / 画像回帰 / Storybook ページの完了条件を定義する。

## Impact

- `crates/katana-ui-core/src/molecule/structured/` に `diagnostics_list.rs` を追加する。
- `CodeDiff` molecule（既存）を fix_preview slot に embed する。
- empty state は別 change `09-add-empty-state` を embed として活用できる（依存方向: 09 が早く完了する場合）。
- consumer (`katana` problems panel、`katana` editor diagnostics、`katana-chat-ui` tool result) は KUC molecule に置き換え可能になる。
