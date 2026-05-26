# Storybook menu priority order

作成日: 2026-05-25

## 方針

Storybook menu page には優先順位番号を付ける。
ただし、番号は change ディレクトリ名には入れない。

理由は、実装中に順序を組み換える必要が出た場合でも、`storybook-page-<menu-page>` の参照や OpenSpec change 名を rename せず、この表だけを更新できるようにするためである。

## 組み換えルール

- `SB-001` から `SB-077` までを現在の実装順とする。
- 番号は優先順位であり、安定 ID ではない。
- 安定 ID は `storybook-page-<menu-page>` とする。
- 順序を変更する場合は、この表を更新し、必要なら変更理由を `並べ替え理由` に追記する。
- 親 tasks や leaf change の change 名は、番号変更だけでは rename しない。

## 実施状況の読み方

- `実装状況` は Storybook page の現在地を表す。
- `DoD 状況` は leaf change の harness 完了判定を表す。
- `次アクション` はその page で次に着手する作業を表す。
- 次に作業する対象は、`DoD 状況` が未完了で最小の `SB-*` とする。
- `次をすすめて`、`continue`、`次` のような依頼では、`rtk proxy python3 scripts/next-storybook-page-change.py --json` が返す `change` を使う。

## 優先順位

| priority | menu page | leaf change | 実装状況 | DoD 状況 | 次アクション | 並べ替え理由 |
| --- | --- | --- | --- | --- | --- | --- |
| SB-001 | `panel` | `storybook-page-panel` | page別描画あり | 完了 | 完了 | Storybook shell / scroll / inspector の基盤 |
| SB-002 | `theme-tokens` | `storybook-page-theme-tokens` | page別描画あり | 完了 | 完了 | light / dark / token 契約の基盤 |
| SB-003 | `row` | `storybook-page-row` | page別描画あり | 完了 | 完了 | layout 基盤 |
| SB-004 | `column` | `storybook-page-column` | page別描画あり | 完了 | 完了 | layout 基盤 |
| SB-005 | `stack` | `storybook-page-stack` | page別描画あり | 完了 | 完了 | overlay / z order の基盤 |
| SB-006 | `grid` | `storybook-page-grid` | page別描画あり | 完了 | 完了 | settings / list 系の配置基盤 |
| SB-007 | `align-center` | `storybook-page-align-center` | page別描画あり | 完了 | 完了 | preview bounds の基礎検査 |
| SB-008 | `scroll-area` | `storybook-page-scroll-area` | page別描画あり | 完了 | 完了 | panel / list / tree / command の scroll 基盤 |
| SB-009 | `split-pane` | `storybook-page-split-pane` | page別描画あり | 完了 | 完了 | Storybook shell と app layout の基盤 |
| SB-010 | `text` | `storybook-page-text` | page別描画あり | 完了 | 完了 | 文字描画と font fallback の基盤 |
| SB-011 | `icon` | `storybook-page-icon` | page別描画あり | 完了 | 完了 | icon / button / menu の基盤 |
| SB-012 | `divider` | `storybook-page-divider` | page別描画あり | 完了 | 完了 | section / menu / settings の基礎部品 |
| SB-013 | `spacer` | `storybook-page-spacer` | page別描画あり | 完了 | 完了 | layout spacing の基礎部品 |
| SB-014 | `button` | `storybook-page-button` | page別描画あり | 完了 | 完了 | 操作 UI の基礎 |
| SB-015 | `text-button` | `storybook-page-text-button` | page別描画あり | 完了 | 完了 | button family |
| SB-016 | `svg-button` | `storybook-page-svg-button` | page別描画あり | 完了 | 完了 | button family |
| SB-017 | `icon-text-button` | `storybook-page-icon-text-button` | page別描画あり | 完了 | 完了 | button family |
| SB-018 | `badge` | `storybook-page-badge` | page別描画あり | 完了 | 完了 | 状態表示の基礎 |
| SB-019 | `key-cap` | `storybook-page-key-cap` | page別描画あり | 完了 | 完了 | shortcut 表示の基礎 |
| SB-020 | `progress-bar` | `storybook-page-progress-bar` | page別描画あり | 完了 | 完了 | loading / status の基礎 |
| SB-021 | `loading-dots` | `storybook-page-loading-dots` | page別描画あり | 完了 | 完了 | loading atom |
| SB-022 | `spinner` | `storybook-page-spinner` | page別描画あり | 完了 | 完了 | loading atom |
| SB-023 | `skeleton` | `storybook-page-skeleton` | page別描画あり | 完了 | 完了 | loading placeholder |
| SB-024 | `motion` | `storybook-page-motion` | page別描画あり | 完了 | 完了 | animation / reduced motion の基盤 |
| SB-025 | `form-field` | `storybook-page-form-field` | page別描画あり | 完了 | 完了 | input / settings の wrapper |
| SB-026 | `text-input` | `storybook-page-text-input` | page別描画あり | 完了 | 完了 | 入力 UI の基礎 |
| SB-027 | `text-area` | `storybook-page-text-area` | page別描画あり | 完了 | 完了 | 複数行入力 |
| SB-028 | `checkbox` | `storybook-page-checkbox` | page別描画あり | 完了 | 完了 | 選択 UI の基礎 |
| SB-029 | `radio` | `storybook-page-radio` | page別描画あり | 完了 | 完了 | 選択 UI の基礎 |
| SB-030 | `toggle` | `storybook-page-toggle` | page別描画あり | 完了 | 完了 | binary setting |
| SB-031 | `segmented-toggle` | `storybook-page-segmented-toggle` | page別描画あり | 完了 | 完了 | segmented setting |
| SB-032 | `select-box` | `storybook-page-select-box` | page別描画あり | 完了 | 完了 | pulldown selection |
| SB-033 | `combo-box` | `storybook-page-combo-box` | page別描画あり | 完了 | 完了 | input + selection |
| SB-034 | `selection-list` | `storybook-page-selection-list` | page別描画あり | 完了 | 完了 | list selection |
| SB-035 | `slide-control` | `storybook-page-slide-control` | page別描画あり | 完了 | 完了 | numeric option |
| SB-036 | `color-swatch` | `storybook-page-color-swatch` | page別描画あり | 完了 | 完了 | color option |
| SB-037 | `color-picker-rgba` | `storybook-page-color-picker-rgba` | page別描画あり | 完了 | 完了 | color editing |
| SB-038 | `menu` | `storybook-page-menu` | page別描画あり | 完了 | 完了 | menu foundation |
| SB-039 | `context-menu` | `storybook-page-context-menu` | page別描画あり | 完了 | 完了 | placement / keyboard / submenu |
| SB-040 | `menu-button` | `storybook-page-menu-button` | page別描画あり | 完了 | 完了 | trigger + menu |
| SB-041 | `tooltip` | `storybook-page-tooltip` | page別描画あり | 完了 | 完了 | lightweight overlay |
| SB-042 | `popover` | `storybook-page-popover` | page別描画あり | 完了 | 完了 | anchored overlay |
| SB-043 | `hover-card` | `storybook-page-hover-card` | page別描画あり | 完了 | 完了 | rich hover overlay |
| SB-044 | `modal` | `storybook-page-modal` | page別描画あり | 完了 | 完了 | native/modal flow |
| SB-045 | `modal-overlay` | `storybook-page-modal-overlay` | page別描画あり | 完了 | 完了 | overlay modal flow |
| SB-046 | `notification-toast` | `storybook-page-notification-toast` | page別描画あり | 完了 | 完了 | transient feedback |
| SB-047 | `toast-stack-manager` | `storybook-page-toast-stack-manager` | page別描画あり | 完了 | 完了 | multi toast feedback |
| SB-048 | `banner` | `storybook-page-banner` | page別描画あり | 完了 | 完了 | inline feedback |
| SB-049 | `tabs` | `storybook-page-tabs` | page別描画あり | 完了 | 完了 | preset / navigation 基盤 |
| SB-050 | `toolbar` | `storybook-page-toolbar` | page別描画あり | 完了 | 完了 | action rail |
| SB-051 | `breadcrumb` | `storybook-page-breadcrumb` | page別描画あり | 完了 | 完了 | path navigation |
| SB-052 | `side-menu` | `storybook-page-side-menu` | page別描画あり | 完了 | 完了 | side navigation |
| SB-053 | `closeable-tab-strip` | `storybook-page-closeable-tab-strip` | page別描画あり | 完了 | 完了 | document/tab navigation |
| SB-054 | `collapsible-panel` | `storybook-page-collapsible-panel` | page別描画あり | 完了 | 完了 | side panel / shell composition |
| SB-055 | `card` | `storybook-page-card` | page別描画あり | 完了 | 完了 | surface container |
| SB-056 | `list` | `storybook-page-list` | page別描画あり | 完了 | 完了 | collection foundation |
| SB-057 | `tree-view` | `storybook-page-tree-view` | page別描画あり | 完了 | 完了 | Storybook navigation と structured data |
| SB-058 | `virtualization` | `storybook-page-virtualization` | page別描画あり | 完了 | 完了 | list / tree / command scaling |
| SB-059 | `chip` | `storybook-page-chip` | page別描画あり | 完了 | 完了 | tag / filter atom |
| SB-060 | `attachment-chip` | `storybook-page-attachment-chip` | page別描画あり | 完了 | 完了 | attachment surface |
| SB-061 | `chip-group` | `storybook-page-chip-group` | page別描画あり | 完了 | 完了 | chip collection |
| SB-062 | `empty-state` | `storybook-page-empty-state` | page別描画あり | 完了 | 完了 | zero-result state |
| SB-063 | `diagnostics-list` | `storybook-page-diagnostics-list` | page別描画あり | 完了 | 完了 | structured issue list |
| SB-064 | `settings-list` | `storybook-page-settings-list` | page別描画あり | 完了 | 完了 | settings form |
| SB-065 | `search-box` | `storybook-page-search-box` | page別描画あり | 完了 | 完了 | search input |
| SB-066 | `search-control-strip` | `storybook-page-search-control-strip` | page別描画あり | 完了 | 完了 | search controls |
| SB-067 | `command-palette` | `storybook-page-command-palette` | page別描画あり | 完了 | 完了 | command launcher |
| SB-068 | `dynamic-array-editor` | `storybook-page-dynamic-array-editor` | page別描画あり | 完了 | 完了 | editable collection |
| SB-069 | `status-bar` | `storybook-page-status-bar` | page別描画あり | 完了 | 完了 | app status surface |
| SB-070 | `shortcut-combo` | `storybook-page-shortcut-combo` | page別描画あり | 完了 | 完了 | shortcut atom |
| SB-071 | `shortcut-cheatsheet` | `storybook-page-shortcut-cheatsheet` | page別描画あり | 完了 | 完了 | shortcut collection |
| SB-072 | `drag-and-drop` | `storybook-page-drag-and-drop` | page別描画あり | 完了 | 完了 | drag interaction |
| SB-073 | `skeleton-cluster` | `storybook-page-skeleton-cluster` | page別描画あり | 完了 | 完了 | loading collection |
| SB-074 | `window-control-button-group` | `storybook-page-window-control-button-group` | page別描画あり | 完了 | 完了 | window intent |
| SB-075 | `startup-state-panel` | `storybook-page-startup-state-panel` | page別描画あり | 完了 | 完了 | startup composition |
| SB-076 | `accordion` | `storybook-page-accordion` | page別描画あり | 完了 | 完了 | disclosure pattern |
| SB-077 | `code-diff` | `storybook-page-code-diff` | page別描画あり | 完了 | 完了 | complex structured renderer |
