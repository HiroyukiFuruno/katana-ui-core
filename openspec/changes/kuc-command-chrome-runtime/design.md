## Context

KUC core は framework-neutral な render model と molecule を提供している。既存 `Toolbar` は
typed action / group / overflow / placement を持ち、既存 `SearchControlStrip` は query、option、
navigation、replace event を持つ。しかし `ToolbarAction` は icon を持たず、`SearchControlStrip`
renderer には英語 literal が固定され、private Storybook SVG rasterizer が `resvg` / `tiny-skia`
を直接使う。このまま KLE/KDV が同じ機能を補うと、icon cache、raster policy、popup geometry、
visible string、event lifecycle が各 consumer に複製される。

KLE v0.1.0 の editor は KUC component を compose し、host-provided `UiIconProps` と KUC typed
event だけを境界にする。KUC は KatanA、KLE、KDV の command enum、Markdown、search engine、
filesystem、viewer state を認識しない。`platform-text-raster-runtime` は別 change として text /
emoji glyph、grapheme layout、hit-test の共通化を進めており、本 change の visible command chrome
はその public runtime を消費する adapter を前提にする。

## Goals / Non-Goals

**Goals:**

- SVG icon parsing、paint policy、pixel cache、RGBA output を KUC の一箇所に集約する。
- `Toolbar` と `SearchControlStrip` を KatanA 固有語なしの public component contract として完成する。
- floating toolbar の anchor / viewport clamp / focus / outside click / dropdown / keyboard navigation を
  KUC の typed state/event で再利用可能にする。
- command chrome の visible text と editable find/replace text が egui default font atlas に依存せず、
  KUC text-raster の同じ grapheme layout を使用できる adapter boundary を設ける。
- KUC Storybook は実 component/event/frame record の feedback surface とし、correctness は
  contract / interaction / numeric rendering / guard test で判定する。
- KLE/KDV に local SVG parser/cache、generic toolbar renderer、generic search form state を残さない。

**Non-Goals:**

- この change の実装対象に KatanA repository を含めない。KatanA は editor behavior の読み取り専用の
  specification source および KLE/KUC consumer の後段検証対象であり、adapter patch、依存更新、host UI の
  修正は KUC/KLE の strict contract が完成するまで開始しない。
- KUC に KatanA / KLE / KDV の icon enum、Markdown command、search algorithm、editor/viewer mutation、
  file/image ingest、i18n preset を持たせない。
- `katana-ui-core` core crate に `egui`、`resvg`、`tiny-skia`、windowing、platform font discovery を
  直接依存させない。
- text/emoji raster、IME、main editor canvas を本 change で再設計しない。これらは
  `platform-text-raster-runtime` と consumer adapter の strict contract に従う。
- Storybook screenshot / video を release correctness の唯一の根拠にしない。

## Decisions

### 1. Runtime と framework adapter を core crate から分離する

`katana-ui-core-svg-raster` を workspace public crate とし、`katana-ui-core` の `UiIconProps` と
`UiSvgPaintPolicy` を入力にする。`resvg` / `tiny-skia` はこの crate に閉じる。shared
`katana-ui-core-egui-adapter` を optional adapter crate とし、`katana-ui-core`、SVG runtime、
text-raster runtime、`egui` を依存に持たせる。`kuc-text-surface-adapter` が text-surface module を
所有し、本 change は同 crate の command-chrome module だけを所有する。

core crate に実装する案は framework-neutral boundary を破るため採用しない。Storybook private
module のままにする案は KLE/KDV が consumer になれず duplication を残すため採用しない。KLE が
adapter を持つ案は consumer ごとに coordinate、texture cache、input lifecycle が分岐するため採用しない。

### 2. SVG raster API は host icon を opaque `UiIconProps` として扱う

public API は次の概念を持つ。

```text
UiSvgRasterRequest { icon: UiIconProps, width_px, height_px, color: RgbaColor }
UiSvgRaster { width_px, height_px, rgba_unmultiplied, metadata }
UiSvgRasterizer::rasterize(request) -> Result<UiSvgRaster, UiSvgRasterError>
```

