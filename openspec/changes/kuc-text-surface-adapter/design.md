## Context

`katana-ui-core-text-raster` は platform font resolution、RGBA raster、grapheme bounds、hit-test を返す。
KUC `TextArea` は generic edit state/action/event を持つ。しかし actual surface の texture、pointer、
keyboard、IME、selection/caret、gutter、annotation、accessibility、egui adapter は存在しない。その不足を
KLE の `PlatformTextSurface` / `LineGutterModel` が埋めており、generic UI renderer が consumer に重複している。

KUC は既に `TextArea`、`UiTextSelectionModel`、`UiTextSpan`、`ContextMenu`、`DiagnosticsList`、
`HoverCard`、`ScrollArea`、placement、accessibility node を持つ。本 change はこれらを `TextSurface` と
KUC-owned egui adapter に compose する。KatanA/KLE/KDV の document、Markdown、diagnostic/search algorithm、
clipboard backend、history storage、host command は KUC に導入しない。

## 実装再監査による完了条件の補正

2026-07-14 の KLE migration 前再監査で、既存 adapter が input event と frame state を出せても、full editor
surface の完了根拠には不足することを確認した。`TextSurfaceState.scroll_x/scroll_y` は更新されるが paint/layout
origin に反映されず、`TextSurfaceFrameRecord.content_bounds` は text content の矩形であって consumer が必要とする
viewport/surface 矩形ではない。さらに gutter row は label/marker のみで active/hovered/diagnostic のような
generic visual state を運べない。この状態で KLE が ScrollArea、gutter renderer、座標補正を補うことは禁止する。

KLE の `PlatformTextSurface` / `LineGutterModel` を削除する前に、KUC は次を満たさなければならない。

1. adapter record は text `content_bounds` と allocated `surface_bounds` / clipped `viewport_bounds` を分離し、
   background、pointer hit target、AccessKit root、artifact が同じ surface bounds を参照する。
2. wheel/controlled scroll は one KUC scroll state を更新し、raster layout、gutter、annotation、selection、caret、
   hit test、IME cursor rect、AccessKit bounds、frame record を同一 offset で移動する。consumer は scroll geometry を
   再構成しない。
3. gutter row は caller-defined visual role と semantic paint を additive に受け、active/hovered/marker/diagnostic
   を KUC 固有 enum にせず同じ raster/layout/frame record で描画する。
4. empty/single-line surface は transparent base raster と platform-raster placeholder を使い、KUC TextSurface が
   input role、caret、IME、texture identity を一貫して出す。

この補正を通過するまで task 1.4、2.3、2.4 は partial であり、既存の event-only test は visual scrolling や
editor migration の完了証拠ではない。

## Goals / Non-Goals

**Goals:**

- visible text、caret、selection、preedit、hit-test、gutter、annotation、scroll viewport が一つの KUC
  platform-text-raster layout と同じ frame record を使用する。
- multiline editing、IME、Japanese、`⭐️`、ZWJ、pointer selection、keyboard movement、clipboard/history
  request、read-only、accessibility を generic typed action/event で扱う。
- line number / marker gutter と range annotation を host/domain-independent data にし、diagnostic/search/
  syntax/active-line の意味付けと document position は consumer に残す。
- shared `katana-ui-core-egui-adapter` crate が text surface と command chrome の actual egui rendering を
  所有する。
- KLE は KUC surface props を editor DTO から組み立て、KUC event を editor domain event/action に map
  するだけにする。

**Non-Goals:**

- KUC に Markdown syntax engine、search algorithm、diagnostic/lint engine、file IO、clipboard backend、
  undo history、KatanA/KLE/KDV type/enum を追加しない。
- KUC core crate に egui/cosmic-text/windowing dependency を導入しない。
- KDV viewer/export content SVG raster、KatanA document lifecycle、host global shortcut priority を実行しない。
- KLE の char-based public DTO を KUC type へ置換しない。boundary conversion は KUC-provided grapheme
  conversion query を使う thin binding とする。

## Decisions

### Existing `TextArea` を compose する additive `TextSurface`

`TextAreaAction` / `TextAreaEvent` は public enum のため、clipboard/history/annotation/gutter variant を直接
追加しない。new `TextSurface` が existing TextArea state/action/event、`UiTextSelectionModel`、
`UiTextSpan`、platform raster layout を compose する。public wrapper は `TextSurfaceProps`、state、action、
event、frame record を持つ。existing TextArea action は wrapper action に包み、copy/cut/paste/undo/redo は
new typed request event として consumer に戻す。KUC は clipboard/history を実行せず、read-only と selection
の generic enablement rule のみを決定する。

### One raster layout determines all text geometry

TextSurface props は text、span/style、font/theme token、wrap/viewport、read-only、selection、preedit、focus
request を受ける。visible pixels、selection/caret/preedit rect、pointer hit-test、grapheme range は platform
raster/layout が唯一の source of truth である。consumer は byte/char offset を KUC query で grapheme range
に変換し、KUC は range decoration を geometry に変換する。manual text measurement と scalar-only editing
を禁止する。

### 公開 API 形状と依存方向

`katana-ui-core` の new `text_surface` module は `TextArea` を内部 state machine として compose するが、
`katana-ui-core-text-raster` を依存しない。core の public contract は次の additive types に限定する。

| 型 | 所有者と責務 |
|---|---|
| `TextSurfaceProps` | stable surface id、`TextArea` configuration、styled span、viewport、focus/read-only、generic gutter/annotation/context presentation input |
| `TextSurfaceState` | wrapped `TextAreaState`、selection/preedit/focus、scroll and pointer drag anchor。clipboard/history/document state は持たない |
| `TextSurfaceAction` | wrapped `TextAreaAction`、pointer/scroll/focus/context target、clipboard/history request intent。host shortcut dispatch は受けない |
| `TextSurfaceEvent` | wrapped `TextAreaEvent`、generic gutter/marker/annotation/context activation、copy/cut/paste/undo/redo request。clipboard/history side effect は返さない |
| `TextSurfaceLayout` | one raster/layout result を表す raster identity、content bounds、grapheme box、line box。platform pixel buffer/renderer handle は core に置かない |
| `TextSurfaceFrameRecord` | one `TextSurfaceLayout` から導く text/selection/caret/preedit/annotation/gutter rect、hit target、scroll/focus/accessibility snapshot。adapter と artifact の共通 input |

`katana-ui-core-text-raster` は `PlatformTextRaster` を `TextSurfaceLayout` に変換する adapter-only
conversion を提供する。shared `katana-ui-core-egui-adapter` は platform raster の RGBA upload と actual
egui input/draw を担当し、同じ `TextSurfaceFrameRecord` から Storybook artifact と AccessKit mapping を作る。
KLE/KDV/KatanA はこの conversion/adapter を再実装せず、consumer domain DTO と KUC props/event の mapping
だけを持つ。

### Accessibility tree の公開形状

