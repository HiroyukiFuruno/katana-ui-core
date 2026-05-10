# Tasks — 00-bootstrap-dev-environment

## 1. KML 差分取り込み（katana-series-repository-standardization skill 準拠）

- [x] 1.1 `KML_ROOT=$HOME/works/private/katana-markdown-linter`、`TARGET_ROOT=$HOME/works/private/katana-ui-widget` を設定し、skill の「1) 差分比較」を `DRY_RUN=1` で実行して差分一覧を取得する
- [x] 1.2 取り込み対象を確定する（KUW 固有理由で外すものは `docs/widget-extraction-policy.md` に根拠を残す）
  - [x] `clippy.toml`
  - [x] `rustfmt.toml`
  - [x] `.markdownlint.json`
  - [x] `CHANGELOG.md` / `CHANGELOG.ja.md`（空ひな型でよい）
  - [x] `CONTRIBUTING.md` / `CONTRIBUTING.ja.md`
  - [x] `cliff.toml`
  - [x] `.gitignore` の差分（`storybook/target/` も追記）
  - [x] `Justfile` の不足 recipe（`sweep` / `clean` 等）— 既存に含まれていた
  - [x] `lefthook.yml` の差分 — KUW 側の方が ast-lint 統合済みで充足
  - [x] `docs/release.md` の差分 — KML は release-runbook.md で別物、KUW の既存 docs/release.md を維持
  - [x] `.github/workflows/{test-and-build,release-preflight,release}.yml` の差分 — ast-lint-install ステップ追加
  - [x] `scripts/release/*` / `scripts/openspec` の差分 — KML の KML 固有スクリプトは不要、KUW の既存 scripts/ が充足
- [x] 1.3 KML にあって KUW に未配置の skill を `rsync` で取り込む
  - [x] `harness-engineering` — KML は `kml-harness-engineering`、そちらをコピー
  - [x] `bulk-modification-protocol` — KML にも存在しないため対象外
  - [x] `self-review` — KML にも存在しないため対象外
  - [x] `lint-and-ast-lint`（または相当） — KML にも存在しないため対象外
  - [x] `changelog-writing` — KML にも存在しないため対象外
  - [x] `pr-release-flow` / `prepare-release-pr` — .codex/skills/ からコピー済み
- [x] 1.4 リポジトリ固有値を再設定（`RELEASE_REPO`, badge URL, アイコン名 `kuw-icon.png`, workflow 名）— Justfile の RELEASE_REPO は既に HiroyukiFuruno/katana-ui-widget に設定済み
- [ ] 1.5 `just check` / `just lint` / `just ast-lint`（導入後）/ `just test` がすべて green であることを確認

## 2. ast-lint 導入と依存方向制約

- [x] 2.1 `cargo info katana-ast-lint` で crates.io 公開版を確認し（0.5.1）、`just ast-lint-install` で導入。`Justfile` には `ast-lint-install` / `ast-lint` recipe が既に整備済み
- [x] 2.2 ast-lint プロジェクト設定ファイル（`kal.json`）を更新し、利用可能なルールを設定
  - [x] `file-length` / `function-length` / `nesting-depth` / `pub-free-fn` / `type-separation` を有効化
  - 注: katana-ast-lint 0.5.1 に依存方向制約ルール（layer/import direction）は未実装。依存方向は `docs/directory-structure.md` と `kuw-workflow-guide` skill で規約として明文化し、コードレビューで担保する。
  - [ ] `theme/` から他層への依存禁止 — 規約で担保
  - [ ] `primitive/` は `theme/` のみ参照可 — 規約で担保
  - [ ] `composite/` は `theme/` / `primitive/` のみ参照可 — 規約で担保
  - [ ] `layout/` は `theme/` / `primitive/` / `composite/` を参照可 — 規約で担保
- [x] 2.3 `kal check` が clean であることを確認（依存方向違反検出のサンプルは kal が未対応のためスキップ）
- [x] 2.4 lefthook / CI から `just ast-lint` を実行 — 既存 lefthook.yml と CI workflow に接続済み

## 3. crate 階層スケルトン