request は physical pixel size を明示し、zero / overflow / configured maximum を超える size は typed
error にする。runtime は `CurrentColor`、`StrokeOnly`、`FillOnly`、`StrokeAndFill` を一貫して処理し、
`currentColor` / stroke / fill の正規化、alpha、invalid SVG を KUC で決定する。cache key は normalized
SVG source、view box、paint policy、physical size、actual RGBA を含み、role / label / host command id は
含めない。cache は rasterizer instance に閉じた bounded LRU とし、stable request の cache hit と
pixel equality を数値 test で固定する。

KatanA icon enum や asset catalog を KUC に移す案は generic API を破るため採用しない。文字 glyph や
Unicode symbol を icon fallback とする案は OS/font に依存し、`⭐️` 問題と同型の失敗を再導入するため
採用しない。raster error は placeholder glyph に置換せず typed error / unavailable state として返す。

### 3. Existing `Toolbar` を compose する新しい icon-capable contract を追加する

`ToolbarAction` は private field の builder であっても、`ToolbarEvent` の enum variant 追加や既存
serialization の意味変更は consumer を壊し得る。そこで `CommandChromeAction` / `CommandChromeToolbar`
を新設し、内部で既存 `ToolbarAction`、`ToolbarState`、overflow、placement を compose する。
`CommandChromeAction` は generic action id、visible label、accessibility label、tooltip、disabled、group、
optional `UiIconProps`、optional dropdown を持つ。icon prop は host が渡す opaque data であり、KUC は
command 名や asset identity を解釈しない。

`CommandChromeDisplayMode::IconOnly` では non-empty icon と accessible name（accessibility label
または tooltip）が両方必須になる。`IconLeading` / `IconTrailing` / `LabelOnly` は icon 無し action を
明示的に描画できるが、IconOnly の icon 欠落は contract violation とし、Unicode/文字 glyph への
silent fallback を許可しない。既存 `ToolbarAction::new(id, label)` と existing Toolbar の API/serde/event
は変更しない。

#### 3.1 Command toolbar の public model を先に固定する

`CommandChromeAction` は private field と builder を持つ additive DTO とする。最低限の model は
`id`、`label`、`icon: Option<UiIconProps>`、`tooltip`、`accessibility_label`、`disabled`、`priority`、
`accelerator`、`group_id`、`split` であり、`to_toolbar_action()` は既存 `ToolbarAction` を内部合成する。
`CommandChromeToolbar` は actions、display mode、density、overflow strategy を所有し、既存
`ToolbarState` / overflow planner に渡す `ToolbarOptions` を内部で生成する。既存 `ToolbarAction`、
`ToolbarOptions`、`ToolbarEvent` の field / variant / serialization は変更しない。

`IconOnly` の validation は `icon.svg_source.trim().is_empty()`、および accessibility label と tooltip の
両方が空であることを個別の typed violation として返す。invalid action は presentation から除外し、
label / Unicode / emoji の substitute を生成しない。action activation、overflow、split dropdown、
accelerator は既存 Toolbar state が返した event を new command-chrome event に一対一で写像する。

#### 3.2 Dropdown は KUC の action presentation/state として定義する

KatanA のコードブロック種別選択は primary command と独立した汎用 menu-only dropdown である。一方で、
通常の command chrome は primary command を保持したまま secondary half から menu を開く split dropdown
も必要とする。このため `CommandChromeDropdown` は `CommandChromeAction` の additive field とし、
`CommandChromeDropdownTrigger::{Primary, SplitSecondary}`、non-empty item list、generic item id を持つ。
`Primary` は action 全体の press / keyboard activation を dropdown open に変換し、`SplitSecondary` は
primary command activation を維持して secondary half だけを dropdown trigger にする。consumer 固有の
`MarkdownAuthoringOp`、`CodeBlockKind`、file format、command enum は DTO に入れない。

`CommandChromeDropdownItem` は stable item id、visible label、optional accessibility label / tooltip、optional
`UiIconProps`、disabled、selected state を持つ。visible label は必須で、platform text raster のみで描画する。
icon は SVG raster のみで描画し、label / icon の Unicode glyph substitute、egui menu text、OS font lookup を
禁止する。item activation は `DropdownItemActivated { action_id, item_id }` という typed event で consumer に
返し、consumer はその id を自身の domain operation に対応付けるだけとする。

