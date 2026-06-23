# Storybook page: window-control-button-group

## Why

Storybook の完了判定を旧 01〜24 ではなく menu page 単位へ分離する。
`window-control-button-group` page は `requirements.rs` と Storybook menu に存在するため、独立した leaf change として `storybook-ui-harness` の必須構成を満たす必要がある。

## What Changes

- `window-control-button-group` page の専用 preview、preset、Inspector option、state / action / event、操作または受動契約、自動テストを leaf change 単位で追跡する。
- 既存の入力元 change は参照元に下げ、完了判定はこの leaf change で行う。
- `draw_page` の page 別描画があるだけでは完了扱いにしない。

## Capabilities

### New Capabilities

- `storybook-page-window-control-button-group`: `window-control-button-group` Storybook page の harness 契約を定義する。

## Impact

- 対象 page: `window-control-button-group`
- Storybook group: `Runtime`
- 入力元: 19-add-title-bar-window-chrome
- 現状: page別描画未作成
- 親棚卸し: `openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-change-split.md`
