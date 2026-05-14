## 方針

静的に検出できる「不備の形」を優先してルール化する。見た目の良し悪しや操作感そのものは `ast-lint` で断定しない。

## 対象ルール

1. `no-test-only-runtime-api`
   - `ops.rs` / `view.rs` の主要 API が `#[cfg(test)]` に閉じている場合に失敗する。
   - 例: `SplitPane` の drag / reset / cursor がテスト専用になり、実行時に使えない状態。

2. `interactive-component-requires-callback`
   - `Toggle` / `SelectBox` / `SegmentedToggle` / `ColorSwatch` / `TextInput` / `SearchBox` / `Accordion` など、操作可能な型に `on_change` / `on_toggle` / `on_submit` / `on_close` 相当の契約がない場合に失敗する。

3. `no-storybook-box-leak`
   - `storybook/src/**/*.rs` で `Box::leak` を使って表示文字列の lifetime を逃がしている場合に失敗する。

4. `no-helper-only-view-file`
   - `view.rs` が定数と単純 getter だけで、view-ready な構造体またはレンダリング用モデルを返していない場合に警告または失敗にする。

5. `openspec-done-requires-evidence`
   - `tasks.md` の `[x]` に対して、対応する実装ファイル・テスト・Storybook ページの最低限の存在を確認する。

6. `file-length-design-review`
   - `file-length` / `type-separation` が発火した場合、単純に `view()` を別ファイルへ逃がしただけでは解決扱いにしない。
   - 対象 widget の責務境界を `types.rs`（データ契約） / `ops.rs`（実行時の状態遷移） / `mod.rs`（builder と resolve） / `view.rs`（Floem イベント接続と描画）として見直す。
   - Storybook のライブセルが、逃がした view ではなく widget 本体の runtime API を使っていることを確認する。

## 非対象

- Storybook 上での実際の見た目。
- Light/Dark の視覚的な破綻。
- UI 操作の体感品質。

これらは実行確認とスクリーンショット確認で扱う。
