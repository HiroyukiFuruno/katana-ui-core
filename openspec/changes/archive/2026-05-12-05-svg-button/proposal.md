## Why

ツールバー / ヘッダ / 各種フォームで「アイコンのみのクリック可能要素」は最頻出。既存の Adapter `button + icon` 直書きでは hover / active / disabled / focus-ring / a11y label が呼び出し側ごとにバラつくため、汎用 widget として固定する。`Icon` primitive と `theme` を素直に合成した最小ボタン。

## What Changes

- `composite/button/svg/` に `SvgButton` widget を提供。
- props: `icon: IconSource`、`size`、`variant`（`Plain`, `Subtle`, `Filled`）、`tone`（`Neutral`, `Accent`, `Danger`）、`disabled`、`loading`、`a11y_label`（必須）、`on_click`。
- `loading=true` の場合は icon の代わりに `Spinner` を中央に表示。
- hover / active / focus-ring の色はすべて theme トークンから引く。

## Capabilities

### New Capabilities

- `widget-svg-button`: アイコンのみのクリック可能要素。状態（hover/active/focus/disabled/loading）と variant/tone を theme トークンで統一。

## Impact

- toolbar / modal header / search-box の clear ボタンなど、後続 widget で `SvgButton` を内蔵して使う想定。
- a11y label 必須化により「アイコンのみで意味が伝わらない」問題を構造的に防ぐ。
