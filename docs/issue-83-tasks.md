# Issue #83

対象: KUC の Card 子インデントを公開 render-model API で設定可能にする。
既定の16pxと既存JSON互換性を維持し、0pxおよび任意pxを指定可能にする。
描画・測定・hit/scroll の座標を一致させ、数値契約と既存品質検査で確認する。
外部repoの変更・採用は範囲外。v0.4.2 の Draft PR、review、merge、公開crate確認、整理までを完了条件とする。

- [x] 公開APIと後方互換シリアライズ
- [x] raster host / Storybook の座標計算
- [x] 回帰テスト・既存検査・差分レビュー

- [ ] 依存関係調査とrelease-check
- [ ] Draft PR・レビュー対応・Ready・required checks・merge
- [ ] 公開crate・Release確認・Issue更新・ローカル整理

## 利用方法

```rust
use katana_ui_core::render_model::{UiCommonProps, UiNode, UiNodeKind};
let card = UiNode::new(UiNodeKind::Card, "")
    .common(UiCommonProps::default().child_indent(0));
```

`None`（省略）は従来の16px、`Some(0)` は追加インデントなし。paddingは別途加算する。
`UiCommonProps`を全フィールド列挙して生成している場合、新フィールドまたは`..Default::default()`を追加する。
公開版採用後のconsumer検証: 560px本文の原点・幅、入れ子とpadding、折返しとscroll量、リンク/ボタンのhit座標、既存5画面の回帰を検査する。consumer側の基準は変更しない。

## 検証証跡

- 公開API/旧JSON/560pxヒット契約: `crates/katana-ui-core/tests/card_child_indent_contract.rs`
- core/Storybookの座標と残幅: `ui_tree_canvas_geometry.rs` / `ui_tree_canvas_hit_metrics.rs` の単体テスト
- リリース検査ログ: `/tmp/kuc-issue83-release-check-2.log`
- 依存調査ログ: `/tmp/kuc-issue83-outdated.log` / `/tmp/kuc-issue83-dependencies.log`

## 容量不足からの復旧

2026-10-19: 全体検査はディスク容量不足で停止（`/tmp/kuc-issue83-unit.log`、`/tmp/kuc-issue83-check.log`）。公開API回帰4件は通過済み。未使用 `kuc-coverage-target`（38GB）のみ削除済み。削除前後の証跡: `/tmp/kuc-issue83-storage-before.log` / `/tmp/kuc-issue83-storage-after.log`。仮想ディスクの空き領域を返却した結果、macOS空き容量は削除直前4.9GiBから45GiBへ回復した（`/tmp/kuc-issue83-storage-trim.log`）。全体検査を再実行し、coverageを再生成する。PR・公開は未完了。

通常品質ゲート（lint・4,635テスト・native IME・guard）通過。最終の公開API回帰4件も通過（`/tmp/kuc-issue83-contract-final.log`）。独立した最終差分レビューは新規指摘なし。coverageを再生成中。
