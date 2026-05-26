# Storybook menu change split

作成日: 2026-05-25

## 結論

実装と完了判定の単位は、旧 01〜24 ではなく Storybook menu page とする。
`requirements.rs` と `catalog/story_paths_*.rs` に存在する 77 page を正本にし、各 page に 1 つの leaf change を割り当てる。

既存の `NN-add-*`、archive 済み 01〜24、`archive/2026-05-25-katana-widget-parity-backlog`、`archive/2026-05-25-ui-core-interaction-visual-parity` は入力元として読む。
ただし、それらをそのまま完了判定単位にはしない。

## 分割ルール

- leaf change 名は `storybook-page-<menu-page>` を基本形にする。
- 既存 change が複数 menu page を束ねている場合、その change は umbrella / 入力元に下げる。
- 依存が強い UI でも、Storybook harness の完了判定は leaf change ごとに行う。
- leaf change は必ず `theme / preset / Inspector option / state-action-event / 操作テスト / 数値化された rendering contract` を持つ。
- `draw_page` の page 別描画があるだけでは ready にしない。

## 棚卸し集計

- Storybook menu page: 77
- `requirements.rs` required page: 77
- menu に存在し required に存在しない page: 0
- required に存在し menu に存在しない page: 0
- `draw_page` page 別描画あり: 49
- `draw_page` page 別描画未作成: 28

## Leaf change map

