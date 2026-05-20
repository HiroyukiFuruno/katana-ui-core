# Tasks — 16-add-virtualized-list-and-tree

## 1. 設計確定

- [x] 1.1 `VirtualizationConfig` の typed option を確定する。
- [x] 1.2 `RowHeightProvider` の Fixed / Variable / Estimated を確定する。
- [x] 1.3 `keep_focused_in_window` の挙動を確定する。
- [x] 1.4 accessibility-aware aria (`aria-setsize` / `aria-posinset`) の規約を確定する。

## 2. 共通 virtualization 実装

- [x] 2.1 `interaction/virtualization.rs` に `VirtualizationPlanner::compute_visible_range` を実装する。
- [x] 2.2 `measured_overrides` の merge と scroll 補正を実装する。
- [x] 2.3 単体テストで Fixed / Variable / Estimated の 3 providers を網羅する。

## 3. 各 molecule への組み込み

- [x] 3.1 `List` に virtualization option を追加する。
- [x] 3.2 `SelectionList` に virtualization option を追加する（section header は常時描画）。
- [x] 3.3 `TreeView` に virtualization option を追加する（展開ノードのフラット化を考慮）。
- [x] 3.4 `CommandPalette` に virtualization option を追加する（filtered list を対象）。
- [x] 3.5 `DiagnosticsList` に virtualization option を追加する（group + item flat 列を対象）。

## 4. accessibility

- [x] 4.1 aria-setsize / aria-posinset を virtualization 中でも正しく報告することを実装する。
- [x] 4.2 keep_focused_in_window で focused row が virtual_range 外でも描画されることを実装する。
- [x] 4.3 screen reader announce が `n of total` 形式で出ることを実装する。

## 5. 自動テスト

- [x] 5.1 `VirtualizationPlanner::compute_visible_range` の純粋な契約テスト（境界 / overscan / Variable）を作る。
- [x] 5.2 scroll で virtual_range が更新され、event は項目 id ベースで安定であることを検証する。
- [x] 5.3 keep_focused_in_window が focus 維持を保証することを検証する。
- [x] 5.4 aria-setsize / aria-posinset の announce 値を検証する。
- [x] 5.5 既存 preset は virtualization=disabled で挙動が変わらないことを回帰する。
- [x] 5.6 各 molecule に 10k 件入力を渡し、描画行数が overscan + viewport に収まることを自動テストで検証する。

## 6. 数値化された描画 / 入力契約

- [x] 6.1 List 10k 件、TreeView 1万 node、CommandPalette 1万 item の scroll 位置別 visible range を自動テストで検証する。
- [x] 6.2 keep_focused_in_window の有効 / 無効の差を focused row sentinel と announce で自動テスト検証する。
- [x] 6.3 SelectionList の section header と virtual row の state 契約を自動テストで検証する。

## 7. Storybook ページ

- [x] 7.1 各 molecule のページに「Virtualization」preset を追加する。
- [x] 7.2 settings で virtualization を enable / disable、overscan、row height provider を切替えできるようにする。
- [x] 7.3 visible range と total count の表示 inspector を追加する。

## 8. ドキュメント

- [x] 8.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に Virtualization 行を追加する。
- [x] 8.2 adapter 責務（row 測定）を `docs/compat-adapters.md` に追記する。

## 9. 品質ゲート / DoD

- [x] 9.1 `cargo test -p katana-ui-core` をパスする。
- [x] 9.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 9.3 `openspec validate 16-add-virtualized-list-and-tree --strict` をパスする。
- [x] 9.4 入力回帰、state / event / action contract、数値化された描画契約、Storybook requirement gate をパスする。
