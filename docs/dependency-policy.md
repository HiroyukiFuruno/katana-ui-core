# Dependency policy

作成日: 2026-05-17
対象: `katana-ui-core` workspace dependencies

## 目的

KUC core に入れてよい依存と、外部変換層に逃がす依存を分ける。
この文書は `ui-core-root-plan` の P0-C / P1-A / P1-J の判定基準として使う。

## Dependency classification

| dependency | allowed in core | allowed outside core | reason | verification command |
| --- | --- | --- | --- | --- |
| framework-native view crate | no | dedicated adapter only | framework-native view 型を持つため core dependency にしない | `cargo tree -p katana-ui-core --locked` |
| framework runtime crate | no | dedicated adapter / storybook | framework event loop / runtime に属するため core dependency にしない | `cargo tree -p katana-ui-core --locked` |
| framework renderer crate | no | dedicated adapter / raster runtime | framework rendering 実装に属するため core dependency にしない | `cargo tree -p katana-ui-core --locked` |
| `egui` | no | `katana-ui-core` の `egui` feature / Storybook | KUCの中立contractをegui input / outputへ接続する opt-in module に閉じる | `cargo tree -p katana-ui-core --features egui --locked` |
| `cosmic-text` | no | `katana-ui-core` の `text-raster` feature | platform font、color emoji、grapheme shapingを opt-in module に閉じる | `cargo tree -p katana-ui-core --features text-raster --locked` |
| `resvg` / `tiny-skia` | no | `katana-ui-core` の `svg-raster` feature | SVG iconを opt-in module からRGBA出力へ変換する | `cargo tree -p katana-ui-core --features svg-raster --locked` |
| `image` | no | `katana-ui-core` の `storybook-artifacts` feature | private artifact出力に限定し、default featureでは有効化しない | `cargo tree -p katana-ui-core --features storybook-artifacts --locked` |
| `minifb` | no | storybook only | Storybook visual snapshot の framebuffer window にだけ使う。UI framework adapter として扱わない | `cargo tree -p katana-ui-core-storybook --locked` |
| `serde` | yes | yes | neutral DTO serialization に使う場合のみ core で許可 | `cargo tree -p katana-ui-core --locked` |
| `thiserror` | yes | yes | neutral error type に使う場合のみ core で許可 | `cargo tree -p katana-ui-core --locked` |
| `katana-*` domain crate | no | no | KUC は Katana domain-neutral であるため禁止 | `cargo tree -p katana-ui-core --locked` |

## Feature policy

| feature | default | allowed target | meaning | release gate |
| --- | --- | --- | --- | --- |
| `default` | no optional framework | core | framework なしで compile できる core surface | `just check` |
| `text-raster` | no | `katana-ui-core` | `text_raster` module とその runtime dependency | `cargo check -p katana-ui-core --features text-raster --locked` |
| `svg-raster` | no | `katana-ui-core` | `svg_raster` module とその runtime dependency | `cargo check -p katana-ui-core --features svg-raster --locked` |
| `egui` | no | `katana-ui-core` | `egui` module と raster modules | `cargo check -p katana-ui-core --features egui --locked` |

Core crate の `default` feature は framework-native feature を有効化してはならない。

## Core boundary contracts

KUC core は外部 runtime / renderer を実装しないが、利用側が実 app を組み立てるための中立契約は提供する。

- `UiAdapterCoveragePlan` は public API consumer tree を走査し、core が提供する node kind、action、surface の不足を数値化する。
- `AdapterActionBridge` は KUC component action と state transition を中立 action として接続する。
- `AdapterHostActionBridge` は host action plan を中立 action id で解決し、framework-native callback を core API へ混ぜない。
- これらは core crate の契約であり、framework / raster実装は公開された専用crateからcoreへ逆流させない。

## Guardrail requirements

- core crate の dependency tree に `katana-*` domain crate が出たら失敗。
- Storybook は `katana-ui-core` の公開 module / feature を実consumerとして組み合わせ、privateな重複rendererを持たない。
