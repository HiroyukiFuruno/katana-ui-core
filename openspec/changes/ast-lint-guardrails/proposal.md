## Why

01〜21 の実装で、`tasks.md` の `[x]` と実装実体がずれる問題が発生した。特に、操作可能な widget に callback がない、`view.rs` が表示実体ではなく定数ヘルパだけになっている、実行時 API が `#[cfg(test)]` に閉じている、といった不備は人手監査だけでは漏れやすい。

## What Changes

- `katana-ast-lint` で検出できる静的ルールを追加する。
- `katana-ui-widget` 側に、そのルールを有効化する設定または実行導線を追加する。
- 01〜21 の実装不備として再発しやすいパターンを `just ast-lint` で検出できるようにする。

## Capabilities

### New Capabilities

- `kuw-ast-lint-guardrails`: Storybook / widget 実装 / OpenSpec task の整合を静的に検査する。

### Modified Capabilities

- `repository-quality-gate`: `just ast-lint` が UI widget 実装の形骸化を検出する。

## Impact

- `katana-ast-lint` 側にルール追加が必要。
- 本リポジトリ側では `kal check` の設定またはルール適用対象の追加が必要。
- 静的解析で判断できない見た目・操作感は Storybook 実行確認と併用する。
