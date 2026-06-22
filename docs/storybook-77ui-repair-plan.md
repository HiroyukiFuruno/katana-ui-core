# Storybook 77 UI Repair Plan

作成日: 2026-06-13

## 結論

既存の `verified` 判定は、この計画の DoD としては無効とする。
77 UI は、低レイヤーから 1 UI ずつ、RED -> 実装修正 -> GREEN -> Storybook ユーザー確認の順で再監査する。
ユーザーが Storybook で触って OK を出すまで、その UI の DoD は満たしていない。

## 固定ルール

1. 同時に複数 UI を完了扱いしない。
2. 1 UI ごとに、最初に依存関係と public API contract を読む。
3. ユーザー指摘は必ず再現性のある自動テストへ落とす。
4. TDD RED を必須にする。先に失敗するテストを作り、失敗ログを確認してから実装する。
5. Storybook 専用 state、Inspector だけの変化、preset label だけの変化、core public API を通らない代替動作は禁止する。
6. smoke / requirement gate / snapshot は補助証跡であり、ユーザー確認の代替にしない。
7. マルチプラットフォーム前提を守る。macOS native window だけで成立する挙動、OS 固有座標、adapter 固有イベントを core contract の代替にしない。
8. egui と同様に、core は platform-neutral な model / props / state / action / event / callback を持ち、adapter はそれを描画・入力変換するだけに寄せる。
9. 2026-06-19 以降、ユーザー Storybook 確認は release-readiness の主 blocker ではなく劣後確認とする。`manual_acceptance_pending` は台帳と final DoD gate には残すが、自動 evidence が揃っている場合は `storybook-interaction-smoke` / `release-readiness-check` を止めない。

## 1 UI ごとの DoD

各 UI は以下をすべて満たすまで `manual_acceptance_pending` とする。

1. 依存レイヤー、core public API、Storybook harness、既存テストを確認済み。
2. public props/options、state、action、event、callback の所有者が明確。
3. 必須操作を UI ごとに明記済み。
   - pointer
   - keyboard
   - scroll
   - drag
   - context menu
   - focus
   - hover
   - resize
4. ユーザー指摘または想定破綻に対する RED test を先に追加し、失敗を確認済み。
5. RED test を最小実装で GREEN にした。
6. 代表値ではなく、複数項目、2個目以降、末尾、スクロール後、disabled/readonly、focus 後 keyboard など該当 edge を確認済み。
7. Storybook 画面でユーザーが手で触り、OK を出した。

## レイヤー順 Queue

`status` は初期状態ではすべて `manual_acceptance_pending`。
`previous_verified_invalidated=true` は、旧台帳や manifest の verified をこの計画の完了根拠にしないことを示す。