`TextSurfaceFrameRecord` は snapshot ではなく、adapter がそのまま actual accessibility bridge に投影できる
`TextSurfaceAccessibilityTree` を持つ。tree は次の renderer-neutral data に限定する。

- root input node は label、content bounds、focused、editable、read-only、disabled reason、selection の
  grapheme range を持つ。
- optional gutter row / marker node は logical row、opaque marker id、host-provided label を持つ。icon-bearing
  marker node は同 frame の KUC-derived `marker_bounds` を使い、row bounds と同一にしない。icon を持たない
  legacy marker は source compatibility のため whole-row bounds を維持する。role は KUC の generic
  `AccessibilityRole` を使い、editor-specific role を追加しない。
- context-selection target は host-provided context label、現在の grapheme selection、content bounds を持つ。
  menu item / host command / clipboard payload は保持しない。
- copy/cut/paste/undo/redo は semantic kind、consumer-provided localized label、enabled state の
  `TextSurfaceAccessibilityAction` として公開する。KUC は action label を英語で fallback せず、label が無い
  action は accessibility tree に追加しない。

selection、gutter、context target の range / bounds は全て当該 `TextSurfaceLayout` から生成する。adapter が
byte/char offset や line coordinate を再計算することは許可しない。

### フレーム整合性不変条件

- 一つの frame は一つの `TextSurfaceLayout` identity だけを持つ。caret、selection、preedit、annotation、
  gutter row、hit-test、accessibility range は必ずその identity の box を参照する。
- `TextSurfaceAction::Pointer*` は当該 frame の grapheme box で解決する。独自の text measurement や
  scalar/byte offset 推定を行わない。
- 外部 byte/char offset は、selection または annotation range になる前に `TextSurfaceLayout` の grapheme
  conversion query を通る。`⭐️`、ZWJ、combining mark、日本語の fixture を必須とする。
- overlap order は selection > preedit > annotation（priority 降順、caller index 昇順）> base text とする。
  同じ順を record と draw で使用し、renderer が再計算しない。
- `TextSurfaceFrameRecord` は host command 名、Markdown token、search/diagnostic kind、clipboard payload、
  history entry、filesystem/document reference を持たない。

### Gutter and annotation remain generic visual data

`TextSurfaceGutter` は zero-based logical row id、host-provided display label、active/hovered、generic marker
id、accessibility label を持つ。`TextSurfaceAnnotation` は generic grapheme range、visual role、semantic color
token、underline/outline/fill、priority、tooltip を持つ。KUC は diagnostic/search/syntax/editor line の enum
を持たず、`GutterRowActivated` / `GutterMarkerActivated` / `AnnotationActivated` を返す。overlap precedence は
selection > preedit > explicit annotation > base text、同 priority は caller index に固定する。

### Range-anchored automatic gutter marker

automatic numbered gutter の既発行 `TextSurfaceGutterRowId` override は source-compatible に維持する。ただし
consumer が source 更新のたびに row を列挙したり、row id を再解決したりすることは許可しない。追加の
`TextSurfaceAutomaticGutterRangeOverride` は UTF-8 byte range、開始 anchor policy、opaque marker id、priority、
visual/accessibility data、および optional `UiIconProps` を受ける。`UiIconProps` は generic host-provided SVG
data であり、KUC に host/source 固有の icon 名、diagnostic/search/Markdown enum を追加しない。

range override の row は current `TextSurfaceLayout` の grapheme/line data から KUC が毎 frame 解決する。無効な
UTF-8 boundary または current text 外の range は marker を発行しない。開始 anchor policy は newline boundary
で開始位置を含む行を採るか、次行を採るかを明示する。複数候補は priority 降順、同 priority は presentation
input order 昇順で一意に選ぶ。既発行 row-id override は range marker が無い行で従来どおり適用する。

KUC frame は resolved marker と同一 `TextSurfaceGutterFrame` に row bounds と distinct な `marker_bounds` を
生成する。adapter はその marker bounds だけを pointer hit target、AccessKit node、SVG raster cache key、paint
operation、artifact に使う。icon size/position/clipping/tint は adapter-only の SVG raster runtime が決定し、
consumer は座標、SVG parser、raster cache、fallback pixel を持たない。

### Controlled consumer synchronization preserves KUC interaction state

KLE/KDV のような controlled consumer は、source value、selection、read-only policy、span、annotation、
search presentation を frame ごとに更新できる必要がある。一方で consumer が `TextSurface`、toolbar、or search
strip を再構築すると、KUC が所有する focus、drag、scroll、IME/preedit、dropdown、tooltip、input identity、
raster/texture cache を失う。この change は construction builder だけでなく、次の additive controlled
presentation synchronization contract を提供する。

1. `TextSurface` は source value、UTF-8 byte selection/caret、input/read-only policy、span、annotation、
   generic gutter/accessibility presentation を event を発生させずに一方向同期できる。同期は KUC-owned
   focus、drag、scroll、IME/preedit、layout、texture state を再初期化しない。
2. KUC は automatic numbered-gutter mode を持つ。logical row の列挙、display label、row bounds、scroll
   alignment、raster、pointer/AccessKit target は KUC が text layout から作る。consumer は sparse な opaque
   row id に対する marker、visual role、accessibility override だけを与え、row model や座標を作らない。
3. `CommandChromeToolbar`、`FloatingCommandToolbar`、`CommandChromeSearchStrip` は opaque control data、
   localized presentation、enabled/selected/visible state、query/replace/options/result/capability を event
   なしで同期できる。同期後も KUC-owned focus、dropdown、tooltip、input state id を保持する。
4. floating toolbar は consumer が selection-anchor と viewport の fact を渡すが、KUC adapter が actual
   panel size を measure して final placement を決める。consumer は panel dimensions、coordinate arithmetic、
   clamp を持たない。anchor/viewport は current `TextSurfaceFrameRecord` から copy する data であり、
   consumer measurement ではない。

これらの API は KatanA/KLE/KDV、Markdown、document/search provider、diagnostic kind を参照しない。同期 API が
core `apply_action` を呼んだように event を合成すること、consumer が private action を使って state を偽装する
こと、KUC adapter 以外で generic text/form/popup geometry を所有することを禁止する。

- controlled sync は `TextSurfaceViewportSizing` を含む interaction ownership を保持する。`Fixed` / `AdapterMeasured`
  のどちらかはコンストラクション時のみ決まり、`synchronize_*` 系 API や source 更新で勝手に切り替わってはならない。
  consumer は viewport 測定・panel 計算・fallback 幾何計算を行わず、測定と配置は adapter 側の helper で統一する。

Real-egui acceptance は、external value/selection update during IME、Japanese/`⭐️`/ZWJ、automatic gutter
multiline edit and scroll、controlled search/replace focus preservation、floating resize/scroll/outside/Escape/
focus-return を actual `RawInput` と AccessKit/frame-record/pixel/artifact hash で検証する。

### Controlled scroll request and acknowledgement