dropdown open state は action id、focused item index、trigger rect、viewport rect、menu size、
`PlacementEngine` output、resolved bounds を KUC core が保持する。adapter は actual-egui action rect と
platform-raster measurement を KUC layout action に渡すが、position / clamp / flip / close state を独自に
算出しない。outside click、Escape、explicit dismiss、item activation は typed close reason と focus transition
を返す。open 中の `ArrowUp` / `ArrowDown` / `Home` / `End` は enabled item だけを roving focus し、
`Enter` / `Space` は focused enabled item を activate する。disabled trigger / item は open / activation event
を一切 emit しない。

`EguiCommandChromeFrameRecord` は dropdown trigger bounds、resolved menu bounds、item bounds、raster identity、
focused item、AccessKit node identity を同一 record に含める。KUC Storybook、scripted artifact、real-egui draw は
この record を共有する。KLE は popup geometry、menu state、item renderer、keyboard routing、font/raster cache
を持たない。

`FloatingCommandToolbar` は `CommandChromeToolbar` に加えて `anchor: Rect`、`viewport: Rect`、
`panel_size: Size`、placement priority、open state、roving focus、optional focus-return target を持つ。
consumer surface click、outside click、escape は generic close action として受け、KUC は editor / Markdown
coordinate を解釈しない。close state transition は `Closed { reason }` を一度だけ、focus-return target が
ある場合だけ `FocusReturnRequested { target }` を一度だけ出す。placement は common `PlacementEngine` の
output をそのまま保存し、adapter が別の座標計算を行う余地を残さない。

### 4. Floating command toolbar は既存 toolbar / placement を compose する

新しい `FloatingCommandToolbar` model は `CommandChromeToolbar`、anchor rect、viewport rect、open/focus
state、close policy、split/dropdown state を持つ。内部で existing ToolbarOptions/state/overflow algorithm を
利用する。anchor と viewport は logical pixel の KUC rect type で受け、placement は既存 engine を利用して
上下反転・viewport clamp・deterministic tie-break を解決する。

action activation、split dropdown open/close、keyboard roving focus、outside click close、editor click
close、escape close、focus return を KUC typed event として出す。consumer は caret rect / viewport / enabled
state / action id を渡し、KUC は Markdown selection や editor coordinate を解釈しない。inside pointer
interaction は toolbar を close せず、disabled action / disabled split half は event を出さない。

floating toolbar が open dropdown を持つ間、Escape はまず dropdown を閉じ、次の Escape で floating toolbar を
閉じる。outside click は dropdown と toolbar の両方の bounds を KUC record で判定する。menu-only dropdown の
open / item activation は floating toolbar を閉じず、focus は toolbar と dropdown の KUC roving state 間だけを
遷移する。

KLE で popup coordinate を算出する案、または button inventory を KLE に持つ案は consumer-specific
state machine になるため採用しない。

### 4.1 Shared egui command-chrome adapter contract

`katana-ui-core-egui-adapter::command_chrome` は `EguiCommandChromeAdapter` を公開する。instance は
KUC-owned `PlatformTextRasterizer`、`UiSvgRasterizer`、bounded RGBA texture cache、および query / replace
用の KUC `TextSurface` state を所有する。query / replace state は `CommandChromeSearchStrip` の値と
同期する controlled input であり、KLE/KDV の local `TextEdit` state、font atlas、glyph cache、検索 form
state を保持しない。

adapter API は次の三つに限定する。

| API | 入力 | 出力 |
| --- | --- | --- |
| `show_toolbar` | `CommandChromeToolbar`、injected raster/paint style | `CommandChromeToolbarEvent` と `EguiCommandChromeFrameRecord` |
| `show_floating_toolbar` | `FloatingCommandToolbar`、injected raster/paint style | `FloatingCommandToolbarEvent` と同一 record |
| `show_search_strip` | `CommandChromeSearchStrip`、injected raster/paint style | `CommandChromeSearchEvent`、TextSurface typed events、同一 record |

core model に追加する accessor は renderer-neutral な action id、label、tooltip、accessible label、split state、
focused action と controlled TextSurface value synchronization だけである。adapter は private field、toolbar
layout、search value、label string を再構成しない。

