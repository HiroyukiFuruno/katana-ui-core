# Tasks — ast-lint-guardrails

## 1. ルール設計

- [ ] 1.1 `no-test-only-runtime-api` の検出対象を定義する
- [ ] 1.2 `interactive-component-requires-callback` の対象 component 一覧を定義する
- [ ] 1.3 `no-storybook-box-leak` の対象パスを `storybook/src/**/*.rs` に限定する
- [ ] 1.4 `no-helper-only-view-file` の失敗条件と警告条件を分ける
- [ ] 1.5 `openspec-done-requires-evidence` の最低限の証跡条件を定義する
- [x] 1.6 `file-length-design-review` の責務境界と完了条件を定義する

## 2. `katana-ast-lint` 実装

- [ ] 2.1 `katana-ast-lint` に `no-test-only-runtime-api` を追加する
- [ ] 2.2 `katana-ast-lint` に `interactive-component-requires-callback` を追加する
- [ ] 2.3 `katana-ast-lint` に `no-storybook-box-leak` を追加する
- [ ] 2.4 `katana-ast-lint` に `no-helper-only-view-file` を追加する
- [ ] 2.5 `katana-ast-lint` に `openspec-done-requires-evidence` を追加する
- [ ] 2.6 `katana-ast-lint` に `file-length-design-review` の証跡チェックを追加する

## 3. 本リポジトリへの適用

- [ ] 3.1 `katana-ui-widget` の `kal check` で新ルールが有効になるよう設定する
- [ ] 3.2 `just ast-lint` で 01〜21 の再発パターンが検出されることを確認する
- [ ] 3.3 既存の正当な実装が誤検知される場合は、ルール側の判定を調整する
- [ ] 3.4 file-length 発火時に `types.rs` / `ops.rs` / `mod.rs` / `view.rs` / Storybook の責務レビュー証跡がない場合に失敗することを確認する

## 4. 回帰確認

- [ ] 4.1 `Box::leak` を含む Storybook サンプルで lint が失敗するテストを追加する
- [ ] 4.2 `#[cfg(test)]` だけの runtime API サンプルで lint が失敗するテストを追加する
- [ ] 4.3 callback 不在の interactive component サンプルで lint が失敗するテストを追加する
- [ ] 4.4 `just ast-lint` が通る
- [ ] 4.5 `view()` 移動だけで file-length を回避したサンプルで lint が失敗するテストを追加する

## 5. 完了確認

- [ ] 5.1 `RUSTFLAGS="-D warnings" cargo check --workspace --locked`
- [ ] 5.2 `just storybook-check`
- [ ] 5.3 `just ast-lint`