| Order | Layer | UI | Depends On | Primary Risk | Status |
| ---: | --- | --- | --- | --- | --- |
| 01 | foundation | theme-tokens | theme model, palette, font metrics | hardcoded color / platform-specific rendering | manual_acceptance_pending |
| 02 | foundation | text | theme-tokens, text renderer | clipping, baseline, font fallback | manual_acceptance_pending |
| 03 | foundation | icon | theme-tokens, SVG source, bounds | external SVG not respected | manual_acceptance_pending |
| 04 | foundation | divider | theme-tokens, layout metrics | separator role and geometry drift | manual_acceptance_pending |
| 05 | foundation | spacer | layout metrics | fake gap instead of layout contract | manual_acceptance_pending |
| 06 | foundation | badge | text, icon, theme-tokens | compact bounds and token mismatch | manual_acceptance_pending |
| 07 | foundation | key-cap | text, theme-tokens | platform key labels and sizing | manual_acceptance_pending |
| 08 | foundation | skeleton | motion policy, theme-tokens | reduced motion and placeholder geometry | manual_acceptance_pending |
| 09 | foundation | loading-dots | motion policy, theme-tokens | animation phase not contract-backed | manual_acceptance_pending |
| 10 | foundation | spinner | motion policy, theme-tokens | reduced motion and segment geometry | manual_acceptance_pending |
| 11 | foundation | progress-bar | progress model, theme-tokens | percent/mode state mismatch | manual_acceptance_pending |
| 12 | foundation | color-swatch | color model, theme-tokens | color contrast and selection ring | manual_acceptance_pending |
| 13 | layout | row | layout engine | axis/gap/alignment only label-driven | manual_acceptance_pending |
| 14 | layout | column | layout engine | axis/gap/alignment only label-driven | manual_acceptance_pending |
| 15 | layout | stack | layout engine | z-order/focus not state-backed | manual_acceptance_pending |
| 16 | layout | grid | layout engine | cell selection and track mismatch | manual_acceptance_pending |
| 17 | layout | align-center | layout engine | center contract not projected | manual_acceptance_pending |
| 18 | layout | panel | layout engine, scroll state | nested scroll state leakage | manual_acceptance_pending |
| 19 | clickable | button | text, theme, action model | press/focus/keyboard mismatch | manual_acceptance_pending |
| 20 | clickable | text-button | button | label-only variant | manual_acceptance_pending |
| 21 | clickable | svg-button | button, icon | SVG/aria options not public-backed | manual_acceptance_pending |
| 22 | clickable | icon-text-button | button, icon, text | slot alignment and action mismatch | manual_acceptance_pending |
| 23 | clickable | chip | text, icon, selection state | selected/dismiss/disabled edge | manual_acceptance_pending |
| 24 | clickable | card | layout, button-like action | interactive=false mutation | manual_acceptance_pending |
| 25 | clickable | shortcut-combo | key-cap, text | platform shortcut mismatch | manual_acceptance_pending |
| 26 | clickable | window-control-button-group | platform command model | OS-specific behavior leaking into core | manual_acceptance_pending |
| 27 | clickable | startup-state-panel | app lifecycle model | retry/cancel not event-backed | manual_acceptance_pending |
| 28 | clickable | attachment-chip | chip, upload status | retry/status transition mismatch | manual_acceptance_pending |
| 29 | clickable | chip-group | chip, collection layout | multi-chip focus/dismiss/reorder | manual_acceptance_pending |
| 30 | clickable | empty-state | text, icon, button action | recovery action not core-backed | manual_acceptance_pending |
| 31 | text-entry | text-input | text renderer, input state | real keyboard input/caret/readonly | manual_acceptance_pending |
| 32 | text-entry | text-area | text-input, scroll state | multiline input/scroll/resize | manual_acceptance_pending |
| 33 | text-entry | search-box | text-input, clear/submit action | typing/filter/clear not live | manual_acceptance_pending |
| 34 | text-entry | search-control-strip | search-box, segmented controls | multi-toggle/query state | manual_acceptance_pending |
| 35 | text-entry | combo-box | text-input, overlay, selection | filter/select/open state | manual_acceptance_pending |
| 36 | text-entry | command-palette | text-input, list, overlay | keyboard navigation/escape | manual_acceptance_pending |
| 37 | binary-choice | checkbox | core checkbox state, row hit targets | multi-check, mark quality, disabled edge | manual_acceptance_pending |
| 38 | binary-choice | radio | core radio state, group selection | exclusivity and keyboard selection | manual_acceptance_pending |
| 39 | binary-choice | toggle | core selection state | disabled preset/body diff | manual_acceptance_pending |
| 40 | binary-choice | segmented-toggle | toggle, selection group | per-segment state/focus | manual_acceptance_pending |
| 41 | selection | list | layout, selection state, scroll | keyboard/scroll retention | manual_acceptance_pending |
| 42 | selection | select-box | overlay, list selection | open/select/disabled edge | manual_acceptance_pending |
| 43 | selection | selection-list | list, multi-row selection | row 2+ and scroll selection | manual_acceptance_pending |
| 44 | selection | side-menu | list, route selection | hover expansion/scroll route | manual_acceptance_pending |
| 45 | selection | breadcrumb | text, selection model | overflow and route selection | manual_acceptance_pending |
| 46 | selection | dynamic-array-editor | list, text entry, actions | add/remove/reorder state | manual_acceptance_pending |
| 47 | selection | shortcut-cheatsheet | list, key-cap, search | scroll/keyboard selection | manual_acceptance_pending |
| 48 | selection | settings-list | list, form controls | field activation and scroll | manual_acceptance_pending |
| 49 | selection | virtualization | list, virtual range | offscreen/scroll range bugs | manual_acceptance_pending |
| 50 | selection | diagnostics-list | list, filters, keyboard nav | filter/keyboard/scroll retention | manual_acceptance_pending |
| 51 | selection | tree-view | tree model, scroll hit target | scroll-after-click reset, deep item selection | manual_acceptance_pending |
| 52 | tabs | tabs | selection, overflow, drag | scroll/drag/context menu | manual_acceptance_pending |
| 53 | tabs | closeable-tab-strip | tabs, close/pin model | close/pin/drag/overflow | manual_acceptance_pending |
| 54 | overlay | menu | overlay, list, keyboard | focus/keyboard/context menu | manual_acceptance_pending |
| 55 | overlay | context-menu | overlay, pointer anchor | right click/submenu/disabled item | manual_acceptance_pending |
| 56 | overlay | menu-button | button, menu | trigger/menu state split | manual_acceptance_pending |
| 57 | overlay | tooltip | hover/focus trigger, overlay placement | delay/open state not public-backed | manual_acceptance_pending |
| 58 | overlay | modal | overlay, focus trap | escape/focus return/native boundary | manual_acceptance_pending |
| 59 | overlay | modal-overlay | modal, backdrop | backdrop close and focus state | manual_acceptance_pending |
| 60 | overlay | notification-toast | toast model | dismiss/hover/timer state | manual_acceptance_pending |
| 61 | overlay | toast-stack-manager | notification-toast, queue | queue overflow/pause/dismiss | manual_acceptance_pending |
| 62 | overlay | popover | overlay placement | trigger/escape/focus retention | manual_acceptance_pending |
| 63 | overlay | hover-card | popover, hover/focus | hover/focus open and inner focus | manual_acceptance_pending |
| 64 | overlay | toolbar | button group, overflow menu | action group/overflow/keyboard | manual_acceptance_pending |
| 65 | overlay | accordion | disclosure, tree mode | multiple/disabled/keyboard | manual_acceptance_pending |
| 66 | overlay | collapsible-panel | panel, disclosure | resize/focus/keyboard toggle | manual_acceptance_pending |
| 67 | feedback | banner | text, actions | dismiss/details/action state | manual_acceptance_pending |
| 68 | feedback | status-bar | segmented action, toolbar | segment hover/select/keyboard | manual_acceptance_pending |
| 69 | feedback | form-field | text-input, validation | label-to-control focus/required | manual_acceptance_pending |
| 70 | feedback | code-diff | text, scroll sync | collapsed/keyboard/scroll sync | manual_acceptance_pending |
| 71 | scroll-drag-resize | slide-control | binary/drag value model | drag/keyboard value update | manual_acceptance_pending |
| 72 | scroll-drag-resize | scroll-area | panel scroll, scrollbar model | component scroll vs global scroll | manual_acceptance_pending |
| 73 | scroll-drag-resize | split-pane | layout, drag handle | ratio clamp/keyboard resize | manual_acceptance_pending |
| 74 | scroll-drag-resize | color-picker-rgba | color-swatch, drag/input | picker drag/value sync | manual_acceptance_pending |
| 75 | scroll-drag-resize | drag-and-drop | drag runtime, scroll area | autoscroll/drop/keyboard drag | manual_acceptance_pending |
| 76 | runtime | motion | motion policy | runtime phase vs UI gesture substitution | manual_acceptance_pending |
| 77 | runtime | skeleton-cluster | skeleton, motion | cluster reduced motion/focus | manual_acceptance_pending |

## 現在の P0 既知不具合

| UI | 指摘 | 必須 RED test |
| --- | --- | --- |
| text-selection-foundation | 表示された文章が選択できず、clipboard copy もできない。全 UI に出る基盤不備。 | Storybook frame が描画 text run の文字列と bounds を保持し、drag selection で選択範囲を作り、Copy で clipboard payload を生成できること。 |
| checkbox | 複数行 checkbox が独立 state を持たない。checked 表示がプロダクト品質ではない。checked preset から state=false になっても mark が残る、または preview/Inspector が checked mark と違う state を出す。disabled preset が見た目だけ disabled で state は idle のままになる。disabled block 後に state label が focused/keyboard false へ落ちる。focus preset が見た目だけ focus で state は idle のままになる。disabled hover が有効 UI のように見える。 | 2行目以降も pointer 連続操作で同時 checked 可能、checked mark が theme token と明確な tick glyph を持つこと。checked preset は core state 初期値として反映され、preview/Inspector も `checked=true` を出し、false へ戻したら mark が消えること。disabled preset は core state 初期値として `disabled=true` を preview/Inspector/live audit metadata に出し、pointer/focus/keyboard/hover のいずれでも有効表示や mutation を出さず、block 後も `disabled=true` を保持すること。focus preset は core state 初期値として `focused=true` を preview/Inspector/live audit metadata に出すこと。 |
| progress-bar | progress bar が live state/action に接続されておらず、動作しないハリボテ。 | preview action / option / timed tick のいずれかで progress state が変わり、rendered fill 幅、state label、event が同期して変化すること。 |
| tree-view | 縦長ツリーを末尾付近までスクロールしてクリックすると上部へ戻る。 | 複数 directory open、末尾付近 scroll、visible row click 後に `panel_scroll.navigation_y` と root scroll が保持され、クリック行が viewport 内に残ること。 |