| group | menu page | leaf change | 既存入力元 | 現状 |
| --- | --- | --- | --- | --- |
| Atoms | `text` | `storybook-page-text` | archive/2026-05-12-02-text-primitive | page別描画あり |
| Atoms | `icon` | `storybook-page-icon` | archive/2026-05-12-03-icon-primitive | page別描画あり |
| Atoms | `chip` | `storybook-page-chip` | 07-add-chip-and-attachment-chip | page別描画あり |
| Atoms | `button` | `storybook-page-button` | archive/2026-05-25-ui-core-interaction-visual-parity | page別描画あり |
| Atoms | `text-button` | `storybook-page-text-button` | archive/2026-05-12-06-text-button | page別描画あり |
| Atoms | `svg-button` | `storybook-page-svg-button` | archive/2026-05-12-05-svg-button | page別描画あり |
| Atoms | `icon-text-button` | `storybook-page-icon-text-button` | archive/2026-05-12-07-icon-text-button | page別描画あり |
| Atoms | `text-input` | `storybook-page-text-input` | archive/2026-05-12-12-text-input | page別描画あり |
| Atoms | `text-area` | `storybook-page-text-area` | 06-add-multiline-text-input | page別描画あり |
| Atoms | `checkbox` | `storybook-page-checkbox` | archive/2026-05-25-ui-core-interaction-visual-parity | page別描画あり |
| Atoms | `radio` | `storybook-page-radio` | archive/2026-05-25-ui-core-interaction-visual-parity | page別描画あり |
| Atoms | `badge` | `storybook-page-badge` | archive/2026-05-12-15-badge | page別描画あり |
| Atoms | `divider` | `storybook-page-divider` | establish-kuc-atoms-molecules-catalog | page別描画あり |
| Atoms | `spacer` | `storybook-page-spacer` | establish-kuc-atoms-molecules-catalog | page別描画あり |
| Atoms | `key-cap` | `storybook-page-key-cap` | archive/2026-05-12-16-key-cap | page別描画あり |
| Atoms | `skeleton` | `storybook-page-skeleton` | 17-add-skeleton-loader | page別描画あり |
| Atoms | `loading-dots` | `storybook-page-loading-dots` | archive/2026-05-12-04-spinner-primitive | page別描画あり |
| Atoms | `spinner` | `storybook-page-spinner` | archive/2026-05-12-04-spinner-primitive | page別描画あり |
| Atoms | `progress-bar` | `storybook-page-progress-bar` | archive/2026-05-12-04-spinner-primitive / 12-add-multi-segment-status-bar | page別描画あり |
| Atoms | `color-swatch` | `storybook-page-color-swatch` | archive/2026-05-12-11-color-swatch | page別描画あり |
| Atoms | `toggle` | `storybook-page-toggle` | archive/2026-05-12-08-toggle | page別描画あり |
| Atoms | `slide-control` | `storybook-page-slide-control` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり |
| Data | `card` | `storybook-page-card` | archive/2026-05-12-17-card | page別描画あり |
| Data | `list` | `storybook-page-list` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり |
| Data | `tree-view` | `storybook-page-tree-view` | 16-add-virtualized-list-and-tree / archive/2026-05-25-katana-widget-parity-backlog | page別描画あり |
| Data | `diagnostics-list` | `storybook-page-diagnostics-list` | 08-add-diagnostics-list | page別描画あり |
| Data | `empty-state` | `storybook-page-empty-state` | 09-add-empty-state | page別描画あり |
| Data | `settings-list` | `storybook-page-settings-list` | 14-add-sectioned-settings-form | page別描画あり |
| Data | `attachment-chip` | `storybook-page-attachment-chip` | 07-add-chip-and-attachment-chip | page別描画あり |
| Data | `chip-group` | `storybook-page-chip-group` | 07-add-chip-and-attachment-chip | page別描画あり |
| Data | `code-diff` | `storybook-page-code-diff` | archive/2026-05-25-24-code-diff | page別描画あり |
| Feedback | `context-menu` | `storybook-page-context-menu` | 01-add-context-menu | page別描画あり |
| Feedback | `tooltip` | `storybook-page-tooltip` | archive/2026-05-12-14-tooltip | page別描画あり |
| Feedback | `modal` | `storybook-page-modal` | archive/2026-05-12-20-modal-overlay | page別描画あり |
| Feedback | `popover` | `storybook-page-popover` | archive/2026-05-12-21-popover / 04-add-rich-popover-and-hover-card | page別描画あり |
| Feedback | `modal-overlay` | `storybook-page-modal-overlay` | archive/2026-05-12-20-modal-overlay | page別描画あり |
| Feedback | `notification-toast` | `storybook-page-notification-toast` | 11-add-toast-stack-manager | page別描画あり |
| Feedback | `hover-card` | `storybook-page-hover-card` | 04-add-rich-popover-and-hover-card | page別描画あり |
| Feedback | `toast-stack-manager` | `storybook-page-toast-stack-manager` | 11-add-toast-stack-manager | page別描画あり |
| Feedback | `banner` | `storybook-page-banner` | 10-add-inline-banner-alert | page別描画あり |
| Feedback | `collapsible-panel` | `storybook-page-collapsible-panel` | 15-add-collapsible-sidebar-shell | page別描画あり |
| Forms | `form-field` | `storybook-page-form-field` | establish-kuc-atoms-molecules-catalog | page別描画あり |
| Forms | `search-box` | `storybook-page-search-box` | archive/2026-05-12-13-search-box | page別描画あり |
| Forms | `segmented-toggle` | `storybook-page-segmented-toggle` | archive/2026-05-12-09-segmented-toggle | page別描画あり |
| Forms | `select-box` | `storybook-page-select-box` | archive/2026-05-12-10-select-box | page別描画あり |
| Forms | `combo-box` | `storybook-page-combo-box` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり |
| Forms | `menu-button` | `storybook-page-menu-button` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり |
| Forms | `dynamic-array-editor` | `storybook-page-dynamic-array-editor` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり |
| Forms | `selection-list` | `storybook-page-selection-list` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり |
| Forms | `closeable-tab-strip` | `storybook-page-closeable-tab-strip` | 03-add-workspace-tab-bar | page別描画あり |
| Forms | `search-control-strip` | `storybook-page-search-control-strip` | 22-add-search-control-strip | page別描画あり |
| Foundation | `panel` | `storybook-page-panel` | ui-core-root-plan / establish-kuc-atoms-molecules-catalog | page別描画あり |
| Foundation | `theme-tokens` | `storybook-page-theme-tokens` | archive/2026-05-12-01-theme-tokens | page別描画あり |
| Layout | `row` | `storybook-page-row` | ui-core-root-plan | page別描画あり |
| Layout | `column` | `storybook-page-column` | ui-core-root-plan | page別描画あり |
| Layout | `stack` | `storybook-page-stack` | ui-core-root-plan | page別描画あり |
| Layout | `grid` | `storybook-page-grid` | ui-core-root-plan | page別描画あり |
| Layout | `scroll-area` | `storybook-page-scroll-area` | 00-add-scroll-area-contract | page別描画あり |
| Layout | `split-pane` | `storybook-page-split-pane` | 00-add-split-pane-contract / archive/2026-05-12-19-split-pane | page別描画あり |
| Layout | `align-center` | `storybook-page-align-center` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり |
| Navigation | `menu` | `storybook-page-menu` | ui-core-root-plan / establish-kuc-atoms-molecules-catalog | page別描画あり |
| Navigation | `command-palette` | `storybook-page-command-palette` | 21-add-command-launcher-results | page別描画あり |
| Navigation | `breadcrumb` | `storybook-page-breadcrumb` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり |
| Navigation | `side-menu` | `storybook-page-side-menu` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり |
| Navigation | `tabs` | `storybook-page-tabs` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり |
| Navigation | `status-bar` | `storybook-page-status-bar` | 12-add-multi-segment-status-bar | page別描画あり |
| Navigation | `toolbar` | `storybook-page-toolbar` | 05-add-toolbar-overflow | page別描画あり |
| Runtime | `virtualization` | `storybook-page-virtualization` | 16-add-virtualized-list-and-tree | page別描画あり |
| Runtime | `motion` | `storybook-page-motion` | 18-add-animation-primitives | page別描画あり |
| Runtime | `shortcut-combo` | `storybook-page-shortcut-combo` | 13-add-shortcut-combo-display | page別描画あり |
| Runtime | `shortcut-cheatsheet` | `storybook-page-shortcut-cheatsheet` | 13-add-shortcut-combo-display | page別描画あり |
| Runtime | `drag-and-drop` | `storybook-page-drag-and-drop` | 02-add-drag-drop-primitive | page別描画あり |
| Runtime | `window-control-button-group` | `storybook-page-window-control-button-group` | 19-add-title-bar-window-chrome | page別描画あり |
| Runtime | `skeleton-cluster` | `storybook-page-skeleton-cluster` | 17-add-skeleton-loader | page別描画あり |
| Runtime | `color-picker-rgba` | `storybook-page-color-picker-rgba` | archive/2026-05-25-23-color-picker-complete-parity / archive/2026-05-12-22-rgba-color-picker | page別描画あり |
| Runtime | `accordion` | `storybook-page-accordion` | archive/2026-05-25-18-accordion | page別描画あり |
| Runtime | `startup-state-panel` | `storybook-page-startup-state-panel` | 20-add-splash-screen-template | page別描画あり |

## 次の作業

- 77 leaf change の実体を作る前に、`scripts/assert-storybook-ui-harness.py` がこの表の全 page を検査対象にする。
- 既存 `NN-add-*` と archive 済み入力元は page 別 leaf change へ分割済みの参照専用にする。
- `docs/storybook-consumer-readiness-audit.md` の status は、この表の leaf change 単位に再生成する。
