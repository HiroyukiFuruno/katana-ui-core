# Tasks — 00-bootstrap-dev-environment

## 1. KML 差分取り込み（katana-series-repository-standardization skill 準拠）

- [ ] 1.1 `KML_ROOT=$HOME/works/private/katana-markdown-linter`、`TARGET_ROOT=$HOME/works/private/katana-ui-widget` を設定し、skill の「1) 差分比較」を `DRY_RUN=1` で実行して差分一覧を取得する
- [ ] 1.2 取り込み対象を確定する（KUW 固有理由で外すものは `docs/widget-extraction-policy.md` に根拠を残す）
  - [ ] `clippy.toml`
  - [ ] `rustfmt.toml`
  - [ ] `.markdownlint.json`
  - [ ] `CHANGELOG.md` / `CHANGELOG.ja.md`（空ひな型でよい）
  - [ ] `CONTRIBUTING.md` / `CONTRIBUTING.ja.md`
  - [ ] `cliff.toml`
  - [ ] `.gitignore` の差分（`storybook/target/` も追記）
  - [ ] `Justfile` の不足 recipe（`sweep` / `clean` 等）
  - [ ] `lefthook.yml` の差分
  - [ ] `docs/release.md` の差分
  - [ ] `.github/workflows/{test-and-build,release-preflight,release}.yml` の差分
  - [ ] `scripts/release/*` / `scripts/openspec` の差分
- [ ] 1.3 KML にあって KUW に未配置の skill を `rsync` で取り込む
  - [ ] `harness-engineering`
  - [ ] `bulk-modification-protocol`
  - [ ] `self-review`
  - [ ] `lint-and-ast-lint`（または相当）
  - [ ] `changelog-writing`
  - [ ] `pr-release-flow` / `prepare-release-pr`
- [ ] 1.4 リポジトリ固有値を再設定（`RELEASE_REPO`, badge URL, アイコン名 `kuw-icon.png`, workflow 名）
- [ ] 1.5 `just check` / `just lint` / `just ast-lint`（導入後）/ `just test` がすべて green であることを確認

## 2. ast-lint 導入と依存方向制約

- [ ] 2.1 `cargo info katana-ast-lint` で crates.io 公開版を確認し、`cargo install katana-ast-lint --version <確認したversion> --locked --force` で導入。`Justfile` に `ast-lint-install` / `ast-lint` recipe を整備
- [ ] 2.2 ast-lint プロジェクト設定ファイルを追加し、以下の依存方向ルールを定義
  - [ ] `theme/` から他層への依存禁止
  - [ ] `primitive/` は `theme/` のみ参照可
  - [ ] `composite/` は `theme/` / `primitive/` のみ参照可。異なるサブカテゴリ（`button` / `selector` / `input` / `indicator`）間の参照は禁止、同一サブカテゴリ内の sub-widget 間は許可
  - [ ] `layout/` は `theme/` / `primitive/` / `composite/` を参照可
- [ ] 2.3 ルール違反のサンプルを 1 ケース仕込み、ast-lint が検出することを確認後、削除
- [ ] 2.4 lefthook / CI から `just ast-lint` を実行するように接続

## 3. crate 階層スケルトン

- [ ] 3.1 `crates/katana-ui-widget/src/` 配下に空のディレクトリと `mod.rs` を作成
  - [ ] `theme/{color,spacing,typography}/mod.rs`
  - [ ] `primitive/{text,icon,spinner}/mod.rs`
  - [ ] `composite/button/{svg,text,icon_text}/mod.rs`
  - [ ] `composite/selector/{toggle,segmented,select,color}/mod.rs`
  - [ ] `composite/input/{text,search}/mod.rs`
  - [ ] `composite/indicator/{tooltip,badge,key_cap}/mod.rs`
  - [ ] `layout/{card,accordion,split,modal,popover}/mod.rs`
- [ ] 3.2 `lib.rs` から `pub mod theme;` `pub mod primitive;` `pub mod composite;` `pub mod layout;` を宣言（既存 `WidgetRegistry` は互換のため一旦残す）
- [ ] 3.3 `cargo check -p katana-ui-widget` が成功することを確認

