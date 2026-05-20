## Why

`katana-chat-ui` の composer は attachments（ファイル / 画像 / URL / ペーストプレビュー）を、`katana` editor は paste preview や inline reference を、`katana` explorer header は filter tag を、いずれも「アイコン + ラベル + 状態 + dismiss / action」を持つチップ（chip / pill）状の UI で表現している。これらは現状 KUC の外で個別実装されている。

`Badge` atom は表示専用で dismiss / action / tone variant が薄い。`Button` を流用すると角丸 / icon + label / dismiss + main action の組み合わせを表現しづらい。`Chip` を新規 atom として、`AttachmentChip` をリッチ拡張 molecule として導入する。

## What Changes

- `widget::atoms` に `Chip` atom を追加する:
  - option: `label`, `leading_icon`, `trailing_icon`, `tone`, `variant`（Solid / Soft / Outline / Ghost）, `size`, `interactive`, `selected`, `disabled`, `dismissible`, `accessibility_label`
  - action: `Press` / `Dismiss`
  - event: `ChipPressed` / `ChipDismissed` / `Focus` / `Blur`
  - state: `selected`, `focused`, `disabled`, `callback_log`
- `widget::molecules` に `AttachmentChip` molecule を追加する:
  - option: `kind`（File / Image / URL / Paste / Resource）, `name`, `meta`（size / mime / status）, `icon`, `thumbnail`, `progress`, `status`（Pending / Uploading / Ready / Error）, `actions`
  - action: `OpenPreview` / `Dismiss` / `Retry`
  - thumbnail / progress / status icon の組み合わせを `Chip` の slot として持つ
- chip 並びの容器として `ChipGroup` molecule（行折返し / 横スクロール / 数値超過時の overflow）を追加する。

## Capabilities

### New Capabilities

- `kuc-chip-atom`: Chip atom の option / action / event / state / preset / preview / settings / 自動テスト / 数値化された描画契約 / Storybook ページの完了条件を定義する。
- `kuc-attachment-chip`: AttachmentChip molecule と ChipGroup molecule の完了条件を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `Badge`（表示専用）と `Chip`（interactive + dismiss + tone variant）の責務境界を明記する。

## Impact

- `crates/katana-ui-core/src/atom/` に `chip.rs` を追加する。
- `crates/katana-ui-core/src/molecule/` に `attachment_chip.rs` / `chip_group.rs` を追加する。
- 既存 `Badge` Storybook で「dismiss / interactive が必要なら Chip を使う」明示する。
- consumer (`katana-chat-ui` composer attachments、`katana` explorer header filter) は KUC chip 系に置き換え可能になる。