Consumer が generic text-surface の scroll position を操作する場合、KUC は opaque request token を持つ
`TextSurfaceScrollRequest` を controlled presentation として受ける。target は `LogicalRow`、UTF-8
`ByteOffset`、UTF-8 `ByteRange`、`RelativePixels`、alignment は `Nearest` / `Start` / `Center` / `End`
に限る。consumer は text/line domain をこの typed input に変換するだけであり、line-to-pixel、range-to-
rect、visible range、scroll clamp、scroll origin は計算しない。

KUC adapter は current `TextSurfaceLayout` と synchronized viewport/scroll bounds が揃う frame で request を
解決する。request token は idempotency key であり、同じ token の synchronization は scroll を再適用せず、
user wheel 後の scroll position、focus、drag、IME/preedit、raster/texture state を巻き戻さない。新 token は
前 request を置き換え、同一 frame の typed `ScrollRequestAcknowledged` または typed rejection、frame record、
artifact に反映する。KUC core/adapter は KLE/KatanA の document、Markdown、editor action を知らない。

### Controlled focus request and acknowledgement

Consumer が generic TextSurface の focus を host lifecycle に合わせて要求する場合、KUC は opaque request
token と requested state を持つ `TextSurfaceFocusRequest` を controlled presentation として受ける。KUC
adapter は actual surface response が allocation された後にのみ request を一度発行する。request token は
idempotency key であり、同じ token の同期は user pointer、Escape、outside-click による後続 focus state を
再適用又は巻き戻さない。new token は previous request を置き換える。

KUC adapter は request issued を typed `FocusRequestAcknowledged` と frame record/artifact event に投影する。
これは eframe focus memory への request が受理されたことを記録する event であり、その frame の actual
focused state を偽装しない。actual focus state は KUC の next frame synchronization と existing typed
`FocusChanged` event で表す。focus request が TextSurface value、selection/caret、scroll、IME/preedit、raster/
texture state を再生成することはない。KLE/KatanA の document、command、shortcut semantics は KUC に持ち込まない。

### Scroll precision and visible-row facts

`TextSurfaceScrollRequest::RelativePixels` は logical pixel float を transport value として受け、KUC が adapter
scale/rounding policy を適用して internal scroll offset に正規化する。consumer は f32 value を i32 へ丸めず、
scroll clamp 又は residual を保持しない。acknowledgement は KUC が適用した integer offset を返す。

KUC `TextSurfaceFrameRecord` は current layout と viewport に基づく `visible_logical_rows` を持つ。これは row
bounds と viewport intersection を KUC が解決した immutable fact であり、consumer は text newline の列挙や
pixel to line conversion で同じ値を再計算してはならない。空 content の range、partial visible line、scroll
offset、gutter の有無を core/actual adapter tests で固定する。

### Viewport sizing policy

`TextSurfaceProps` は viewport sizing policy を持ち、既定は `Fixed` とする。

`Fixed` は caller-specified viewport を preserve する legacy 動作を維持し、surface 幅を
`min(ui.available_width, configured_width)`、高さを `configured_height` で確定して使用する。
このモードでは `synchronize_measured_viewport_size` は呼ばれず、consumer の幾何計算に依存しない。

`AdapterMeasured` は adapter が `ui.available_width/height` を測定し、`synchronize_measured_viewport_size` を
`TextSurface` の現在値として明示的に更新する。measurement は adapter-owned（`surface_extent_for_ui`）として
行い、consumer 由来 geometry を再利用しない。mode は構築時に `adapter_measured_viewport()` を通じて
明示し、以後の state 変更で勝手に切り替わらない。

### One shared KUC egui adapter owns actual rendering

`katana-ui-core-egui-adapter` を new optional workspace crate とし、`katana-ui-core`、text-raster、future
SVG runtime、egui だけに依存させる。adapter instance が rasterizer、texture cache、focus、drag anchor、IME
output、accessibility bridge を持つ。KLE/KDV は use するだけで再実装しない。adapter は first-class
`TextSurfaceFrameRecord` を先に生成し、RGBA texture upload、draw、pointer/keyboard/IME、AccessKit、
Storybook artifact を同 record から処理する。

text surface path で `egui::TextEdit`、egui font registration/measurement、OS font lookup、manual glyph
raster、Unicode/emoji icon fallback は禁止する。egui は input collection、focus、IME output、texture upload、
actual draw の boundary に限定する。

### Shared egui adapter の concrete contract

shared crate `katana-ui-core-egui-adapter` は workspace member とし、`katana-ui-core`、
`katana-ui-core-text-raster`、`katana-ui-core-svg-raster`、`egui` だけに依存する。KUC core、
text-raster、SVG raster、KLE/KDV/KatanA host の依存方向を逆転させない。adapter crate の public API は
KUC DTO と egui boundary type だけを受け、host callback、document、Markdown、clipboard backend、undo store、
font path、command name を受けない。

text-surface module は次の責務に分ける。

| 型 / module | 責務 |
| --- | --- |
| `EguiTextSurfaceAdapter` | platform rasterizer、bounded texture cache、egui focus / drag / IME bridge の instance state。KLE/KDV は instance state を再実装しない。 |
| `TextSurfaceRasterStyle` | host 注入済み font token、semantic color、line height、wrap / scale input。surface text / selection / preedit は style に持たず `TextSurface` state から一度だけ組み立てる。 |
| `TextSurfacePaintStyle` | background、selection、caret、annotation visual-role color を consumer-neutral token / RGBA で受ける。KUC は diagnostic / search / Markdown color enum を持たない。 |
| `EguiTextSurfaceFrameRecord` | `TextSurfaceFrameRecord`、raster identity、RGBA texture bounds、draw layer order、egui hit target id を同一 frame の immutable output として保存する。texture handle や host callback は serializable record に含めない。 |
| `EguiTextSurfaceOutput` | actual egui input から `TextSurface` に適用した typed action/event と frame record を返す。adapter は consumer callback や clipboard/history side effect を実行しない。 |

adapter は `TextSurfaceProps.spans` と wrapped `TextAreaState` から platform text-raster request を生成する
shared helper を使用する。IME preedit は source selection を置換した composed text を一度だけ rasterize し、
`text_surface_layout_with_composition` を使用する。host / KLE が preedit 用の別 font measurement、別 texture、
byte-to-pixel conversion を持つことは許可しない。

actual egui event mapping は次で固定する。

- focus / blur、pointer click / drag、wheel scroll、context-menu request、keyboard caret / selection movement、
  select-all、delete、text input、IME preedit / commit / cancel、copy / cut / paste、undo / redo を
  `TextSurfaceAction` と `TextSurfaceEvent` にのみ変換する。
- pointer / drag と gutter hit は current `TextSurfaceFrameRecord` の bounds と `TextSurfaceLayout` query だけで
  解決する。adapter は text measurement、line reconstruction、scalar offset 推定を行わない。
