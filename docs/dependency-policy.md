# Dependency policy

作成日: 2026-05-17
対象: `katana-ui-core` workspace dependencies

## 目的

KUC core に入れてよい依存と、adapter crate に閉じる依存を分ける。
この文書は `ui-core-root-plan` の P0-C / P1-A / P1-J の判定基準として使う。

## Dependency classification

| dependency | allowed in core | allowed in adapter | reason | verification command |
| --- | --- | --- | --- | --- |
| `floem` | no | yes | framework-native view 型を持つため core dependency にしない | `cargo tree -p katana-ui-core --locked` |
| `floem_reactive` | no | yes | Floem adapter runtime に属するため core dependency にしない | `cargo tree -p katana-ui-core --locked` |
| `floem_renderer` | no | yes | Floem rendering 実装に属するため core dependency にしない | `cargo tree -p katana-ui-core --locked` |
| `gpui` | no | yes | GPUI adapter に閉じる framework dependency | `cargo tree -p katana-ui-core --locked` |
| `egui` | no | yes | egui compatibility adapter に閉じる framework dependency | `cargo tree -p katana-ui-core --locked` |
| `fontdue` | no | storybook only | Storybook visual snapshot の文字描画にだけ使う。core API と adapter API には入れない | `cargo tree -p katana-ui-core --locked` |
| `image` | no | storybook only | Storybook visual snapshot の PNG 出力にだけ使う。core API と adapter API には入れない | `cargo tree -p katana-ui-core --locked` |
| `minifb` | no | storybook only | Storybook visual snapshot の framebuffer window にだけ使う。UI framework adapter として扱わない | `cargo tree -p katana-ui-core --locked` |
| `serde` | yes | yes | neutral DTO serialization に使う場合のみ core で許可 | `cargo tree -p katana-ui-core --locked` |
| `thiserror` | yes | yes | neutral error type に使う場合のみ core で許可 | `cargo tree -p katana-ui-core --locked` |
| `katana-*` domain crate | no | no | KUC は Katana domain-neutral であるため禁止 | `cargo tree -p katana-ui-core --locked` |

## Feature policy

| feature | default | allowed target | meaning | release gate |
| --- | --- | --- | --- | --- |
| `default` | yes | core | framework なしで compile できる core surface | `just check` |
| `floem-adapter` | no | `katana-ui-core-floem` | Floem view conversion | adapter compile test |
| `egui-adapter` | no | future `katana-ui-core-egui` | core 確立後の egui compatibility conversion | crate 作成後に compile test |
| `gpui-adapter` | no | future `katana-ui-core-gpui` | core 確立後の GPUI compatibility conversion | crate 作成後に compile test |

Core crate の `default` feature は adapter feature を有効化してはならない。

## Guardrail requirements

- core crate の dependency tree に `floem` / `gpui` / `egui` が出たら失敗。
- core crate の dependency tree に `katana-*` domain crate が出たら失敗。
- adapter crate の dependency は optional feature または crate boundary に閉じる。
- Storybook は `katana-ui-core` だけを参照し、adapter crate や framework dependency を使わない。
