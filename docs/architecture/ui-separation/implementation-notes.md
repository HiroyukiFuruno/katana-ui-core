# ui-core-root-plan implementation notes

作成日: 2026-05-17

## Workspace current members

`Cargo.toml` の current members は次の通り。

- `crates/katana-ui-core`
- `crates/katana-ui-core-storybook`
- `examples/kuc-consumer-app`

external runtime / renderer crate は active workspace に含めない。
現在は `examples/kuc-consumer-app` の shell で、主要 node kind と action / event / state contract を public API だけで検証する。
Storybook smoke は `katana-ui-core` core-only の検証に固定し、framework-native runtime / renderer 経由にはしない。

## Legacy implementation

旧実装は参照資料として `tmp/` 配下へ退避した。
新しい UI は旧実装と同等範囲を最低ラインにし、runtime / window / surface / 状態（state）の一意管理を加えた +α としてゼロから作り直す。

## Release dry-run note

`katana-ui-core` の `cargo package --allow-dirty` は通過した。
KUC active release は core crate の package / publish dry-run、consumer app contract、Storybook core-only gate を対象にする。
external runtime / renderer の package / publish dry-run は KUC active release に含めない。
