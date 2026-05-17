# ui-core-root-plan implementation notes

作成日: 2026-05-17

## Workspace current members

`Cargo.toml` の current members は次の通り。

- `crates/katana-ui-core`
- `crates/katana-ui-core-egui`
- `crates/katana-ui-core-floem`
- `crates/katana-ui-core-gpui`
- `crates/katana-ui-core-storybook`

egui / GPUI 互換 adapter crate は skeleton のみ作成した。
framework-native 実装は、KUC core と primary adapter 候補を確立した後続段階で扱う。
Storybook smoke は `katana-ui-core` core-only の検証に固定し、Floem 経由にはしない。

## Legacy Floem implementation

旧 Floem 実装は参照資料として `tmp/trash/2026-05-17-181114/legacy-floem-reference/` に退避した。
新しい UI は旧実装と同等範囲を最低ラインにし、runtime / window / surface / 状態（state）の一意管理を加えた +α としてゼロから作り直す。

## Release dry-run note

`katana-ui-core` の `cargo package --allow-dirty` は通過した。
`katana-ui-core-floem` の通常 package / publish dry-run は、未公開の `katana-ui-core` を crates.io index で解決できない。
このため `scripts/release/verify-primary-adapter-release.sh` で初回公開前の adapter gate を定義した。
`katana-ui-core` が未公開の場合は package file list / compile / test を実行し、公開済みの場合は通常の package / publish dry-run を実行する。
