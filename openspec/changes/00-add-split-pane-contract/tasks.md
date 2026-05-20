# Tasks — 00-add-split-pane-contract

## 1. 設計確定

- [x] 1.1 `SplitPaneOptions` の axis / ratio / min / max / reset / handle / resize mode を確定する。
- [x] 1.2 primary slots を `first` / `second` の 2 pane contract として固定する。
- [x] 1.3 `CollapsiblePanel` / `AppShell` / viewer-editor sync との境界を `design.md` に固定する。

## 2. 中核実装

- [x] 2.1 `layout::SplitPane` に typed options と 2 pane slot API を追加する。
- [x] 2.2 `UiAction` に `SetRatio` / `ResizeBy` / `ResetRatio` / `StartResize` / `EndResize` を追加する。
- [x] 2.3 `UiEvent` に `RatioChanged` / `ResizeStarted` / `ResizeEnded` / `ResizeRejected` を追加する。
- [x] 2.4 handle props を render model へ追加する。
- [x] 2.5 persistence は event に留め、KUC が storage を持たないようにする。

## 3. 連携

- [x] 3.1 `15-add-collapsible-sidebar-shell` の `CollapsiblePanel` と option enum を共有しない。
- [x] 3.2 Storybook の panel layout で `SplitPane` と `CollapsiblePanel` の用途差を表示する。

## 4. 自動テスト

- [x] 4.1 ratio が min / max に clamp されることを検証する。
- [x] 4.2 drag resize の event 順が `ResizeStarted` → `RatioChanged` → `ResizeEnded` になることを検証する。
- [x] 4.3 keyboard resize が axis と step に従うことを検証する。
- [x] 4.4 reset が `reset_percent` へ戻ることを検証する。
- [x] 4.5 `AppShell` / sidebar collapse option が `SplitPane` public API に存在しないことを guard で検証する。

## 5. 自動回帰

- [x] 5.1 horizontal / vertical の主要 preset を contract test で回帰する。
- [x] 5.2 min / max / reset / disabled resize を contract test で回帰する。
- [x] 5.3 drag 中 handle、focus ring、keyboard focus 状態を state / event contract で回帰する。
- [x] 5.4 light / dark theme は Storybook panel theme gate と typed props で回帰する。

## 6. Storybook ページ

- [x] 6.1 `Layouts > SplitPane` に typed split contract の page を追加する。
- [x] 6.2 settings で axis、ratio、min、max、reset、resize mode を切替えできるようにする。
- [x] 6.3 state に ratio、dragging、focused handle、last event を表示する。
- [x] 6.4 action / event log に resize / reset / rejected event を表示する。
- [x] 6.5 quality に clamp、event order、public API guard の結果を表示する。

## 7. ドキュメント

- [x] 7.1 `docs/ui-separation-plan.md` の SplitPane 項目へ typed contract を追記する。
- [x] 7.2 `docs/inventory/katana-katana-chat-ui-kdv-kle-ui-needs.md` の SplitPane 行と同期する。

## 8. 品質ゲート / DoD

- [x] 8.1 `openspec validate 00-add-split-pane-contract --strict` をパスする。
- [x] 8.2 `cargo test -p katana-ui-core` をパスする。
- [x] 8.3 `cargo clippy -p katana-ui-core -p katana-ui-core-storybook --all-targets -- -D warnings` をパスする。
- [x] 8.4 自動回帰 / 入力回帰 / 静的検査の CI gate をパスする。
