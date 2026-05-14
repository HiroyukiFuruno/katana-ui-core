## Why

設定パネル / 個別オプション切替で必要となる on/off スイッチ。OS ネイティブ風 (摘み付き) よりも、フラットな矩形トグルを Floem で素直に表現する。`Theme` のトークンで状態色を統一する。

## What Changes

- `composite/selector/toggle/` に `Toggle` widget を提供。
- props: `value: bool`、`on_change: Fn(bool)`、`size`、`disabled`、`a11y_label`（必須）。
- `value` 反映でつまみが左右にアニメーション移動。色は `theme/color` の accent / surface を使い分け。
- focus-ring は theme spacing トークン。

## Capabilities

### New Capabilities

- `widget-toggle`: フラット矩形の on/off スイッチ。状態色 / 動作 / a11y 必須化を統一。

## Impact

- 設定 UI / 個別オプション切替で利用される。
- 「何のためのトグルか」の文脈は `a11y_label` で必須化することで、Storybook 上でも意味のあるサンプルを書かせる。
