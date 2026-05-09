## Why

ダイアログのアクション、フォームの submit、トーストの「閉じる」など、テキストラベルの汎用ボタンは UI で最頻出。`Text` primitive と theme を組み合わせ、variant（強弱）と tone（中立/危険/成功）を統一する `TextButton` を提供する。

## What Changes

- `composite/button/text/` に `TextButton` widget を提供。
- props: `label`、`variant`（`Primary`, `Secondary`, `Ghost`, `Link`）、`tone`（`Neutral`, `Accent`, `Danger`, `Success`）、`size`（`Sm`, `Md`, `Lg`）、`disabled`、`loading`、`on_click`。
- `loading=true` の場合は `Spinner` をラベル先頭に表示し、ラベルは半透明にする。
- focus-ring と padding は theme spacing を参照。

## Capabilities

### New Capabilities

- `widget-text-button`: テキストラベルの汎用ボタン。variant/tone/size/state を theme トークンで統一。

## Impact

- modal-overlay (20)、tooltip (14)、各種フォーム合成で利用。
- Link variant は `<a>` のような外観だがイベントは `on_click` 経由（ナビゲーションは消費側責務）。
