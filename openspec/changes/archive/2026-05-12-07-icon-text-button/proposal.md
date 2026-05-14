## Why

「アイコン + ラベル」の組合せボタンは command palette の項目、ツールバーのプライマリアクション、空状態の CTA など多用される。`SvgButton` と `TextButton` を別々に組まずに済むよう、合成済み widget として 1 件提供する（呼び出し側コードが冗長になるのを防ぐ）。

## What Changes

- `composite/button/icon_text/` に `IconTextButton` widget を提供。
- props: `icon: IconSource`、`label`、`icon_position`（`Leading` / `Trailing`、既定 Leading）、`variant` / `tone` / `size` / `disabled` / `loading` / `on_click`。
- `variant` / `tone` / `size` は `TextButton` と意味的に一致させる（ユーザの mental model を分断しない）。
- icon と label の間隔は `theme/spacing` の固定値。

## Capabilities

### New Capabilities

- `widget-icon-text-button`: アイコン + ラベルの組合せボタン。`SvgButton` と `TextButton` の API と一貫性を保つ。

## Impact

- toolbar、空状態 CTA、ドロップダウンのトリガなどで合成して使う。
