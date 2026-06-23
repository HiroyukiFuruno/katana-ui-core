# Design — EmptyState molecule

## 目的

「空である」状態を統一テンプレートで表現する molecule を追加する。あらゆる list / view / panel に embed できる。

## 採用方針

### 1. 構造

- アイコン or イラスト（どちらかでよい）
- 見出し（heading）必須
- 補足説明（body）optional
- primary / secondary action（`Button` atom）optional
- alignment と size と tone

### 2. tone のマッピング

- `Neutral`: 通常空、無色傾向
- `Subtle`: バックグラウンド情報
- `Accent`: 利用促進 (call-to-action)
- `Warning`: 軽い注意
- `Danger`: エラー由来の空

### 3. action

- primary は強調表示
- secondary は ghost / link 表示
- どちらも `Button` atom を子に持つ
- callback は `EmptyStateEvent::Actioned { id: Primary | Secondary, action_id }` event

### 4. accessibility

- heading は root label として保持し、支援技術向け payload は `accessibility_label` に出す
- live region announce: tone と heading を `announce_payload()` で読み上げる

### 5. 内部レイアウト

- vertical stack: illustration / icon → heading → body → action row
- alignment=Center で中央寄せ、Leading で左寄せ

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| consumer 毎に `Card` + `Text` + `Button` を組む | テンプレートが揃わず、空表示の視覚的一貫性が崩れる。layout snapshot / render command / theme token / accessibility payload の契約から漏れる。 |
| `Card` molecule に empty option を追加 | Card は一般 surface であり、empty 状態用の icon/illustration/action 構成は別責務。 |

## Out of scope

- illustration の SVG ライブラリ管理：consumer 責務
- インライン onboarding tour：別 widget
- アニメーション：`add-animation-primitives-18`

## 影響範囲

- DiagnosticsList / SelectionList / TreeView / CommandPalette / SearchBox の empty 表示で embed
- consumer 側の empty 表示を統一できる
