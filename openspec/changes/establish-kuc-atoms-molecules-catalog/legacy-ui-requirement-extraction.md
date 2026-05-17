# Legacy UI requirement extraction

作成日: 2026-05-18
対象: archive 01〜17、19〜22、active 18 / 23 / 24

## 読み替え方針

旧 change の `[x]` は KUC 完了根拠にしない。
ここでは旧 change から、KUC の atoms / molecules に必要な `option`、`action`、`event`、`state`、`preset`、`test`、`storybook` 要件だけを抽出する。

## 01 Theme / Panel theme

- source: `archive/2026-05-12-01-theme-tokens`
- option: color、spacing、typography、radius、shadow、border、z-index、light / dark、panel theme id
- action: theme switch、style sheet replacement
- event: theme change event、panel re-render request
- state: selected theme id、resolved token snapshot
- preset: light、dark、Katana accent
- test: theme serialization、theme diff、token fallback、panel theme configured
- storybook: global theme control、navigation / preview / story root への theme 反映

## 02 Text

- source: `archive/2026-05-12-02-text-primitive`
- option: content、text role、color override、accessibility label、font role
- action: none for display text、style replacement
- event: none for passive text
- state: resolved typography、resolved color、line metrics
- preset: heading、body、caption、code、muted、Japanese、mixed language、emoji
- test: role resolution、color override、mixed-language vertical centering、emoji fallback
- storybook: role grid、light / dark、Japanese / English / mixed / emoji samples

## 03 Icon

- source: `archive/2026-05-12-03-icon-primitive`
- option: icon source、size、color token、custom SVG、accessibility label
- action: none for passive icon
- event: none for passive icon
- state: resolved size、resolved color、parsed icon summary
- preset: preset SVG、custom SVG、small / medium / large、accent / muted
- test: size token resolution、color token resolution、dark mode color
- storybook: icon grid、custom SVG、theme追従

## 04 Spinner / LoadingDots

- source: `archive/2026-05-12-04-spinner-primitive`
- option: size、color、speed、reduced motion、label、dot count
- action: animation tick、reduced motion toggle
- event: animation frame event
- state: animation phase、running / paused、reduced motion
- preset: spinner sizes、loading dots、fast / slow、with label、reduced motion
- test: default props、reduced motion fixed frame、animation state serialization
- storybook: loading feedback group、light / dark、animation state summary

## 05 SvgButton

- source: `archive/2026-05-12-05-svg-button`
- option: icon、size、variant、tone、disabled、loading、accessibility label
- action: click、focus、hover、active、loading suppress
- event: pointer click、focus event、keyboard activation
- state: hover、active、focused、disabled、loading、callback log
- preset: plain、subtle、filled、danger、disabled、loading
- test: disabled suppresses click、theme color resolution、focus ring
- storybook: live icon button、callback log、state variants、theme追従

## 06 TextButton

- source: `archive/2026-05-12-06-text-button`
- option: label、variant、tone、size、disabled、loading
- action: click、keyboard activation
- event: click event、focus event、command event
- state: hover、active、focused、disabled、loading、callback log
- preset: primary、secondary、ghost、link、danger、success、disabled、loading
- test: disabled style、loading suppression、tone / variant matrix
- storybook: text button grid、callback log、light / dark

## 07 IconTextButton

- source: `archive/2026-05-12-07-icon-text-button`
- option: icon、label、icon position、variant、tone、size、disabled、loading
- action: click、keyboard activation
- event: click event、focus event、command event
- state: hover、active、focused、disabled、loading、callback log
- preset: leading icon、trailing icon、primary、secondary、disabled、loading
- test: icon / label spacing、disabled / loading、theme追従
- storybook: icon position comparison、callback log、TextButton / SvgButton consistency

## 08 Toggle

- source: `archive/2026-05-12-08-toggle`
- option: checked value、size、disabled、accessibility label
- action: toggle、keyboard activation
- event: change event、focus event
- state: checked、focused、disabled、callback log
- preset: on、off、disabled、large / compact
- test: disabled blocks change、state transition、a11y label required
- storybook: live toggle、state reflection、callback log、light / dark

## 09 SegmentedToggle