## 進行ログ形式

各 UI の作業開始時に、この形式で追記する。

```text
## UI: <page>
- Layer:
- Dependencies:
- Existing ledger verdict:
- Existing manifest verdict:
- User reported failures:
- Required operations:
- RED test:
- RED result:
- Fix:
- GREEN result:
- Storybook user confirmation:
- Final status:
```

## UI: tree-view

- Layer: selection
- Dependencies: tree model, navigation panel scroll state, row hit target, root shell scroll state
- Existing ledger verdict: 旧台帳では `実証済み` だが、ユーザー実操作で scroll reset が出たため新 DoD では無効。
- Existing manifest verdict: `verified` は新 DoD では無効。smoke は scroll 後の navigation row click retention を保証していなかった。
- User reported failures: 縦長 TreeView / navigation tree で複数 directory を開き、末尾付近まで scroll してクリックすると上部へ戻る。
- Required operations: pointer, scroll, focus, keyboard, context menu。今回の対象は pointer after scroll。
- RED test:
  - `navigation_scroll_is_retained_when_selecting_visible_row_after_deep_scroll`
  - `navigation_scroll_retained_when_selecting_tree_view_after_deep_scroll`
- RED result: `navigation_y` が `1756 -> 0` に戻る失敗を確認。
- Fix: page/preset/instance 切替で Preview/Inspector scroll は復元するが、Root/Navigation scroll は Storybook shell state として保持する。
- GREEN result:
  - `rtk cargo test -p katana-ui-core-storybook navigation_scroll_is_retained_when_selecting_visible_row_after_deep_scroll --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib navigation_scroll_retained_when_selecting_tree_view_after_deep_scroll --locked`
  - `rtk cargo test -p katana-ui-core-storybook navigation --locked`
  - `rtk cargo fmt --check`
  - `rtk just storybook-check`
- Storybook user confirmation: pending
- Final status: manual_acceptance_pending
- Manual confirmation entrypoint:
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window tree-view`
- Manual confirmation smoke:
  - `rtk just storybook-manual-acceptance-smoke`
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 1 tree-view`

## UI: checkbox

- Layer: binary-choice
- Dependencies: core checkbox state, binary choice row hit target, theme token mark rendering
- Existing ledger verdict: 旧台帳の `実証済み` は新 DoD では無効。ユーザー実操作で checked preset と core state の不一致が出たため、Storybook user confirmation なしに完了扱いしない。
- Existing manifest verdict: `verified` は新 DoD では無効。smoke は checked preset から state=false へ戻した後の mark 消去を保証していなかった。
- User reported failures: checked preset で state は `false` になっているのに、表示の check mark が外れない。
- Required operations: pointer, keyboard, focus, hover, disabled block, multiple row independent checked state。今回の対象は checked preset の pointer toggle と mark rendering。
- RED test:
  - `checkbox_checked_public_prop_reaches_render_node_props`
  - `checkbox_checked_preset_does_not_keep_mark_after_state_turns_false`
  - `checkbox_rows_toggle_independently_and_can_both_be_checked`
  - `checkbox_control_and_row_meet_modern_hit_target_size`
  - `checkbox_rows_controls_and_status_use_rounded_modern_chrome`
  - `checkbox_labels_controls_and_status_use_readable_text_runs`
  - `checkbox_preview_does_not_draw_storybook_runtime_overlay_over_controls`
  - `checkbox_controls_have_bottom_padding_inside_component_frame`
  - `checkbox_clicked_disabled_preset_does_not_mutate_or_render_checked_state`
  - `checkbox_disabled_preset_mutes_control_button_labels`
  - `checkbox_clicked_snapshot_keeps_preview_status_and_inspector_state_consistent`
  - `checkbox_focus_preset_keeps_row_labels_visible`
  - `checkbox_focus_preset_only_draws_focus_ring_on_active_row`
  - `checkbox_inspector_settings_rows_are_not_rendered_as_current_state_values`
  - `checkbox_disabled_preset_blocks_focus_and_keyboard_toggle`
  - `checkbox_disabled_preset_blocks_pointer_toggle_and_preserves_mark`
  - `checkbox_hover_does_not_emit_click_event_or_mutate_checked_state`
  - `checkbox_hover_feedback_tracks_the_actual_row`
  - `checkbox_keyboard_second_toggle_removes_checked_mark_and_state`
  - `checkbox_keyboard_toggle_applies_to_focused_secondary_row_only`
  - `checkbox_control_toggle_and_reset_update_mark_and_state_together`
  - live audit `checkbox_hover_no_click_event`
  - live audit `checkbox_hover_secondary_row`
  - live audit `checkbox_keyboard_toggle_off`
  - live audit `checkbox_keyboard_focused_secondary_row`
  - live audit `checkbox_control_toggle_reset`
  - live audit `checkbox_disabled_pointer_block`
  - live audit `checkbox_disabled_snapshot_click_block`
  - live audit `checkbox_disabled_controls_are_muted`
  - live audit `checkbox_focus_labels_visible`
  - live audit `checkbox_focus_single_active_row`
  - live audit `checkbox_inspector_options_are_labeled`
  - live audit `checkbox_modern_spacing`
  - live audit `checkbox_snapshot_state_consistency`
  - `checkbox_row_toggle_matches_core_public_checkbox_action_snapshot`
