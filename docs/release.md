# リリース手順

## 方針

`release/vX.Y.Z` ブランチから `master` へ取り込み依頼（Pull Request）を作る。
取り込み依頼では通常の品質ゲート（quality gate）とリリース前検査を必須にする。
取り込み（merge）後は自動実行基盤（GitHub Actions）がタグ（tag）、GitHubリリース（GitHub Release）、crates.io公開を実行する。
4つの公開crate、private Storybook、consumer contractを同じrelease gateで検証する。

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
- 4つの公開crateについて、対象版番号（version）がcrates.ioに未公開であること
- `katana-ui-core` の梱包（package）と公開の事前実行（publish dry-run）
- 依存する3 crateの梱包内容確認。未公開の同版coreを事前解決できないため、通常の検証付きpublishは公開時に依存順で行う

## 公開順序

`release/vX.Y.Z` の取り込み（merge）後に `Release` ワークフロー（workflow）が動く。
順序は次の通り。

1. `just VERSION=vX.Y.Z release-check`
2. リリースタグ（release tag）作成
3. GitHubリリース（GitHub Release）作成
4. `katana-ui-core` をcrates.ioに公開し、レジストリ反映を確認
5. `katana-ui-core-text-raster` と `katana-ui-core-svg-raster` を公開し、各反映を確認
6. `katana-ui-core-egui-adapter` を公開し、反映を確認

## 必要な秘匿値

自動実行基盤（GitHub Actions）には次の秘匿値（secret）が必要。
値はcrates.ioの API トークン（API token）を使う。

repo root で実行する。

```bash
gh secret set CARGO_REGISTRY_TOKEN
```

トークン（token）は秘匿値として扱い、リポジトリ（repository）に保存しない。
