# Tasks — 18-add-animation-primitives

## 1. 設計確定

- [ ] 1.1 motion tokens（duration / easing / distance）を確定する。
- [x] 1.2 `MotionSpec` / `MotionPrimitive` の typed model を確定する。
- [x] 1.3 `ReducedMotionPolicy` 3 種類と `disable_in` context list を確定する。
- [ ] 1.4 各 molecule の default motion を確定する。

## 2. theme / interaction 実装

- [x] 2.1 `theme/` に motion tokens を追加する。
- [x] 2.2 `interaction/motion.rs` を新設する。
- [ ] 2.3 reduced-motion runtime query を `accessibility` module 経由で受ける hook を実装する。
- [ ] 2.4 adapter contract に reduced-motion query 責務を明記する。

## 3. 各 molecule への組み込み

- [ ] 3.1 Popover / HoverCard / ContextMenu / Modal / NotificationToast / ToastStackManager / Banner / Accordion / DragPreview / Skeleton に `motion` option を追加する。
- [ ] 3.2 default motion で既存 preset の挙動を破壊しないことを保証する。
- [ ] 3.3 reduced-motion=true で全 molecule が Instant 動作することを保証する。

## 4. 自動テスト

- [x] 4.1 `MotionResolver::compute(reduced_motion, spec)` が Reduced で Instant を返すことを検証する。
- [ ] 4.2 各 molecule の motion default が dataset 通りであることを検証する。
- [x] 4.3 `disable_in = [Storybook]` で Storybook context のアニメーションが Instant になることを検証する。
- [x] 4.4 Force / Ignore モードで OS 設定を上書きできることを検証する。
- [x] 4.5 Shimmer が reduced 時に無効化されることを検証する。

## 5. 画像回帰

- [ ] 5.1 各 molecule の Force=Reduced 状態の表示が静止 frame であることを回帰する。
- [ ] 5.2 各 molecule の motion default の中間 frame（25% / 50% / 75%）を回帰する。

## 6. Storybook ページ

- [x] 6.1 `Foundation > Motion` ノードを catalog に追加する。
- [ ] 6.2 preset「4 primitive」「reduced-motion respect」「Force / Ignore」「per-molecule motion」を実装する。
- [ ] 6.3 settings で primitive / token / reduced-motion を切替えできるようにする。

## 7. ドキュメント

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に Motion 行を追加する。
- [ ] 7.2 `docs/compat-adapters.md` に reduced-motion query 責務を追記する。

## 8. 品質ゲート / DoD

- [x] 8.1 `cargo test -p katana-ui-core` をパスする。
- [x] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 8.3 `openspec validate 18-add-animation-primitives --strict` をパスする。
- [ ] 8.4 画像 / 入力回帰 CI gate をパスする。
