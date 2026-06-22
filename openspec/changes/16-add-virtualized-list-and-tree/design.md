# Design — 共通 Virtualization

## 目的

大量項目を扱う molecule に共通の仮想化 API を導入し、accessibility / scroll / focus を仮想化中も保つ。

## 採用方針

### 1. 共通 module

`interaction/virtualization.rs` に純関数を置く:

```text
compute_visible_range(
  viewport_offset: f32,
  viewport_height: f32,
  row_heights: RowHeightProvider,
  total_count: usize,
  overscan: usize,
) -> VirtualRange { start, end, total }
```

- `RowHeightProvider`:
  - `Fixed(f32)`: すべて固定高
  - `Variable(fn(index) -> f32)`: 行ごと別
  - `Estimated(estimated, measured_overrides: HashMap<index, f32>)`: 推定 + measured 上書き

### 2. molecule 側 contract

- 各 molecule は option `virtualization: VirtualizationConfig` を受ける（default は enabled=false）
- 描画する row 範囲は `state.virtual_range`
- scroll event を受けて offset 更新 → 範囲再計算
- selection / focus は項目 id ベースで管理し、virtual_range の影響を受けない

### 3. accessibility

- virtualization-aware aria:
  - 全件分の `total_count` を `aria-setsize` 相当として announce
  - 各描画 row は `aria-posinset = index + 1` を持つ
  - screen reader が「3 of 1024」と読める

### 4. focused 行の常時描画

- `keep_focused_in_window=true` のとき、focused row が virtual_range 外でも例外的に描画
- scroll で focused row が見えなくなるとき、自動 scroll-into-view を発火

### 5. row height の測定

- adapter 側で row の実 height を測定し、`measured_overrides` を更新
- 測定済み行の合計 height を変えた場合、scroll position を維持するため offset 補正を行う

### 6. molecule ごとの差分

- `List`: 単純 1 列 row
- `SelectionList`: section header と row 混在、virtualization は row だけ対象 / section header は常時描画
- `TreeView`: 展開状態を考慮した可視ノード列を仮想化対象とする
- `CommandPalette`: filter 後の visible list を virtualization 対象
- `DiagnosticsList`: group + item の可視列を virtualization 対象

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| 各 molecule が独自の virtualization を持つ | 同じバグや非互換が複数箇所に発生する。 |
| virtualization なしで強行 | 数千件規模で描画コスト爆発、operability が崩れる。 |
| external runtime 専属で virtualization を提供 | runtime ごとに挙動差が出て、accessibility が揃わない。 |

## Out of scope

- horizontal virtualization：v2 以降
- 動的に列数を変える grid virtualization：v2 以降
- scroll 動作のアニメーション：`add-animation-primitives-18`

## 影響範囲

- `List` / `SelectionList` / `TreeView` / `CommandPalette` / `DiagnosticsList` の option / state / contract test を更新
- adapter 側で row 測定 callback の責務追加
- 既存 preset の挙動を破壊しない（default で virtualization=disabled）