- `Copy` / `Cut` / `Paste` / `Undo` / `Redo` は KUC request event を出すだけであり、egui clipboard、host
  clipboard、history store を adapter から呼ばない。read-only / disabled / selection enablement は
  `TextSurface` core と accessibility tree の rule を共有する。
- context target は current frame の grapheme selection / bounds を event と accessibility tree の両方へ出す。
  menu content / command dispatch は existing KUC `ContextMenu` と consumer binding の責務である。
- adapter は `TextSurfaceAccessibilityTree` を actual AccessKit node role / label / selection / disabled state /
  gutter / context target に投影する。consumer state assertion だけでは完了証跡にしない。

draw order は background、gutter、selection、preedit、priority-sorted annotation、platform RGBA texture、caret
に固定する。draw、hit-test、accessibility、Storybook artifact は一つの `EguiTextSurfaceFrameRecord` を参照し、
adapter と Storybook が別の layout / fallback renderer を生成してはならない。

### Accessibility and artifacts are real contracts

TextSurface は focus、editable/read-only、selection、gutter/annotation/context targets、disabled reason を
renderer-neutral accessibility tree として出す。adapter は実 AccessKit node に map し、テストは actual node
role/label/state を query する。Storybook、GIF/manifest、jitter check、acceptance artifact は frame record
だけを使い、fallback canvas、manual measure、shape count を根拠にしない。

### Storybook TextSurface migration is an adapter runtime, not a Canvas preview

現在の `katana-ui-core-storybook` は `minifb` と独自 `Canvas` による広域カタログを持つ。この経路は既存
component inventory の表示には残してよいが、`text-area` の shared TextSurface acceptance evidence にしては
ならない。Canvas 側へ line/gutter/IME/selection を模倣実装することは、KUC adapter と同じ責務の renderer を
二重化し、KLE/KDV で避けるべき fallback と同じ問題を再導入するため禁止する。

TextSurface Storybook は次の二経路を明確に分ける。

| 経路 | 所有者 | 用途 | acceptance evidence |
| --- | --- | --- | --- |
| existing `minifb` catalog | Storybook visual catalog | 全 component の一覧、既存 page navigation | TextSurface の renderer/input 正しさには使わない |
| `eframe` TextSurface runtime | Storybook + `katana-ui-core-egui-adapter` | `text-area` page の interactive window、actual egui input/IME/focus | `EguiTextSurfaceAdapter::show` が返す frame record と adapter-owned artifact のみ |
| headless scripted runtime | Storybook + `egui::Context` | deterministic event sequence、motion artifact、numeric jitter | interactive runtime と同じ `show` path、同じ surface/adapter instance model |

`eframe` は Storybook crate にだけ追加し、KUC core/text-raster crate には追加しない。`--open-window text-area`
は `eframe` TextSurface runtime へ dispatch し、その他の catalog page は既存 `minifb` runtime を維持する。これに
より visible Storybook と scripted acceptance がともに shared adapter の actual `show(ui, ...)` path を通る。

#### Shared paint-plan and artifact boundary

artifact のために Storybook が canvas layout、glyph raster、scroll origin、selection/caret/IME/gutter/annotation
geometryを再計算することは許可しない。`katana-ui-core-egui-adapter::text_surface` は one
`EguiTextSurfaceFrameRecord` と platform raster から one `TextSurfacePaintPlan` を作る。egui painter と artifact
encoder はこの immutable plan の consumer であり、どちらも text layout を生成しない。artifact encoder は
adapter crate に置き、Storybook は `TextSurfaceArtifactFrame`（record hash、paint-plan hash、RGBA pixel hash、
surface/viewport/content bounds、typed events）を受け取り PNG/GIF/manifest を保存するだけにする。

`TextSurfaceArtifactFrame` は少なくとも script step id、monotonic frame index、`EguiTextSurfaceFrameRecord` の
stable hash、surface/viewport/content bounds、scroll offset、texture/placeholder identity、source RGBA hash、
layer-plan hash、artifact RGBA hash、non-transparent pixel count、typed `TextSurfaceEvent`、selection/caret/preedit/
gutter/annotation/accessibility summary を持つ。

artifact encoder は `TextSurfacePaintPlan` の layer order（background、gutter、selection、preedit、annotation、
platform texture、caret）をそのまま用いる。別の `Canvas` renderer、manual glyph table、egui shape count、
string parse による action synthesis を生成・参照した場合は AST guard と contract test が失敗する。

#### Script and motion contract

TextSurface Storybook は fixture と script を KUC generic data だけで持つ。initial fixture は multiline Japanese、
`⭐️`、gutter visual role、generic annotation を含める。release candidate script は minimum として focus、wheel
scroll、visible-line pointer press/release、IME preedit、`⭐️` commit、copy/history request、context target を
actual egui input event として通す。script runner は core action を直接呼んで state を作ってはならない。

各 step は one actual egui frame を生成し、manifest は step input、typed events、record hash、paint-plan hash、
pixel hash、surface/viewport/content bounds、scroll offset を一対一に保存する。numeric gate は同一 input の
manifest/GIF/PNG hash が決定的であること、wheel 後に surface/viewport bounds が不変で one scroll offset を
共有すること、`⭐️` が variation selector を含む一 grapheme として color output/hit-test/caret を持つこと、
artifact hash が adapter output と一致して fallback source が無いことを検査する。

この contract は Storybook screenshot を正しさの唯一の根拠にしない。screenshots/GIF はユーザーが判断するための
補助 deliverable であり、actual input、frame-record、pixel、AST、AccessKit contract と常に同時に通る。

### CommandChrome Storybook は実 command surface の完全な証跡にする

Katana の editor controls は、inline format（bold/italic/strikethrough/inline code）、heading 1--3、
bullet/numbered/quote、code-block kind dropdown、image action、selection anchored floating toolbar、
editor context menu、search/replace controls に分かれる。KLE v0.1.0 はこの可視・入力上の capability を
欠落させてはならない。ただし Markdown AST/文字列変換、`CodeBlockKind` catalog、image ingest、document
search algorithm、file lifecycle は KUC の責務ではない。

KUC の `CommandChromeToolbar`、`FloatingCommandToolbar`、`CommandChromeSearchStrip` と shared
`EguiCommandChromeAdapter` は、consumer が注入する opaque action id、localized label、accessibility label、
icon props、disabled/selected state、dropdown item、search capability だけを扱う。KUC は `Markdown`、
KatanA/KLE/KDV の command enum、search provider、clipboard/history backend、host global shortcut priority を
参照しない。KLE は editor-domain command inventory と current selection/search state からこの generic model を
組み、次の typed event を domain action に map するだけにする。

