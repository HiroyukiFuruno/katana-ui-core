## 0. 設計基準と責務境界

- [x] 0.1 KLE v0.1.0 design と照合し、generic command chrome は KUC、host command / Markdown / file IO は consumer の責務として固定する
- [x] 0.2 existing `ToolbarAction` / `ToolbarEvent` / `SearchControlStripAction` / `SearchControlStripEvent` の public compatibility を確認し、required field / enum variant 追加ではなく new additive CommandChrome DTO/event を採用する
- [x] 0.3 KDV の platform text/emoji raster と viewer/export SVG raster の境界を監査し、前者は既存 `platform-text-raster-runtime` task 4.1 / 5.1、command icon SVG は本 change に分離する
- [x] 0.4 各 requirement に KUC unit / integration / real-egui / frame-record / guard の検証種別を割り当て、Storybook screenshot/video を完了根拠にしない
- [x] 0.5 KatanA を読み取り専用 specification source と固定し、KUC public command/search model、close/focus transition、localized result-summary template の実装前 contract を design に明記する

## 1. Public SVG Icon Raster Runtime

- [x] 1.1 `katana-ui-core-svg-raster` crate の workspace/public API を追加し、core crate に raster dependency を漏らさない
- [x] 1.2 `UiSvgRasterRequest` / `UiSvgRaster` / metadata / typed error / bounded rasterizer cache を実装し、physical size・unpremultiplied RGBA・cache key の contract を固定する
- [x] 1.3 `UiSvgPaintPolicy` の currentColor / stroke / fill / alpha 正規化を実装し、invalid SVG・zero/oversize/overflow が Unicode/emoji fallback なしの typed error になることを検証する
- [x] 1.4 identical request の pixel equality/cache reuse、color/policy/size cache split、deterministic eviction、maximum allocation を数値 unit test で検証する
- [x] 1.5 KUC Storybook の private SVG rasterizer を public runtime adapter に置換し、private parser/cache が残らないことを source/dependency guard で検証する

## 2. Generic Command Chrome Model

- [x] 2.1 `CommandChromeAction` / `CommandChromeToolbar` / display mode / validation / typed event を追加し、existing Toolbar action/group/overflow/state を内部 compose する
- [x] 2.2 `IconOnly` の non-empty `UiIconProps` と accessible-name 必須、disabled/split/dropdown/overflow/keyboard interaction を contract test で検証する
- [x] 2.3 `FloatingCommandToolbar` の anchor/viewport/placement/focus/dismiss model を追加し、outside click、editor click、escape、focus return、viewport clamp を数値/state test で検証する。open dropdown がある Escape は dropdown だけを先に close し、次の Escape が toolbar close/focus return へ進むことを core/actual-egui test で固定した
- [x] 2.4 existing Toolbar public API / serialization fixture / exhaustive consumer fixture が source-compatible であることを compile and regression test で検証する
- [x] 2.5 `CommandChromeDropdown` / trigger kind / item DTO / placement state / typed open-close-select-keyboard event を additive に追加し、menu-only と split-secondary、disabled、roving focus、consumer-domain non-leak を core contract test で固定する

## 3. Generic Search Control Presentation

- [x] 3.1 `SearchControlStrings` と structured result-summary parameter model を追加し、command-chrome renderer の全 visible/accessibility string を host injection にする
- [x] 3.2 existing `SearchControlStrip` を compose する `CommandChromeSearchStrip` / capability / close DTO-action-event を追加し、search/replace engine と host state を KUC に入れない
- [x] 3.3 regex/replace/navigation/close の unavailable state と injected disabled reason を実装し、disabled control が typed operation request を emit しないことを検証する
- [x] 3.4 legacy SearchControlStrip rendering を compatibility path として保持しつつ、new command-chrome path が fixed English render を呼ばないことを contract/source test で検証する
- [x] 3.5 existing SearchControlStrip public API / serialization fixture / exhaustive consumer fixture が source-compatible であることを compile and regression test で検証する

## 4. KUC-Owned Egui Adapter and Same-Surface Record

