---
name: prepare-release-pr
description: KUC の release/vX.Y.Z ブランチを master 基準の署名付き1コミットへ整え、新規pushまたは既存PRのforce-with-lease更新に備える。
---

# KUC release branch の署名準備

このskillはコミット履歴をrelease PR用に整える。PR作成、レビュー、merge、公開確認は [PR・Release flow](../pr-release-flow/SKILL.md) と [impl-release workflow](../../workflows/impl-release.md) に従う。

## 事前条件

- checkoutはKUCで、対象branchは `release/vX.Y.Z`。masterを直接編集しない。
- `rtk git status --short --branch` で対象外の差分がないことを確認する。
- `rtk git fetch origin master --prune --tags` 後、`rtk proxy just VERSION=vX.Y.Z release-target-check` を通す。
- 追加worktree、stashによる保全、別repoの操作は行わない。

## 署名付き1コミット

差分とorigin/master基準を確認してから、対象release branch上で次を実行する。

```sh
rtk git merge-base --is-ancestor origin/master HEAD
rtk git diff --check
rtk git diff origin/master --stat
rtk git status --short
rtk git diff --name-only
rtk git ls-files --others --exclude-standard
rtk git reset --soft origin/master
rtk git status --short
rtk git diff --cached --name-status
rtk git add -- <reviewed-target-paths>
rtk git diff --cached --check
rtk git diff --cached --stat
rtk proxy sh -c 'printf "Issue #<N>: <日本語の変更概要>\n\nKUC release/vX.Y.Z\n" > tmp/release-commit-message.txt'
rtk git commit -S -F tmp/release-commit-message.txt
rtk git log -1 --show-signature --oneline
```

`merge-base --is-ancestor` が成功すればorigin/masterの履歴がrelease branchへ統合済みである。その確認後だけ `rtk git reset --soft origin/master` を実行して、release branch固有の差分を1コミットへまとめる。確認が失敗した場合はrelease branch上で `rtk git merge origin/master` を行い、競合を解消して限定検証を通してから同じ確認へ戻る。origin/masterを未確認のまま削除するresetや、未確認の全量stageは行わない。未コミット・未追跡pathは一件ずつ内容を精査し、対象pathだけをstageする。

## Push

- まだremoteに対象release branchを出していない新規PRは、`rtk proxy env KUC_PUSH_CONFIRMED=1 git push -u origin release/vX.Y.Z` を一度だけ使う。
- 既存PRの履歴を署名付き1コミットへ更新する場合は、remoteの先端を確認して `rtk proxy env KUC_PUSH_CONFIRMED=1 git push --force-with-lease origin HEAD:release/vX.Y.Z` を使う。
- `--force`、`--no-verify`、`--admin`、手動tag作成、直接publishは使わない。

push後はDraft PRを維持し、レビュー、指摘reply/resolve、P0/P1ゼロ、Ready化、required checksの順で進める。各工程が失敗した場合は原因を修正して同じ工程へ戻り、別の迂回経路を作らない。公開はmerge後のGitHub Actionsとregistry確認を正とし、ローカルの代替release経路を作らない。
