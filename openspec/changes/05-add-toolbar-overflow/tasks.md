# Tasks — 05-add-toolbar-overflow

## 1. 設計確定

- [ ] 1.1 `ToolbarAction` の typed option（priority / accelerator / split / group_id / tooltip）を確定する。
- [ ] 1.2 overflow strategy（Hide / Menu / Custom）を確定する。
- [ ] 1.3 display_mode（IconOnly / IconLeading / IconTrailing / LabelOnly）と density（Compact / Default / Spacious）を確定する。
- [ ] 1.4 accelerator が consumer 側 keyboard listener と整合する仕様を確定する。

## 2. 中核実装

- [ ] 2.1 `molecule/toolbar/` に再配置（または `basic.rs` から分離）し、新 option を実装する。
- [ ] 2.2 `actions.rs` で `Press` / `Activate` / `OpenOverflow` / `OpenSplitDropdown` / `ToggleGroupCollapse` を実装する。
- [ ] 2.3 `events.rs` で `Command` / `OverflowOpened` / `SplitDropdownOpened` / `AcceleratorTriggered` を実装する。
- [ ] 2.4 `state.rs` で active_action / overflow_visible / split_open / measured_widths を持たせる。
- [ ] 2.5 overflow しきい値計算を純関数として `overflow.rs` に実装する。
- [ ] 2.6 accelerator マッチを純関数として `accelerator.rs` に実装する。

## 3. 共通依存

- [ ] 3.1 overflow popup placement に共通 placement engine（`04-add-rich-popover-and-hover-card`）を使う。
- [ ] 3.2 右クリック menu に `ContextMenu`（`01-add-context-menu`）を統合できる hook option を追加する。

## 4. 自動テスト

- [ ] 4.1 overflow 計算が priority 順に hidden を選ぶことを純関数で検証する。
- [ ] 4.2 display_mode 変更時に measured_widths が再計算されることを検証する。
- [ ] 4.3 split action の primary / secondary が独立 disabled、accelerator 表示位置を検証する。
- [ ] 4.4 accelerator マッチが focus 移動なしで `Command` を発火することを検証する。
- [ ] 4.5 IconOnly のとき accessibility_label が欠落していると静的検査でエラーになることを検証する。
- [ ] 4.6 group_id 境界に divider が入り、同一 group 内には入らないことを検証する。
- [ ] 4.7 キーボードナビゲーション（Arrow / Home / End / Enter / Space）を検証する。

## 5. 画像回帰

- [ ] 5.1 default / overflow trigger 表示 / overflow menu 展開を回帰する。
- [ ] 5.2 split action（normal / split open / disabled）を回帰する。
- [ ] 5.3 display_mode 4 種類、density 3 種類を回帰する。
- [ ] 5.4 group divider / group section header を回帰する。
- [ ] 5.5 light / dark theme での accelerator KeyCap を回帰する。

## 6. Storybook ページ

- [ ] 6.1 既存 Toolbar Storybook ページに「overflow」「split action」「display mode」「density」「accelerator」preset を追加する。
- [ ] 6.2 settings で action 追加 / priority 変更 / overflow strategy / display_mode / density を切替えできるようにする。
- [ ] 6.3 measured width をシミュレートし、container 幅変更で overflow が再計算されることを可視化する。

## 7. ドキュメント

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` の Toolbar 行に拡張内容を追記する。
- [ ] 7.2 IconOnly 時の accessibility 必須化を `docs/widget-extraction-policy.md` に追記する。

## 8. 品質ゲート / DoD

- [ ] 8.1 `cargo test -p katana-ui-core` をパスする。
- [ ] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [ ] 8.3 `openspec validate 05-add-toolbar-overflow --strict` をパスする。
- [ ] 8.4 画像回帰 / 入力回帰の CI gate をパスする。
