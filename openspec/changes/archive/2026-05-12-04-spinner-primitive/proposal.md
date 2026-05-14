## Why

ローディング表示は button / input / tooltip / modal など多くの widget で要求される横断的なフィードバック。各 widget が独自実装すると見た目とアニメーション周期がずれるため、最小の `Spinner` primitive を 1 つだけ用意して再利用させる。

## What Changes

- `primitive/spinner/` に `Spinner` widget を提供。
- props: `size`（theme spacing トークン or 明示 pt）、`color`（theme color トークン）、`speed`（既定 1 周/sec）。
- 実装は円周上の弧 or 点列を `requestAnimationFrame` 相当のティックで回転（Floem の animation API を利用）。
- インデターミネート専用。プログレス値表示は対象外（必要になったら別 change）。

## Capabilities

### New Capabilities

- `widget-spinner-primitive`: 一定速度で回転するインデターミネート ローディング表示 widget。

## Impact

- 05 (svg-button), 12 (text-input), 13 (search-box), 20 (modal-overlay) などで「読み込み中」を示すために合成される想定。
