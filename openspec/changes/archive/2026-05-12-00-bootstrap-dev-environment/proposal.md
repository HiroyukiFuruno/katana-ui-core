## Why

`katana-ui-widget` は `chore: KUWリポジトリ標準構成を横展開` (d81b8fd) で OpenSpec / opsx 系 skill と CI/release のひな型を取り込んだが、Adapter 前提の汎用 UI widget を**安全に切り出して育てるための開発基盤**は未整備である。具体的には次が欠けている。

- 階層間の依存方向（`theme ← primitive ← composite ← layout`）を機械的に強制する **ast-lint ルール** が無い。命名や口頭ルールでは時間とともに崩れる（Harness Engineering の核：機械的制約への昇格）。
- 実装した widget を**目視確認できる場所**（Storybook 相当）が無い。Legibility が低い状態では実装に着手すべきでない。
- KML 標準（katana-markdown-linter）と比較して**まだ揃っていない設定ファイル / skill** がある（`clippy.toml`, `rustfmt.toml`, `CHANGELOG`, `CONTRIBUTING`, `.markdownlint.json`, `harness-engineering` / `bulk-modification-protocol` / `self-review` / `kuw-workflow-guide` 等）。
- widget 抽出方針・除外方針（Katana domain は対象外）を**docs として明文化していない**ため、後続 change 群（01〜21）の判断軸が曖昧になる。

01 以降の widget change を着手する前に、これらを 1 度にまとめて整える。

## What Changes

- `katana-series-repository-standardization` skill に従い、KML を参照リポジトリとして **KUW との差分を取り込む**（`clippy.toml`、`rustfmt.toml`、`CHANGELOG.md` / `CHANGELOG.ja.md`、`CONTRIBUTING.md` / `CONTRIBUTING.ja.md`、`.markdownlint.json`、`cliff.toml`、不足 skill 群、`Justfile` recipe 差分など）。
- `crates/katana-ui-widget/src/` に **階層スケルトン** を配置する: `theme/` / `primitive/` / `composite/{button,selector,input,indicator}/` / `layout/`。各ディレクトリは空の `mod.rs` のみ（実装は 01 以降）。
- **ast-lint プロジェクト規約** を導入し、依存方向を機械的に強制する:
  - `theme` は他のいずれにも依存してはならない。
  - `primitive` は `theme` にのみ依存できる。
  - `composite` は `theme` / `primitive` に依存できる。**異なるサブカテゴリ間**（例: `composite/button/` と `composite/selector/`）の参照は禁止。同一サブカテゴリ配下の sub-widget 間（例: `composite/input/search/` から `composite/input/text/`）は許可。
  - `layout` は `theme` / `primitive` / `composite` に依存できる。
- リポジトリルートに **`storybook/` ディレクトリを新設**し、`crates/` の外で独立した Cargo プロジェクトとして管理する（workspace member には**含めない**、`crates/katana-ui-widget` を `path` 依存で参照、独自の `Cargo.lock` を保持）。Adapter アプリとして widget 一覧 + ページ表示の最小骨格を実装する。
- `Justfile` に `storybook` recipe（起動）と `storybook-check` recipe（cargo check のみ）を追加。`scripts/` に必要な補助スクリプトを置く。
- KUW 専用の **`kuw-workflow-guide` skill** を新規作成し、widget 抽出方針 / Adapter 前提 / Katana domain 除外ルール / Storybook 別管理ルール / 階層と ast-lint 制約を明文化する。
- `docs/` に **`widget-extraction-policy.md`**（抽出 / 除外判断軸）と **`directory-structure.md`**（階層と依存方向）を追加。
- `README.md` を Adapter 前提・階層構造・Storybook 起動手順を反映する内容に更新。

## Capabilities

### New Capabilities

- `widget-foundation-layout`: `theme` / `primitive` / `composite/{button,selector,input,indicator}` / `layout` の階層スケルトンと、その依存方向に対する ast-lint 制約。
- `widget-storybook`: `crates/` の外側にある独立した Adapter アプリで、登録された widget をページ単位で目視確認できる。各 widget change はここに 1 ページ以上を追加する。
- `kuw-workflow-guide`: KUW における widget 抽出 / 除外 / Adapter 実装 / Storybook 連携の運用ルールを skill として codify。

### Modified Capabilities

- なし（初回整備のため）。

## Impact

- `crates/katana-ui-widget/src/lib.rs` が階層 `mod` 宣言中心に切り替わる（既存の `WidgetRegistry` マーカは互換のため一旦残す）。
- リポジトリルートに `storybook/` が増える（`crates/` には含めない）。`storybook/target/` は `.gitignore` 対象。
- KML との差分取り込みにより `Justfile`、`.gitignore`、CI workflow、scripts に変更が発生し得る。差分は適用前に `DRY_RUN=1` で確認する。
- 01〜21 の widget change はすべて本 change の前提（階層スケルトン、ast-lint、Storybook、抽出方針 docs）に依存する。
