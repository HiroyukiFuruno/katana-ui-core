---
name: pr-release-flow
description: PRの更新（squash・署名付きコミット）からrelease実行までを同じ運用で回す。`/pr-release-flow` で実行し、CI再発防止の順番を守る。
---

# PR・Release 共通運用フロー

このスキルは `/pr-release-flow` として使う。

## 使う場面
- PRを更新して再度 merge 可能状態へする
- release前の PR 前提を揃えたい
- release実行と検証までの運用を固定したい

## 実行順（固定）

1. 事前確認
   - `git status --short --branch`
   - `git fetch origin --prune --tags`
2. PRをsquash署名済み1コミット化
   - 署名前確認: `PR_NUMBER=<n> PR_NO_PUSH=1 just prepare-release-pr`
   - 実行前に PR head 名が `release/v` であることを確認。`release-v` 系なら先に `release/v` に rename してから squash。
   - push付き: `PR_NUMBER=<n> just prepare-release-pr`
3. PR CI確認
   - `gh pr checks <n> --repo HiroyukiFuruno/katana-markdown-linter --watch`
4. Merge
   - `gh pr merge --merge --delete-branch <n>`
5. release（必要時）
   - `git switch main && git pull --ff-only origin main`
   - `just VERSION=vX.Y.Z release-target-check`
   - `just VERSION=vX.Y.Z release-check`
   - `just VERSION=vX.Y.Z release`
   - `just VERSION=vX.Y.Z release-verify`

## 必須ルール

- `prepare-release-pr` の PR更新は `squash` + `-S`（署名）に固定。
- `release/v` 以外（例: `release-v*`）のブランチは受け付けず、squash 前に rename する。
- Mergeは `--admin` を使わない。
- `main` を必ずベースにして確認。
- `release` targetの失敗時は別ルート `gh workflow run` に逃げない。

## 参照

- `.codex/workflows/pr-release-runbook.md`
- `.codex/skills/prepare-release-pr/SKILL.md`
- `just/release.just` の `prepare-release-pr`
