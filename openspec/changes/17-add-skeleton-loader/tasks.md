# Tasks — 17-add-skeleton-loader

## 1. 設計確定

- [x] 1.1 `SkeletonShape` / `SkeletonSize` / animation 4 種類を確定する。
- [x] 1.2 SkeletonCluster の 6 preset を確定する。
- [x] 1.3 reduced-motion との連携方式を確定する。

## 2. 中核実装

- [x] 2.1 `atom/skeleton.rs` を新設する。
- [x] 2.2 `molecule/skeleton_cluster.rs` を新設する。
- [x] 2.3 `widget::atoms` / `widget::molecules` の re-export を更新する。

## 3. 自動テスト

- [x] 3.1 shape 4 種類の数値化された layout props を検証する。
- [x] 3.2 SkeletonSize の Fill / Auto / Fixed の挙動を検証する。
- [x] 3.3 animation 4 種類で render props が正しく変わることを検証する。
- [x] 3.4 reduced-motion=true で animation が None に降格することを検証する。
- [x] 3.5 SkeletonCluster の 6 preset が安定 layout であることを検証する。
- [x] 3.6 accessibility live region announce が cluster 単位で 1 件のみ発火することを検証する。

## 4. 数値化された描画契約

- [x] 4.1 shape × tone × animation 主要 subset を render props で回帰する。
- [x] 4.2 SkeletonCluster の 6 preset を child count / width / live region で回帰する。
- [x] 4.3 light / dark theme は tone token と radius / animation props の contract で回帰する。

## 5. Storybook ページ

- [x] 5.1 `Atom > Skeleton` ノードを追加する。
- [x] 5.2 `Molecule > SkeletonCluster` ノードを追加する。
- [x] 5.3 preset「list loading」「message loading」「card loading」「paragraph loading」「code block loading」「image card loading」を実装する。
- [x] 5.4 settings で shape / size / animation / tone を切替えできるようにする。

## 6. ドキュメント

- [x] 6.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に Skeleton 行を追加する。

## 7. 品質ゲート / DoD

- [x] 7.1 `cargo test -p katana-ui-core` をパスする。
- [x] 7.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 7.3 `openspec validate 17-add-skeleton-loader --strict` をパスする。
- [x] 7.4 数値化された描画契約、state / event / action contract、Storybook requirement gate をパスする。
