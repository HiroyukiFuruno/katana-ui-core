## 1. 公開回帰

- [x] 1.1 #52: legacy document typographyの整数座標契約を回帰テストで固定し、fractional baselineを明示したhostだけへlogical cursorを限定する。
- [x] 1.2 #52: render、wrap/layout、action hit、node hitが同じtypography選択を通ることを検証し、KDV registry-onlyの95/95再検証条件を記録する。
- [x] 1.3 #54: KUC所有のUnicode evidence pin resolverをconsumer artifactの既定issuerへ接続し、consumerがplatform policyを注入せず実行できる回帰テストを追加する。

## 2. 公開

- [x] 2.1 v0.3.10へ更新し、format、check、release-check、diff checkを通す。
- [ ] 2.2 Draft PR、review、thread disposition、Ready、CI、mergeを完了する。
- [ ] 2.3 GitHub Release、crates.io、KDV/KLE registry-only再検証、branch hygieneを完了する。

## 最新指示と実行台帳

- 最新指示: v0.3.10を正式リリースするまで継続する。目的が明確な承認済み工程で停止・再承認要求をしない。
- [/] 承認済み操作・担当引継ぎで再承認を求めない規則をAGENTSへ反映した。
- 承認範囲: commit / push / review reply / merge / release / cleanup。閾値緩和、未公開path/git依存、稼働targetのcleanは禁止。
- 既存検証: `/private/tmp/kuc-v0310-check-final.status` = 0、`/private/tmp/kuc-v0310-release-check.status` = 1。各同名 `.log` に出力を保持。
- coverage不足: Docker volume `kuc-coverage-target` の `kuc-workspace-coverage.lcov`。unicode_evidence、renderer_cursor、renderer_accordion_cursor、renderer_row_cursorの4ファイル。
- 依存調査: `cargo outdated --workspace --depth 1` と `--depth 999` はともに終了0。lockfileではegui群0.36.2など外部20packageが互換更新済みで、追加更新対象なし。ホストと最終coverage snapshotのCargo.lock SHA-256は `036a92e061a4217e1f273a36c1b35eca29c276b4b80a66c2fe8b434f28d07d3d` で一致する。
- memory ad-hoc noteへの追記は外部編集hookが拒否したため未実施。製品リリースの進行は継続する。
- 追加回帰: all-features lib `unicode_evidence` 55件、`logical_cursor` 3件が終了0。fmt / diff check / guardは終了0。独立製品レビューはP0/P1/P2指摘なし。
- 最終ゲート: `rtk proxy just VERSION=v0.3.10 release-check`。ログ `/private/tmp/kuc-v0310-release-final.log`、終了状態 `/private/tmp/kuc-v0310-release-final.status` に保持する。
- 最終ゲートsession_id 26860は終了1。全製品分岐は網羅、cursor.rs:633のassert失敗時メッセージ用`node.kind()`だけが未到達。静的メッセージへ変更して余計な未到達式を除去し、数値assertは維持した。
- 再実行ログは `/private/tmp/kuc-v0310-release-final-2.log`、終了状態は同名 `.status`。runningの場合は既存実行を回収し、重複起動しない。
- 再実行session_idは43856。KDV既存task `01a04a99-ba8b-7f12-8b68-188c349307de` とKLE既存task `019f2593-b323-7480-910e-700dfeecd8b9` は公開後のregistry-only再検証を準備済み。公開通知までは依存変更・ゲート実行を保留する。
- session43856は終了0。strict LCOV coverage passed、package検証、publish dry-run、単一公開crate検査、未公開version検査を完了した。最終ログは `kuc-v0310-release-final-2.log`。