- source: `archive/2026-05-12-09-segmented-toggle`
- option: selected value、options、segment label / icon、disabled、size
- action: select segment、keyboard navigation
- event: selection change、focus event
- state: selected value、focused segment、disabled segment、callback log
- preset: text segments、icon segments、disabled option、empty options
- test: selected marker、empty options fallback、disabled segment behavior
- storybook: real segmented control、selected marker、callback log

## 10 SelectBox

- source: `archive/2026-05-12-10-select-box`
- option: selected value、options、placeholder、disabled、long list、placement
- action: open、close、select option、outside close、keyboard navigation
- event: open event、close event、selection change、focus event
- state: open、selected value、highlighted option、disabled、callback log
- preset: short options、long options、placeholder、disabled、light / dark
- test: open / close state、select closes panel、disabled trigger、long list bounds
- storybook: trigger + options panel、live selection、callback log

## 11 ColorSwatch

- source: `archive/2026-05-12-11-color-swatch`
- option: selected color token、palette、size、disabled
- action: select color
- event: color change、focus event
- state: selected color、focused swatch、disabled、callback log
- preset: token palette、custom palette、disabled、compact / regular
- test: selected ring、disabled blocks change、theme token resolution
- storybook: palette grid、selection preview、callback log、light / dark

## 12 Input / TextInput

- source: `archive/2026-05-12-12-text-input`
- option: value、placeholder、leading icon、trailing slot、size、disabled、readonly、invalid、clear action
- action: type、clear、focus、blur、submit via Enter
- event: key input、text input、IME committed text、emoji input、focus event、change event
- state: value、focused、disabled、readonly、invalid、cursor / selection summary
- preset: default、leading icon、trailing action、readonly、disabled、invalid、Japanese、emoji
- test: disabled / readonly behavior、clear action、IME / emoji input、vertical centering
- storybook: input preview、settings for value and flags、state log、event log

## 13 SearchBox

- source: `archive/2026-05-12-13-search-box`
- option: value、placeholder、disabled、search options、leading search icon、clear button
- action: input、clear、submit、toggle option、Esc clear
- event: change event、submit event、clear event、option change、key event
- state: value、focused、selected options、callback log
- preset: default、all controls、disabled、regex / word / case controls
- test: submit on Enter、Esc clear、option toggles、SVG theme color
- storybook: Material UI風の枠内アイコン、option controls、callback log

## 14 Tooltip

- source: `archive/2026-05-12-14-tooltip`
- option: label、placement、delay、max width、hover / focus trigger
- action: hover open、focus open、close on leave / blur
- event: pointer enter / leave、focus event、open / close event
- state: visible、anchor summary、placement、timer summary
- preset: top、bottom、start、end、edge flip、focus trigger
- test: delay、focus open、placement flip、close lifecycle
- storybook: hover / focus target、real overlay、operation log

## 15 Badge

- source: `archive/2026-05-12-15-badge`
- option: label、tone、variant、size、leading icon
- action: none for passive badge
- event: none for passive badge
- state: resolved tone、resolved typography、resolved shape
- preset: neutral、accent、danger、warning、success、small / regular
- test: tone / variant / size matrix、theme追従
- storybook: dense badge grid、status examples、light / dark

## 16 KeyCap

- source: `archive/2026-05-12-16-key-cap`
- option: key label、modifier combo、size、tone、platform display
- action: none for passive key display
- event: none for passive key display
- state: resolved platform label、resolved key combo
- preset: single key、modifier combo、macOS、non-macOS、compact
- test: macOS / non-macOS modifier display、size / tone resolution
- storybook: shortcut samples、platform variants、light / dark

## 17 Card

- source: `archive/2026-05-12-17-card`
- option: variant、padding、interactive、header、body、footer、actions slot
- action: card click、child click、focus
- event: card click event、focus event、child event propagation
- state: hover、active、focused、interactive、callback log
- preset: plain、elevated、outlined、interactive、form card、nested controls
- test: interactive hover / active、child operation isolation、theme shadow / border
- storybook: card with TextInput / Button / Badge / Accordion、callback log

## 18 Accordion

- source: `openspec/changes/18-accordion`
- option: header node、expanded、disabled、controlled / uncontrolled、indicator position、trigger area、multiple、tree mode、reduced motion、body border
- action: toggle、group expand / collapse
- event: toggle event、callback log、focus event
- state: expanded、selected、depth、show lines、disabled、callback log
- preset: default closed、default open、indicator positions、disabled、single / multiple、tree mode、reduced motion
- test: disabled blocks toggle、controlled / uncontrolled、multiple group、tree mode rendering、reduced motion
- storybook: full-row header、live toggle、callback log、light / dark

