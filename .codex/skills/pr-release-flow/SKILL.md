---
name: pr-release-flow
description: katana-ui-core の release/vX.Y.Z ブランチを署名済み1コミットに整え、Draft PR、レビュー、merge後のActions・registry検証までを一貫して進める。
---

# KUC PR・Release フロー

KUC の release/vX.Y.Z ブランチと master 向けPRを扱う。詳細な実装・品質手順は [impl-release workflow](../../workflows/impl-release.md) を参照する。

## 前提確認

```sh
rtk git status --short --branch
rtk git fetch origin --prune --tags
rtk proxy just --list --unsorted
rtk proxy just VERSION=vX.Y.Z release-target-check
```

既存差分を混ぜず、現在のKUC checkoutで `release/vX.Y.Z` を使用する。別repo、追加worktree、`fix/*` や `feature/*` ブランチは作らない。

## PR準備とレビュー

1. [prepare-release-pr](../prepare-release-pr/SKILL.md) の手順で master 基準の差分を署名付き1コミットへまとめる。
2. 新規release branchなら通常の `rtk proxy env KUC_PUSH_CONFIRMED=1 git push -u origin release/vX.Y.Z` を使う。既存PRの更新では対象を確認したうえで `rtk proxy env KUC_PUSH_CONFIRMED=1 git push --force-with-lease origin HEAD:release/vX.Y.Z` を使う。
3. `rtk proxy gh pr create --draft --base master --head release/vX.Y.Z` でDraft PRを作る。既存PRは新規作成せず更新する。
4. Draftのまま `rtk proxy gh pr comment <PR> --body '@codex review'` を行い、自己レビューも差分・受入条件・DoDに対して実施する。
5. 指摘を修正し、各threadへreplyしてresolveする。P0/P1が0件であることを確認してから `rtk proxy gh pr ready <PR>` を実行する。
6. `rtk proxy gh pr checks <PR> --watch` でrequired checksを確認する。

## Merge後と公開確認

1. `rtk proxy gh pr merge --merge <PR>` を使う。`--admin` と `--delete-branch` は使わない。
2. masterへmergeされた後、GitHub ActionsのRelease workflowとcrates.io上の `katana-ui-core` 公開版を確認する。ローカルでtag作成、GitHub Release作成、`cargo publish` の直接実行で迂回しない。
3. registryから取得したKUCのpublic APIを単独consumer smokeで検証する。外部consumerの採用・受入はKUC release gateにしない。
4. 対応Issueの受入証跡を確認してから `rtk proxy gh issue close <ISSUE>` を行う。
5. `branch-hygiene` スキルに従い、merge済みのローカルrelease branchや不要なworktreeだけを整理する。リモートbranchは削除しない。

完了は、レビュー、required checks、merge、Actions、registry、単独smoke、Issue、整理の各証跡が揃った時点とする。