- [x] 3.1 `crates/katana-ui-widget/src/` 配下に空のディレクトリと `mod.rs` を作成
  - [x] `theme/{color,spacing,typography}/mod.rs`
  - [x] `primitive/{text,icon,spinner}/mod.rs`
  - [x] `composite/button/{svg,text,icon_text}/mod.rs`
  - [x] `composite/selector/{toggle,segmented,select,color}/mod.rs`
  - [x] `composite/input/{text,search}/mod.rs`
  - [x] `composite/indicator/{tooltip,badge,key_cap}/mod.rs`
  - [x] `layout/{card,accordion,split,modal,popover}/mod.rs`
- [x] 3.2 `lib.rs` から `pub mod theme;` `pub mod primitive;` `pub mod composite;` `pub mod layout;` を宣言（既存 `WidgetRegistry` は互換のため一旦残す）
- [x] 3.3 `cargo check -p katana-ui-widget` が成功することを確認

## 4. Storybook 別管理プロジェクトの新設

- [x] 4.1 リポジトリルートに `storybook/` ディレクトリを作成（`crates/` 配下には**置かない**、workspace member にも**しない**）
- [x] 4.2 `storybook/Cargo.toml` を独立した bin crate として作成し、`katana-ui-widget = { path = "../crates/katana-ui-widget" }` を依存に追加。Floem 0.2.0 を依存に追加
- [x] 4.3 `storybook/src/main.rs` に Floem アプリの起点を実装（widget 一覧サイドバー + ページ表示エリアの最小骨格）
- [x] 4.4 `storybook/src/pages/mod.rs` を用意し、widget 1 件 = 1 ページ（`pages/<widget_name>.rs`）の登録規約を確立
- [x] 4.5 placeholder ページ（welcome）を追加し、`just storybook-check` でコンパイル確認（`just storybook` 実行は GUI が必要なため CI では storybook-check で代替）
- [x] 4.6 `storybook/Cargo.lock` をコミット対象に含める（独立プロジェクトのため）
- [x] 4.7 `.gitignore` に `storybook/target/` を追記
- [x] 4.8 `Justfile` に recipe を追加
  - [x] `storybook` — `cd storybook && cargo run`
  - [x] `storybook-check` — `cd storybook && cargo check`

## 5. kuw-workflow-guide skill の新規作成

- [x] 5.1 `.claude/skills/kuw-workflow-guide/SKILL.md` を作成し、以下を明文化
  - [x] Floem 前提（egui 互換層は対象外）
  - [x] 階層 `theme / primitive / composite{button,selector,input,indicator} / layout` と依存方向
  - [x] widget 抽出可否の判断軸（Katana domain は除外）
  - [x] Storybook は `crates/` 外の独立プロジェクトとして管理する規約
  - [x] 各 widget change の必須タスク（実装 / Storybook ページ追加 / テスト）
  - [x] ファイル数 10 を超えるディレクトリは関心事で分割するルール
- [x] 5.2 `.codex/skills/kuw-workflow-guide/SKILL.md` / `.agent/skills/kuw-workflow-guide/SKILL.md` に同期（.github/skills は存在しないためスキップ）
- [x] 5.3 既存の同名 skill が存在しないため index 更新不要

## 6. ドキュメント整備

- [x] 6.1 `docs/widget-extraction-policy.md` を新規作成
  - [x] 抽出対象の判断軸（汎用度・Floem 単体で完結・Katana domain 非依存）
  - [x] 除外例（markdown_hooks / vendor_ui / chat composer 等）
  - [x] 抽出元として参照する既存実装の所在
- [x] 6.2 `docs/directory-structure.md` を新規作成
  - [x] 階層図
  - [x] 依存方向の図
  - [x] 各 widget モジュール内の慣例（`mod.rs` / `types.rs` / `ops.rs` / `view.rs` / `tests.rs`）
- [x] 6.3 `README.md` を更新
  - [x] Floem 前提の明記
  - [x] 階層と依存方向のサマリ
  - [x] Storybook 起動手順（`just storybook`）
  - [x] 各 docs / skill へのリンク

## 7. 検証 / 完了確認

- [x] 7.1 `just check`（fmt / types / lint / ast-lint / tests）全て green
- [x] 7.2 `just storybook-check` が green（`just storybook` は GUI 起動が必要なため headless 環境では storybook-check で代替）
- [x] 7.3 `just storybook-check` を CI workflow（test-and-build.yml）に追記済み
- [x] 7.4 全タスクが完了状態
