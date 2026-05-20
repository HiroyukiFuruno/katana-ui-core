## Why

`katana-chat-ui` の composer は `ChatUiComposerInputKind::ImeMultilineEditor`（複数行 IME 対応の入力エディタ）を必要とするが、これは document editor（KLE 担当）ではなく「入力フォーム」用途である。`katana` 側でも検索バーや簡易メモ入力など複数行の入力が必要な箇所がある。

KUC の `Input` atom は 1 行入力前提で、複数行・自動行数調整（auto-grow）・最大行数 / 最小行数・行折返し方針・スクロール挙動・IME composition の複数行扱い・複数行の placeholder・行末改行コミットの違い（Enter で改行 vs Cmd/Ctrl+Enter で送信）を契約していない。これは「document editor」とは異なる「multi-line input control」の責務である。

document editor（KLE）/ document preview（KDV）は対象外だが、「複数行のフォーム入力」は core 取り込み対象である。

## What Changes

- `widget::atoms` に `TextArea` atom を追加する（または `Input` atom に `kind = SingleLine | MultiLine { ... }` を typed enum で持たせる）。
- 採用: `TextArea` を独立 atom として追加し、`Input` は 1 行限定で維持する（責務分離）。
- `TextArea` の typed option:
  - `value`, `placeholder`, `font_role`, `disabled`, `readonly`, `invalid`
  - `min_rows`, `max_rows`, `auto_grow`
  - `wrap_policy`: `Soft` / `Hard` / `None`
  - `submit_key`: `Enter` / `ModEnter` / `Disabled`
  - `newline_key`: `Enter` / `ShiftEnter` / `Disabled`
  - `tab_behavior`: `InsertTab` / `MoveFocus`
  - `ime_enabled`, `selection_visible`
  - `leading_slot`, `trailing_slot`（Input と同等）
- action: `Type` / `Submit` / `InsertNewline` / `Clear` / `MoveCaret` / `Select` / `IMECommit`
- event: `KeyInput` / `TextInput` / `IMEComposition` / `IMECommit` / `EmojiInput` / `Submit` / `Change` / `Focus` / `Blur` / `Resize`（auto-grow による行数変化）
- multi-line IME composition（候補文字列内の改行 / カーソル位置）と emoji 入力（surrogate pair / 結合絵文字）の挙動を契約に含める。
- auto-grow が `max_rows` を超えたら内部スクロールに切り替わる。
- `kuc-storybook-catalog` に composer 様 preset、検索（複数行）preset、長文 preset を追加する。

## Capabilities

### New Capabilities

- `kuc-text-area-atom`: TextArea atom の option / action / event / state / preset / preview / settings / 自動テスト / 数値化された描画契約 / Storybook ページの完了条件を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `Input` atom（1 行）と `TextArea` atom（複数行）の責務分離を明記する。

## Impact

- `crates/katana-ui-core/src/atom/` に `text_area/` を追加する。
- `widget::atoms` の re-export に `TextArea` を追加する。
- 既存 `TextInput` 型は `Input` の alias のまま維持（後方互換）。
- consumer (`katana-chat-ui` composer) は KUC `TextArea` に置き換える前提で migration ガイドが必要になる。
- adapter（floem / egui / gpui）に multi-line input + IME の adapter contract 拡張が必要。
