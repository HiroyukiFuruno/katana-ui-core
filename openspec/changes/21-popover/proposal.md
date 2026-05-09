## Why

ドロップダウン / メニュー / 補足情報のように「特定の要素にアンカーされて浮かぶ小窓」が必要。SelectBox (10) / Tooltip (14) は本 change 完了前は最小自前実装で動かしているが、最終的にはここで導入する `Popover` をベースにレイヤー管理を共通化する。`Modal` と異なり画面全体を覆わず、特定アンカー周辺に位置決めする責務を持つ。

## What Changes

- `layout/popover/` に `Popover` widget を提供。
- props: `open: bool`、`on_close: Fn()`、`anchor: AnchorRef`、`placement`（`Top` / `Bottom` / `Start` / `End`、画面端で自動反転）、`offset`、`dismiss_on_outside_click: bool`（既定 true）、`dismiss_on_esc: bool`（既定 true）、`children: View`。
- 位置決めは anchor の global rect を取得して placement 規則に従って配置。画面端に近い場合は反対方向に flip。
- フォーカス管理は modal よりも軽量（trap しない、ただしキーボード操作可能）。
- 10 (SelectBox) / 14 (Tooltip) を本 change 完了後に Popover ベースに置き換える追従 task を本 change の tasks に含める。

## Capabilities

### New Capabilities

- `widget-popover`: アンカー追従 + 自動反転 + dismiss 慣習 を統一したアンカー型 overlay。

### Modified Capabilities

- `widget-select-box`: 内部実装を `Popover` ベースに置換（API は変更なし）。
- `widget-tooltip`: 内部実装を `Popover` ベースに置換（API は変更なし）。

## Impact

- 10 / 14 が暫定実装から置換されるため、両者の Storybook ページに「リファクタ後も同じ見た目で動く」回帰確認を含める。
- 将来のドロップダウンメニュー / コンテキストメニュー widget の基盤になる。