## 19 SplitPane

- source: `archive/2026-05-12-19-split-pane`
- option: direction、ratio、min first、min second、handle width、double-click reset
- action: drag handle、double-click reset、keyboard ratio change
- event: ratio change、drag start / move / end、focus event
- state: ratio、dragging、hover handle、orientation
- preset: horizontal、vertical、min clamp、50/50 reset、nested content
- test: ratio clamp、double-click reset、direction layout、handle hover
- storybook: draggable split preview、ratio log、light / dark

## 20 ModalOverlay / Modal

- source: `archive/2026-05-12-20-modal-overlay`
- option: open、title、size、dismiss policy、Esc close、backdrop close、focus trap、footer、native window mode
- action: open、close、Esc、backdrop click、footer action、focus return
- event: open event、close event、focus return、dismiss event
- state: open、focused element、dismiss reason、native window summary、callback log
- preset: small、medium、custom、Esc enabled / disabled、with footer、overlay dialog、native modal
- test: Esc close、backdrop close / suppressed、focus trap、focus return、native window callback
- storybook: Modal と OverlayDialog の違い、open / close controls、native log

## 21 Popover

- source: `archive/2026-05-12-21-popover`
- option: open、anchor、placement、offset、width、outside click、Esc、focus handling
- action: open、close、outside click、Esc、select content
- event: open event、close event、outside click event、key event
- state: open、anchor summary、placement、focus summary、callback log
- preset: 4 placements、auto flip、offset、fixed width、select-box-backed
- test: placement calculation、auto flip、outside click / Esc、focus handling
- storybook: actual popover opening、placement controls、close condition logs

## 22 ColorPicker

- source: `archive/2026-05-12-22-rgba-color-picker`
- option: RGB / RGBA value、mode、readonly、disabled、title、trigger size、alpha、eyedropper callback slot
- action: open、close、change color plane、change hue、change alpha、numeric input、eyedropper callback
- event: color change、open / close、readonly / disabled suppression、callback log
- state: selected color、panel open、mode、readonly、disabled、preview value
- preset: RGB、RGBA、readonly、disabled、alpha variants、dark / light
- test: readonly / disabled blocks change、all controls update same state、theme追従
- storybook: live panel、preview reflection、value label、image-equivalent sample

## 23 ColorPicker parity

- source: `openspec/changes/23-color-picker-complete-parity`
- option: overlay panel、close button、outside click、Esc、trigger border、trigger size xs / sm / mid / large / xlarge、title、RGB / RGBA mode
- action: open overlay、close via outside / Esc / button、drag color plane、drag alpha、change trigger size
- event: open / close event、color update、alpha update、callback log
- state: panel open、selected RGB / RGBA、alpha、trigger config、readonly、disabled
- preset: RGB only、RGBA、readonly、disabled、all trigger sizes、with / without border
- test: continuous update、no default eyedropper if unimplemented、trigger sizing、seamless hue bar
- storybook: opened panel above content、value proof、RGBA trigger preview

## 24 CodeDiff

- source: `openspec/changes/24-code-diff`
- option: before / after source、first line number、line count、split / inline mode、split direction、show all unchanged、highlight ranges、theme
- action: switch display mode、switch split direction、expand / collapse unchanged block、scroll sync
- event: mode change、expand / collapse event、scroll sync event
- state: diff model、collapsed blocks、mode、direction、line pairing、summary counts
- preset: split left-right、split top-bottom、inline、addition only、deletion only、no diff、whitespace diff、trailing newline、long line、Japanese text
- test: LCS model、character highlight、multi-byte ranges、line count mismatch、trailing newline、empty lines、similar row pairing
- storybook: mode controls、collapse controls、theme comparison、long line / sync preview

## 移管結果

- 01〜24 は全て current KUC target へ読み替え済み。
- `katana-widget-parity-backlog` は追加 UI と旧 01〜24 要件の入力元として扱う。
- `ui-core-interaction-visual-parity` は Storybook / visual / guard 要件の入力元として扱う。
- active `18` / `23` / `24` は要件移管後の archive 候補として扱う。