| KUC typed event | KLE の薄い binding | KUC が持たないもの |
| --- | --- | --- |
| `CommandActivated` | authoring/image 等の host command request | Markdown source transform、image ingest |
| `DropdownItemActivated` | code-block kind 又は consumer command request | code-block language catalog の意味 |
| `FocusChanged` / `FocusReturnRequested` | TextSurface focus restore request | editor cursor/selection mutation |
| `CommandChromeSearchEvent` | host search/replace/navigation request | search algorithm、document mutation |
| `TextSurfaceEvent` from search inputs | query/replace DTO update | OS clipboard/history side effect |

KLE の authoring menu、toolbar、search/replace shell、context-menu item composition は KUC component を compose
する consumer binding に留める。KLE に icon glyph、font measurement、dropdown geometry、tooltip paint、search
input renderer、frame artifact renderer、widget-local focus state machine を置かない。KatanA source は parity
inventory と end-to-end expected transform の参照専用であり、この change で編集しない。

#### One adapter-owned paint plan per command surface

現在の `EguiCommandChromeAdapter` は actual egui input と platform text/SVG raster を所有している。これを
Storybook evidence と同一にするため、toolbar、floating toolbar、search strip の各 `show_*` call は actual
frame record と immutable `CommandChromePaintPlan` を一緒に返す。plan は adapter 内で final bounds と raster
texture を一度だけ確定し、actual egui painter と artifact encoder が同じ operations を消費する。

- plan operation は clip bounds、fill 又は platform RGBA texture、layer order を持つ。layer order は panel /
  action fill / icon texture / label texture / focus ring / dropdown / tooltip とし、search input は既存
  `TextSurfaceArtifactFrame` の immutable plan をそのまま compose する。
- `show_toolbar`、`show_floating_toolbar`、`show_search_strip` は direct `painter` geometry/raster を
  Storybook に漏らさない。実 draw は plan consumer に限定し、artifact 用の別 layout、glyph raster、icon
  fallback、shape count は生成しない。
- one `CommandChromeArtifactFrame` は component records、component paint plans、record/plan hashes、typed
  toolbar/floating/search/text events、AccessKit summary を持つ。複数 component を表示する frame は adapter
  provided composition helper が plan を順序どおり連結するだけで、Storybook は bounds を再計算しない。
- raw RGBA plan encoder は TextSurface と共通の KUC Storybook infrastructure に寄せる。ただし command
  semantics と TextSurface layer enum は混ぜず、共通化は pixel blend/PNG/GIF I/O に限定する。

この境界により、KLE/KDV が command chrome の renderer を再実装せず、TextSurface と command chrome が同じ
platform raster/texture cache/egui boundary を使う。`⭐️` を含む日本語 query/replace/tooltip/label は OS font
または egui glyph に委譲せず、adapter の platform text-raster texture だけで描画する。

#### Katana parity fixture と actual-input script

KUC Storybook は domain-neutral component library であるため、fixture の action ids/text は Storybook sample
data として注入する。fixture は Katana の control inventory を以下の generic command data として表す。

| fixture group | required visual and interaction coverage |
| --- | --- |
| inline group | 4 actions、selection-dependent disabled state、hover tooltip、keyboard focus/activation |
| structural group | heading 1--3、bullet/numbered/quote、group separators を含む stable order |
| code block | split-secondary dropdown、17 injected item ids/labels、pointer open、Arrow/Home/End/Enter/Space、outside/Escape dismiss、selected/disabled/focus state |
| auxiliary action | icon + localized accessible label を持つ image-like injected action |
| floating toolbar | selection anchor から actual bounds を受け、hover tooltip、command activation、outside click/Escape close と editor-focus-return event |
| search / replace | query and replacement `TextSurface`、case/word/regex、previous/next、replace visibility、replace one/all、close、disabled capability、Japanese/`⭐️` text input |

script runner は前 frame record の action/control/dropdown bounds を click target として使用し、座標を手計算しない。
minimum sequence は idle、hover、focus、disabled press、toolbar command activation、split dropdown open、dropdown
keyboard navigation/item activation、outside/Escape dismissal、floating open/tooltip/close、query focus、Japanese/
`⭐️` input、replace visible/input、navigation、replace one/all、capability-disabled control、close とする。各 step
は actual `egui::RawInput` で one frame を走らせ、core model の `apply_action` を直接呼んで state を作っては
ならない。

#### Floating toolbar visibility is consumer input, not a Storybook shortcut

`FloatingCommandToolbar` は selection/context/host policy を認識しない。したがって visible
initial state は generic consumer-provided component state として additive に指定できなければならない。
Storybook が `FloatingCommandToolbarAction::Open` を直接呼び、pointer/selection without a consumer
state transition を偽装することは許可しない。

KUC core は `FloatingCommandToolbar` construction 時に layout と visibility を受け取る additive
builder/state API を提供する。さらに controlled presentation synchronization は visibility/anchor/viewport を
更新しても KUC-owned dropdown/focus/tooltip state を維持する。visible state は KUC placement engine が actual
panel measurement と placement/bounds を adapter 内で解決するが、`Opened` event を合成しない。consumer は
actual `TextSurface` selection event や host-provided selection state からこの input を更新する。KLE はその
consumer mapping を持つが、KUC は selection の editor semantics、Markdown、cursor model を知らない。

Storybook fixture は visible consumer state を injected generic data として作り、actual adapter path で
tooltip、pointer activation、outside/Escape dismissal、focus-return event を検証する。open gesture 自体は
KLE/KatanA integration で actual text selection to consumer-state mapping として検証する。これにより、
KUC Storybook は core `apply_action` direct script、KLE-only selection emulator、manual floating bounds を
持たない。

#### Stable control identity is a consumer-provided artifact contract

`CommandChromeSearchStrip` の query/replace `TextSurface` identity は underlying
`SearchControlStrip` state id から構成される。auto-generated state id を artifact frame record/paint plan
へ混入させると、同一 fixture の別 process run で PNG/GIF/manifest hash が変わる。これは artifact
comparison を緩める理由にならない。

`SearchControlStrip` は additive `stable_state_id` builder を提供し、consumer が lifecycle-stable
opaque id を注入できるようにする。KUC はその id を Markdown/document/search-provider semantics と
解釈しない。Storybook は fixed generic sample id を、KLE は document/host lifecycle から受けた stable
id を渡す。query/replace slot id はその stable parent id から KUC adapter が一意に導出する。

deterministic Storybook gate は independently constructed fixture でも component record hash、paint-plan
hash、RGBA pixel hash、manifest/GIF hash が一致することを検証する。auto idの run-local counter や
pointer position を hash comparison から除外してはならない。

#### Strict acceptance and guardrail contract

CommandChrome Storybook artifact は `command-chrome-motion.gif`、numbered PNG、manifest を出力する。manifest は
script step、raw event、typed event、component frame record hash、paint plan hash、RGBA pixel hash、action/
dropdown/search/floating bounds、focused target、dropdown open state、AccessKit summary、Japanese/`⭐️` variation
selector and color-texture evidence を保存する。re-run は file hashes が一致し、artifact encoder が adapter
output の immutable plan 以外を入力に取らないことを test で固定する。