- RED result:
  - 先に失敗を確認。最初は preset 選択後の core state が checked=true にならず、描画側でも state=false 後の mark 領域に accent が残った。
  - 2 行目 click が 1 行目と同じ単一 state に吸収され、複数 checkbox を同時 checked にできない状態として失敗を確認。
  - row 幅不足と flat rectangle chrome を RED で確認。
  - 18px mark / 32px row / 8px mark-label gap の旧い寸法では modern checkbox として弱く、status column も row に接触する状態を RED で確認。
  - control/status text が 16px 未満の小さい text run になっている状態を RED で確認。
  - clicked snapshot で Storybook runtime overlay text `clicked 1` が checkbox core controls に重なる状態を RED で確認。
  - checkbox control row が component frame 下端に flush し、下端余白を持たない状態を RED で確認。
  - disabled preset の clicked snapshot shortcut が window interaction disabled block を通らず、checked glyph と `count 1` を出す状態を RED で確認。
  - disabled preset で state read/toggle/reset controls が通常 text 色のままで、押せる enabled control に見える状態を RED で確認。
  - focus preset で focus row background を label 描画後に上書きし、`Markdown Linter` / `Strict mode` の可視文字ピクセルが消える状態を RED で確認。
  - focus preset が 2 行すべてに focus ring を出し、単一 focus の UI として誤る状態を RED で確認。
  - checkbox Inspector の Settings が `disabled: false -> true` のように現在 state 値へ見える表記を出し、option mutation と current state を混同させる状態を RED で確認。
  - clicked snapshot 内で preview status と Inspector state/action/event が食い違っていないことを contract 化。
  - disabled preset でも focus/keyboard が通ってしまう状態を RED で確認。
  - manifest acceptance check `checkbox_hover_no_click_event` が live audit 未接続で manual smoke に失敗する状態を RED で確認。
  - hover state が preview 全体 boolean だけで、2 行目 hover でも 1 行目に hover feedback が出る状態を RED で確認。
  - manifest acceptance check `checkbox_keyboard_toggle_off` が live audit 未接続で manual smoke に失敗する状態を RED で確認。
  - manifest acceptance check `checkbox_keyboard_focused_secondary_row` が live audit 未接続で manual smoke に失敗する状態を RED で確認。
  - manifest acceptance check `checkbox_control_toggle_reset` が live audit 未接続で manual smoke に失敗する状態を RED で確認。
  - manifest acceptance check `checkbox_disabled_pointer_block` が live audit 未接続で manual smoke に失敗する状態を RED で確認。
- Fix: checked preset を描画時の `preset || state` で代替せず、未操作時だけ core checkbox state へ checked=true を注入する。専用描画は core state の `is_checkbox_checked_at(index)` のみを見る。row hit target は `CheckboxToggle(index)` を返し、`checkbox_state` / `checkbox_secondary_state` を独立に更新する。hover は checkbox row index を state に持たせ、実際に pointer が乗った row だけへ hover feedback を出す。disabled preset は window interaction の pointer/focus/keyboard 経路にも渡し、checked/focus/mark mutation を block する。disabled preset は row だけでなく state read/toggle/reset controls の label も muted にする。clicked snapshot shortcut でも disabled preset は default state のまま返し、core public API を通らない checked/action_count 代替を禁止する。binary choice chrome は 20px mark、36px row、244px row 幅、12px mark-label gap、16px row/status gap、rounded row/control/status、13px font による 16px 以上の label/control/status text run に更新する。checkbox/radio の focus/hover/selected row は border 色を先に決めてから label を描き、label を上書きしない。checkbox focus preset は active row のみに focus ring を出す。checkbox/radio/toggle/segmented-toggle の Inspector Settings は `option.*` 表記にし、現在 state と option mutation を混同させない。checkbox/radio の inline state surface では Storybook runtime overlay を描かず、binary choice frame/action rect を 156px にして controls の下端余白を確保する。
- GREEN result:
  - `rtk cargo test -p katana-ui-core checkbox_checked_public_prop_reaches_render_node_props --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib checkbox_checked_preset_does_not_keep_mark_after_state_turns_false --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib checkbox_rows_toggle_independently_and_can_both_be_checked --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib checkbox_contract_tests --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib visual_interaction_checkbox --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_rows_controls_and_status_use_rounded_modern_chrome --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_labels_controls_and_status_use_readable_text_runs --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_preview_does_not_draw_storybook_runtime_overlay_over_controls --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_controls_have_bottom_padding_inside_component_frame --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_clicked_disabled_preset_does_not_mutate_or_render_checked_state --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_disabled_preset_mutes_control_button_labels --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_clicked_snapshot_keeps_preview_status_and_inspector_state_consistent --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_focus_preset_keeps_row_labels_visible --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_focus_preset_only_draws_focus_ring_on_active_row --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_inspector_settings_rows_are_not_rendered_as_current_state_values --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_disabled_preset_blocks_focus_and_keyboard_toggle --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_hover_does_not_emit_click_event_or_mutate_checked_state --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_hover_feedback_tracks_the_actual_row --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_keyboard_second_toggle_removes_checked_mark_and_state --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_keyboard_toggle_applies_to_focused_secondary_row_only --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_control_toggle_and_reset_update_mark_and_state_together --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_disabled_preset_blocks_pointer_toggle_and_preserves_mark --locked`
  - `rtk cargo test -p katana-ui-core-storybook visual_interaction_checkbox --locked`
  - `rtk cargo test -p katana-ui-core-storybook checkbox_row_toggle_matches_core_public_checkbox_action_snapshot --locked`
  - `PYTHONPATH=scripts rtk proxy python3 scripts/storybook_manual_acceptance_smoke.py --page checkbox`
  - `rtk cargo test -p katana-ui-core-storybook --lib window_interaction --locked`
  - `rtk just storybook-smoke`
  - `rtk just storybook-interaction-smoke`
  - `rtk cargo fmt --check`
- Storybook user confirmation: pending
- Final status: manual_acceptance_pending
- Manual confirmation entrypoint:
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window checkbox`
- Manual confirmation smoke:
  - `rtk just storybook-manual-acceptance-smoke`
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 1 checkbox`

## UI: tooltip

- Layer: overlay
- Dependencies: hover/focus trigger, overlay placement, disclosure state
- Existing ledger verdict: 旧台帳の `実証済み` はユーザー確認 DoD では過大評価。
- Existing manifest verdict: `verified` は新 DoD では無効。ユーザーから動作していないと指摘済み。
- User reported failures: tooltip が Storybook 上で動作していない。
- Required operations: pointer, hover, focus。
- RED test: `repeated_hover_at_same_target_is_idempotent_for_event_pages`
- RED result: 同一 hover target で event が積み増されうる状態を確認。
- Fix: 同一 page/x/y/action_count の hover を idempotent に扱い、Tooltip hover/focus は core Tooltip action 経由で screen state へ反映する。
- GREEN result:
  - `rtk cargo test -p katana-ui-core-storybook repeated_hover_at_same_target_is_idempotent_for_event_pages --locked`
  - `rtk cargo test -p katana-ui-core-storybook visual_interaction_tooltip_tests --locked`
