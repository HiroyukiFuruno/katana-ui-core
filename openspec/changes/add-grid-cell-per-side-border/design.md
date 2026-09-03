## Context

`GenericGrid` は KDV の XLSX frame を `UiNodeKind::Grid` と `UiGridProps` に変換するが、`UiGridCellAppearance` は罫線情報を持たない。KDV は四辺ごとの style/color を既に public frame まで保持しているため、KUC が正規の公開モデルと raster host の描画を提供すれば、consumer 固有の描画実装なしに fidelity を改善できる。

既存 `UiBorder` は汎用 node の一様 border 用であり、Excel セルの四辺別属性や線種を表せない。追加 API は `UiGridCellAppearance` に限定し、既存の `UiBorder` 利用者へ意味変更を与えない。

## Goals / Non-Goals

**Goals:**

- 四辺別の有無、線種、色を serializable な公開 grid API として表現する。
- raster host が各セルの矩形・clip・scroll 座標を尊重して、辺単位で罫線を描画する。
- 指定がないセルは現行の grid 表示を保つ。
- KDV の frame から公開 API だけを使う consumer regression を追加する。

**Non-Goals:**

- Office style の全ての線種を pixel-perfect に LibreOffice と同一化すること。
- KDV worker、office2pdf、KatanA 固有の見た目補正を KUC に持ち込むこと。
- 既存 `UiBorder` の型・意味・描画契約を変更すること。

## Decisions

### Grid 専用の型を追加する

`UiGridBorderSide` と `UiGridCellBorders` を `UiGridCellAppearance` に追加する。`UiBorder` の拡張ではなく grid 専用にすることで、node border と Office cell border の責務を混同しない。

`UiGridBorderSide` は `UiGridBorderLineStyle`、optional color、デフォルトの未描画状態を持つ。KDV worker の style string は KDV thin projection で enum へ変換し、未知値は `Solid` 相当の可視線へ正規化する。これにより公開 API は型安全を保ちつつ、元の罫線情報を捨てない。

代替の raw string API は採用しない。consumer ごとの解釈差が生じ、Issue #48 の objective fidelity を測定不能にするためである。

### 後方互換な serialized field にする

`UiGridCellAppearance.borders` には `#[serde(default)]` を付ける。既存 serialized frame に field が無い場合は `UiGridCellBorders::default()` となり、追加罫線を描画しない。

### Raster host に grid 専用 renderer を置く

`UiNodeKind::Grid` は通常の container path ではなく grid renderer へ dispatch する。renderer は materialized `UiGridCell` の `clipped_bounds` を viewport と交差させ、fill/text を描画した後、各辺を clip 内に限定して描画する。色は `#RRGGBB` / `#RGB` を優先し、未指定または不正値では既存 grid line palette を使う。

thin/medium/thick は対応する pixel width、dashed/dotted/dash-dot 系は deterministic pattern、double は二本線として描く。未指定/none は描画しない。

### 重複辺の決定性

隣接セルが同じ座標に異なる辺を指定しても、各セルの属性を独立に rasterize する。同一ピクセルの最終色は後順の materialized cell となるが、KDV frame のセル順序は決定的である。merged span は anchor cell の `clipped_bounds` を使用し、内部線を追加しない。

## Risks / Trade-offs

- [公開 struct への field 追加は struct literal consumer をコンパイル不互換にし得る] → v0.x の additive API として release note に明記し、serialized input は default で互換にする。
- [未知の Office border style] → visibility を失わない `Solid` fallback とし、入力 style を KDV evidence に残す。
- [細い線が scroll/clip 境界で消える] → clip と edge intersection の回帰テストを持ち、physical raster scale の両方で検証する。
- [grid renderer が現行の raster host coverage を増やす] → public core に回帰テストを置き、strict coverage は下げない。

## Migration Plan

1. KUC `release/v0.3.4` に型・renderer・回帰を統合して公開する。
2. KDV が path/git override を使わず registry `katana-ui-core` artifact を採用する。
3. KDV frame を `UiGridCellAppearance.borders` へ thin projection し、fixture と consumer frame を再検証する。
4. 問題時は KDV 側の新 API 採用を戻せるが、公開 KUC の additive field は残す。

## Open Questions

- KatanA の最終 host 側 screenshot baseline を、KDV consumer frame と同じ fixture hash でどのように固定するかは、KUC artifact 公開後に KatanA consumer acceptance で決定する。
