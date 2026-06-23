# Design — SettingsList molecule

## 目的

「セクション付きフォーム」を統一 molecule として確立し、settings 画面の構築コストとデザイン揺れを解消する。

## 採用方針

### 1. 階層

```text
SettingsList
  └─ SettingsSection (collapsible)
        └─ SettingsField (label + description + control)
```

複数階層（section の中の sub-section）は v1 では持たない。必要なら field 内 `Custom(UiTree)` で表現。

### 2. control typed enum

`SettingsControl` は既存 atom / molecule を再利用する typed enum。

- `Toggle`: on/off
- `Select`: 1 件選択
- `Combo`: 入力 + 選択
- `Input` / `TextArea`: 文字列入力
- `Number`: 数値入力（slider または input）
- `Chips`: 複数選択
- `Radio`: 1 件選択
- `ColorPicker`: 色選択
- `Custom`: consumer 提供 UiTree

control の child state は SettingsField の child `UiStateId` で分離。

### 3. dirty visualization

- `None`: 何もしない
- `Marker`: ラベル横に小さな dot
- `Highlight`: 行 background を強調

### 4. reset_to_default

- field に `reset_to_default: Option<DefaultValue>` を持たせる
- 値が default と異なるとき reset アイコンを表示
- 押下で `FieldReset { id }` を発火

### 5. query filter

- query は label / description / section label を case-insensitive substring 検索
- マッチした field / section だけ表示
- 全 section の field がフィルタアウトされたら EmptyState を表示

### 6. キーボードナビゲーション

- Tab で field 間移動
- Section header に Space / Enter で collapse toggle
- field の reset ボタンには Tab で到達可能

### 7. accessibility

- 各 field の label と control は ARIA で関連付け
- section header は role=button (collapsible のとき)
- query 入力中、結果数を live region announce

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| consumer 側で `FormField` を縦に並べる | section / collapse / search filter / dirty visualization / reset の責務が consumer 任せ。 |
| `Accordion` を section 代わりに使う | Accordion は単体動作の汎用 disclosure。settings 固有の dirty / reset / search filter が組み込めない。 |
| 単一巨大 Form widget | sections / fields / control が typed enum の方が拡張・テスト容易。 |

## Out of scope

- 「設定値の永続化」：consumer 責務
- 「設定値の検証 (validation)」のロジック：consumer 提供 callback で受ける
- 「ドキュメントへのジャンプ / link」：description 内 hyperlink は consumer の責務

## 影響範囲

- consumer の settings 画面構築を統一できる
- `FormField` を子に使う
- `SearchBox`（query）を embed