- Storybook user confirmation: pending
- Final status: manual_acceptance_pending
- Manual confirmation entrypoint:
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window tooltip`
- Manual confirmation smoke:
  - `rtk just storybook-manual-acceptance-smoke`
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 1 tooltip`

## UI: modal

- Layer: overlay
- Dependencies: modal state, native-window boundary, focus trap, escape action
- Existing ledger verdict: 旧台帳の `実証済み` はユーザー確認 DoD では過大評価。
- Existing manifest verdict: `verified` は新 DoD では無効。ユーザーから動作していないと指摘済み。
- User reported failures: modal が Storybook 上で動作していない。
- Required operations: pointer, keyboard, focus。
- RED test: `visual_interaction_modal_tests` と `modal_contract` の core escape/focus contract。
- RED result: Storybook user confirmation は未完了。
- Fix: Modal pointer/escape/focus は core Modal action と focus trap contract 経由で screen state へ反映する。
- GREEN result:
  - `rtk cargo test -p katana-ui-core-storybook visual_interaction_modal_tests --locked`
  - `rtk cargo test -p katana-ui-core modal_contract --locked`
- Storybook user confirmation: pending
- Final status: manual_acceptance_pending
- Manual confirmation entrypoint:
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window modal`
- Manual confirmation smoke:
  - `rtk just storybook-manual-acceptance-smoke`
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 1 modal`

## UI: text

- Layer: foundation
- Dependencies: TextRenderer, Canvas/frame render metadata, pointer drag, keyboard shortcut, clipboard adapter boundary
- Existing ledger verdict: 旧台帳には単体 UI として存在しないが、新 DoD では全 UI の表示 text が選択/copy 可能であるべき基盤要件として追加する。旧 verified 判定はこの要件を覆っていない。
- Existing manifest verdict: machine-readable manifest は text selection / clipboard copy を全 UI 共通 operation として扱っていないため不足。
- User reported failures: UI として表示した文章が選択できず、clipboard copy もできない。
- Required operations: pointer drag selection, keyboard copy shortcut, clipboard payload generation, clipped text bounds handling, multi-platform adapter boundary。
- RED test:
  - `rendered_storybook_text_runs_are_selectable_and_copyable`
  - `every_required_storybook_page_exposes_selectable_copyable_text_runs`
  - `live_audit_covers_text_selection_and_copy_for_every_required_page`
  - `storybook_window_drag_selection_updates_state_and_copy_payload`
- RED result:
  - 先に、描画後 Canvas に selectable text run が存在せず、表示テキストから copy payload を生成できない状態として失敗を確認。
  - 次に、required 77 page 全体で Canvas text run metadata と selection copy payload が欠落しても text page だけのテストでは検出できない状態として失敗を確認。
  - 次に、live audit が `text` page 代表だけで、他 76 page の pointer drag selection / keyboard copy shortcut を検証していない状態として失敗を確認。
  - 次に、window interaction 経由の pointer drag selection / keyboard copy shortcut を検証する audit helper が存在せず、実操作保証がない状態として失敗を確認。
- Fix: Canvas/TextRenderer が描画 text run の文字列と bounds を保持し、viewport 変換時も bounds を移動する。required 77 page は selectable/copyable text metadata と selection payload 生成を自動テストで確認する。window interaction は pointer drag で `text_selection_start/end` を更新し、copy shortcut 経由で clipboard payload を生成する。live audit は required 77 page すべてに `text_drag_selection` / `text_keyboard_copy` scenario を追加し、drag selection highlight の repaint も数値化する。Storybook user confirmation は未完了として残す。
- GREEN result:
  - `rtk cargo test -p katana-ui-core-storybook --lib rendered_storybook_text_runs_are_selectable_and_copyable --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib every_required_storybook_page_exposes_selectable_copyable_text_runs --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib live_audit_covers_text_selection_and_copy_for_every_required_page --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib storybook_window_drag_selection_updates_state_and_copy_payload --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib visual_interaction_text --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_selection --locked`
  - `env RUSTFLAGS="-D warnings" rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --headless-interaction-audit`
  - `rtk python3 scripts/storybook_manifest_interaction_smoke.py --manifest docs/storybook-77ui-interaction-manifest.json --audit target/storybook-live-interaction-audit.json`
  - `PYTHONPATH=scripts rtk python3 scripts/assert-storybook-ui-harness.py --root .`
  - `rtk cargo fmt --check`
- Storybook user confirmation: pending
- Final status: manual_acceptance_pending
- Manual confirmation entrypoint:
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window text`
- Manual confirmation smoke:
  - `rtk just storybook-manual-acceptance-smoke`
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 1 text`

## UI: progress-bar

- Layer: foundation
- Dependencies: progress model, theme tokens, live preview action/state/event
- Existing ledger verdict: 旧台帳の `実証済み` は新 DoD では無効。ユーザー実操作で動かないハリボテとして指摘されたため、live state/action 証跡を再確認する。
- Existing manifest verdict: `verified` は新 DoD では無効。progress state が操作で変化する証跡を要求する。
- User reported failures: progress bar が動作していないハリボテ。
- Required operations: pointer/action, option update, timed tick, state/event callback, rendered fill width change。Storybook user confirmation は未完了。
- RED test:
  - `progress_bar_repeated_preview_actions_advance_meter_state`
  - `progress_bar_percent_option_updates_state_and_meter_width`
  - `progress_bar_dedicated_render_uses_core_progress_bar_public_api`
  - `progress_bar_timed_tick_advances_via_core_progress_action`
  - `progress_bar_runtime_frame_ticks_accumulate_before_advancing`
  - `progress_bar_indeterminate_segment_moves_on_runtime_tick`
  - `test_rejects_progress_bar_without_indeterminate_motion_contract`
  - `test_rejects_verified_progress_bar_without_tick_and_core_evidence`
  - `test_rejects_required_operation_missing_from_declared_operation_kinds`
- RED result:
  - `progress-bar` preview action が専用 state を持たず、generic spec の `percent=64` と固定幅 82% 相当に留まる失敗を確認。
  - Inspector `progress.percent` option が state label と body diff だけを変え、`screen_state.progress_percent` は 65 のまま残る失敗を確認。
  - dedicated progress render が core `ProgressBar` public API を参照せず、Storybook 側の固定計算だけで fill 幅を作れる状態として失敗を確認。
  - timed tick operation が存在せず、実 window loop から progress state へ到達しない状態として失敗を確認。
  - indeterminate preset の segment が runtime tick 後も同じ x 座標に残り、動いていないハリボテ表示として失敗を確認。
  - manifest/manual smoke が indeterminate segment motion の live audit 証跡を要求せず、percent 変化だけで progress-bar runtime を十分と見なす状態として失敗を確認。
  - manifest validator が `progress-bar` の `verified` evidence に timed tick/core public API guard を要求せず、古い evidence のまま通る状態として失敗を確認。
  - manifest smoke が `operation_kinds` 未宣言の `timed_tick` を required operation として受け入れる状態として失敗を確認。
