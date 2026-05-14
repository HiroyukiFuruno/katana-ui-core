## Why

ステータス / カテゴリ / カウント表示の小さなラベル。README にも「badgeなどの共有UI部品」と明記された KUW のスコープ要件。`Text` を直に色付けすると padding / 角丸 / tone 切替が呼び出し側で重複するため、`Badge` widget として固定する。

## What Changes

- `composite/indicator/badge/` に `Badge` widget を提供。
- props: `label: String`、`tone`（`Neutral`, `Accent`, `Danger`, `Warning`, `Success`, `Info`）、`variant`（`Solid` / `Subtle` / `Outline`）、`size`（`Sm` / `Md`）、`leading_icon: Option<IconSource>`。
- 角丸 / padding / typography role はすべて theme トークン経由。

## Capabilities

### New Capabilities

- `widget-badge`: ステータス / カテゴリ表示用の小ラベル。tone × variant × size を統一。

## Impact

- リスト項目装飾、ヘッダの状態表示、unresolved 表示（README 記載）などで利用。
