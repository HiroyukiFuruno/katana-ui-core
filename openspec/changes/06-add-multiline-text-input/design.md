# Design — TextArea atom（multi-line input control）

## 目的

document editor（KLE）に踏み込まない範囲で、複数行のフォーム入力 + IME + auto-grow を持つ汎用の入力 atom を提供する。chat composer / 検索 / 簡易メモ で使う。

## 採用方針

### 1. `Input` と `TextArea` を分離

- `Input` は 1 行限定（既存 contract そのまま）
- `TextArea` は複数行、auto-grow、submit_key / newline_key 切替えを持つ独立 atom
- `Input::validate()` は改行を含む value を拒否し、利用側を `TextArea` へ誘導する

両者を同じ enum にまとめる案も検討したが、option / action / event が大幅に異なり、Storybook preset が混在すると入力回帰の対象が把握しづらくなるため別 atom に分離する。

### 2. submit_key と newline_key

- chat composer は「Enter で送信、Shift+Enter で改行」が標準
- form では「Ctrl/Cmd+Enter で送信、Enter で改行」が標準
- 両方を選べるよう、独立した 2 つの option として持つ

```text
SubmitKey = Enter | ModEnter | Disabled
NewlineKey = Enter | ShiftEnter | Disabled
```

両方が同じキーに割り当てられている場合は static check でエラーにする。

### 3. auto-grow

- `min_rows`, `max_rows`, `auto_grow: bool`
- auto_grow=true のとき、内容 height に応じて行数を調整
- max_rows 超過時は内部スクロールに切替え
- `Resize` event を行数変化時に発火

### 4. wrap_policy

- `Soft`: 表示上で折り返すが、内部 value に改行を入れない
- `Hard`: 折返し位置で実際に `\n` を挿入（rare）
- `None`: 折り返しなし、横スクロール

### 5. tab behavior

- `InsertTab`: タブ文字を挿入（コード入力寄り）
- `MoveFocus`: 次のフォーカス要素に移る（フォーム寄り）
- chat composer のデフォルトは `MoveFocus`（accessibility 標準）

### 6. multi-line IME

- IME composition 中は composition string を inline overlay として表示
- caret / selection は composition 中も保持し、composition 終了で commit
- composition 中の改行や絵文字確定を `IMEComposition { phase, string }` / `IMECommit { string }` で報告
- adapter は composition 中の preedit string と caret 位置を返す責務を持つ
- adapter request は `input_kind = Multiline`、`phase`、`preedit`、`commit_text`、`caret` を DTO として渡す

### 7. emoji 入力

- surrogate pair / 結合絵文字（ZWJ）を 1 つの grapheme として扱う
- `EmojiInput { graphemes }` event を補助として発火（adapter が判定）
- caret 移動は grapheme 単位

### 8. accessibility

- role=textbox (multiline=true 相当)
- aria-label or label association を必須
- screen reader 用に「行 X / 列 Y」announce template を持つ

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| `Input` に `multiline: bool` option を追加 | option の組み合わせが線形に増え、submit_key / newline_key / auto-grow / wrap が 1 行入力では無意味になる。preset が散らかる。 |
| document editor（KLE）に統合する | KLE は document 編集（syntax highlight / diagnostics / decoration / gutter）が中核。chat composer の責務とは異なるため統合は過剰。 |
| 文字列を `value: String` 1 つだけにする | caret / selection / composition の状態が表現できず、IME / accessibility の契約を満たせない。 |

## Out of scope

- syntax highlighting：document editor 領域（KLE）
- gutter / line numbers / decorations：KLE
- 大規模文書（>10MB）のレンダリング最適化：KLE
- markdown preview：KDV
- collaborative editing（CRDT 等）：別 change / 別 layer

## 影響範囲

- `Input` atom はそのまま 1 行入力に固定する
- chat composer の `ImeMultilineEditor` を KUC `TextArea` に置き換える前提
- adapter（floem / egui / gpui）に multi-line + IME の compile-gate を追加
- Storybook は `Atom > TextArea` に分類し、chat composer / search multiline / long text / auto grow / max rows / IME input / emoji input の preset を出す
