# Storybook Consumer Contract Readiness Audit

実施日: 2026-05-23  
対象: `crates/katana-ui-core-storybook/src/requirements.rs` の required 77 pages

## 集計

- ready: 6
- partial: 32
- not-ready: 39

## 判定基準

- ready: `public API / typed props / typed state / typed action / typed event / callback log / hit target or rendering contract` が自動テスト/guard と紐づく。
- partial: dedicated surface/preset/spec はあるが、consumer contract の証跡（とくに hit target / rendering contract か typed action-event-state の連結）が不足。
- not-ready: required page が generic renderer 依存（`scripts/storybook_reflection_audit.py --strict` の `missing-dedicated-surface`）。

## Page 別分類

| page | status | evidence | missing |
| --- | --- | --- | --- |
| text | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state の consumer 証跡、hit target or rendering contract の page 固有検査 |
| icon | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state の consumer 証跡、hit target or rendering contract の page 固有検査 |
| chip | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| button | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と hit target/rendering contract の page 固有 ready 証跡 |
| text-button | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と hit target/rendering contract の page 固有 ready 証跡 |
| svg-button | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と hit target/rendering contract の page 固有 ready 証跡 |
| icon-text-button | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と hit target/rendering contract の page 固有 ready 証跡 |
| text-input | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | typed callback log/action-event-state の page 固有 ready 証跡、hit target/rendering contract |
| text-area | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | typed callback log/action-event-state の page 固有 ready 証跡、hit target/rendering contract |
| checkbox | ready | callback log/state/action/event/hit target: `crates/katana-ui-core-storybook/src/catalog/tests/atom_and_basic_contracts.rs`, `crates/katana-ui-core-storybook/src/visual/window_interaction/tests/checkbox_contract_tests.rs` | - |
| radio | ready | callback log/state/action/event/hit target: `crates/katana-ui-core-storybook/src/catalog/tests/atom_and_basic_contracts.rs`, `crates/katana-ui-core-storybook/src/visual/window_interaction/tests/radio_contract_tests.rs` | - |
| badge | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と hit target/rendering contract の page 固有 ready 証跡 |
| divider | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| spacer | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| key-cap | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と hit target/rendering contract の page 固有 ready 証跡 |
| skeleton | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| loading-dots | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と rendering contract の page 固有 ready 証跡 |
| spinner | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と rendering contract の page 固有 ready 証跡 |
| progress-bar | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と rendering contract の page 固有 ready 証跡 |
| color-swatch | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と rendering contract の page 固有 ready 証跡 |
| toggle | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と hit target/rendering contract の page 固有 ready 証跡 |
| slide-control | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| card | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | action-event-state の page 固有契約、rendering contract の page 固有検査 |
| list | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| menu | partial | `crates/katana-ui-core-storybook/src/catalog/tests/atom_and_basic_contracts.rs` で存在確認あり | action-event-state と hit target/rendering contract の page 固有 ready 証跡 |
| context-menu | partial | `crates/katana-ui-core-storybook/tests/context_menu_story_contract.rs` で preset/visual ケースあり | callback log/action-event-state と hit target 契約の page 固有 ready 証跡 |
| banner | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| toast-stack-manager | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| tooltip | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と rendering contract の page 固有 ready 証跡 |
| modal | partial | `crates/katana-ui-core-storybook/tests/molecule_story_interaction_contract.rs` で相互作用検査あり | hit target 契約と typed state/action/event の page 固有 ready 証跡 |
| tabs | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| toolbar | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| form-field | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| breadcrumb | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| accordion | partial | `crates/katana-ui-core-storybook/src/catalog/tests/popover_and_accordion_contracts.rs` で catalog 契約検査あり | hit target/rendering contract と typed callback ready 証跡 |
| code-diff | partial | `crates/katana-ui-core-storybook/tests/layout_story_interaction_contract.rs` で story 検査あり | typed callback log/action-event-state と hit target/rendering contract |
| color-picker-rgba | partial | `crates/katana-ui-core-storybook/src/catalog/color_picker_story_tests.rs` で story 契約検査あり | hit target/rendering contract と typed action-event-state ready 証跡 |
| combo-box | ready | callback log/state/action/event/hit target/rendering contract: `crates/katana-ui-core-storybook/src/catalog/tests/atom_and_basic_contracts.rs`, `crates/katana-ui-core-storybook/src/visual/window_interaction/tests/selection_control_tests.rs` | - |
| command-palette | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| dynamic-array-editor | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| menu-button | partial | `scripts/assert-menu-button-contract.sh` と `crates/katana-ui-core-storybook/tests/molecule_story_interaction_contract.rs` で契約検査あり | hit target/rendering contract と typed callback ready 証跡 |
| modal-overlay | partial | `scripts/assert-overlay-lifecycle.sh` と `crates/katana-ui-core-storybook/tests/molecule_story_interaction_contract.rs` で契約検査あり | hit target/rendering contract と typed callback ready 証跡 |
| notification-toast | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| popover | partial | `crates/katana-ui-core-storybook/src/catalog/tests/popover_and_accordion_contracts.rs` で契約検査あり | hit target/rendering contract と typed callback ready 証跡 |
| hover-card | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| search-box | ready | callback log/state/action/event/hit target/rendering contract: `crates/katana-ui-core-storybook/src/catalog/tests/atom_and_basic_contracts.rs`, `crates/katana-ui-core-storybook/src/visual/window_interaction/tests/selection_control_tests.rs` | - |
| search-control-strip | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| segmented-toggle | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と hit target/rendering contract の page 固有 ready 証跡 |
| select-box | ready | callback log/state/action/event/hit target/rendering contract: `crates/katana-ui-core-storybook/src/catalog/tests/atom_and_basic_contracts.rs`, `crates/katana-ui-core-storybook/src/visual/window_interaction/tests/selection_control_tests.rs` | - |
| selection-list | ready | callback log/state/action/event/hit target/rendering contract: `crates/katana-ui-core-storybook/src/catalog/tests/atom_and_basic_contracts.rs`, `crates/katana-ui-core-storybook/src/visual/window_interaction/tests/selection_control_tests.rs` | - |
| side-menu | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | callback log/action-event-state と hit target/rendering contract の page 固有 ready 証跡 |
| status-bar | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| shortcut-combo | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| shortcut-cheatsheet | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| settings-list | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| collapsible-panel | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| virtualization | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| skeleton-cluster | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| motion | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| window-control-button-group | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| startup-state-panel | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| attachment-chip | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| chip-group | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| diagnostics-list | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| empty-state | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| tree-view | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | P0 対象外。hit target/rendering contract と typed callback ready 証跡の再固定が未完了 |
| drag-and-drop | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| closeable-tab-strip | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| panel | partial | `crates/katana-ui-core-storybook/src/panel/panel_verify.rs` で panel 契約検査あり | callback log/action-event-state と hit target 契約の ready 証跡 |
| row | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| column | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| stack | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| grid | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| scroll-area | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| split-pane | partial | `crates/katana-ui-core-storybook/tests/layout_story_interaction_contract.rs` で相互作用検査あり | callback log/action-event-state と hit target/rendering contract の ready 証跡 |
| align-center | not-ready | `scripts/storybook_reflection_audit.py --strict` で `missing-dedicated-surface` | dedicated surface 未実装 |
| theme-tokens | partial | `crates/katana-ui-core-storybook/src/visual/interaction_spec_runtime.rs` で typed spec は定義済み | rendering contract の page 固有 ready 証跡 |
