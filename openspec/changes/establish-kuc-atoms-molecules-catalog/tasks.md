# Tasks — establish-kuc-atoms-molecules-catalog（再評価リセット版）

## リセット方針（2026-05-23）

- 既存の完了マーク（`[x]` / `[/]`）は consumer harness 基準では信用しない。以後は原則 `[ ]` から再評価する。
- Storybook は外部利用と同じ `public API` / typed props / typed state / typed action / typed event / layout bounds / hit target / rendering contract を通す場として扱う。
- 目視、スクリーンショット、fallback、Storybook 専用状態、ログだけ、文字だけ preview は `ready` 根拠にしない。
- checkbox / radio / SelectBox(pulldown) など基礎部品から再検証する。

## 1. 停止条件として残す完了項目（物理実装済み）

- [x] 1.1 「完了済み前提を信用しない」リセット方針を tasks に固定する。（このファイル冒頭で維持）
- [x] 1.2 `docs/storybook-consumer-contract.md` を正本として保持する。（consumer contract の停止条件）
- [x] 1.3 `scripts/assert-storybook-consumer-contract.py` を guard として保持する。（自動検査の停止条件）
- [x] 1.4 `Justfile` の `kuc-guardrails` で consumer contract guard を実行する。（再発防止の停止条件）

## 2. 全 page readiness audit（必須・最優先）

- [x] 2.1 全 Storybook page の readiness audit（consumer contract 観点）を最初に実施する。
- [x] 2.2 page ごとに `public API` / typed props / typed state / typed action / typed event / layout bounds / hit target / rendering contract の未達一覧を作る。
- [x] 2.3 `ready` を名乗るページは、未達 0 かつ対応テスト/guard の参照行を紐付ける。
- [x] 2.4 audit 結果を tasks と test/guard へ追記し、目視根拠の行を除外する。

## 3. P0 再着手（基礎部品）

- [x] 3.1 Checkbox: option / action / event / state / preset / preview / settings / test を consumer contract 粒度で再実装・再検証する。
- [x] 3.2 Radio: option / action / event / state / preset / preview / settings / test を consumer contract 粒度で再実装・再検証する。
- [x] 3.3 SelectBox（pulldown）: 開閉、候補選択、focus、hit target、typed state 反映を再実装・再検証する。
- [x] 3.4 ComboBox: 入力、絞り込み、選択、typed action/event の一致を再実装・再検証する。
- [x] 3.5 SearchBox: 入力、クリア、検索条件操作、typed state/action の一致を再実装・再検証する。
- [x] 3.6 SelectionList: 単一/複数選択、キーボード操作、state 分離を再実装・再検証する。

## 4. TreeView（今回の未コミット差分の扱い）

- [x] 4.1 TreeView 表示修正は完了扱いにしない。depth / disclosure / guide line / marker の契約を P0 と同粒度で再評価する。
- [x] 4.2 `molecule_heavy.rs` / `dedicated_dod_molecule_tree.rs` / `dedicated_dod_molecule_tree_parts.rs` / `visual_tests.rs` の未コミット差分は、readiness audit 完了まで「検証待ち」として扱う。
- [x] 4.3 TreeView の visual contract test を削った状態で完了扱いしない。必要な検査ケースを再追加する。

## 5. 01〜24 の完了取消しと再評価

- [x] 5.1 01〜24 全項目は「完了扱いを取り消し」、consumer harness 基準で再評価する。（`docs/legacy-01-24-consumer-recheck.md`）
- [x] 5.2 各項目について option / action / event / state / preset / preview / settings / test / visual を最小 1 ケース以上で再固定する。（`docs/legacy-01-24-consumer-recheck.md` と `crates/katana-ui-core-storybook/tests/legacy_01_24_catalog_contract.rs`）
- [x] 5.3 Storybook の見た目説明ではなく、public API と自動テスト契約に追跡できる形で再記録する。（`docs/legacy-01-24-consumer-recheck.md` と `scripts/assert-storybook-consumer-contract.py`）

## 6. Storybook UI Harness 棚卸し（2026-05-25）

- [x] 6.1 `storybook-ui-harness` skill の必須構成に照らし、実装側の `requirements.rs`、`catalog/story_paths_*.rs`、`visual/dedicated.rs`、`visual/storybook_ui_option_contract.rs`、`catalog/preset_labels.rs`、`visual/legacy_01_24_contract*.rs` を解析した。
- [x] 6.2 `requirements.rs` の required 77 pages と Storybook menu 77 pages は一致しており、現時点で「menu にはあるが required にはない」page はない。
- [x] 6.3 01〜24 の unique page 26 件はすべて `visual/dedicated.rs::draw_page` の page 別描画へ到達している。`scripts/assert-storybook-ui-harness.py` と `cargo test -p katana-ui-core-storybook --locked legacy_01_24` は通過済み。
- [x] 6.4 旧 01〜24 ではなく Storybook menu 77 pages を change 分割の正本にする。leaf change 名は `storybook-page-<menu-page>` とし、対応表を `storybook-menu-change-split.md` に固定した。
- [x] 6.5 既存の `NN-add-*` / archive / parity backlog / interaction parity change は、menu page leaf change の入力元または umbrella として扱い、完了判定単位から外す。
- [x] 6.6 追加対象を 51 pages ではなく、Storybook menu 77 pages 全体へ拡張する。既に page 別描画がある 38 pages も、leaf change の `option / action / event / state / preset / preview / settings / test / visual` 契約へ接続するまで完了扱いにしない。
- [x] 6.7 優先順位番号は change 名に入れず、`storybook-menu-priority-order.md` で `SB-001`〜`SB-077` として管理する。順序組み換えはこの表だけを更新する。
- [/] 6.8 `scripts/assert-storybook-ui-harness.py` を、required page だけでなく `storybook-menu-change-split.md` の 77 leaf change と `storybook-menu-priority-order.md` の priority も入力にして、menu / required / option contract / preset labels / `draw_page` page 別分岐 / `window_interaction` 操作テストの接続漏れを検出できる guard へ拡張する。（leaf change / priority と menu / required の接続検査は実装済み。`window_interaction` 操作テストの page 別接続検査は未完了）
- [x] 6.9 `docs/storybook-consumer-readiness-audit.md` を 2026-05-25 の skill audit 基準で再生成し、generic renderer 経由の page と 77 leaf change の残作業をこの tasks / `storybook-menu-change-split.md` と一致させる。
- [x] 6.10 ユーザーが `次をすすめて`、`continue`、`次` と依頼したときに full の OpenSpec change ディレクトリ名を要求しないよう、`scripts/next-storybook-page-change.py --json` と `storybook-ui-harness` skill に次 leaf change 解決手順を固定する。

## User Review Phase（未対応フィードバック）

- [x] 2026-05-23 ユーザー指摘: consumer harness 条件が実作業に反映されず、見た目修正や `just check` 通過へ戻る再発を止める。（`scripts/assert-storybook-consumer-contract.py` に legacy recheck doc guard を追加）
- [x] 2026-05-23 ユーザー指摘: checkbox / radio / SelectBox(pulldown) など基礎部品が未成立のため、P0 再着手で優先復旧する。（P0 は checkpoint 済みのまま `docs/storybook-consumer-readiness-audit.md` の ready 維持、legacy 追跡表へ接続）
- [/] 2026-05-25 ユーザー指摘: 01〜24 を `storybook-ui-harness` skill に照らして残作業を更新し、01〜24 外でも Storybook menu に存在する UI を追加対象にする。（6.1〜6.10 に棚卸し結果と次作業解決手順を固定。`window_interaction` の page 別 guard 拡張は未完了）
