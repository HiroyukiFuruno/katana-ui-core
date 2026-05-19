# Tasks — 20-add-splash-screen-template

## 1. 設計確定

- [x] 1.1 `StartupState` と retry / cancel event を確定する。
- [x] 1.2 accessibility role と live region announce を確定する。
- [x] 1.3 splash template、full-screen layout、background image を KUC 対象外にする境界を確定する。

## 2. 中核実装

- [x] 2.1 `molecule/structured/startup_state_panel.rs` を新設する、または既存 `EmptyState` / `Banner` / `ProgressBar` の composition contract として実装する。
- [x] 2.2 option / action / event / state を実装する。
- [x] 2.3 `widget::molecules` の re-export に `StartupStatePanel` を追加する場合でも template API は追加しない。
- [x] 2.4 `SplashScreen` / full-screen template が public API に入らない guard を用意する。

## 3. 連携

- [x] 3.1 progress 表示に `ProgressBar` atom を使う。
- [x] 3.2 Error の retry / cancel に `Button` atom を使う。
- [x] 3.3 アニメーションに `18-add-animation-primitives` の MotionSpec を使う。

## 4. 自動テスト

- [x] 4.1 state の遷移（Idle → Loading → Error → Idle）が `StartupStateChanged` を順に発火することを検証する。
- [x] 4.2 Error の retry 押下で `StartupRetried` が発火することを検証する。
- [x] 4.3 progress=None で indeterminate、Some(u8) で determinate bar が出ることを検証する。
- [x] 4.4 accessibility role が Idle/Loading=status、Error=alert に切替わることを検証する。
- [x] 4.5 full-screen / centered layout option が public API に存在しないことを検証する。
- [x] 4.6 reduced-motion 時、loading animation が Instant / None になることを検証する。

## 5. 画像回帰

- [ ] 5.1 Idle / Loading(determinate, indeterminate) / Error の 4 状態を回帰する。
- [ ] 5.2 retry / cancel action の有無を回帰する。
- [ ] 5.3 version label の有無を回帰する。
- [ ] 5.4 light / dark theme を回帰する。

## 6. Storybook ページ

- [x] 6.1 `Structured > StartupStatePanel` ノードを追加する。
- [x] 6.2 preset「app boot」「session init」「update install」「error retry」を実装する。
- [ ] 6.3 settings で state / progress / label / retry / cancel を切替えできるようにする。

## 7. ドキュメント

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に StartupStatePanel または composition contract 行を追加する。

## 8. 品質ゲート / DoD

- [x] 8.1 `cargo test -p katana-ui-core` をパスする。
- [x] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 8.3 `openspec validate 20-add-splash-screen-template --strict` をパスする。
- [ ] 8.4 画像 / 入力回帰 CI gate をパスする。