draw order は panel background、action state fill、SVG RGBA texture、platform-text RGBA texture、focus ring
に固定する。icon-only action は host-provided SVG raster failure を typed adapter error として返し、文字、
Unicode、emoji、placeholder glyph に置換しない。label / tooltip / accessibility text は consumer 注入文字列を
そのまま使用し、adapter に固定 English を追加しない。

`EguiCommandChromeFrameRecord` は action / split / overflow / search query / replace / option / navigation /
close の hit rect、raster identity、enabled state、focus target、floating placement、draw layer order を持つ。
actual egui draw、scripted Storybook artifact、numeric regression test は同じ record だけを消費する。adapter が
別の measured width、button inventory、fallback pixels を作ることは許可しない。

actual event mapping は次で固定する。

- action click / accelerator / keyboard roving focus / split click / overflow click は
  `CommandChromeToolbarAction` を経由して `CommandChromeToolbarEvent` にする。
- floating outside click、consumer surface click、escape は `FloatingCommandToolbarAction::Dismiss` にする。
- query / replace TextSurface の text/IME commit は `SearchControlStripAction::SetSearchQuery` /
  `SetReplaceValue` にし、option、previous、next、replace one/all、close は
  `CommandChromeSearchAction` にする。
- disabled action/capability は interaction event を出さない。clipboard/history backend、search engine、host
  command callback は adapter が呼ばない。
- action buttons は actual AccessKit `Button` node、query / replace は shared TextSurface AccessKit input node
  を出す。role / label / disabled / focus state は actual `TreeUpdate` を query する test で確認する。

`egui::TextEdit`、egui font registration/measurement、OS font lookup、`painter().text`、manual glyph raster、
Unicode / emoji icon fallback は command-chrome adapter production source で禁止する。この禁止は KUC guardrail
と `just ast-lint` の両方で機械検査する。

#### 4.1.1 Search surface state, key routing, and evidence plan

`EguiCommandChromeAdapter` は `SearchSurfaceState` を instance 内に一つだけ保持する。state は
`CommandChromeSearchStrip::state_id_model()` を owner identity とし、query / replace にそれぞれ
`{strip-state}:query` / `{strip-state}:replace` の stable `TextArea` state id を割り当てる。strip identity が
変わる時だけ surface を再生成し、同一 identity の frame 間では `TextSurface::synchronize_value` で host
controlled value を同期する。この同期は user event を発生させず、KLE/KDV/egui local string state は保持しない。

query / replace は `min_rows = max_rows = 1`、auto-grow disabled、IME enabled、newline disabled の KUC
`TextSurface` とする。placeholder、accessible label、disabled reason、foreground/background/selection/preedit/
caret color、input width、height、gap、button padding は all injected `SearchControlStrings` / command chrome style
DTO から得る。adapter は fixed visible text、fixed color、fixed text width、local font/glyph fallback を持たない。
query/replace の visible pixels、caret、selection、preedit、hit target、AccessKit input node は shared
`EguiTextSurfaceAdapter` の frame record をそのまま使用する。

single-line search requires a generic egui-boundary `EguiTextSurfaceInputPolicy`: it can suppress a supplied set of
keys before TextSurface selection/edit handling, but it cannot synthesize host actions or inspect host state. Command
chrome supplies `Enter`, `ArrowUp`, `ArrowDown`, and `Escape` only while query owns focus. This prevents query keyboard
commands from moving the text caret and prevents a focused text surface from being consumed by toolbar roving focus.
`show_toolbar` likewise processes roving keyboard input only when one of its actual action responses owns focus.

The query keyboard mapping is fixed from the read-only KatanA document-search source behavior and remains generic at
the KUC event boundary:

| Actual focused query input | KUC action | Resulting typed event |
| --- | --- | --- |
| text / IME commit (including Japanese and `⭐️`) | `SetSearchQuery` exactly once per `TextAreaEvent::Change` | `SearchQueryChanged` |
| `Enter` or `ArrowDown` | `Navigate(Next)` | `SearchNavigationRequested(Next)` |
| `Shift+Enter` or `ArrowUp` | `Navigate(Previous)` | `SearchNavigationRequested(Previous)` |
| `Escape` | `RequestClose` | `CloseRequested` |