- Fix: `StorybookScreenState` に progress 専用 percent state を追加し、preview action で `65 -> 82 -> 99` と進める。Inspector `progress.percent` は同じ progress state へ 82 を反映する。描画は `has_widget_action()` の固定代替ではなく core `ProgressBar::new(...).progress(...)` の public props から percent を取り、fill 幅、percent label、state label を決める。runtime window loop は progress-bar page で frame tick を累積し、250ms ごとに core `UiAction::progress_changed` 経由で progress state を進める。manifest validator は `progress-bar` verified evidence に timed tick と dedicated core public API guard の証跡を要求する。
- Live audit fix: `progress-bar` に `timed_tick` operation を追加し、`progress_tick/progress_changed` と component body repaint を manifest smoke の required operation として要求する。indeterminate motion preset は segment の x 座標が tick で変わることを `progress_indeterminate_segment_motion` として live audit / manual acceptance smoke へ追加する。`timed_tick` は top-level `operation_kinds` に宣言し、未宣言 operation を required にした場合は smoke が落ちる。
- GREEN result:
  - `rtk cargo test -p katana-ui-core-storybook --lib progress_bar_repeated_preview_actions_advance_meter_state --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib progress_bar_percent_option_updates_state_and_meter_width --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib progress_bar_dedicated_render_uses_core_progress_bar_public_api --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib progress_bar_timed_tick_advances_via_core_progress_action --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib progress_bar_runtime_frame_ticks_accumulate_before_advancing --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib progress_bar_indeterminate_segment_moves_on_runtime_tick --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib progress_bar_live_audit_reports_indeterminate_segment_motion --locked`
  - `PYTHONPATH=scripts rtk python3 -m unittest scripts.test_storybook_ui_harness.StorybookUiHarnessTest.test_rejects_verified_progress_bar_without_tick_and_core_evidence`
  - `PYTHONPATH=scripts rtk python3 -m unittest scripts.test_storybook_manifest_interaction_smoke.StorybookManifestInteractionSmokeTest`
  - `PYTHONPATH=scripts rtk python3 scripts/test_storybook_manual_acceptance_smoke.py`
  - `PYTHONPATH=scripts rtk python3 scripts/assert-storybook-ui-harness.py --root .`
  - `env RUSTFLAGS="-D warnings" rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --headless-interaction-audit`
  - `rtk python3 scripts/storybook_manifest_interaction_smoke.py --manifest docs/storybook-77ui-interaction-manifest.json --audit target/storybook-live-interaction-audit.json`
  - `rtk cargo test -p katana-ui-core-storybook --lib visual_interaction_progress_bar --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib progress_bar_inspector_options_mutate_progress_loading_tone_and_size_semantic_state --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib visual_interaction_foundation_options --locked`
  - `env RUSTFLAGS="-D warnings" rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --visual-snapshot target/storybook-progress-clicked.png progress-bar dark preset-0 clicked`
  - `rtk cargo fmt --check`
