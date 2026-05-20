# Tasks — 13-add-shortcut-combo-display

## 1. 設計確定

- [x] 1.1 `KeyCombo` / `KeyModifiers` / `KeyKind` / `NamedKey` の typed model を確定する。
- [x] 1.2 platform_display の 4 モードと separator 4 モードを確定する。
- [x] 1.3 accessibility_label 自動生成テンプレートを確定する。

## 2. 中核実装

- [x] 2.1 `atom/shortcut_combo.rs` を新設する。
- [x] 2.2 `molecule/shortcut_cheatsheet.rs` を新設する。
- [x] 2.3 `widget::atoms` / `widget::molecules` の re-export を更新する。
- [x] 2.4 platform 検出は adapter callback 経由で取得する hook を定義する。

## 3. 自動テスト

- [x] 3.1 KeyCombo serialize / parse round trip を検証する。
- [x] 3.2 platform_display=MacOS / Windows / Linux で正しい記号 / 英字が出ることを検証する。
- [x] 3.3 separator 4 種類の表示順序を検証する。
- [x] 3.4 accessibility_label 自動生成が platform に応じて正しいことを検証する。
- [x] 3.5 ShortcutCheatsheet の query が partial match で正しく filter することを検証する。
- [x] 3.6 SelectShortcut event 発火を検証する。

## 4. 数値化された描画契約

- [x] 4.1 ShortcutCombo の platform_display × separator × tone × size の主要 subset を props contract で検証する。
- [x] 4.2 ShortcutCheatsheet の Two-Column / One-Column を render interaction contract で検証する。
- [x] 4.3 light / dark theme を theme token contract で検証する。

## 5. Storybook ページ

- [x] 5.1 `Atom > ShortcutCombo` ノードを追加する。
- [x] 5.2 `Molecule > ShortcutCheatsheet` ノードを追加する。
- [x] 5.3 preset「macOS」「Windows」「Linux」「cheatsheet sample」「カテゴリ filter」を実装する。
- [x] 5.4 settings で platform_display / separator / size / tone を切替えできるようにする。

## 6. ドキュメント

- [x] 6.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に ShortcutCombo / ShortcutCheatsheet 行を追加する。

## 7. 品質ゲート / DoD

- [x] 7.1 `cargo test -p katana-ui-core` をパスする。
- [x] 7.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 7.3 `openspec validate 13-add-shortcut-combo-display --strict` をパスする。
- [x] 7.4 数値化された描画契約と Storybook requirement gate をパスする。
