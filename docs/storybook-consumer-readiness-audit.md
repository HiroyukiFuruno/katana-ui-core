# Storybook Consumer Contract Readiness Audit

実施日: 2026-05-25
対象: `requirements.rs` と Storybook menu に存在する 77 page

## 結論

ready / partial / not-ready の旧分類は、旧 01〜24 や umbrella change 単位の混在を含むため、この文書では完了根拠にしない。
以後は Storybook menu page ごとの `storybook-page-<menu-page>` leaf change を完了判定単位にする。

## 集計

- Storybook menu page: 77
- leaf change 作成済み: 77
- `draw_page` page 別描画あり: 77
- `draw_page` page 別描画未作成: 0
- leaf change DoD 完了: 77

## 判定基準

leaf change は次を満たすまで ready にしない。

- `requirements.rs` と Storybook menu に page が存在する。
- `visual/dedicated.rs::draw_page` から page 専用描画へ到達する。
- `catalog/preset_labels.rs` に page 固有の 4 つ以上の preset がある。
- `visual/storybook_ui_option_contract.rs` に page 固有の 4 つ以上の option contract がある。
- Inspector が option contract を表示し、代表 option が state / action / event / preview 差分へ反映される。
- 操作可能な UI は `window_interaction` 経由の click / wheel / drag テストを持つ。
- light / dark theme の描画差分を数値化された rendering contract で固定する。

## Page 別 leaf change