- Storybook user confirmation: pending
- Final status: manual_acceptance_pending
- Manual confirmation entrypoint:
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window progress-bar`
- Manual confirmation smoke:
  - `rtk just storybook-manual-acceptance-smoke`
  - `rtk cargo run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 48 progress-bar`

## UI: form-field

- Layer: feedback
- Dependencies: text-input, validation state, label-to-control focus, theme tokens
- Existing ledger verdict: 旧台帳では `実証済み` だが、新 DoD ではユーザー確認がないため無効。
- Existing manifest verdict: `verified` は新 DoD では無効。window interaction gate が preset repaint の穴を検出した。
- User reported failures: 直接指摘ではないが、`window_interaction` gate が `theme field` preset の component body diff=0 を検出。
- Required operations: pointer, focus, settings/preset, validation state。今回の対象は preset repaint。
- RED test: `every_required_page_preset_tab_repaints_component_body`
- RED result: `form-field preset theme field did not repaint component body: diff=0` を確認。
- Fix: `form-field` dedicated renderer の preset index を `StoryPresetLabels::for_page("form-field")` と合わせ、`required` を index 3、`theme field` を index 4 として描画へ反映する。
- GREEN result:
  - `rtk cargo test -p katana-ui-core-storybook every_required_page_preset_tab_repaints_component_body --locked`
  - `rtk cargo test -p katana-ui-core-storybook window_interaction --locked`
  - `rtk cargo fmt --check`
  - `rtk just storybook-check`
- Storybook user confirmation: pending
- Final status: manual_acceptance_pending

## UI: row / column / stack / grid / align-center

- Layer: layout alignment
- Dependencies: core layout public option model, Inspector settings hit target, layout state bridge, component body repaint contract
- Existing ledger verdict: 旧台帳では `実証済み` 扱いだったが、新 DoD ではユーザー確認がないため無効。
- Existing manifest verdict: `verified` は新 DoD では無効。layout option の action/event が core layout 契約に接続されているかが不十分だった。
- User reported failures: 直接指摘ではないが、`visual_interaction_layout_options_tests` が layout option の semantic action mismatch を検出。
- Required operations: pointer, keyboard, focus, resize。今回の対象は Inspector pointer option update。
- RED test: `layout_inspector_options_mutate_axis_gap_alignment_and_overflow_semantic_state`
- RED result: `align-center` の `axis` option で action が `settings_axis` となり、core layout option event の `layout_option_changed` ではない失敗を確認。
- Fix: live layout page の Inspector option は既存通り対応 preset tab を追従させ、その上で `LayoutStoryState::apply_option` の public layout option update も発火する契約を維持する。`align-center` も `is_live_layout_page` と一致するよう test contract に含める。
- GREEN result:
  - `rtk cargo test -p katana-ui-core-storybook visual_interaction_layout_options_tests --locked`
  - `rtk cargo test -p katana-ui-core-storybook window_interaction --locked`
  - `rtk cargo fmt --check`
  - `rtk just storybook-check`
- Storybook user confirmation: pending
- Final status: manual_acceptance_pending

## Regression gate progress

- RED gate: `rtk just storybook-regression`
- RED result: Rust tests と Storybook harness は通過したが、最後の `kal check` / `ast-lint` が `error-first`, `file-length`, `magic-numbers` で失敗。
- Fixed:
  - `ui_tree_canvas_text_lines.rs` の `if let Ok(...)` 成功ネスト 3 件を helper 化して解消。
  - `interactive_preset.rs` の control hover border width/radius literal を named constants 化。
  - `coordinate_tests.rs` の pixel-center `0.5` literal を named constant 化。
  - `document_viewer/adapter.rs` の plan gap epsilon `0.5` を named constant 化。
  - `node_factory_accordion_tests.rs` の factory media width `120` を named constant 化。
  - `node_factory_alert.rs` の alert border width `4` を named constant 化。
  - `node_factory_blockquote_tests.rs` の quote depth / child index / fixture rect literals を named constants 化。
  - `node_factory_code_tests.rs` の code fixture width/height literals を named constants 化。
  - `node_factory_html_badge.rs` の compact/full badge metric literals を const metric presets 化。
  - `node_factory_html_image_tests.rs` の media width / fixture rect literals を named constants 化。
  - `node_factory_list_row.rs` の task marker / indent width literals を named constants 化。
  - `node_factory_list_tests.rs` の RGB shift literals を named constants 化。
  - `node_factory_media_control_tests_support.rs` の diagram control child index literal を named constant 化。
  - `node_factory_media_impl_tests.rs` の viewport zoom / pan / media surface fixture literals を named constants 化。
  - `node_labels_tests.rs` と `document_viewer_node_contract_tests.rs` の viewer rect / dimension fixture literals を named constants 化。
  - `document_viewer_tests.rs` の viewport / render area / runtime surface fixture literals を named constants 化。
  - `dedicated_dod_form_choice_marks.rs` の checkbox glyph pixel path を named constants 化。
  - `dedicated_dod_molecule_split_pane*.rs` の default ratio percent を shared constant 化。
  - `live_interaction_audit_scroll.rs` の split-pane keyboard resize expected ratio を named constants 化。
  - `live_interaction_audit_tabs.rs` の tab pointer inset を named constant 化。
  - `live_interaction_audit_toolbar.rs` の motion keyboard phase fixture を named constants 化。
  - `screen_state.rs` の TreeView scroll-retention hit-test x 座標を named constant 化。
  - `screen_state_code_diff.rs` の collapsed block fixture を named constants 化。
  - `screen_state_popover.rs` の offset / arrow fixture を named constants 化。
  - `screen_state_side_menu.rs` の route / scroll fixture を named constants 化。
  - `selection_screen_state_actions.rs` と `selection_screen_state_core.rs` の selection scroll contract を named constants 化。
  - `text_emoji_color_tests.rs` と `text_raster.rs` の emoji sample / RGB mask literals を named constants 化。
  - `ui_tree_canvas_checkbox.rs` の checkbox mark geometry / color literals を named constants 化。
  - `ui_tree_canvas_choice_control.rs` の radio dot / slider geometry literals を named constants 化。
  - `ui_tree_canvas_context_menu.rs` の divider / check / shortcut geometry literals を named constants 化。
  - `ui_tree_canvas_control.rs` の divider / loading / button / toggle geometry literals を named constants 化。
  - `ui_tree_canvas_entry.rs` の text entry padding / select arrow / label offset literals を named constants 化。
  - `ui_tree_canvas_extensions.rs` の RGBA channel index / shift / sample length literals を named constants 化。
  - `ui_tree_canvas_hit.rs` と `ui_tree_canvas_hit_tests.rs` の whitespace hit width factor を shared constants 化。
- GREEN result:
  - `rtk cargo test -p katana-ui-core-storybook --lib underline_span_draws_line_below_document_text --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib strikethrough_span_draws_line_even_for_whitespace --locked`
  - `rtk cargo test -p katana-ui-core --test interactive_preset_contract --locked`
  - `rtk cargo test -p katana-ui-core --lib coordinate_tests --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib details_html_without_open_attribute_renders_closed_accordion --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib details_html_body_markdown_is_rendered_as_structured_kdv_nodes --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib alert_nodes_normalize_gfm_kind_labels --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_roles_cover_alert_and_footnote_nodes --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib blockquote --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib code_block_has_copy_control_without_losing_code_body --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib html_badge --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib valid_svg_data_uri_still_renders_image_surface --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib broken_katana_svg_data_uri_renders_image_surface_for_export_surface_parity --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib list_node --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib diagram_controls_use_katana_controller_frame_without_style_contract --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib media --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib node_labels --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib code_block_height_comes_from_viewer_rect_not_kuc_text_metrics --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib code_block_uses_viewer_rect_x_as_container_padding --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib footnote_definition_uses_marker_column_and_body_spans --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib document_viewer --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib checkbox --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib split_pane --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib scroll_area --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib tabs --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib motion --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib toolbar --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib tree_view --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib navigation_scroll_is_retained_when_selecting_visible_row_after_deep_scroll --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib code_diff --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib popover --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib side_menu --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib selection --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib emoji --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_raster --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib radio --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib slider --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib context_menu --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib button --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib loading --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib toggle --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_input --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_area --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib select --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas_extensions --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas_hit --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib table --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_lines --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_metrics --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_role --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib span_style --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib tree_view --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib tree_canvas --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib command_palette --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib dynamic_array_editor --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib hover_card --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib search_box --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib modal_overlay --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib notification_toast --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib drag_and_drop --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib runtime_structured --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib scroll_area --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib shortcut_cheatsheet --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib split_pane --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib presentation --locked`
  - `rtk cargo fmt --check`
- Guard progress: `magic-numbers`, `prohibited-attributes`, `pub-free-fn`, and `type-separation` are cleared from the current `rtk just ast-lint` failure list.
- Type-separation cleanup:
  - `presentation.rs` delegates through `presentation_frame.rs`; crate-root free function export remains removed.
  - `live_interaction_audit.rs` report / scenario / summary types are separated.
  - `text.rs` public text renderer and related text data types are separated.
  - `ui_tree_canvas_text.rs` and `ui_tree_canvas.rs` renderer contract types are separated without changing the public renderer name.
  - core `SettingsList`, settings hit-test types, `TreeView` hit-test types, and `FileTree` contract types are separated without changing public API names.
- Type-separation GREEN result:
  - `rtk cargo test -p katana-ui-core-storybook --lib presentation --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib crate_root_exports_window_presentation_type --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib collapsible_panel --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_raster --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas_text --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas --locked`
  - `rtk cargo test -p katana-ui-core --lib --locked`
  - `rtk cargo fmt --check`
- File-length cleanup:
  - core `file_tree.rs` の public contract type と TreeView model construction を分離し、残件から除外。
  - `diagnostics_list_state.rs` の state type / default construction を分離し、残件から除外。
  - `clickable_operation.rs` の focus operation を分離し、残件から除外。
  - `ui_tree_canvas_control.rs` の loading renderer を分離し、残件から除外。
  - `ui_tree_canvas_text_wrap.rs` の span wrap state を分離し、残件から除外。
  - `settings_list_state.rs` の state type / fixture を分離し、残件から除外。
  - `scroll_operation.rs` の hit detection と scroll limit 計算を分離し、残件から除外。
  - `selection_screen_state_actions.rs` の combo action を分離し、残件から除外。
  - `live_interaction_audit_clickable.rs` の shortcut-combo scenarios を分離し、残件から除外。
  - `screen_state_forms_bridge.rs` の checkbox bridge を分離し、残件から除外。
  - `ui_tree_canvas_text_metrics.rs` の role 判定と document typography を分離し、残件から除外。
  - `node_factory_list_tests.rs` の list helper / fixture を分離し、残件から除外。
  - `node_factory_tests.rs` の HTML / alert / text-role contract tests を分離し、残件から除外。
  - `visual_layout_live_tests.rs` の stack/grid live operation tests を分離し、残件から除外。
  - `visual_layout_row_tests.rs` の align-center / scroll-area / split-pane layout operation tests を分離し、残件から除外。
  - `live_interaction_audit_scroll.rs` の split-pane scenarios を分離し、残件から除外。
  - `live_interaction_audit_list.rs` の settings-list / side-menu scenarios を分離し、残件から除外。
  - `live_interaction_audit_text_entry.rs` の command-palette / select-box / combo-box / search scenarios を分離し、残件から除外。
  - core `file_tree_tests.rs` の hit-test / item hit-target contracts を分離し、残件から除外。
  - `ui_tree_canvas_scroll.rs` の scroll-area height measurement を分離し、残件から除外。
  - `text_raster.rs` の font family selection / RGB pixel blending を分離し、残件から除外。
  - core `settings/render_tests.rs` の render / hit / action / target contracts を分離し、残件から除外。
  - `live_interaction_audit_toolbar.rs` の page-family scenarios を分離し、残件から除外。
  - `screen_state.rs` の preview dispatch / settings contract / runtime action bridge を分離し、残件から除外。
  - `ui_tree_canvas.rs` の geometry / renderer methods を分離し、残件から除外。
  - `ui_tree_canvas_hit.rs` の hit geometry / collector methods を分離し、残件から除外。
  - `ui_tree_canvas_hit_tests.rs` の hit scenario tests を分離し、残件から除外。
  - `ui_tree_canvas_tests.rs` の renderer regression tests を描画対象別に分離し、残件から除外。
  - `ui_tree_canvas_text_lines_tests.rs` の text line regression tests と helper を分離し、残件から除外。
  - `ui_tree_canvas_text_role.rs` の alert / block / list / geometry renderer を分離し、残件から除外。
  - `window_interaction/button_operation/apply_operation.rs` の menu apply logic を分離し、残件から除外。
  - `window_interaction/button_operation.rs` の hover dispatch を分離し、残件から除外。
  - `window_interaction/clickable_operation/structured_operation.rs` の focus / keyboard structured operation を分離し、残件から除外。
  - `window_interaction/layout_operation.rs` の state impl / accessors を分離し、残件から除外。
  - `window_interaction/runtime_structured_state.rs` の runtime structured state impl を UI family 別に分離し、残件から除外。
- File-length GREEN result:
  - `rtk cargo test -p katana-ui-core --lib --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib diagnostics_list --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib window_interaction --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas_control --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib loading --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_wrap --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas_text --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib settings_list --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib combo_box --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib selection --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib shortcut_combo --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib checkbox --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_metrics --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib list_node --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib node_factory --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib visual_layout_live --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib visual_layout --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib align_center --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib scroll_area --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib split_pane --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib settings_list --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib side_menu --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib selection_list --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib combo_box --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib select_box --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib command_palette --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib search_box --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib search_control --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_input --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_area --locked`
  - `rtk cargo test -p katana-ui-core --lib file_tree --locked`
  - `rtk cargo test -p katana-ui-core --lib --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas_scroll --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_raster --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text --locked`
  - `rtk cargo test -p katana-ui-core --lib settings --locked`
  - `rtk cargo test -p katana-ui-core --lib --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib live_interaction_audit --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib screen_state --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas_hit --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib text_lines --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib ui_tree_canvas_text --locked`
  - `rtk cargo test -p katana-ui-core-storybook --lib window_interaction --locked`
  - `rtk cargo fmt --check`
  - `rtk just ast-lint`
  - `rtk just storybook-check`
- Current automated release-readiness status (2026-06-19):
  - `rtk just ast-lint`: GREEN (`storybook ui harness passed`; file-length has no remaining Storybook failure).
  - `rtk just kuc-guardrails`: GREEN, including `scripts/assert-kuc-release-readiness.py --self-test` and runtime check.
  - `rtk python3 scripts/assert-kuc-release-readiness.py --self-test && rtk python3 scripts/assert-kuc-release-readiness.py`: GREEN.
  - `rtk cargo test -p katana-ui-core --test host_action_plan_contract --locked`: GREEN.
  - `rtk cargo test -p katana-ui-core-storybook --locked`: GREEN in the latest full Storybook run (`1509 passed`).
  - `rtk just storybook-interaction-pending-only`: GREEN; live audit reports `live_interactions=420 passed=402` and all interaction smoke failures are manual pending only.
  - `rtk python3 scripts/storybook_manual_acceptance_smoke.py --page text`: GREEN.
  - 2026-06-19 update: `rtk just storybook-interaction-smoke` and `rtk just release-readiness-check` are GREEN with `manual_acceptance_pending` treated as non-blocking when live interaction evidence covers the required contract.
  - `rtk just storybook-manual-acceptance-final-gate`: remains RED until `text`, `checkbox`, `progress-bar`, `tooltip`, `modal`, and `tree-view` receive user Storybook confirmation. No manual acceptance row is self-approved. The next user-confirmation target remains `text`.
