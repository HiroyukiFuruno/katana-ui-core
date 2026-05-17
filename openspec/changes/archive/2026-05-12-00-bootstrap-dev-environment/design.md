# Design — 00-bootstrap-dev-environment

## 全体像

本 change は 01〜21 の widget change が安全に走るための **Harness** を一度に整備するもの。`harness-engineering` skill の 3 本柱（System / Scaffolding / Leverage）に対応させると以下になる。

| 柱 | 本 change での具体策 |
|---|---|
| System | KML 標準の取り込み（CI / Justfile / lefthook / release scripts）、ast-lint の crates.io 公開版導入と CI 接続 |
| Scaffolding | crate 階層スケルトン、Storybook（独立プロジェクト）、kuw-workflow-guide skill、docs |
| Leverage | 階層 + 依存方向制約により、widget 追加が「ディレクトリを 1 つ生やす」だけで完結する状態を作る |

## ディレクトリ階層と命名

```
katana-ui-widget/
├── crates/katana-ui-widget/src/
│   ├── lib.rs
│   ├── theme/
│   │   ├── color/
│   │   ├── spacing/
│   │   └── typography/
│   ├── primitive/
│   │   ├── text/
│   │   ├── icon/
│   │   └── spinner/
│   ├── composite/
│   │   ├── button/
│   │   │   ├── svg/
│   │   │   ├── text/
│   │   │   └── icon_text/
│   │   ├── selector/
│   │   │   ├── toggle/
│   │   │   ├── segmented/
│   │   │   ├── select/
│   │   │   └── color/
│   │   ├── input/
│   │   │   ├── text/
│   │   │   └── search/
│   │   └── indicator/
│   │       ├── tooltip/
│   │       ├── badge/
│   │       └── key_cap/
│   └── layout/
│       ├── card/
│       ├── accordion/
│       ├── split/
│       ├── modal/
│       └── popover/
├── storybook/                 # ★crates/ 外、独立 Cargo プロジェクト
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/
│       ├── main.rs
│       └── pages/
└── docs/
    ├── widget-extraction-policy.md
    └── directory-structure.md
```

階層命名は **役割そのもの**（theme / primitive / composite / layout）であり、atom/molecule/organism のようなアトミックデザイン用語は採用しない。理由は次の 2 点。

1. ライブラリ利用者に web のメタファを強制したくない。Floem 前提の Rust crate として「素朴で説明的な役割名」のほうが理解コストが低い。
2. 階層境界の意味（依存方向）は ast-lint で機械的に守るため、命名で示す必要がない。

各ディレクトリは配下のファイル数が 10 を超えた時点で関心事による分割を行う。本 change ではこの規約自体を `kuw-workflow-guide` に明記する。

## 依存方向と ast-lint 制約

```
theme  ←  primitive  ←  composite  ←  layout
                            ↑ サブカテゴリ間は横断不可
```

- 異なるサブカテゴリ間（例: `composite/button/` と `composite/selector/`）の参照は禁止。共通部分が出たら `primitive/` に降ろす（または `theme/` のトークンに昇華する）。
- 同一サブカテゴリ配下の sub-widget 間（例: `composite/input/search/` が `composite/input/text/` を内部利用）は許可。これは「ある親サブカテゴリは 1 つのまとまった責務」とみなすため。
- `layout/` は composite を自由に組み合わせてよい。
- ast-lint ルールは「import / `use` の対象パスを階層で照合」する形で実装する想定（具体構文は katana-ast-lint の機能に従う）。

## Storybook を crates/ 外に置く理由

- ライブラリ本体の **依存ツリーに dev/demo 専用クレートを混入させない**（`cargo publish` の影響範囲を最小化）
- ライブラリ側の workspace member に含めると、Storybook 用の重い依存（Floem の事例コード、画像、サンプル文字列など）が `cargo build --workspace` の標準経路に乗ってしまう
- 独立 Cargo プロジェクトにすることで `cargo build` / `cargo run` が Storybook 単体で完結し、ローカル動作確認の単位が明確になる
- `Cargo.lock` を独立して保持できるため、Storybook 専用の依存更新がライブラリ側のロックに影響しない

`storybook/` は **workspace member に含めない**。`katana-ui-widget` は `path = "../crates/katana-ui-widget"` で参照する。

## widget 抽出可否の判断軸（docs/widget-extraction-policy.md に格納）

- ✅ 抽出対象: Katana domain に依存しない、Floem 単体で完結する、汎用 UI 部品として他プロジェクトでも使えるもの
- ❌ 除外: markdown 描画 / KMM / chat 用 vendor 制御 / linter 結果表示 / workspace ファイルツリーなど Katana 固有の domain ロジックを含むもの
- 抽出元の参考実装（移植不可、仕様抽出のみ）:
  - `../katana/crates/katana-ui/src/widgets/` (egui)
  - `../katana/crates/katana-ui/src/views/` (egui)
  - `../katana-chat-ui/crates/katana-chat-ui-floem/src/widget/` (Floem)

## ストーリーブックの最小契約

- 1 widget = 1 ページ（`storybook/src/pages/<widget_name>.rs`）
- ページは少なくとも以下を表示
  1. デフォルト状態
  2. 主要バリアント（プロパティ違い）
  3. インタラクション可能な状態（hover / focus / disabled / active 等、該当するもの）
- 01 以降の widget change は **ページ追加が DoD（Definition of Done）の必須項目** とする

## 非対象（明示）

- 各 widget の実装そのもの（01〜21 で行う）
- Katana / katana-chat-ui の既存コードの書き換え（消費側適用は別 change）
- crates.io への初回 publish（標準構成側の release ワークフローに任せる）
