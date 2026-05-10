---
name: prepare-release-pr
description: PRブランチをmain基準で1コミット化し、gpg署名付きにまとめて更新する。`/prepare-release-pr` で実行し、PR merge要件（Verified）を満たす状態へ寄せる。
---

# PR署名前処理（準備）

このスキルは、`/prepare-release-pr` として呼び出します。  
`prepare-release-pr` は PR 側の差分を `main`（既定）へ `squash` して1コミット化し、署名します。

## 実行する内容

1. `just prepare-release-pr` を実行し、次をまとめて行う。
2. リモートPRブランチを `PR_BRANCH` で受け取り、`release/vX.Y.Z` プレフィックスであることを確認して指定ベース(`PR_BASE_BRANCH`, 既定 `main`)へ `git reset --soft` する。
3. 変更を `gpg` で1コミットにまとめる。
4. `git push --force-with-lease` で対象PRブランチを更新する。

## 変数

- `PR_BRANCH`（必須）  
  - 例: `origin/release/v0.18.0-7963409835081887532` か `release/v0.18.0-7963409835081887532`
  - `release-v` 系のブランチ名（例: `origin/release-v0.18.0`）は受け付けない。`release/v0.18.0` に rename してから実行する。
- `PR_NUMBER`（任意）
  - PR番号を直接与える（例: `85`）
  - `PR_BRANCH` が未指定なら `gh pr view` でブランチ名を解決する
- `PR_REMOTE`（任意）  
  - 既定: `origin`
- `PR_BASE_BRANCH`（任意）  
  - 既定: `main`
- `PR_LOCAL_BRANCH`（任意）  
  - 既定: `fix/<branch>-signed`
- `PR_COMMIT_MESSAGE`（任意）  
  - 既定: `Prepare release branch for merge checks`
- `PR_NO_PUSH`（任意）  
  - `1` / `true` ならローカルコミットまでで終了し、push をしない。

## 事前チェック

- `git status --short` がクリーンであること。
- 対象PRブランチにコミット対象差分があること（差分が無ければ停止）。
- GPGキー設定済みで `git commit -S` が通る環境であること。
- PR ブランチ名が `release/v` で始まること。
  - `origin/release-vX.Y.Z` など別フォーマットは受理せず、先に `release/vX.Y.Z` に rename してから再実行する。

## 運用ルール（他のPR/Releaseに共通）

- PRの更新は`squash + 署名1コミット`に統一。
- `PR_NO_PUSH=1` で事前署名検証をしてから push する。
- PR更新後は `gh pr checks <PR>` を確認してから merge。

## 実行コマンド

```sh
cd /Users/hiroyuki_furuno/works/private/katana-markdown-linter
git fetch --all --prune
git branch -m release-v0.18.0-7963409835081887532 release/v0.18.0-7963409835081887532
git push origin --delete release-v0.18.0-7963409835081887532
git push origin release/v0.18.0-7963409835081887532
PR_BRANCH=origin/release/v0.18.0-7963409835081887532 just prepare-release-pr
PR_NUMBER=85 just prepare-release-pr
PR_NUMBER=85 PR_NO_PUSH=1 just prepare-release-pr
```
