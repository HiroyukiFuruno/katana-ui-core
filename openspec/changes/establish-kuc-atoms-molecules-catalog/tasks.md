# Tasks — establish-kuc-atoms-molecules-catalog（再評価リセット版）

## リセット方針（2026-05-23）

- 既存の完了マーク（`[x]` / `[/]`）は consumer harness 基準では信用しない。以後は原則 `[ ]` から再評価する。
- Storybook は外部利用と同じ `public API` / typed props / typed state / typed action / typed event / layout bounds / hit target / rendering contract を通す場として扱う。
- 目視、スクリーンショット、fallback、Storybook 専用状態、ログだけ、文字だけ preview は `ready` 根拠にしない。
- checkbox / radio / SelectBox(pulldown) など基礎部品から再検証する。

## 1. 停止条件として残す完了項目（物理実装済み）

- [/] 1.1 「完了済み前提を信用しない」リセット方針を tasks に固定する。（このファイル冒頭で維持）
- [x] 1.2 `docs/storybook-consumer-contract.md` を正本として保持する。（consumer contract の停止条件）
- [x] 1.3 `scripts/assert-storybook-consumer-contract.py` を guard として保持する。（自動検査の停止条件）
- [x] 1.4 `Justfile` の `kuc-guardrails` で consumer contract guard を実行する。（再発防止の停止条件）

## 2. 全 page readiness audit（必須・最優先）

- [ ] 2.1 全 Storybook page の readiness audit（consumer contract 観点）を最初に実施する。
- [ ] 2.2 page ごとに `public API` / typed props / typed state / typed action / typed event / layout bounds / hit target / rendering contract の未達一覧を作る。
- [ ] 2.3 `ready` を名乗るページは、未達 0 かつ対応テスト/guard の参照行を紐付ける。
- [ ] 2.4 audit 結果を tasks と test/guard へ追記し、目視根拠の行を除外する。

## 3. P0 再着手（基礎部品）

- [ ] 3.1 Checkbox: option / action / event / state / preset / preview / settings / test を consumer contract 粒度で再実装・再検証する。
- [ ] 3.2 Radio: option / action / event / state / preset / preview / settings / test を consumer contract 粒度で再実装・再検証する。
- [ ] 3.3 SelectBox（pulldown）: 開閉、候補選択、focus、hit target、typed state 反映を再実装・再検証する。
- [ ] 3.4 ComboBox: 入力、絞り込み、選択、typed action/event の一致を再実装・再検証する。
- [ ] 3.5 SearchBox: 入力、クリア、検索条件操作、typed state/action の一致を再実装・再検証する。
- [ ] 3.6 SelectionList: 単一/複数選択、キーボード操作、state 分離を再実装・再検証する。

## 4. TreeView（今回の未コミット差分の扱い）

- [ ] 4.1 TreeView 表示修正は完了扱いにしない。depth / disclosure / guide line / marker の契約を P0 と同粒度で再評価する。
- [ ] 4.2 `molecule_heavy.rs` / `dedicated_dod_molecule_tree.rs` / `dedicated_dod_molecule_tree_parts.rs` / `visual_tests.rs` の未コミット差分は、readiness audit 完了まで「検証待ち」として扱う。
- [ ] 4.3 TreeView の visual contract test を削った状態で完了扱いしない。必要な検査ケースを再追加する。

## 5. 01〜24 の完了取消しと再評価

- [ ] 5.1 01〜24 全項目は「完了扱いを取り消し」、consumer harness 基準で再評価する。
- [ ] 5.2 各項目について option / action / event / state / preset / preview / settings / test / visual を最小 1 ケース以上で再固定する。
- [ ] 5.3 Storybook の見た目説明ではなく、public API と自動テスト契約に追跡できる形で再記録する。

## User Review Phase（未対応フィードバック）

- [ ] 2026-05-23 ユーザー指摘: consumer harness 条件が実作業に反映されず、見た目修正や `just check` 通過へ戻る再発を止める。
- [ ] 2026-05-23 ユーザー指摘: checkbox / radio / SelectBox(pulldown) など基礎部品が未成立のため、P0 再着手で優先復旧する。
