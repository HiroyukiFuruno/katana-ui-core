# Directory Structure

作成日: 2026-05-17
対象: `katana-ui-core` の中核 crate（core crate）

## 目的

KUC の中核 crate を、特定の画面フレームワーク（UI framework）に依存しない形で保つ。
この文書は README の階層説明と、`ui-core-root-plan` の依存方向タスクの判定基準として使う。

## Crate hierarchy

```text
crates/
├── katana-ui-core/
│   └── src/
│       ├── lib.rs
│       ├── theme/
│       ├── event/
│       ├── render_model/
│       ├── atom/
│       ├── molecule/
│       ├── widget/
│       ├── layout/
│       ├── runtime/
│       ├── window/
│       └── surface/
└── katana-ui-core-storybook/
```

## Core dependency direction

```text
theme
event
render_model
  <- atom
  <- molecule
  <- layout
runtime / window / surface
```

| layer | path | allowed dependency |
| --- | --- | --- |
| `theme` | `crates/katana-ui-core/src/theme/` | KUC DTO のみ |
| `event` | `crates/katana-ui-core/src/event/` | KUC DTO のみ |
| `render_model` | `crates/katana-ui-core/src/render_model/` | `theme`, `event` |
| `atom` | `crates/katana-ui-core/src/atom/` | `theme`, `event`, `render_model` |
| `molecule` | `crates/katana-ui-core/src/molecule/` | `theme`, `event`, `render_model`, `atom` |
| `widget` | `crates/katana-ui-core/src/widget/` | public re-export / composition boundary for `atoms` and `molecules` |
| `layout` | `crates/katana-ui-core/src/layout/` | `theme`, `event`, `render_model`, `atom`, `molecule` |
| `runtime` | `crates/katana-ui-core/src/runtime/` | KUC DTO / trait のみ |
| `window` | `crates/katana-ui-core/src/window/` | `runtime`, KUC DTO / trait |
| `surface` | `crates/katana-ui-core/src/surface/` | `runtime`, `window`, KUC DTO / trait |

## Forbidden core dependencies

中核 crate（core crate）は次を直接依存に持たない。

- framework-native view crate
- framework runtime crate
- framework renderer crate
- KatanA application domain crate

判定表は [`dependency-policy.md`](dependency-policy.md) に固定する。

## External runtime boundary

repo 外の runtime / renderer は、KUC の `render_model` / `runtime` / `window` / `surface` を受け取り、任意の実行環境へ接続してよい。
ただし、この repository の active workspace、release gate、公開 crate には含めない。

KUC 中核 crate（core crate）は framework-native 型を公開しない。
KUC が保証するのは、外部 runtime / renderer が消費できる中立 DTO / trait / action / event / state contract までとする。

## Module layout

各 UI 要素は責務で分ける。
1 ファイルが大きくなる場合は、行数だけで切らず、次の単位で分割する。

```text
<component>/
├── mod.rs
├── types.rs
├── model.rs
├── event.rs
├── render.rs
└── tests.rs
```

| file | responsibility |
| --- | --- |
| `types.rs` | public DTO, enum, identifier |
| `model.rs` | state model and immutable update operation |
| `event.rs` | KUC event input and output |
| `render.rs` | `UiNode` / `UiTree` generation |
| `tests.rs` | model, event, render serialization tests |

`view.rs` は中核 crate（core crate）では使わない。
framework-native view construction は KUC active workspace に置かない。

## Widget layer

初期公開境界は `widget::atoms` と `widget::molecules` に限定する。
利用側はこの 2 階層を組み合わせて UI を構築する。

```text
widget/
├── atoms/
└── molecules/
```

`organisms`、`templates`、`pages` は現時点の公開 API ではない。
Storybook 自身を構成する shell / navigation / inspector は内部構成部品として実装してよいが、公開 widget 階層へ昇格させる場合は別 change で扱う。

## Storybook

Storybook は KUC の部品を実画面で触ってフィードバックするための画面であり、静的見本帳ではない。
左ペインは KUC TreeView、preset 切替は KUC Tabs、各部品ページは preview と settings を持つ。
中央本文は全件カード一覧ではなく、選択中 UI の layout、option、action、event、state、rendering を扱う場にする。
Navigation / Preview / Details は panel ごとに独立した縦スクロール state を持つ。
Storybook は完了判定の根拠ではない。部品の正しさは自動テスト、数値化された layout / rendering contract、入力回帰、guard で判定する。

```bash
just storybook
just storybook-check
```

Storybook は `adapter` / `adapter` / `adapter` を使わない。互換層ごとの検証は各互換 crate の compile / unit test に閉じる。
中核 crate（core crate）の dependency tree に framework dependency が出たら失敗とする。

## Verification

変更時は次を確認する。

```bash
cargo tree -p katana-ui-core --locked
just check
```

`cargo tree -p katana-ui-core --locked` に `adapter` / `adapter` / `adapter` / KatanA application domain crate が出てはいけない。
