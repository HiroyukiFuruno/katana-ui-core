# リリース手順

## 方針

`release/vX.Y.Z` ブランチから `master` へ取り込み依頼（Pull Request）を作る。
取り込み依頼では通常の品質ゲート（quality gate）とリリース前検査を必須にする。
取り込み（merge）後は自動実行基盤（GitHub Actions）がタグ（tag）、GitHubリリース（GitHub Release）、crates.io公開を実行する。
単一の公開crate、private Storybook、consumer contractを同じrelease gateで検証する。

## 必須検査

GitHub のブランチ保護（branch protection）では、KUC repo 内で次を必須検査（required check）にする。

- `Test and Build (macos-latest)`
- `Test and Build (ubuntu-latest)`
- `Test and Build (windows-latest)`
- `preflight`

## リリース前検査

`release-preflight` は通常の取り込み依頼（Pull Request）で `just check` を実行する。
`release/v...` ブランチでは追加で `just VERSION=vX.Y.Z release-check` を実行する。

内容は次の通り。

- 整形確認（format）、静的検査（lint）、単体テスト（unit test）、抽象構文木検査（AST lint）
- カバレッジ（coverage）。関数・行ともに100%を必須にする
- `Cargo.toml` の版番号（version）とブランチ版番号（branch version）の一致
- 対象版番号（version）が公開済みrelease lineから自然な次版であること
- `katana-ui-core` について、対象版番号（version）がcrates.ioに未公開であること
- `katana-ui-core` の梱包（package）と公開の事前実行（publish dry-run）
- workspace package は `katana-ui-core`、`katana-ui-core-storybook`、`kuc-consumer-app` の3 packageに限定すること

## 公開順序

`release/vX.Y.Z` の取り込み（merge）後に `Release` ワークフロー（workflow）が動く。
順序は次の通り。

1. `just VERSION=vX.Y.Z release-check`
2. リリースタグ（release tag）作成
3. GitHubリリース（GitHub Release）作成
4. GitHub Actions の OIDC Trusted Publishing で `katana-ui-core` だけをcrates.ioに公開し、レジストリ反映を確認

## リリース後の自動クリーンアップ

`Release` ワークフローでは `GitHub Release` が公開された後に、公開済み前提でクリーンアップを実行する。

対象:
- `release/vX.Y.Z` 形式の release ブランチ
- ローカル/リモート同時監査（`git branch` と `git branch -r`）

削除条件（ローカル/リモートともに該当）:
- `default branch` ではないこと
- 対象ブランチがデフォルトブランチへ `merged` していること
- 対応する worktree が存在しない（`unused`）
- 対応する worktree が dirty でない（`clean`）

保持条件（削除不能で失敗として報告）:
- `dirty`（未コミット差分あり）
- `unmerged`（`merged` 条件を満たさない）
- `in-use`（worktree で参照中）
- `default`（デフォルトブランチ）

保持対象が存在した場合、`cleanup` は失敗扱いとなり、release ワークフローでの失敗報告トリガーとなる。

## crates.io 認証

公開は `rust-lang/crates-io-auth-action` と crates.io Trusted Publishing の
OIDC連携だけで行う。長期の `CARGO_REGISTRY_TOKEN`、`cargo login`、ローカルからの
`cargo publish` は使用しない。Trusted Publisher は次の組み合わせを登録し、Actions が
発行した短期 token を `publish-crates.sh` に渡す。

- `katana-ui-core` + `.github/workflows/release.yml`
- `katana-ui-core` + `.github/workflows/release-publish-retry.yml`

retry workflowは単一crateへ統合済みのrelease sourceだけを受け付ける。旧分割crateを
列挙するtagはOIDC認証前にfail closedとし、対応していない認証経路で一部だけを
再公開しない。
