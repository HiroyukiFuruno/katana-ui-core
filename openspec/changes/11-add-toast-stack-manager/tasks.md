# Tasks — add-toast-stack-manager-11

## 1. 設計確定

- [x] 1.1 position × stack 方向の対応表を確定する。
- [x] 1.2 dedup strategy 3 種類と `replace_resets_duration` を確定する。
- [x] 1.3 queued の最大数とドロップ時の event 仕様を確定する。
- [x] 1.4 pause_on_hover の trigger 条件（hover / focus）を確定する。

## 2. 中核実装

- [x] 2.1 `molecule/toast_stack_manager.rs` を新設する。
- [x] 2.2 option / action / event / state を実装する。
- [x] 2.3 queue promotion と dedup を純関数として実装する。
- [x] 2.4 duration timer と pause / resume を純関数として実装する。
- [x] 2.5 `widget::molecules` の re-export に `ToastStackManager` を追加する。

## 3. 既存 NotificationToast との接続

- [x] 3.1 ToastStackManager は内部で `NotificationToast` を render する。
- [ ] 3.2 `NotificationToast` 単体 API は維持し、Storybook ページから ToastStackManager へのリンクを足す。

## 4. 自動テスト

- [x] 4.1 max_visible を超えた enqueue が queued に積まれることを検証する。
- [x] 4.2 visible が timeout / dismiss されたら queued の先頭が promote されることを検証する。
- [x] 4.3 dedup = ById で同 id 再投入時に `ToastReplaced` が発火することを検証する。
- [x] 4.4 pause_on_hover で hover 中 timer が停止し、離脱で resume することを検証する。
- [x] 4.5 position 6 種類の積み方向と stack_gap が反映されることを検証する。
- [x] 4.6 action button 押下で `ToastDismissed { reason: Action }` が発火することを検証する。
- [x] 4.7 queued 上限超過時に最古がドロップされ warning event が発火することを検証する。

## 5. 画像回帰

- [ ] 5.1 position 6 種類の積みを回帰する。
- [ ] 5.2 severity 4 種類（Info/Success/Warning/Danger）の見た目を回帰する。
- [ ] 5.3 action 0/1/2 個の layout を回帰する。
- [ ] 5.4 stack_gap、enter / exit 方向のスナップショットを回帰する。
- [ ] 5.5 light / dark theme を回帰する。

## 6. Storybook ページ

- [x] 6.1 `Disclosure > ToastStackManager` ノードを追加する。
- [ ] 6.2 preset「位置 6 種類」「dedup ById」「pause_on_hover」「queue 上限超過」「action 付き toast」を実装する。
- [ ] 6.3 settings で position / max_visible / dedup / duration / pause_on_hover / stack_gap を切替えできるようにする。

## 7. ドキュメント

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に ToastStackManager 行を追加する。
- [ ] 7.2 NotificationToast ページから ToastStackManager への参照を追記する。

## 8. 品質ゲート / DoD

- [x] 8.1 `cargo test -p katana-ui-core` をパスする。
- [x] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 8.3 `openspec validate 11-add-toast-stack-manager --strict` をパスする。
- [ ] 8.4 画像 / 入力回帰の CI gate をパスする。