Replace text / IME commit maps exactly once per `Change` to `SetReplaceValue`; replace-one/all remain explicit button
operations and do not execute a replacement in KUC. Search options, previous/next, replace-one/all, and close are
rendered as injected-label or injected-SVG generic controls with stable ids. Icon-only controls require host-provided
`UiIconProps` plus an injected accessible name; missing SVG never falls back to a Unicode or text glyph. Disabled
capabilities, disabled replace mode, and zero-result navigation allocate an accessible disabled control and emit no
operation request.

`EguiCommandChromeSearchOutput` contains `CommandChromeSearchEvent`s, query/replace `TextSurfaceEvent`s, and an
`EguiCommandChromeSearchFrameRecord`. The record owns query/replace TextSurface records plus every option/navigation/
replace/close hit rect, raster identity, enabled/disabled state, focus target, and layer order. Real egui draw,
Storybook scripted sequence, deterministic motion manifest, and numeric tests consume this record; no fallback canvas
or independently measured button rectangle is permitted.

The acceptance suite shall use `egui::Context::run_ui` and actual `RawInput` to prove: Japanese/`⭐️` type and IME
commit retain the exact string and one typed change event; query key routing produces the table above without changing
the query selection; actual click/keyboard controls produce only typed KUC events; disabled controls produce none;
AccessKit reports input/button labels, roles, bounds, focus, and disabled state; two identical interaction sequences
produce equal frame records and raster identities. A source/dependency guard shall scan every command-chrome adapter
production module, including future sibling modules.

### 5. Existing `SearchControlStrip` を compose する presentation/capability/close contract を追加する

`SearchControlStrings` は strip label、query/replace placeholder、option tooltip と accessible label、
previous/next、result summary、replace one/all、close、disabled reason の全 visible / accessible text を
持つ。result summary は `{active}` / `{count}` の typed parameter で format する renderer-neutral contract
にし、新規 command chrome renderer に English literal を残さない。

`CommandChromeSearchStrip` は existing `SearchControlStrip` の query/options/navigation/replace state と
event を内部で compose し、strings、capabilities、close interaction を追加する。close は既存
`SearchControlStripAction/Event` に variant を追加せず、new
`CommandChromeSearchAction::RequestClose` / `CommandChromeSearchEvent::CloseRequested` として出す。
legacy `SearchControlStrip::into(UiNode)` は backward compatibility のため残すが、KUC command chrome
adapter と KLE/KDV は必ず injected presentation path を使う。必要な readonly getter の追加は additive method
に限定する。

`SearchControlCapabilities` は regex / replace / close / navigation の availability と disabled reason を
持つ。regex を処理できない host は toggle を disabled にして injected reason を表示し、KUC も adapter
も regex を実行しない。`ReplaceMode` は existing compatibility を保ちつつ capability と併せて実行可否を
決定する。

search state / engine / editor mutation を KUC に置く案、または visible string を enum match で補完する案は
host i18n と responsibility boundary を破るため採用しない。

#### 5.1 Command search の public presentation model を先に固定する

`SearchControlStrings` は default English literal を持たない required presentation DTO とする。strip、query、
replace、各 option、previous/next、replace one/all、close の visible / tooltip / accessibility text と、
unavailable capability の disabled reason を host から受け取る。result summary は serializable
`SearchResultSummaryTemplate` と `{active}` / `{count}` parameters により format し、closure や locale enum を
持ち込まない。

`CommandChromeSearchStrip` は private `SearchControlStrip` を state machine として compose し、
`SearchControlCapabilities` と `SearchControlStrings` を presentation boundary として所有する。new action/event
は legacy action/event を wrapped するか、`RequestClose` / `CloseRequested` を additively 表現する。unavailable
regex、replace、navigation、close は state を変えず operation request を emit しない。legacy render function は
互換性のため残し、new command-chrome model は legacy render result を受け取らない。

### 6. egui adapter は KUC model を render し、text/interaction の根拠を共有する

egui adapter は `FloatingCommandToolbar` / `SearchControlStrip` / `UiSvgRasterizer` / platform text-raster
output を受け、KUC-typed interaction action/event のみを返す。icon texture cache は SVG runtime metadata を
key にし、adapter instance に閉じる。label、tooltip、result summary、input text、caret、selection の visible
pixels / hit target は KUC text-raster layout を使用する。egui は pointer、keyboard、IME event collection と
RGBA texture upload に限定し、`egui::TextEdit`、font registration、OS font lookup、emoji/icon glyph
substitute を command chrome path に使用しない。

