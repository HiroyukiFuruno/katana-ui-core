# v0.5.0 追加要件: #112

Issue: https://github.com/HiroyukiFuruno/katana-ui-core/issues/112

Open Issue再確認で追加を検出したため、初期15件と同じPR・同じリリースへ含める。

## 完了条件

- [x] 公開host projection leaseへ、項目IDとstatic enabledから最新のboolを返す汎用resolverを注入できる。未指定は既存の静的投影を維持する。
- [x] 閉時は照会しない。初回表示を含む表示frameで最新値を解決し、そのframeの描画・操作可否・AccessKit・公開recordへ一貫して反映する。
- [x] resolver失敗を型付きで返し、古いenabledを成功出力へ使わない。identity/revision/stale lease/one-shotを維持する。
- [x] 公開full-rootの実入力回帰で、閉時0回・初回即時反映・表示中更新・disabled拒否・閉鎖後停止・型付き失敗・stale/再同期を検査する。
- [ ] 既存品質条件を維持し、v0.5.0のregistry版で公開APIのcompileと入力回帰を実行する。

## 境界

メニュー開閉・配置・描画・入力・focus・AccessKitはKUCが所有する。hostは有効状態のみ返す。
外部リポジトリの採用・受入・固有処理を完了条件へ加えない。

## 検証記録

- 公開root入力回帰: 最新9件成功、exit0（session11758、tmp/v0.5.0-issue112-dismiss-fixed2.log）。最終full gateでも再回収する。
- Escapeとメニュー外クリックはresolverより先に処理し、閉鎖frameと閉鎖後frameの照会を0とする。resolver失敗中でも閉鎖できる公開回帰を追加する。旧実装のdismiss不能を実際に再現済み（tmp/v0.5.0-issue112-dismiss-repro.log）。
- 型付きerror境界UT: session22578、1件成功、exit0、tmp/v0.5.0-issue112-error-boundary.log。
- registry-only fixtureへ同じ入力回帰をコピー済み。公開後の実行と全体品質検査は未完。