| group | page | leaf change | 入力元 | page別描画 | 残作業 |
| --- | --- | --- | --- | --- | --- |
| Atoms | `text` | `storybook-page-text` | archive/2026-05-12-02-text-primitive | page別描画あり | harness DoD 完了 |
| Atoms | `icon` | `storybook-page-icon` | archive/2026-05-12-03-icon-primitive | page別描画あり | harness DoD 完了 |
| Atoms | `chip` | `storybook-page-chip` | 07-add-chip-and-attachment-chip | page別描画あり | harness DoD 完了 |
| Atoms | `button` | `storybook-page-button` | archive/2026-05-25-ui-core-interaction-visual-parity | page別描画あり | harness DoD 完了 |
| Atoms | `text-button` | `storybook-page-text-button` | archive/2026-05-12-06-text-button | page別描画あり | harness DoD 完了 |
| Atoms | `svg-button` | `storybook-page-svg-button` | archive/2026-05-12-05-svg-button | page別描画あり | harness DoD 完了 |
| Atoms | `icon-text-button` | `storybook-page-icon-text-button` | archive/2026-05-12-07-icon-text-button | page別描画あり | harness DoD 完了 |
| Atoms | `text-input` | `storybook-page-text-input` | archive/2026-05-12-12-text-input | page別描画あり | harness DoD 完了 |
| Atoms | `text-area` | `storybook-page-text-area` | 06-add-multiline-text-input | page別描画あり | harness DoD 完了 |
| Atoms | `checkbox` | `storybook-page-checkbox` | archive/2026-05-25-ui-core-interaction-visual-parity | page別描画あり | harness DoD 完了 |
| Atoms | `radio` | `storybook-page-radio` | archive/2026-05-25-ui-core-interaction-visual-parity | page別描画あり | harness DoD 完了 |
| Atoms | `badge` | `storybook-page-badge` | archive/2026-05-12-15-badge | page別描画あり | harness DoD 完了 |
| Atoms | `divider` | `storybook-page-divider` | establish-kuc-atoms-molecules-catalog | page別描画あり | harness DoD 完了 |
| Atoms | `spacer` | `storybook-page-spacer` | establish-kuc-atoms-molecules-catalog | page別描画あり | harness DoD 完了 |
| Atoms | `key-cap` | `storybook-page-key-cap` | archive/2026-05-12-16-key-cap | page別描画あり | harness DoD 完了 |
| Atoms | `skeleton` | `storybook-page-skeleton` | 17-add-skeleton-loader | page別描画あり | harness DoD 完了 |
| Atoms | `loading-dots` | `storybook-page-loading-dots` | archive/2026-05-12-04-spinner-primitive | page別描画あり | harness DoD 完了 |
| Atoms | `spinner` | `storybook-page-spinner` | archive/2026-05-12-04-spinner-primitive | page別描画あり | harness DoD 完了 |
| Atoms | `progress-bar` | `storybook-page-progress-bar` | archive/2026-05-12-04-spinner-primitive / 12-add-multi-segment-status-bar | page別描画あり | harness DoD 完了 |
| Atoms | `color-swatch` | `storybook-page-color-swatch` | archive/2026-05-12-11-color-swatch | page別描画あり | harness DoD 完了 |
| Atoms | `toggle` | `storybook-page-toggle` | archive/2026-05-12-08-toggle | page別描画あり | harness DoD 完了 |
| Atoms | `slide-control` | `storybook-page-slide-control` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり | harness DoD 完了 |
| Data | `card` | `storybook-page-card` | archive/2026-05-12-17-card | page別描画あり | harness DoD 完了 |
| Data | `list` | `storybook-page-list` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり | harness DoD 完了 |
| Data | `tree-view` | `storybook-page-tree-view` | 16-add-virtualized-list-and-tree / archive/2026-05-25-katana-widget-parity-backlog | page別描画あり | harness DoD 完了 |
| Data | `diagnostics-list` | `storybook-page-diagnostics-list` | 08-add-diagnostics-list | page別描画あり | harness DoD 完了 |
| Data | `empty-state` | `storybook-page-empty-state` | 09-add-empty-state | page別描画あり | harness DoD 完了 |
| Data | `settings-list` | `storybook-page-settings-list` | 14-add-sectioned-settings-form | page別描画あり | harness DoD 完了 |
| Data | `attachment-chip` | `storybook-page-attachment-chip` | 07-add-chip-and-attachment-chip | page別描画あり | harness DoD 完了 |
| Data | `chip-group` | `storybook-page-chip-group` | 07-add-chip-and-attachment-chip | page別描画あり | harness DoD 完了 |
| Data | `code-diff` | `storybook-page-code-diff` | archive/2026-05-25-24-code-diff | page別描画あり | harness DoD 完了 |
| Feedback | `context-menu` | `storybook-page-context-menu` | 01-add-context-menu | page別描画あり | harness DoD 完了 |
| Feedback | `tooltip` | `storybook-page-tooltip` | archive/2026-05-12-14-tooltip | page別描画あり | harness DoD 完了 |
| Feedback | `modal` | `storybook-page-modal` | archive/2026-05-12-20-modal-overlay | page別描画あり | harness DoD 完了 |
| Feedback | `popover` | `storybook-page-popover` | archive/2026-05-12-21-popover / 04-add-rich-popover-and-hover-card | page別描画あり | harness DoD 完了 |
| Feedback | `modal-overlay` | `storybook-page-modal-overlay` | archive/2026-05-12-20-modal-overlay | page別描画あり | harness DoD 完了 |
| Feedback | `notification-toast` | `storybook-page-notification-toast` | 11-add-toast-stack-manager | page別描画あり | harness DoD 完了 |
| Feedback | `hover-card` | `storybook-page-hover-card` | 04-add-rich-popover-and-hover-card | page別描画あり | harness DoD 完了 |
| Feedback | `toast-stack-manager` | `storybook-page-toast-stack-manager` | 11-add-toast-stack-manager | page別描画あり | harness DoD 完了 |
| Feedback | `banner` | `storybook-page-banner` | 10-add-inline-banner-alert | page別描画あり | harness DoD 完了 |
| Feedback | `collapsible-panel` | `storybook-page-collapsible-panel` | 15-add-collapsible-sidebar-shell | page別描画あり | harness DoD 完了 |
| Forms | `form-field` | `storybook-page-form-field` | establish-kuc-atoms-molecules-catalog | page別描画あり | harness DoD 完了 |
| Forms | `search-box` | `storybook-page-search-box` | archive/2026-05-12-13-search-box | page別描画あり | harness DoD 完了 |
| Forms | `segmented-toggle` | `storybook-page-segmented-toggle` | archive/2026-05-12-09-segmented-toggle | page別描画あり | harness DoD 完了 |
| Forms | `select-box` | `storybook-page-select-box` | archive/2026-05-12-10-select-box | page別描画あり | harness DoD 完了 |
| Forms | `combo-box` | `storybook-page-combo-box` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり | harness DoD 完了 |
| Forms | `menu-button` | `storybook-page-menu-button` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり | harness DoD 完了 |
| Forms | `dynamic-array-editor` | `storybook-page-dynamic-array-editor` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり | harness DoD 完了 |
| Forms | `selection-list` | `storybook-page-selection-list` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり | harness DoD 完了 |
| Forms | `closeable-tab-strip` | `storybook-page-closeable-tab-strip` | 03-add-workspace-tab-bar | page別描画あり | harness DoD 完了 |
| Forms | `search-control-strip` | `storybook-page-search-control-strip` | 22-add-search-control-strip | page別描画あり | harness DoD 完了 |
| Foundation | `panel` | `storybook-page-panel` | ui-core-root-plan / establish-kuc-atoms-molecules-catalog | page別描画あり | harness DoD 完了 |
| Foundation | `theme-tokens` | `storybook-page-theme-tokens` | archive/2026-05-12-01-theme-tokens | page別描画あり | harness DoD 完了 |
| Layout | `row` | `storybook-page-row` | ui-core-root-plan | page別描画あり | harness DoD 完了 |
| Layout | `column` | `storybook-page-column` | ui-core-root-plan | page別描画あり | harness DoD 完了 |
| Layout | `stack` | `storybook-page-stack` | ui-core-root-plan | page別描画あり | harness DoD 完了 |
| Layout | `grid` | `storybook-page-grid` | ui-core-root-plan | page別描画あり | harness DoD 完了 |
| Layout | `scroll-area` | `storybook-page-scroll-area` | 00-add-scroll-area-contract | page別描画あり | harness DoD 完了 |
| Layout | `split-pane` | `storybook-page-split-pane` | 00-add-split-pane-contract / archive/2026-05-12-19-split-pane | page別描画あり | harness DoD 完了 |
| Layout | `align-center` | `storybook-page-align-center` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり | harness DoD 完了 |
| Navigation | `menu` | `storybook-page-menu` | ui-core-root-plan / establish-kuc-atoms-molecules-catalog | page別描画あり | harness DoD 完了 |
| Navigation | `command-palette` | `storybook-page-command-palette` | 21-add-command-launcher-results | page別描画あり | harness DoD 完了 |
| Navigation | `breadcrumb` | `storybook-page-breadcrumb` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり | harness DoD 完了 |
| Navigation | `side-menu` | `storybook-page-side-menu` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり | harness DoD 完了 |
| Navigation | `tabs` | `storybook-page-tabs` | archive/2026-05-25-katana-widget-parity-backlog | page別描画あり | harness DoD 完了 |
| Navigation | `status-bar` | `storybook-page-status-bar` | 12-add-multi-segment-status-bar | page別描画あり | harness DoD 完了 |
| Navigation | `toolbar` | `storybook-page-toolbar` | 05-add-toolbar-overflow | page別描画あり | harness DoD 完了 |
| Runtime | `virtualization` | `storybook-page-virtualization` | 16-add-virtualized-list-and-tree | page別描画あり | harness DoD 完了 |
| Runtime | `motion` | `storybook-page-motion` | 18-add-animation-primitives | page別描画あり | harness DoD 完了 |
| Runtime | `shortcut-combo` | `storybook-page-shortcut-combo` | 13-add-shortcut-combo-display | page別描画あり | harness DoD 完了 |
| Runtime | `shortcut-cheatsheet` | `storybook-page-shortcut-cheatsheet` | 13-add-shortcut-combo-display | page別描画あり | harness DoD 完了 |
| Runtime | `drag-and-drop` | `storybook-page-drag-and-drop` | 02-add-drag-drop-primitive | page別描画あり | harness DoD 完了 |
| Runtime | `window-control-button-group` | `storybook-page-window-control-button-group` | 19-add-title-bar-window-chrome | page別描画あり | harness DoD 完了 |
| Runtime | `skeleton-cluster` | `storybook-page-skeleton-cluster` | 17-add-skeleton-loader | page別描画あり | harness DoD 完了 |
| Runtime | `color-picker-rgba` | `storybook-page-color-picker-rgba` | archive/2026-05-25-23-color-picker-complete-parity / archive/2026-05-12-22-rgba-color-picker | page別描画あり | harness DoD 完了 |
| Runtime | `accordion` | `storybook-page-accordion` | archive/2026-05-25-18-accordion | page別描画あり | harness DoD 完了 |
| Runtime | `startup-state-panel` | `storybook-page-startup-state-panel` | 20-add-splash-screen-template | page別描画あり | harness DoD 完了 |

## 同期先

- 親 task: `openspec/changes/establish-kuc-atoms-molecules-catalog/tasks.md` 6.1〜6.8
- 分割表: `openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-change-split.md`
- guard: `scripts/assert-storybook-ui-harness.py`