必須の自動 gate は以下とする。

- actual-egui pointer/keyboard/IME interaction と typed event/AccessKit node の contract test。
- action/dropdown/search/floating frame-record hash、pixel hash、stable-bounds、disabled/no-event、focus-return、
  dropdown close reason、`⭐️` variation selector/color texture の numeric test。
- actual painter が artifact と同じ immutable plan を消費する adapter test。Storybook screenshot/GIF 単独を
  正しさの根拠にしない。
- AST/KUC guard で Storybook Canvas、manual geometry/glyph/icon renderer、direct core action script、native
  text painter/font lookup、emoji replacement/fallback、KUC core への Markdown/host dependency を拒否する。
- full Storybook regression、adapter integration tests、`kuc_guardrails`、AST lint、OpenSpec strict を task
  completionの前提とする。

### Automatic gutter state completion (2026-08-13)

The existing automatic gutter contract owns row enumeration, labels, bounds,
markers, rasterization and hit targets, but its current frame record does not
expose the active or hover facts required by a generic editor consumer. A KLE
`LineGutterModel` must not reconstruct those facts from source text, cursor
offsets, diagnostics or egui pointer state. The missing behaviour is therefore
a KUC prerequisite, not a consumer workaround.

`TextSurfaceAutomaticGutterPresentation` gains only one controlled generic
input: a deduplicated collection of logical rows currently requested as
hovered. It contains no label, display number, bounds, colour, marker geometry
or KLE/KatanA diagnostic meaning. Invalid logical rows are ignored by KUC when
the current layout is resolved; they do not create rows or alter the surface.

KUC derives the active gutter row from the current KUC `TextArea` caret and
layout on every frame. It derives `hovered` from the controlled logical-row
input after the same layout resolution. `TextSurfaceGutterFrame` publishes
`active` and `hovered` alongside its existing KUC-issued row id, display label,
marker and bounds. Its accessibility targets and adapter paint plan consume the
same resolved state. The generic paint style receives additive active/hover
gutter treatments; it must not add editor, diagnostic, search or Markdown
enums. A marker remains the generic indication of a host-provided range
override; its semantic interpretation remains in the consumer.

The acceptance proof is a single real-egui `RawInput` sequence containing
multiline Japanese, `⭐️` with VS16, an automatic gutter, a controlled hover
update, caret movement, marker hit, scroll and source replacement. The core,
actual AccessKit tree, adapter paint plan and artifact record must agree on the
same row ids, bounds, active/hover values and marker target. The test rejects
manual row enumeration, local colour/geometry reconstruction and an egui text
fallback. KLE then may map only the returned frame facts to its neutral gutter
DTO; it may not retain `LineGutterModel` or enumerate lines.

### Public artifact compositor is a KUC adapter service

KUC Storybook currently contains separate private RGBA blend/composition paths
for `TextSurfacePaintPlan` and `CommandChromePaintPlan`. That is a generic
adapter responsibility, not a Storybook implementation detail and not a
consumer extension point. Keeping those paths private would force KLE/KDV to
either duplicate pixel composition or depend on a Storybook crate; both violate
the ownership boundary.

`katana-ui-core-egui-adapter::artifact_compositor` therefore provides the only
public deterministic plan compositor. Its additive public request is composed
of:

| Public value | Responsibility |
| --- | --- |
| `ArtifactCanvasBounds` | The root bounds allocated by the actual eframe/egui frame. The caller supplies this fact unchanged; it must not union component bounds or calculate pixel dimensions. |
| `ArtifactPaintPlanRef` | Borrowed `TextSurfacePaintPlan` or `CommandChromePaintPlan`; the enum carries no editor, Markdown, KLE, KDV or KatanA meaning. |
| `ArtifactCompositeRequest` | One explicit canvas and ordered plan references. Caller order represents actual paint order only. |
| `ArtifactCompositeFrame` | Canvas bounds, immutable plan hash, RGBA pixels, pixel hash and non-transparent-pixel count. PNG/GIF/manifest I/O remains a Storybook concern. |

The compositor owns plan clipping, RGBA validation, nearest-pixel texture
sampling, source-over alpha blending, canvas-relative indexing, overflow
rejection and deterministic hashes. It never derives text layout, glyphs,
cursor/gutter geometry, popup placement, action semantics or a fallback image.
Malformed texture byte lengths, zero/invalid canvas bounds and arithmetic
overflow are typed errors; they are never silently replaced, skipped or padded.
Operations outside the supplied actual canvas are clipped by the compositor.

Both existing KUC Storybook artifact paths must call this public API and their
private blend/composite implementations must be removed in the same migration.
KLE/KDV may pass only the plans emitted from their public KUC adapter `show`
calls and their actual root bounds. They may serialize the returned RGBA bytes
to user-facing PNG/GIF/manifest, but may not add a canvas renderer, plan
conversion, geometry union, texture sampler, alpha blender, glyph fallback or
shape-count acceptance path.

The proof consists of unit tests for clipping, alpha composition, malformed
texture rejection and deterministic hashes, plus actual-egui `RawInput` tests
that compose a Japanese/`⭐️` VS16 TextSurface with toolbar, selection-anchored
floating toolbar and search/replace strip. The actual frame records, plan hashes,
colored-star texture bytes, output bounds and RGBA hash must agree across a
repeat run. AST guards reject the removed private compositors and any new
Storybook/consumer manual blend, texture sampling, plan conversion or fallback
renderer.

### Actual ContextMenu adapter closes the editor-command surface gap

KUC already owns the generic core `ContextMenu` model, but the shared egui
adapter has no actual ContextMenu input, placement, AccessKit, paint-plan or
artifact path. KatanA's editor context menu exposes save/format host requests,
the authoring operations that are not shown in the floating toolbar
(horizontal rule, link, table), code-block kinds, and file/clipboard image
requests. A KLE-local right-click menu would repeat generic menu geometry,
focus and raster behavior, so it is forbidden.

KUC adds an additive `context_menu` adapter module. A controlled,
consumer-provided presentation supplies only opaque item ids/tree order,
localized labels/accessibility labels, optional KUC `UiIconProps`, enabled and
checked state, and the host visibility policy. The core/adapter has no
Markdown, KLE, KDV, KatanA, document path, clipboard implementation, authoring
enum or file-operation semantics. It owns retained focus/type-ahead/submenu
state, actual panel self-measurement, viewport clamp, pointer and keyboard
navigation, outside/Escape dismissal, focus return, AccessKit nodes and a
`ContextMenuPaintPlan`/artifact record.

`EguiTextSurfaceAdapter` enriches its existing context-target outcome with an
adapter-owned immutable context anchor fact. A secondary-click uses the actual
pointer position; keyboard/AccessKit invocation uses the current KUC-resolved
selection/caret bounds. The fact includes only KUC selection and KUC-resolved
anchor/viewport data. A consumer may request the menu visible after receiving
this event but cannot calculate, adjust or replace its coordinate. The retained
KUC ContextMenu binding receives that fact directly and never accepts a
consumer pixel DTO.