adapter は actual egui input/event sequence を KUC action に変換するが、host callback を直接呼ばない。
host-specific action mapping は KLE/KDV の thin binding 層で一回だけ行う。adapter が frame record を出力し、
egui draw と Storybook deterministic artifact は同じ rect、raster layer、interaction target、typed state を
利用する。別途 fallback glyph renderer で GIF を生成する経路は禁止する。

### 7. 正しさは KUC test/guard に固定し、Storybook は live feedback に留める

KUC は SVG request validation、raster pixel/cache contract、toolbar/search state-event contract、placement、
focus/dismiss、accessible icon-only validation、real egui event mapping、frame-record equality を automated
test にする。AST/dependency guard は KUC core の heavy dependency 混入、KUC Storybook private SVG raster、
KLE/KDV duplicate SVG rasterizer、hard-coded search English literal、Unicode icon fallback を検出する。

Storybook は同じ public component / adapter を載せ、scripted event sequence と frame-record manifest を
出す。video はユーザー判断の補助証跡であり、test success の代用ではない。

## Risks / Trade-offs

- [SVG feature compatibility] → supported SVG / paint policy を typed error と test fixture に明示し、
  unsupported input を font glyph へ fallback しない。
- [text-raster runtime が未完] → adapter implementation task は runtime public API / contract complete を
  dependency とする。toolbar/search model と SVG runtime の spec/test は先行できる。
- [existing toolbar/search consumer compatibility] → 既存 public struct/enum への required field/variant
  追加を行わず、新しい wrapper DTO/event と additive readonly method に限定し、serialization fixture と
  existing tests の全再実行で確認する。
- [egui input lifecycle] → real egui test で type / IME commit / pointer / keyboard / focus return を typed
  KUC event まで検証し、fixed timeout や label parsing を使用しない。
- [cache growth / stale texture] → bounded cache、deterministic eviction、size/color/policy の cache split、
  texture invalidation test を持つ。
- [cross-repo migration drift] → KUC public API と contract test を先に固定し、KLE/KDV は adapter consumer
  compile と duplicate implementation guard を release gate にする。

## Migration Plan

1. KUC main spec と task ledger をこの design に同期し、`platform-text-raster-runtime` の public contract
   dependency を確認する。implementation はこの design / task / strict validation が揃うまで開始しない。
2. SVG raster runtime を追加し、private KUC Storybook SVG raster path を public runtime adapter に置換する。
3. existing Toolbar/SearchControlStrip を compose する `CommandChromeAction` / `FloatingCommandToolbar` /
   `CommandChromeSearchStrip` と `SearchControlStrings` / capability / close を追加し、framework-neutral
   contract test を通す。
4. KUC egui adapter と same frame record を実装し、real egui event mapping と KUC Storybook scripted
   sequence を通す。
5. KLE は KUC adapter を呼ぶ thin mapping に置換し、temporary KLE-owned authoring/search renderer と
   `StorybookFallbackRenderer` を削除する。
6. KDV は KUC runtime/adapter を consumer として compile し、local SVG/emoji/text duplication を audit する。
7. KUC/KLE/KDV の dependency/AST/contract/release gates が全て通るまで release は行わない。rollback は
   consumer を旧 API のまま残せる additive API を利用し、公開前の local change を revert する。

## Open Questions

- shared adapter crate は `katana-ui-core-egui-adapter` とする。text-surface と command-chrome は同 crate
  内の別 module で実装し、text rasterizer / texture cache / font/input policy を duplicate しない。
- `SearchControlStrings` の result-summary formatter を structured token sequence にするか、host-provided
  formatter closure ではなく serializable template にするかは cross-platform serialization contract test を
  先に書いて決める。closure は DTO serialization を壊すため default 案にしない。
- KDV の現在の emoji/text migration で残る viewer-specific raster path は KDV audit の結果に基づき、
  text-raster change にのみ追加する。SVG command icon runtime と viewer content SVG renderer を混同しない。