## 4. Storybook 別管理プロジェクトの新設

- [ ] 4.1 リポジトリルートに `storybook/` ディレクトリを作成（`crates/` 配下には**置かない**、workspace member にも**しない**）
- [ ] 4.2 `storybook/Cargo.toml` を独立した bin crate として作成し、`katana-ui-widget = { path = "../crates/katana-ui-widget" }` を依存に追加。Floem を依存に追加
- [ ] 4.3 `storybook/src/main.rs` に Floem アプリの起点を実装（widget 一覧サイドバー + ページ表示エリアの最小骨格）
- [ ] 4.4 `storybook/src/pages/mod.rs` を用意し、widget 1 件 = 1 ページ（`pages/<widget_name>.rs`）の登録規約を確立
- [ ] 4.5 placeholder ページ（"Welcome" 等）を 1 つ追加し、`cargo run` で起動・表示確認
- [ ] 4.6 `storybook/Cargo.lock` をコミット対象に含める（独立プロジェクトのため）
- [ ] 4.7 `.gitignore` に `storybook/target/` を追記
- [ ] 4.8 `Justfile` に recipe を追加
  - [ ] `storybook` — `cd storybook && cargo run`
  - [ ] `storybook-check` — `cd storybook && cargo check`

## 5. kuw-workflow-guide skill の新規作成

- [ ] 5.1 `.codex/skills/kuw-workflow-guide/SKILL.md` を作成し、以下を明文化
  - [ ] Floem 前提（egui 互換層は対象外）
  - [ ] 階層 `theme / primitive / composite{button,selector,input,indicator} / layout` と依存方向
  - [ ] widget 抽出可否の判断軸（Katana domain は除外）
  - [ ] Storybook は `crates/` 外の独立プロジェクトとして管理する規約
  - [ ] 各 widget change の必須タスク（実装 / Storybook ページ追加 / テスト / 必要なら ast-lint ルール追加）
  - [ ] ファイル数 10 を超えるディレクトリは関心事で分割するルール
- [ ] 5.2 `.claude/skills/kuw-workflow-guide/SKILL.md` / `.agent/skills/kuw-workflow-guide/SKILL.md` / `.github/skills/kuw-workflow-guide/SKILL.md` にも同期
- [ ] 5.3 既存 `kcu-workflow-guide` / `katana-workflow-guide` を踏襲した index 更新が必要なら反映

## 6. ドキュメント整備

- [ ] 6.1 `docs/widget-extraction-policy.md` を新規作成
  - [ ] 抽出対象の判断軸（汎用度・Floem 単体で完結・Katana domain 非依存）
  - [ ] 除外例（markdown_hooks / vendor_ui / chat composer 等）
  - [ ] 抽出元として参照する既存実装の所在（`../katana/crates/katana-ui/`、`../katana-chat-ui/crates/katana-chat-ui-floem/`）
- [ ] 6.2 `docs/directory-structure.md` を新規作成
  - [ ] 階層図
  - [ ] 依存方向の図
  - [ ] 各 widget モジュール内の慣例（`mod.rs` / `types.rs` / `ops.rs` / `view.rs` / `tests.rs`）
- [ ] 6.3 `README.md` を更新
  - [ ] Floem 前提の明記（既存）
  - [ ] 階層と依存方向のサマリ
  - [ ] Storybook 起動手順（`just storybook`）
  - [ ] 各 docs / skill へのリンク

## 7. 検証 / 完了確認

- [ ] 7.1 `just check` / `just lint` / `just ast-lint` / `just test` 全て green
- [ ] 7.2 `just storybook` で Storybook が起動し placeholder ページが表示される
- [ ] 7.3 `just storybook-check` が CI でも実行できることを確認（CI workflow 更新が必要なら反映）
- [ ] 7.4 `openspec/changes/00-bootstrap-dev-environment/` を完了状態で `openspec verify` 相当の確認
