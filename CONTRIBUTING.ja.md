# katana-ui-core への貢献

ご貢献いただきありがとうございます！

## 開発環境のセットアップ

```bash
# 開発依存のインストール
cargo install just
just ast-lint-install

# 品質チェックの実行
just check
```

## ワークフロー

1. リポジトリをフォークし、`master` からブランチを作成する。
2. widget 階層ルールに従って変更を加える（`docs/directory-structure.md` 参照）。
3. `just check` がパスすることを確認する（fmt / types / lint / ast-lint / tests）。
4. プルリクエストを作成する。

## Widget 階層

詳細は `docs/directory-structure.md` を参照してください。

## Storybook

新しい widget を追加する際は、`storybook/src/pages/` に対応するページを追加してください。起動コマンド：

```bash
just storybook
```

## 行動規範

すべてのやり取りにおいて、相互尊重と建設的な姿勢を心がけてください。
