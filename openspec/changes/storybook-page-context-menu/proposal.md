# Storybook page: context-menu

## Why

Storybook の完了判定を旧 01〜24 ではなく menu page 単位へ分離する。
`context-menu` page は `requirements.rs` と Storybook menu に存在するため、独立した leaf change として `storybook-ui-harness` の必須構成を満たす必要がある。

## What Changes

- `context-menu` page の専用 preview、preset、Inspector option、state / action / event、操作または受動契約、自動テストを leaf change 単位で追跡する。
- 既存の入力元 change は参照元に下げ、完了判定はこの leaf change で行う。
- `draw_page` の page 別描画があるだけでは完了扱いにしない。

## Capabilities

### New Capabilities

- `storybook-page-context-menu`: `context-menu` Storybook page の harness 契約を定義する。

## Impact

- 対象 page: `context-menu`
- Storybook group: `Feedback`
- 入力元: 01-add-context-menu
- 現状: page別描画あり
- 親棚卸し: `openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-change-split.md`
