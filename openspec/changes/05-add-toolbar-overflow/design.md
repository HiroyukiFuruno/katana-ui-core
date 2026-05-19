# Design — Toolbar overflow + split action + density

## 目的

`Toolbar` を「実用的な toolbar 用 molecule」にするため、overflow popup・split action・display mode・density・accelerator を typed option として持たせる。

## 採用方針

### 1. overflow strategy

- `Hide`: はみ出した action を単に表示しない（priority 順）
- `Menu`: はみ出した action を overflow menu に集約。trigger は `…` ボタン
- `Custom`: consumer が `OverflowSlot` で独自に処理（advanced 用、未推奨だが MVP では入れる）

しきい値判定:
1. measured width を取得（adapter 経由 callback、初回は推定値）
2. 表示候補 actions を priority 昇順にソート（priority が高いほど残す）
3. 累積幅が available width を超えるまで追加。残りは hidden に
4. hidden が 1 件以上あれば overflow trigger を表示

### 2. action priority

```text
ToolbarAction {
  id, label, icon, tone, disabled, loading,
  priority: i32,        // 高いほど残る
  accelerator: Option<KeyCombo>,
  tooltip: Option<String>,
  accessibility_label: Option<String>,
  split: Option<SplitAction>,    // primary + secondary
  group_id: Option<GroupId>,
}
```

### 3. split action

- primary action（クリックで実行）
- secondary action は dropdown（クリックで `Menu` を開く）
- visually 区切り線で分割
- 両方が disabled なら全体 disabled
- accelerator は primary 側にだけ表示（secondary は menu 内で表示）

### 4. display mode

- `IconOnly`: icon のみ、Tooltip / accessibility label が必須
- `IconLeading`: icon + label（icon 左）
- `IconTrailing`: icon + label（icon 右）
- `LabelOnly`: label のみ

display mode 変更で measured width が変わるため overflow 再計算が必要。

### 5. density

- `Compact` / `Default` / `Spacious`
- gap / padding / button size を theme token から取得

### 6. accelerator

- `KeyCombo` を typed に持つ（`Cmd+S` / `Ctrl+Shift+P` 等）
- adapter が key event を listen し、accelerator にマッチした action を起動
- Tooltip / overflow menu で KeyCap atom により表示

### 7. groups と divider

- 連続する同 group_id action の間には divider を入れない
- 異なる group_id の境界に divider（option で nullable）を入れる
- group label を持つ場合は overflow menu 側に section header として表示

### 8. キーボードナビゲーション

- Tab で toolbar 内の interactive 要素を順次 focus
- Arrow Left / Right で action 間移動（roving tabindex）
- Home / End で先頭 / 末尾
- Enter / Space で実行
- accelerator 押下時は focus 移動なしで直接 action を起動

### 9. 共通依存

- overflow popup placement は `04-add-rich-popover-and-hover-card` の共通 placement engine を使う
- 右クリック menu は `01-add-context-menu` の `ContextMenu` を使う（option として toolbar の右クリックメニュー有効化）

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| overflow を consumer 側で実装 | しきい値判定と画像回帰が consumer 任せになり、 cross-app での挙動差が積み上がる。 |
| split action を 2 つの隣接 action として並べる | accelerator / disabled / divider / accessibility / tooltip が分裂し、原則「primary + secondary が一体の action」を表現できない。 |
| display mode を CSS-like 文字列で扱う | typed enum でないと未対応値が runtime fallback になり、入力回帰がしづらい。 |

## Out of scope

- 縦向き toolbar：別 change
- 階層化 menu within toolbar action：`01-add-context-menu` の submenu に委ねる
- 動的 action 追加のアニメーション：`add-animation-primitives-18`

## 影響範囲

- `Toolbar` molecule API の拡張（後方互換: 既存 option はデフォルト値で温存）
- 既存 `Toolbar` Storybook preset の更新
- 共通 placement engine 依存
