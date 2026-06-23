# Tasks — 00-add-scroll-area-contract

## 1. 設計確定

- [x] 1.1 `ScrollAreaOptions` の axis / offset / extent / scrollbar / edge threshold を確定する。
- [x] 1.2 nested ScrollArea の state identity と event target を確定する。
- [x] 1.3 KDV / KLE の本文 viewer / editor scroll は対象外であることを `design.md` に固定する。

## 2. 中核実装

- [x] 2.1 `layout::ScrollArea` に typed options を追加する。
- [x] 2.2 `render_model` に scroll props を追加する。
- [x] 2.3 `UiAction` に `ScrollTo` / `ScrollBy` / `ScrollIntoView` / `SetScrollbarVisibility` を追加する。
- [x] 2.4 `ScrollAreaEvent` と `UiEvent::Scroll` に `Scrolled` / `ScrollEdgeReached` / `ScrollCommandRejected` を追加する。
- [x] 2.5 nested ScrollArea で parent / child の state が混ざらないようにする。

## 3. 連携

- [x] 3.1 `02-add-drag-drop-primitive` の autoscroll request が ScrollArea target を参照できる形にする。
- [x] 3.2 `16-add-virtualized-list-and-tree` が scroll offset と visible range を分離して扱える形にする。
- [x] 3.3 Storybook panel scroll state と通常 ScrollArea の契約差分を文書化する。

## 4. 自動テスト

- [x] 4.1 `ScrollTo` / `ScrollBy` が axis と extent に従って clamp されることを検証する。
- [x] 4.2 `ScrollIntoView` が target rect を viewport 内へ収めることを検証する。
- [x] 4.3 nested ScrollArea で child scroll event が parent state を変更しないことを検証する。
- [x] 4.4 edge 到達時だけ `ScrollEdgeReached` が発火することを検証する。
- [x] 4.5 scrollbar visibility / placement の render props が安定することを検証する。

## 5. 自動回帰

- [x] 5.1 vertical / horizontal / both axis の主要 preset を typed props で回帰する。
- [x] 5.2 `Auto` / `Always` / `Hidden` と `Reserved` / `Overlay` の組合せを設定 mutation で回帰する。
- [x] 5.3 nested scroll area の境界、scrollbar、clip が崩れないことを contract test で回帰する。
- [x] 5.4 light / dark theme を Storybook gate の theme token 差分で回帰する。

## 6. Storybook ページ

- [x] 6.1 `Layouts > ScrollArea` に typed scroll contract の page を追加する。
- [x] 6.2 settings で axis、offset、content size、scrollbar visibility、scrollbar placement を切替えできるようにする。
- [x] 6.3 state に offset / viewport / content / edge state を表示する。
- [x] 6.4 action / event log に scroll command と emitted event を表示する。
- [x] 6.5 quality に nested state identity、clamp、edge event の検証結果を表示する。

## 7. ドキュメント

- [x] 7.1 `docs/ui-separation-plan.md` の ScrollArea 項目へ typed contract を追記する。
- [x] 7.2 `docs/inventory/katana-katana-chat-ui-kdv-kle-ui-needs.md` の ScrollArea 行と同期する。

## 8. 品質ゲート / DoD

- [x] 8.1 `openspec validate 00-add-scroll-area-contract --strict` をパスする。
- [x] 8.2 `cargo test -p katana-ui-core` をパスする。
- [x] 8.3 `cargo clippy -p katana-ui-core -p katana-ui-core-storybook --all-targets -- -D warnings` をパスする。
- [x] 8.4 自動回帰 / 入力回帰 / 静的検査の CI gate をパスする。