The context-menu artifact plan becomes an additive `ArtifactPaintPlanRef`
variant after the public artifact compositor prerequisite is in place. It
shares KUC-owned clipping, texture validation and alpha composition; no
Storybook or editor joins component bounds or invents a menu compositor. The
actual-input proof requires right-click and keyboard/AccessKit opening,
Japanese/`⭐️` VS16 labels, nested code-kind selection, disabled item no-event,
outside/Escape close, focus return, pointer-clamped placement, frame/plan/pixel
hash repeatability and colored-texture evidence. KUC guardrails reject
Storybook Canvas/menu geometry and any consumer `egui::Area`/menu/button
implementation on this path.

### Shared text-command surface composition owns root-space allocation

`EguiTextSurfaceAdapter` and `EguiCommandChromeAdapter` are individually
correct components, but a consumer that calls them sequentially in one root
`egui::Ui` can accidentally let the text surface claim all available height.
The search strip is then allocated outside the usable root area. A KLE-local
height reservation, rectangle subtraction, child `Ui` creation, overlay
placement, or fallback search input would repeat generic layout ownership and
is forbidden.

KUC therefore adds a generic retained `EguiTextCommandSurfaceAdapter`. It
accepts only KUC TextSurface and CommandChrome controlled presentations plus
an actual root `egui::Ui`; it has no KLE/KDV/KatanA/Markdown/document/search
provider semantics. It allocates one actual root surface, reserves the
controlled search-strip and toolbar slots before measuring the text viewport,
and invokes the retained TextSurface, floating toolbar and search-strip
adapters in the defined KUC paint order. The floating toolbar consumes only
the current KUC TextSurface frame fact. Consumers receive one immutable output
containing the child typed events, focus/scroll facts, actual root bounds and
ordered child artifact records; they do not receive editable child rectangles
or a callback that can draw into a manually calculated UI.

#### Consumer-safe retained state correction (2026-08-13)

The first internal shape of the adapter accepted separate mutable
`TextSurface`, `CommandChromeToolbar`, `FloatingCommandToolbar`, and
`CommandChromeSearchStrip` arguments. That is not a usable consumer boundary:
`FloatingCommandToolbar` already owns its toolbar, so one retained command
model cannot safely satisfy both mutable arguments, and constructing the first
floating toolbar requires anchor and viewport coordinates before a TextSurface
frame exists. Asking KLE to duplicate the toolbar or fabricate those facts
would violate the ownership boundary this change establishes.

KUC must instead expose one public retained generic model, named
`EguiTextCommandSurface` unless an existing KUC naming convention provides a
clearer equivalent. It owns the controlled `TextSurface`, optional root
`CommandChromeToolbar`, optional `FloatingCommandToolbar`, and optional
`CommandChromeSearchStrip` together with their stable KUC identities. A
consumer may synchronize generic, opaque presentations and call exactly one
root `show(ui, &mut model, style)` path, but may not obtain or pass separate
mutable child models to that path. Optional children reserve no root space when
absent; this permits a selection-only Markdown toolbar without inventing an
empty persistent toolbar or permanently visible search strip.

The model exposes KUC-owned controlled synchronization for its optional
toolbar, floating-toolbar presentation, and search-strip presentation. It must
not make consumers borrow a child model to update those values. In particular,
every non-empty selection in every frame derives the current floating anchor
from that frame's actual TextSurface record and KUC synchronizes the retained
floating model before rendering it. A first selection initializes the deferred
model; a changed selection repositions the same retained model without
discarding KUC dropdown, focus, tooltip, measurement, or interaction identity.
Tests must move an existing selection and assert that the KUC floating record
and artifact anchor/bounds follow the new TextSurface selection while the
retained control identity remains stable.

KUC creates deferred floating state without consumer coordinates. After the
same-frame TextSurface render has produced its real caret, selection and
viewport facts, KUC initializes or synchronizes floating placement and focus
return inside the root adapter. A consumer supplies only generic visibility
policy and opaque command presentation, never `Rect`, viewport, panel size,
`UiNodeId`, a previous frame record, or a bootstrap placeholder. The immutable
output retains the same child events, focus/scroll acknowledgement, actual
bounds and ordered artifact records, with absence represented explicitly for
optional child records.

The contract tests must add a foreign-consumer fixture that constructs the
single retained model with no coordinate or pre-frame inputs, opens a
selection toolbar during its first real RawInput frame, and verifies
Japanese/`⭐️` VS16, IME, search/replace, outside/Escape/focus-return,
AccessKit, reachable root-contained bounds and repeatable artifact hashes.
The guard rejects public root APIs that take both a toolbar and a floating
toolbar by mutable reference, as well as consumer construction of a floating
layout or use of a previous TextSurface frame to bootstrap it.

#### Floating dropdown hit-region correction (2026-08-13)

The selection toolbar's floating panel and its open dropdown are one retained
generic interaction surface. A pointer press on a visible dropdown item must
not be classified as outside merely because that item lies outside the compact
toolbar panel bounds. The KUC adapter therefore owns the current dropdown
trigger, menu, and item bounds as an inclusive interaction region. It resolves
pointer press/release in a visible enabled item before applying outside-dismiss;
the matching typed dropdown activation is emitted exactly once. This rule also
applies to the last visible item in a clamped menu, rather than assuming a
fixed menu height or consumer coordinate.

The correction is generic KUC interaction behavior. KLE/KDV/KatanA consumers
must neither resend a synthetic command nor special-case a final dropdown item.
Actual RawInput tests use separate press and release frames for every code
kind in a 17-item dropdown, including a root-contained final item, and verify
the item's typed event, record/artifact/AccessKit continuity, disabled-item
inertness, true outside dismissal, and Escape closing the menu before the
floating toolbar. A guard rejects consumer hit-test reconstruction and a KUC
implementation that dismisses a pointer contained by the current dropdown
item bounds.

The composition is responsible for stable root/child allocation across input,
search focus, IME preedit, scrolling and menu visibility. It must make the
search input reachable inside the supplied root on every frame, rather than
placing it below the root after TextSurface consumes `available_height`. Its
actual RawInput/AccessKit tests use Japanese and `⭐️` VS16, text editing,
selection-anchored toolbar, search query/replace focus, outside/Escape/focus
return and repeatable TextSurface/CommandChrome artifact hashes. A guard
rejects consumer rectangle subtraction, child-Ui geometry, alternate search
widgets, and direct sequential TextSurface-plus-CommandChrome rendering in
KLE/KDV/Storybook acceptance paths.

#### Retained text-command-context root composition (2026-08-13)

The standalone generic `EguiContextMenuAdapter` must not force a consumer to
draw a second root after `EguiTextCommandSurfaceAdapter`. ContextMenu is a
generic overlay whose anchor originates in the same actual TextSurface frame;
letting KLE/KDV decide when or how to call it would duplicate root interaction,
overlay order, focus return, artifact assembly, and compositor ordering.

