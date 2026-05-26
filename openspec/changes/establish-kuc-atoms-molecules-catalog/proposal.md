## Why

`ui-core-root-plan` の完了は、KUC をフレームワーク非依存（framework-neutral）UI Core として成立させる親設計の完了であり、01〜24 の部品が KUC の最小部品（atoms）/ 組み合わせ部品（molecules）として実用できることの完了根拠ではない。
次フェーズでは、利用側が atoms / molecules を組み合わせて UI を構築できる状態を目標にし、旧個別 change の完了記録を KUC 独自実装の契約へ移し替える必要がある。

## What Changes

- 01〜24 の旧 widget 要件を KUC の `widget::atoms` / `widget::molecules` に再分類し、option、action、event、state、preset、preview、settings、自動テスト、数値化された layout / rendering contract、Storybook ページを UI ごとの完了条件にする。
- core 基盤として、テーマ（theme）、フォント（font）、文字描画、入力、イベント配送、状態所有、レイアウトを先に固める。
- Storybook は、KUC 部品を実画面で触ってフィードバックするための場として定義する。左ペインは KUC 自身の TreeView、preset 切替は KUC 自身の Tabs、各部品は preview と settings を持つ。
- 部品の正しさは Storybook やユーザー操作に委ねず、自動テスト、数値化された layout / rendering contract、入力回帰、静的検査（guard）を CI/CD 品質ゲートにする。
- `archive/2026-05-25-katana-widget-parity-backlog` と `archive/2026-05-25-ui-core-interaction-visual-parity` は要件移管済みの superseded change として扱う。
- `archive/2026-05-25-18-accordion`、`archive/2026-05-25-23-color-picker-complete-parity`、`archive/2026-05-25-24-code-diff` は移管済みの履歴入力元として扱う。
- archive 済み 01〜17、19〜22 は復帰しない。履歴として残し、KUC 実装タスクへの入力元に限定する。
- README と docs は、root architecture の説明、KUC 現在スコープ、旧文書の扱い、Storybook の位置付けが矛盾しないよう更新する。

## Capabilities

### New Capabilities

- `kuc-core-foundation`: theme / font / text / input / event / state / layout の KUC core 契約を定義する。
- `kuc-widget-layer`: `widget::atoms` / `widget::molecules` の公開境界、01〜24 の再分類、将来拡張の余地を定義する。
- `kuc-storybook-catalog`: KUC Storybook の TreeView、Tabs、preview、settings、preset、状態・イベント・操作履歴を定義する。
- `kuc-quality-gates`: 自動テスト、数値化された layout / rendering contract、入力回帰、静的検査を CI/CD 品質ゲートとして定義する。

### Modified Capabilities

- なし。

## Impact

- `openspec/changes/establish-kuc-atoms-molecules-catalog/` が次フェーズの正本 change になる。
- `docs/ui-separation-plan.md`、`docs/architecture/ui-separation/owned-ui-task-map.md`、`docs/architecture/ui-separation/ui-core-parity-gap.md`、`docs/directory-structure.md`、`docs/widget-extraction-policy.md`、`openspec/changes/README.md`、`README.md` の説明を整理する。
- コード実装はこの change の apply フェーズで行う。本 change 作成時点では、既存の Rust 実装差分は実装完了根拠にしない。
- adapter は MVP 外とし、gpui / floem / egui 互換の本実装はこの change の完了条件に含めない。
