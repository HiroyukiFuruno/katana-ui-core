## Why

`katana` の settings 画面、`katana-chat-ui` の chat settings、`katana-markdown-linter` の lint settings は、いずれも「カテゴリ (section) ごとに分かれた、ラベル + 説明 + 入力コントロール（toggle / select / input / number / chip group / radio）が縦に並ぶ form 構造」を取る。

KUC は `FormField` molecule（ラベル + 子コントロール + エラー）を持つが、複数のフィールドを section に束ねた「設定一覧」widget はない。consumer ごとに `Card` + `Text` + `Toggle` を ad hoc に並べており、行間 / セクション余白 / セクション collapse / 「変更あり」マーカー / search filter 等の表現が揺れている。

## What Changes

- `widget::molecules` に `SettingsList` molecule を追加する:
  - option:
    - `sections: Vec<SettingsSection>`
    - `query: Option<String>`（filter）
    - `density: Compact | Default | Spacious`
    - `dirty_visualization: None | Marker | Highlight`
  - action: SetQuery / SetSectionCollapsed / EditField / ResetField
  - event: FieldChanged / FieldReset / SectionCollapsed / QueryChanged
  - state: collapsed_section_ids, query, dirty_field_ids, focused_field_id, callback_log
- `SettingsSection`:
  - id, label, description, icon, collapsible, default_collapsed, fields, footer
- `SettingsField`:
  - id, label, description, control: SettingsControl, reset_to_default, dirty, error
- `SettingsControl` enum:
  - `Toggle(Toggle)`、`Select(SelectBox)`、`Combo(ComboBox)`、`Input(Input)`、`TextArea(TextArea)`、`Number(SlideControl or Input)`、`Chips(ChipGroup)`、`Radio(RadioGroup)`、`ColorPicker(ColorPicker)`、`Custom(UiTree)`

## Capabilities

### New Capabilities

- `kuc-settings-list`: SettingsList molecule の option / action / event / state / preset / preview / settings / 自動テスト / 数値化された描画契約 / Storybook ページの完了条件を定義する。

## Impact

- `crates/katana-ui-core/src/molecule/app_primitives/settings/` 新設。
- consumer (`katana` settings、`katana-chat-ui` chat settings、`katana-markdown-linter` lint settings) は KUC molecule に統一可能になる。
- 内部で既存 atom / molecule（Toggle, SelectBox, ComboBox, Input, TextArea, ChipGroup, Radio, ColorPicker, SlideControl）を再利用。
- search filter は `SearchBox` molecule を embed する。
