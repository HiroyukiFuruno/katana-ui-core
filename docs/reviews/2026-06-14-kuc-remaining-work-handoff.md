# KUC Remaining Work Handoff

作成日: 2026-06-14

このファイルは Storybook page leaf queue 完了後も KUC DoD を完了扱いしないための残作業台帳である。Storybook 画面やスクリーンショットを完了根拠にせず、manifest / guard / 自動テスト / ユーザー確認の状態を分けて扱う。

## P0

- [ ] P0-1 text manual acceptance: `docs/storybook-77ui-interaction-manifest.json` の `text` は `text_drag_selection` / `text_keyboard_copy` / `text_zero_distance_drag_no_selection` と selectable/copyable text run の live audit 証跡を持つが、`manual_acceptance_pending` が残っている。ユーザーが Storybook で text selection / clipboard copy / zero-distance drag no-op を確認して OK するまで `audit_status=verified` に上げない。
- [ ] P0-2 progress-bar manual acceptance: `progress_timed_tick`、`progress_timed_cycle`、`progress_indeterminate_segment_motion` は live audit / manual smoke / guard に接続済みだが、`manual_acceptance_pending` が残っている。ユーザーが Storybook で progress meter と indeterminate segment motion を確認して OK するまで `audit_status=verified` に上げない。
- [ ] P0-3 checkbox manual acceptance: 複数 row checked、checked=false 後の glyph 消去、diagonal check、16px mark / 28px row hit target は自動テスト化済みだが、ユーザーが Storybook で checkbox の見た目と state/display 一致を確認して OK するまで `audit_status=verified` に上げない。
- [ ] P0-4 tooltip manual acceptance: hover/focus と同一 hover target の idempotency は自動テスト化済みだが、ユーザーから「tooltip が動作していない」と指摘済み。ユーザーが Storybook で tooltip 表示を確認して OK するまで `audit_status=verified` に上げない。
- [ ] P0-5 modal manual acceptance: pointer/focus/keyboard escape と core Modal action 経路は自動テスト化済みだが、ユーザーから「modal が動作していない」と指摘済み。ユーザーが Storybook で modal open/close/focus を確認して OK するまで `audit_status=verified` に上げない。
- [ ] P0-6 tree-view manual acceptance: `tree_view_click_after_scroll_keeps_visible_offset` と navigation scroll retention は自動テスト化済みだが、ユーザーが Storybook で深い tree scroll 後の click が先頭へ戻らないことを確認して OK するまで `audit_status=verified` に上げない。

## P1

- [ ] P1-1 final interaction smoke: P0 のユーザー確認後、`text`、`checkbox`、`progress-bar`、`tooltip`、`modal`、`tree-view` の `manual_acceptance_pending` を解除し、`rtk just storybook-interaction-smoke` が manual pending なしで通ることを確認する。

## 現在の確認コマンド

- `rtk just storybook-manual-acceptance-smoke`
- `rtk just storybook-interaction-pending-only`
- `rtk just storybook-requirement-gate`
- `rtk just ast-lint`
- `rtk just storybook-check`
- `rtk cargo test -p katana-ui-core-storybook --locked`
