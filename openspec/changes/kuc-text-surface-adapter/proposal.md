## 背景

KUC には generic な `TextArea` の state/action/event と platform text-raster runtime があるが、実際の
RGBA surface、IME output、selection/caret hit-test、line gutter、annotation、accessibility、egui input
adapter が一つの KUC contract になっていない。この不足を KLE が `platform_text_surface.rs`、
`line_gutter.rs`、個別 egui overlay として実装すると、KLE/KDV/他 consumer で同じ UI component が重複し、
emoji/日本語の表示、caret、input jitter、accessibility が再び diverge する。

## 変更内容

- existing KUC `TextArea`、text selection、platform text-raster runtime を compose する generic
  `TextSurface` contract を追加する。
- text span/style、selection/caret、IME preedit、find/diagnostic/active-line annotation、generic line
  gutter、scroll/viewport、clipboard/undo/redo request、accessibility tree、typed interaction event を
  renderer-neutral model にする。
- KUC-owned optional egui adapter を command chrome change と共有し、KUC text-surface frame record から
  RGBA texture / pointer / keyboard / IME / AccessKit を接続する。
- KLE/KDV が direct text-surface renderer、line gutter renderer、egui font atlas text measurement、OS font
  lookup、manual hit-test、fallback artifact を持たないよう source/dependency guard を追加する。

## Capability

### 新規 Capability

- `kuc-text-surface`: generic multiline text-surface state、selection、annotation、gutter、viewport、
  accessibility、clipboard/history request、typed event の renderer-neutral contract。
- `kuc-text-surface-egui-adapter`: platform text-raster を使う KUC-owned actual egui text-surface adapter、
  same-frame record、IME/keyboard/pointer/accessibility integration contract。

### 既存 Capability の変更

- なし。existing `TextArea` action/event と public DTO の source compatibility を維持するため、new
  `TextSurface` wrapper DTO/action/event を追加する。

## 影響範囲

- KUC `platform-text-raster-runtime` の public raster/layout API を prerequisite とするが、raster runtime
  自体に host/editor semantics を入れない。
- KUC `kuc-command-chrome-runtime` と one shared optional egui adapter crate を利用し、adapter の ownership
  を consumer repo に分散させない。
- KLE は generic surface renderer/gutter/annotation input logic を削除して editor-domain mapping に縮小する。
- KDV は text surface が必要な場所で同 adapter を consumer として利用できる。viewer/export 固有 Markdown
  composition や document content SVG raster は対象外である。
