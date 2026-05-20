## Why

`katana` の status bar（`top_bar/status_bar.rs`、`workspace_toolbar.rs`）、`katana-chat-ui` の usage / vendor 状態の表示、`katana-markdown-linter` の linter summary は、いずれも複数 segment を持つ status bar 構造を取る（leading に file info、center に lint summary、trailing に encoding / line:col / language 等）。

KUC は `StatusBar` molecule と `ProgressBar` atom を持つが、現状は「severity message 1 件 + dismiss + action」と線形 progress に留まる。
複数 segment 構造（leading / center / trailing × 複数 segment、segment ごとの click / popover）と、`katana-chat-ui` の usage 表示で必要な ring / pie 型の progress meter を持たない。

## What Changes

- `StatusBar` molecule の option を拡張するか、別 molecule `StatusBarMultiSegment` を追加する。
- 採用: `StatusBar` molecule を拡張して segment 列を保持できるようにする（後方互換: 既存の `severity + message` モードは default）。
- option:
  - `mode: SingleMessage | MultiSegment`
  - `segments: Vec<StatusBarSegment>`（mode=MultiSegment のとき）
  - `severity: Option<Severity>`（mode=SingleMessage と互換）
  - `density: Compact | Default`
- `StatusBarSegment`:
  - id, label, icon, tone, alignment（Leading | Center | Trailing）, tooltip
  - `interactive: bool`（クリック / popover trigger）
  - `popover: Option<PopoverSpec>`（segment クリックで popover を開く）
- `ProgressMeter` atom を追加する、または `ProgressBar` を拡張する:
  - `shape: Linear | Ring | Pie`
  - `percent: u8`
  - `label`
  - `tone`
  - `tooltip`
- `StatusBarSegment` は `progress: Option<ProgressMeterSpec>` を持てる。
- action: `SegmentPressed` / `SegmentPopoverOpened` / `SegmentPopoverClosed` / `Dismiss`（既存）
- event: 同上 + `SegmentTooltipShown`

## Capabilities

### Modified Capabilities

- `kuc-widget-layer`: `StatusBar` の mode 拡張（SingleMessage / MultiSegment）と segment 構造、segment ごとの popover 機能を明記する。

### New Capabilities

- `kuc-status-bar-segments`: status bar segment と progress meter の option / action / event / state / preset / preview / settings / 自動テスト / 数値化された描画契約 / Storybook ページの完了条件を定義する。

## Impact

- `crates/katana-ui-core/src/molecule/basic.rs`（`StatusBar`）と progress atom を拡張する。
- 既存 SingleMessage モードは default で動作維持。
- consumer (`katana` workspace_toolbar、`katana-chat-ui` usage bar、`katana-markdown-linter` summary) は KUC StatusBar に統一できる。
- segment popover は `add-rich-popover-and-hover-card-04` の共通 placement engine に依存。
