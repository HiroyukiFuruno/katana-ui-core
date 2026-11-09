## 1. 公開APIと既存root経路

- [x] 1.1 Issue #124の既存API・full-root契約をbuilderと統合テストへ対応付け、公開struct互換性を確認する。
- [x] 1.2 Issue #126の空label描画経路・paint operation・bounds・AccessKit契約を現行テストと照合する。

## 2. 検索mode presentation

- [x] 2.1 公開SearchModePresentation型、validity enum、lease builder/getterとmode target validationを実装する。
- [x] 2.2 retained query/include/exclude inputへhintとinvalid状態をfocus/IMEを維持して同期する。
- [x] 2.3 履歴headingを非操作labelとして表示し、空headingでraster requestを出さない。
- [x] 2.4 公開root統合テストでmode switch、lease再同期、空/日本語hint、validity切替、history有無、AX/event境界を検証する。

## 3. 空TabStrip label

- [x] 3.1 TabStripTextRasterizerに空専用の0x0 component rasterを追加し、汎用rasterizerのEmptyText拒否を維持する。
- [x] 3.2 strip/overlay/drag等の全TabStrip paint callsiteで空rasterのtexture/paint operationを省き、bounds/route/AXを維持する。
- [x] 3.3 公開full-editor rootで初期空名と日本語→⭐️(U+FE0F付き)→空名→継続入力を同一rootで検証する。

## 4. 依存更新と完全品質gate

- [x] 4.1 直接/推移/lockfile依存の最新互換候補を調査し、egui-winit 0.37候補のnative IME/semver影響を評価する。
- [ ] 4.2 選定した依存版を更新し、API回帰を含む全品質gateを元の閾値で実行する。
- [ ] 4.3 OpenSpec strict validation、format、API利用例、registry版consumer検証を完了し、終了code/logを保存する。

## 5. 公開と事後整理

- [ ] 5.1 release/v0.6.1を署名済み1コミットにし、Draft PRを作成する。
- [ ] 5.2 Codex/自己reviewを実行し、全指摘修正/reply/resolve、P0/P1=0を確認する。
- [ ] 5.3 Readyへ昇格しrequired checks成功後mergeする。
- [ ] 5.4 公式Release workflow、GitHub Release、crates.io版を個別確認し、公開crateだけで回帰を実行する。
- [ ] 5.5 Issue #124/#126をCloseし、default branch同期・local release branch cleanup・stash/worktree/dirty状態を確認する。