KUC therefore adds a retained generic text-command-context root model and one
`show` call. It retains the existing consumer-safe TextCommandSurface together
with an optional generic ContextMenu presentation and state. The root consumes
only its current TextSurface context-target fact to request opening, owns
right-click/keyboard/AccessKit context requests, submenu/type-ahead, placement,
outside/Escape/focus return, and puts the ContextMenu paint plan above the root
children in KUC-owned canonical artifact order. A consumer supplies an opaque
controlled item tree and visibility policy only; it never supplies anchor
coordinates, root/child rectangles, focus IDs, menu dimensions, overlay order,
or an artifact/compositor plan.

The generic API contains no KLE/KDV/KatanA/Markdown/document/path/clipboard
semantics. Actual RawInput/AccessKit/artifact/compositor tests cover secondary
click, keyboard and accessibility context requests, nested items, disabled
items, outside/Escape/focus return, Japanese/`⭐️` VS16 labels, optional context
absence with no reserved root slot, and deterministic same-root RGBA hashes.
Guards reject consumer sequential TextCommandSurface/ContextMenu display,
consumer popup geometry/hit testing, and consumer-owned overlay ordering.

#### Immutable root artifact-layer order (2026-08-13)

The root already appends its ContextMenu paint plan after the retained text and
command-chrome plans. A consumer must be able to forward that exact KUC
decision to the public compositor without reconstructing it from optional child
artifacts. The root output therefore exposes an owned generic immutable
artifact-layer sequence. Its ContextMenu layer exists only when the root emits
a ContextMenu artifact and is always final. KUC alone creates the sequence and
resolves it to paint-plan references. KLE/KDV/Storybook may clone or forward
the sequence but may not insert, delete, sort, reverse, infer, or merge
layers. This API contains no KatanA/KLE/KDV/Markdown/document/path/clipboard
meaning or consumer geometry.

#### Generic ContextMenu overflow ownership (2026-08-14)

`egui::ScrollArea` は `scroll_source(NONE)` と zero local offset に固定し、visible viewport の clip
だけを担う。scroll position は KUC retained state が唯一の source of truth とし、menu bounds 上の actual
`RawInput::events` の当該 frame `MouseWheel` だけを KUC が clamp して消費する。平滑化された過去 frame の
scroll delta を再利用してはならない。これにより submenu path 切替後に古い wheel が再適用されること、及び
ScrollArea の content coordinate が AccessKit tree に可視域外 node を作ることを避け、record、hit region、
AccessKit、paint plan の visible row を一つの current menu bounds に固定する。

`EguiContextMenuAdapter` must not publish an item as physically selectable when
its bounds are outside the current menu frame. The existing placement resolver
may clamp a menu to a viewport, but that clamp alone is insufficient when the
unclipped item list is taller than the rendered frame: allocating every row
while reporting every row leaks unreachable geometry to consumers and makes an
in-frame pointer press appear as an outside click.

KUC owns a retained vertical scroll offset for the current visible menu level.
It derives the content extent from the measured menu items, clamps the offset
against the resolved frame height, consumes actual wheel input, clips allocation
and paint to the resolved bounds, and resets or re-clamps the offset whenever
the visible submenu level or its content changes. `EguiContextMenuFrameRecord`
contains only rows whose final hit bounds are wholly inside `record.bounds`.
The frame record may expose a generic scroll fact only when it is necessary to
describe the currently KUC-owned visible state; consumers never provide an
offset, geometry, hover target, hit-test result, or synthetic activation.

Pointer actions are resolved against the same clipped row bounds that the
record, AccessKit nodes, paint plan, and artifact expose. A physical press and
release on a refreshed visible leaf produces exactly one core item-selection or
activation event. Disabled rows remain inert before and after scrolling. The
outside-dismiss test excludes a press contained by any currently visible row;
Escape and focus return retain their existing KUC-owned precedence.

The retained hit identity includes the full generic submenu path. A root menu
whose target submenu is not index zero must have the same physical selection
behavior as an index-zero root. In particular, after scrolling `root/Edit`,
opening its nested code submenu, and refreshing the public record, a primary
press/release on a visible first code leaf must emit `ContextMenuEvent::ItemSelected`
exactly once; no consumer-specific id, geometry, or activation fallback is involved.

The contract is generic. Tests use opaque ids and localized labels, including
Japanese, `⭐️` VS16, and a ZWJ label/source, but introduce no KLE/KatanA,
Markdown, document, or host-command type. They exercise a long direct submenu
and a nested long submenu through secondary-click, Shift+F10, and AccessKit
opening; each route scrolls by RawInput, reads fresh public bounds, and uses a
physical primary press/release for the final leaf. Artifact/compositor and
AccessKit assertions use the clipped public frame so deterministic hashes and
overlay order remain KUC-owned.

## Risks / Trade-offs

- [API size] → generic visual data/typed request だけにし、document semantics/side effects は consumer に残す。
- [input jitter] → raster/cache/texture state は adapter instance が所有し、stable frame bounds を numeric test
  で固定する。
- [range conversion] → KUC query 以外の grapheme conversion を consumer guard で禁止する。
- [genericity] → DiagnosticsList/HoverCard/ContextMenu/ScrollArea は既存 KUC component を compose し、新規
  surface API は range/gutter/frame record の不足だけを扱う。
- [cross-change dependency] → platform text-raster runtime 完了後に実装し、command chrome と adapter crate
  を共有するが module ownership を `text_surface` / `command_chrome` に分離する。

## Migration Plan

1. public platform text-raster runtime を完了し、KUC TextArea live integration を準備する。
2. KUC TextSurface/gutter/annotation/frame-record model と framework-neutral test を追加する。
3. shared `katana-ui-core-egui-adapter` crate に text-surface module を実装し、その後 command-chrome module を
   追加する。texture/text logic を duplicate しない。
4. KUC Storybook の `text-area` runtime を adapter live TextSurface に置換し、adapter-owned paint-plan/artifact
   encoder、actual egui scripted motion/manifest gate を追加する。existing Canvas catalog は TextSurface evidence
   path から除外する。
5. KLE `PlatformTextSurface`、`LineGutterModel`、generic egui overlay を thin KUC binding に置換して削除する。
6. KLE は syntax/search/diagnostic/context/authoring domain data を KUC annotation/gutter/ContextMenu/
   DiagnosticsList/CommandChrome へ map する。
7. KUC/KLE/KDV/KatanA guard と real input/accessibility/same-frame artifact gate が通るまで release は行わない。

## Open Questions

- context menu action item は existing KUC `ContextMenu` を使い、TextSurface は generic anchor/selection
  context だけを出す。nested menu focus/dismiss contract test を先に書いて exact API を決める。
- texture cache key は text/SVG runtime metadata が stable になった後に opaque adapter identity として統一する。
