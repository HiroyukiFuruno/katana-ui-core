# Tasks — 14-add-sectioned-settings-form

## 1. 設計確定

- [x] 1.1 `SettingsList` / `SettingsSection` / `SettingsField` / `SettingsControl` の typed model を確定する。
- [x] 1.2 dirty_visualization の 3 種類と reset_to_default の挙動を確定する。
- [x] 1.3 query filter の対象（label / description / section label）を確定する。

## 2. 中核実装

- [x] 2.1 `molecule/app_primitives/settings/` として `SettingsList` を新設する。
- [x] 2.2 option / action / event / state を実装する。
- [x] 2.3 control enum を既存 atom / molecule に bind する。
- [x] 2.4 子 `UiStateId` 分離を実装する。
- [x] 2.5 `widget::molecules` の re-export に `SettingsList` を追加する。

## 3. 連携

- [x] 3.1 query 部分に `SearchBox` を embed する。
- [x] 3.2 empty state に `EmptyState`（`09-add-empty-state`）を embed する。
- [x] 3.3 `FormField` を field 描画に使う。

## 4. 自動テスト

- [x] 4.1 query 切替えで section / field が正しく filter されることを検証する。
- [x] 4.2 control 各種（Toggle / Select / Combo / Input / TextArea / Number / Chips / Radio / ColorPicker / Custom）の event ルーティングを検証する。
- [x] 4.3 dirty_visualization 3 種類が正しく描画 props を変えることを検証する。
- [x] 4.4 reset_to_default のボタン押下で `FieldReset` 発火と値復帰を検証する。
- [x] 4.5 SectionCollapsed の toggle が `collapsed_section_ids` に反映されることを検証する。
- [x] 4.6 query が全件フィルタアウトしたとき EmptyState が表示されることを検証する。
- [x] 4.7 child `UiStateId` が一意で衝突しないことを検証する。

## 5. 画像回帰

- [ ] 5.1 control 各種 × density 3 種類 × dirty_visualization 3 種類を回帰する。
- [ ] 5.2 section collapsed / expanded を回帰する。
- [ ] 5.3 query で zero match の EmptyState を回帰する。
- [ ] 5.4 light / dark theme を回帰する。

## 6. Storybook ページ

- [x] 6.1 `Structured > SettingsList` ノードを追加する。
- [ ] 6.2 preset「app settings」「chat settings」「lint settings」「dirty 表示」「query filter」「reset」を実装する。
- [ ] 6.3 settings で density / dirty_visualization / query / sections / control kind を切替えできるようにする。

## 7. ドキュメント

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に SettingsList 行を追加する。

## 8. 品質ゲート / DoD

- [x] 8.1 `cargo test -p katana-ui-core` をパスする。
- [x] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 8.3 `openspec validate 14-add-sectioned-settings-form --strict` をパスする。
- [ ] 8.4 画像 / 入力回帰 CI gate をパスする。