- [x] 4.1a `CommandChromeAction` / `CommandChromeToolbar` の renderer-neutral accessor と TextSurface controlled-value synchronization を additive に追加し、core compatibility test で固定する
- [/] 4.1b `EguiCommandChromeAdapter` と shared bounded RGBA texture cache を追加し、text / SVG raster only dependency boundary を compile test で固定する。toolbar/search/floating/tooltip と generic menu-only / split-secondary dropdown は actual-egui input まで実装済みで、same-record artifact integration は未完了
- [/] 4.1 shared `katana-ui-core-egui-adapter` の command-chrome module を `kuc-text-surface-adapter` と整合させて追加し、core crate と host application への framework/host dependency leak を compile test で防ぐ。toolbar/search/floating/tooltip/dropdown boundary は実装済みだが、artifact contract は未完了
- [/] 4.2a injected raster/paint style と `EguiCommandChromeFrameRecord` を追加し、toolbar / floating / search の same-record draw contract を数値 test で固定する。toolbar/search/floating/tooltip と dropdown trigger/menu/item/accessibility record は実装済みで、egui draw と deterministic artifact の equality test は未実装
- [x] 4.2 adapter が SVG runtime RGBA texture と platform text-raster layout を使って toolbar/search chrome を描画し、`egui::TextEdit` / font registration / OS font lookup / glyph icon fallback を使わないことを AST test で検証する
- [/] 4.3a query / replace を shared TextSurface で actual egui / IME input から typed search action へ変換し、button / option / navigation / replace / close を real event で検証する。Japanese/`⭐️`/IME/keys/disabled/focus arbitration、icon tooltip、floating composition、generic menu-only / split-secondary dropdown は実証済みだが same-record artifact は未完了
- [/] 4.3 actual egui pointer/keyboard/focus/IME input を new KUC typed action/event へ変換し、enabled/disabled/split/dropdown/search/replace/close の real interaction test を追加する。dropdown trigger、disabled item、item selection、outside/Escape、roving focus、AccessKit は実証済みだが scripted cross-surface sequence と frame-record equality は未完了
- [ ] 4.4 adapter frame record（raster layer、rect、hit target、focus、typed state）を定義し、egui draw と deterministic artifact が同じ record を消費する equality test を追加する。dropdown record は追加済みだが equality test は未実装
- [x] 4.3b query / replace TextSurface state を strip state id で保持し、controlled synchronization、single-line input policy、KatanA-compatible query key routing、toolbar focus arbitration を実装する
- [x] 4.3c query / replace / option / navigation / replace-one/all / close の injected SVG-or-label controls と AccessKit node を実装し、zero-result/disabled capability が operation event を出さないことを real-egui test で固定する
- [/] 4.3d Japanese / `⭐️` type と IME commit、Enter/Shift+Enter/ArrowUp/ArrowDown/Escape、pointer controls、frame-record equality を `egui::Context::run_ui` の actual input sequence で検証する。Japanese/`⭐️`/IME/keys/pointer/disabled は実証済みだが scripted sequence の frame-record equality を追加する

## 5. Storybook, Guardrails, and Consumer Contracts

- [ ] 5.1 KUC Storybook に icon toolbar、floating placement、dropdown、find/replace、localized string、regex unavailable、Japanese と `⭐️` input の real component scenario を追加する
- [ ] 5.2 Storybook scripted event sequence / frame-record manifest / motion artifact を追加し、fallback renderer、manual text measurement、label parsing、fixed wait が release evidence に入らないことを検証する
- [x] 5.3 `scripts/kuc_guardrails.py` / tests と `just ast-lint` を更新し、core heavy dependency、private SVG raster、host-specific import、新 command chrome の fixed literal、Unicode/emoji icon fallback を検出する
- [ ] 5.4 KLE consumer compile/mapping contract と KDV consumer compile/audit contract を追加し、KLE-owned authoring/search renderer と KLE/KDV duplicate command-icon rasterizer が残る場合は fail する release gate を設計どおり更新する
- [ ] 5.5 formatter、clippy、workspace test、KUC guardrails、AST lint、OpenSpec strict validate、KLE/KDV targeted consumer test を実行し、requirement-to-evidence matrix の全行を埋める
- [ ] 5.6 KUC/KLE/KDV の差分を responsibility boundary で自己レビューし、残る KDV direct `cosmic-text` removal は existing `platform-text-raster-runtime` task 4.1 / 5.1 で未完了として明示したまま release blocker にする
