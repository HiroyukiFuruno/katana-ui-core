# Tasks — 19-add-title-bar-window-chrome

## 1. 設計確定

- [ ] 1.1 `TitleBarStyle` 3 種、`WindowControlsPosition` 3 種、`WindowControls` 2 種を確定する。
- [ ] 1.2 draggable_regions の adapter 伝達契約を確定する。
- [ ] 1.3 height token と OS デフォルトを確定する。
- [ ] 1.4 fullscreen 時の TitleBar visibility 仕様を確定する。

## 2. 中核実装

- [ ] 2.1 `molecule/structured/title_bar.rs` を新設する。
- [ ] 2.2 option / action / event / state を実装する。
- [ ] 2.3 controls / slots / drag region の layout を実装する。
- [ ] 2.4 `widget::molecules` の re-export に `TitleBar` を追加する。

## 3. window 連携

- [ ] 3.1 `runtime / window / surface` に `WindowCommand` の EnterFullscreen / ExitFullscreen を追加する。
- [ ] 3.2 adapter contract に window controls dispatch / draggable region transfer 責務を明記する。
- [ ] 3.3 floem / egui / gpui adapter に compile-gate stub を追加する。

## 4. 自動テスト

- [ ] 4.1 style 3 種類の表示 props を検証する。
- [ ] 4.2 position 3 種類で controls 配置が切替わることを検証する。
- [ ] 4.3 draggable_regions に interactive elements が overlay されない（自動 carve-out）ことを検証する。
- [ ] 4.4 controls press で `ControlPressed { which }` 発火、`WindowCommand` 発火が一貫していることを検証する。
- [ ] 4.5 fullscreen 中の auto-hide trigger（hover で再表示）を検証する。
- [ ] 4.6 height token 3 種類の layout snapshot を検証する。

## 5. 画像回帰

- [ ] 5.1 style × position × height の主要 subset を回帰する。
- [ ] 5.2 macOS / Windows / Linux 風 preset を回帰する。
- [ ] 5.3 leading / center / trailing slot 構成を回帰する。
- [ ] 5.4 light / dark theme を回帰する。

## 6. Storybook ページ

- [ ] 6.1 `Structured > TitleBar` ノードを追加する。
- [ ] 6.2 preset「macOS 風」「Windows 風」「Linux 風」「fullscreen 表示」「Custom controls」を実装する。
- [ ] 6.3 settings で style / position / height / controls / slots を切替えできるようにする。

## 7. ドキュメント

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に TitleBar 行を追加する。
- [ ] 7.2 `docs/compat-adapters.md` に window controls dispatch / draggable region 責務を追記する。

## 8. 品質ゲート

- [ ] 8.1 `cargo test -p katana-ui-core` をパスする。
- [ ] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [ ] 8.3 `openspec validate 19-add-title-bar-window-chrome --strict` をパスする。
- [ ] 8.4 adapter compile-gate と画像回帰 CI gate をパスする。
