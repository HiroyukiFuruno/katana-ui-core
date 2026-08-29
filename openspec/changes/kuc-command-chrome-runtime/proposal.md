## 背景

KatanA 互換の editor command chrome には、コンパクトな icon control、キャレットへ
アンカーする toolbar、find/replace control が必要である。KUC には generic model の
一部が存在するが、SVG raster runtime、host から渡す icon slot、可視文言の DI、framework
adapter が欠けており、これらを KLE/KDV に実装すると platform と visual の乖離を再生産する。

## 変更内容

- `UiIconProps` だけを入力とする public かつ renderer-neutral な SVG icon raster runtime を
  追加する。deterministic な RGBA 出力と cache metadata を提供する。
- 既存の generic `Toolbar` に optional icon slot を追加し、typed action / group / placement /
  focus / dismiss event を用いる cursor-anchored floating command toolbar の contract を追加する。
- `SearchControlStrip` を、host 注入の visible/accessibility string、capability state、close
  interaction、および find / navigation / replace の完全な renderer-neutral view model へ拡張する。
- KUC 所有の optional egui command-chrome adapter を追加する。KUC props のみを描画し、KUC の
  typed event のみを返す。host command 名、search/editor side effect は持たない。
- Storybook の private SVG raster path を public runtime へ移し、duplicate rasterizer や
  host-specific semantics を防止する contract / adapter / guard を追加する。

## Capability

### 新規 Capability

- `kuc-svg-icon-raster-runtime`: host icon 非依存の SVG icon rasterization、paint policy、RGBA
  output、cache、size limit、typed error の contract。
- `kuc-command-toolbar`: generic icon-capable toolbar と floating toolbar composition、placement、
  interaction lifecycle、accessibility、typed event の contract。
- `kuc-search-control-strip-contract`: generic かつ localized な find/replace control state、
  capability、accessibility、renderer-neutral view model、typed event の contract。
- `kuc-egui-command-chrome-adapter`: framework-neutral component contract を保持する optional
  egui command chrome adapter の contract。

### 既存 Capability の変更

- なし。現行 main spec は toolbar/search の基準 contract を持たないため、この change で
  archive 時に欠落しない完全な新規 capability を定義する。

## 影響範囲

- SVG raster dependency は framework-neutral core crate 外の new KUC runtime crate に閉じる。
- existing `Toolbar` / `SearchControlStrip` は内部 implementation として compose する。public
  struct literal / exhaustive enum match を壊さないため、icon、localized presentation、close、
  capability、floating lifecycle は新しい additive `CommandChrome` DTO/event に閉じる。
- KUC Storybook は private SVG icon path ではなく runtime と real generic command chrome を
  利用する。
- KLE/KDV は KUC runtime/adapter の consumer になる。KatanA、KLE、KDV、Markdown、filesystem、
  editor、viewer の型・enum・文字列を KUC に導入しない。
