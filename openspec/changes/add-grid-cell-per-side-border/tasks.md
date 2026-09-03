## 1. 公開 grid border モデル

- [x] 1.1 `UiGridBorderLineStyle`、`UiGridBorderSide`、`UiGridCellBorders` を公開・serde 対応で追加する。
- [x] 1.2 `UiGridCellAppearance.borders` を default 互換で追加し、legacy serialized input と四辺別 metadata の回帰を追加する。

## 2. Raster host 描画

- [x] 2.1 `UiNodeKind::Grid` 用の raster renderer を追加し、cell bounds、viewport、scroll、fill/text の基本描画を実装する。
- [x] 2.2 各辺の color、line style、clip、merged anchor を尊重する border 描画を実装する。
- [x] 2.3 明示 border なしの grid と四辺の異なる border のピクセル回帰を追加する。

## 3. Downstream contract と品質

- [/] 3.1 KDV frame を公開 API へ投影する consumer regression を追加し、path/git override なしの registry artifact 採用条件を記録する。KDV 側の実装済みで、registry artifact 採用は KUC 公開後に確認する。
- [/] 3.2 `raster-host-contract`、全体 test/check/lint/strict coverage と consumer link を最新 HEAD で通す。KUC local strict coverage は通過済みで、registry consumer link は公開後に実行する。
- [/] 3.3 KUC Issue #37 と KDV Issue #48 の完了条件・公開境界を更新する。KUC 公開と KDV consumer acceptance の結果を反映待ち。
