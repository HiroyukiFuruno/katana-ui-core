# Tasks — 06-add-multiline-text-input

## 1. 設計確定

- [ ] 1.1 `TextArea` option 一覧（value / placeholder / font_role / disabled / readonly / invalid / min_rows / max_rows / auto_grow / wrap / submit_key / newline_key / tab_behavior / ime_enabled / slot 2種）を確定する。
- [ ] 1.2 submit_key と newline_key の衝突を static check でエラーにするルールを確定する。
- [ ] 1.3 auto-grow と max_rows 超過時の内部スクロール切替仕様を確定する。

## 2. 中核実装

- [ ] 2.1 `atom/text_area/mod.rs` を新設し、`TextArea` 型を作る。
- [ ] 2.2 `options.rs` で typed option を実装する。
- [ ] 2.3 `actions.rs` で `Type` / `Submit` / `InsertNewline` / `Clear` / `MoveCaret` / `Select` / `IMECommit` を実装する。
- [ ] 2.4 `events.rs` で `KeyInput` / `TextInput` / `IMEComposition` / `IMECommit` / `EmojiInput` / `Submit` / `Change` / `Focus` / `Blur` / `Resize` を実装する。
- [ ] 2.5 `state.rs` で value / caret / selection / composition / focused / disabled / readonly / invalid / measured_rows を実装する。
- [ ] 2.6 grapheme cluster を含む caret 移動を純関数として `caret.rs` に実装する。
- [ ] 2.7 auto-grow 計算を純関数として `autogrow.rs` に実装する。

## 3. 公開境界

- [ ] 3.1 `widget::atoms` の re-export に `TextArea` を追加する。
- [ ] 3.2 `Input` atom のドキュメント文字列に「multi-line は TextArea を使う」と明記する。

## 4. adapter contract

- [ ] 4.1 adapter contract に multi-line IME / preedit string / caret 位置の責務を明記する。
- [ ] 4.2 floem adapter に multi-line + IME の compile-gate stub を追加する。
- [ ] 4.3 egui / gpui adapter に同 stub を追加する（差異を README に記載）。

## 5. 自動テスト

- [ ] 5.1 submit_key=Enter, newline_key=ShiftEnter で Enter → `Submit`、Shift+Enter → `InsertNewline` を検証する。
- [ ] 5.2 submit_key と newline_key が同一キーに割り当てられた場合の static check failure を検証する。
- [ ] 5.3 disabled / readonly が action を抑止することを検証する。
- [ ] 5.4 grapheme cluster（emoji / ZWJ / surrogate pair）を 1 単位として caret 移動 / delete することを検証する。
- [ ] 5.5 IME composition 開始 → preedit 更新 → commit のフルライフサイクルを検証する。
- [ ] 5.6 auto_grow=true で min_rows 〜 max_rows 範囲で `Resize` event が発火することを検証する。
- [ ] 5.7 max_rows 超過時に内部スクロールに切替わり、value は全文保持されることを検証する。
- [ ] 5.8 tab_behavior=MoveFocus 時に Tab がフォーカス移動を起こし、`InsertTab` 時はタブ文字を挿入することを検証する。

## 6. 画像回帰

- [ ] 6.1 default / focused / disabled / readonly / invalid 状態を回帰する。
- [ ] 6.2 multi-line 表示（1 行 / 中行数 / max_rows / 超過スクロール）を回帰する。
- [ ] 6.3 IME composition 中の preedit overlay を回帰する。
- [ ] 6.4 leading_slot / trailing_slot 表示を回帰する。
- [ ] 6.5 light / dark theme での placeholder / caret / selection を回帰する。

## 7. Storybook ページ

- [ ] 7.1 `Atom > TextArea` ノードを catalog に追加する。
- [ ] 7.2 preset「chat composer」「検索（複数行）」「長文」「auto-grow」「max_rows 超過」「IME 入力」「emoji 入力」を実装する。
- [ ] 7.3 settings で submit_key / newline_key / tab_behavior / auto_grow / wrap_policy 等を切替えできるようにする。
- [ ] 7.4 state / event / action のログを表示する。

## 8. ドキュメント

- [ ] 8.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に TextArea 行を追加する。
- [ ] 8.2 `docs/widget-extraction-policy.md` に Input vs TextArea の責務境界を追記する。

## 9. 品質ゲート / DoD

- [ ] 9.1 `cargo test -p katana-ui-core` をパスする。
- [ ] 9.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [ ] 9.3 `openspec validate 06-add-multiline-text-input --strict` をパスする。
- [ ] 9.4 画像回帰 / 入力回帰 / 静的検査の CI gate をパスする。
