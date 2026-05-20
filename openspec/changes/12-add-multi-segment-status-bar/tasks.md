# Tasks — add-multi-segment-status-bar-12

## 1. 設計確定

- [x] 1.1 `StatusBarMode` enum と segment 構造を確定する。
- [x] 1.2 segment popover の typed spec を確定する。
- [x] 1.3 density / alignment / progress overlay を確定する。

## 2. 中核実装

- [x] 2.1 `molecule/status_bar.rs` として `StatusBar` を mode 拡張する。
- [x] 2.2 `StatusBarAction` / `StatusBarEvent` に `SegmentPressed` / `SegmentPopoverOpened` / `SegmentPopoverClosed` を追加する。
- [x] 2.3 segment popover は共通 placement engine（`add-rich-popover-and-hover-card-04`）を使う。
- [x] 2.4 後方互換性のため SingleMessage モードのデフォルト挙動を維持する。

## 3. 自動テスト

- [x] 3.1 mode = SingleMessage は既存 API と同等であることを回帰する。
- [x] 3.2 mode = MultiSegment で segment が alignment 別に正しく配置されることを検証する。
- [x] 3.3 segment interactive=true で `SegmentPressed` 発火、popover あり segment でクリック→open を検証する。
- [x] 3.4 progress overlay が指定値で描画 props に反映されることを検証する。
- [x] 3.5 mode と single message の同時設定が validation で reject されることを検証する。
- [x] 3.6 accessibility live region announce の順序を検証する。

## 4. 数値化された描画契約

- [x] 4.1 SingleMessage 主要 preset の互換を contract test で検証する。
- [x] 4.2 MultiSegment の leading / center / trailing 配置、segment 数 1〜5 を render order contract で検証する。
- [x] 4.3 progress overlay、tooltip、popover 表示を render model / placement contract で検証する。
- [x] 4.4 density 2 種類、tone 各種を props contract で検証する。
- [x] 4.5 light / dark theme を theme token contract で検証する。

## 5. Storybook ページ

- [x] 5.1 既存 `Surface > StatusBar` ページに「Multi-segment」preset を追加する。
- [x] 5.2 preset「editor status bar」「chat usage bar」「linter summary」「progress segment」「popover segment」を実装する。
- [x] 5.3 settings で mode / segments / density を切替えできるようにする。

## 6. ドキュメント

- [x] 6.1 `docs/architecture/ui-separation/owned-ui-task-map.md` の StatusBar 行に Multi-segment を追記する。

## 7. 品質ゲート / DoD

- [x] 7.1 `cargo test -p katana-ui-core` をパスする。
- [x] 7.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 7.3 `openspec validate 12-add-multi-segment-status-bar --strict` をパスする。
- [x] 7.4 数値化された描画契約 / 入力回帰 / Storybook requirement gate をパスする。
