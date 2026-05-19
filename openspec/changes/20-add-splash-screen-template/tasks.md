# Tasks — 20-add-splash-screen-template

## 1. 設計確定

- [ ] 1.1 `SplashStatus` / `SplashBackground` / `SplashSize` を確定する。
- [ ] 1.2 accessibility role と live region announce を確定する。
- [ ] 1.3 アニメーション default を確定する（reduced-motion 対応含む）。

## 2. 中核実装

- [ ] 2.1 `molecule/structured/splash_screen.rs` を新設する。
- [ ] 2.2 option / action / event / state を実装する。
- [ ] 2.3 `widget::molecules` の re-export に `SplashScreen` を追加する。

## 3. 連携

- [ ] 3.1 progress 表示に `ProgressBar` atom を使う。
- [ ] 3.2 Error の retry / cancel に `Button` atom を使う。
- [ ] 3.3 アニメーションに `add-animation-primitives-18` の MotionSpec を使う。

## 4. 自動テスト

- [ ] 4.1 status の遷移（Idle → Loading → Error → Idle）が `SplashStatusChanged` を順に発火することを検証する。
- [ ] 4.2 Error の retry 押下で `SplashRetried` 発火、status が Idle に戻ることを検証する。
- [ ] 4.3 progress=None で indeterminate spinner、Some(f32) で determinate bar が出ることを検証する。
- [ ] 4.4 accessibility role が Idle/Loading=status、Error=alert に切替わることを検証する。
- [ ] 4.5 size=Window で中央寄せの layout snapshot を検証する。
- [ ] 4.6 reduced-motion 時、logo アニメーションが Instant になることを検証する。

## 5. 画像回帰

- [ ] 5.1 Idle / Loading(determinate, indeterminate) / Error の 4 状態を回帰する。
- [ ] 5.2 background 3 種類（Solid / Gradient / Image）を回帰する。
- [ ] 5.3 size 2 種類（Embedded / Window）を回帰する。
- [ ] 5.4 light / dark theme を回帰する。

## 6. Storybook ページ

- [ ] 6.1 `Structured > SplashScreen` ノードを追加する。
- [ ] 6.2 preset「app boot」「session init」「update install」「error retry」を実装する。
- [ ] 6.3 settings で status / background / size / progress / label を切替えできるようにする。

## 7. ドキュメント

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に SplashScreen 行を追加する。

## 8. 品質ゲート

- [ ] 8.1 `cargo test -p katana-ui-core` をパスする。
- [ ] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [ ] 8.3 `openspec validate 20-add-splash-screen-template --strict` をパスする。
- [ ] 8.4 画像 / 入力回帰 CI gate をパスする。
