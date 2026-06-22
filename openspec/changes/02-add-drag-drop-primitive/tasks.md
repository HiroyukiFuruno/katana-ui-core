# Tasks — 02-add-drag-drop-primitive

## 1. 設計確定

- [x] 1.1 `DragData` の typed + tag モデルを確定し、共通 tag prefix（`os/`、`katana-ui-core/`、`consumer/`）を `design.md` に確定する。
- [x] 1.2 `DropAcceptance` enum（Reject / Accept{effect, indicator}）を確定する。
- [x] 1.3 `AutoScrollPolicy` の edge zone size / 加速度カーブ / 無効化を確定する。
- [x] 1.4 keyboard drag の操作セット（Space pick up / 矢印移動 / Space drop / Esc cancel）を確定する。

## 2. event model 実装

- [x] 2.1 `event/drag.rs` に `DragEvent` enum（DragStart / DragMove / DragEnter / DragLeave / DragOver / Drop / DragCancel / DragEnd）を実装する。
- [x] 2.2 `UiEvent` enum に `Drag(DragEvent)` を追加する。
- [x] 2.3 event bubbling / capture policy に drag 系の伝搬規則を追加する（disabled node は枝刈り）。
- [x] 2.4 event serialization / ordering test を drag 系で追加する。

## 3. interaction model 実装

- [x] 3.1 `interaction/drag_source.rs` に `DragSource`（payload, allowed_effects, keyboard_draggable）を実装する。
- [x] 3.2 `interaction/drop_target.rs` に `DropTarget`（accept callback, on_enter, on_over, on_leave, on_drop, auto_scroll）を実装する。
- [x] 3.3 `interaction/drag_data.rs` に `DragData` / `DragMetadata` / 共通 tag 定数を実装する。
- [x] 3.4 `interaction/drop_target.rs` に `accept` を純関数化したテスト fixture を実装する。

## 4. atom / molecule 実装

- [x] 4.1 `atom/drag_handle.rs` に `DragHandle` atom（cursor hint, accessibility label）を実装する。
- [x] 4.2 `atom/drop_indicator.rs` に `DropIndicator` atom（kind, orientation, tone, anchor rect）を実装する。
- [x] 4.3 `molecule/drag_preview.rs` に `DragPreview` molecule（label, icon, count badge, opacity）を実装する。
- [x] 4.4 `widget::atoms` / `widget::molecules` の re-export を更新する。

## 5. autoscroll & keyboard drag

- [x] 5.1 autoscroll engine を `interaction/autoscroll.rs` に純関数として実装する。
- [x] 5.2 keyboard drag の state machine を `interaction/keyboard_drag.rs` に実装する。
- [x] 5.3 accessibility announce のフック（announce string template）を実装する。

## 6. adapter contract

- [x] 6.1 adapter contract に native OS DnD の escape hatch（`os/file-list`、`os/url`、`os/text`）の変換責務を明記する。
- [x] 6.2 adapter に DragStart / Drop / Cancel の compile-gate stub を追加する。
- [x] 6.3 external runtime boundary に同 neutral contract を追加する。

## 7. 自動テスト

- [x] 7.1 `DropTarget.accept` が tag mismatch を Reject することを純関数で検証する。
- [x] 7.2 reorder vs insert の indicator 切替えが position 閾値で正しく切り替わることを検証する。
- [x] 7.3 keyboard drag が Space pick up → 矢印で focus 移動 → Space drop の流れで `Drop` を発火することを検証する。
- [x] 7.4 Esc cancel が DragCancel → DragEnd{committed:false} を順に発火することを検証する。
- [x] 7.5 autoscroll が edge zone 内で scroll request を発火することを検証する（純関数テスト）。
- [x] 7.6 disabled node が drag bubbling から除外されることを検証する。

## 8. 数値化 rendering contract

- [x] 8.1 reorder list の drag preview / drop indicator（before / after / inside）の描画契約を数値で検証する。
- [x] 8.2 file drop（外部 OS file）受け入れ時の hover 表示契約を数値で検証する。
- [x] 8.3 tab reorder の ghost + indicator 契約を数値で検証する。
- [x] 8.4 attachment drop（chat composer 想定）の drop zone hover 契約を数値で検証する。

## 9. Storybook ページ

- [x] 9.1 `Interaction > DragAndDrop` ノードを catalog に追加する。
- [x] 9.2 preset「reorder list」「file drop」「tab reorder」「attachment drop」「keyboard drag」を実装する。
- [x] 9.3 settings で `accept` callback / autoscroll / keyboard_draggable を切り替えできるようにする。
- [x] 9.4 event log（DragStart / Move / Enter / Drop / Cancel / End）を表示する。

## 10. ドキュメント

- [x] 10.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に DnD 行を追加する。
- [x] 10.2 `docs/widget-extraction-policy.md` に DnD 採用条件を追記する。
- [x] 10.3 README の adapter policy 節に native DnD escape hatch を明記する。

## 11. 品質ゲート / DoD

- [x] 11.1 `cargo test -p katana-ui-core` をパスする。
- [x] 11.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 11.3 `openspec validate 02-add-drag-drop-primitive --strict` をパスする。
- [x] 11.4 core contract gate をパスする。
