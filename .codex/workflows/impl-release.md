---
description: KUCのIssue/OpenSpec実装から検証、Draft PRのレビュー、Actions公開、registry検証、整理までを進める。
---

# /impl-release vX.Y.Z

KUC自身の要件・public API・自動検証・公開で完了を判定する。
対象範囲とDoDを物理タスクへ固定し、外部repositoryの採用・受入・Issue状態を公開条件へ加えない。

## 状態と対象版

- 対象は現在のKUC checkout、既定branchはmaster、作業branchはrelease/vX.Y.Z。
- 差分、未追跡ファイル、stash件数、ahead/behind、使用中worktreeを確認する。
- 既存release branchがあれば継続する。追加worktreeや保全用stashを作らない。
- 対象Issue/OpenSpecと依存関係、公開済みrelease lineを照合する。

```sh
rtk git fetch origin --tags
rtk proxy just --list --unsorted
rtk proxy just VERSION=vX.Y.Z release-target-check
```

## 実装と検証

1. Issue/OpenSpec、tasks、最新ユーザー指示を一次情報として受入条件を固定する。
2. 複数Issueを一つのPR・releaseへまとめる依頼では、対象を分割公開しない。
3. 直接・推移依存とlockfileを確認し、既存品質を維持できる最新互換版へ更新する。
4. KUC単独の公開APIから入力・state・event・action・layout/rendering contractを自動検証する。
5. 成功した検査だけを完了にし、終了コード・ログ・対象sourceを記録する。

```sh
rtk proxy just check
rtk proxy just VERSION=vX.Y.Z release-check
rtk git diff --check
```

文書・ルールだけの是正はリンク、コマンド、対応guardを検証し、製品full gateを追加する理由にしない。

## Draft PRとレビュー

1. [署名準備](../skills/prepare-release-pr/SKILL.md)に従い、対象差分をorigin/master基準の署名付き1コミットへまとめる。
2. 承認済みpushはKUC_PUSH_CONFIRMED=1を渡して実行する。
3. master向けのDraft PRを一つ作成する。Codexでは、PRの作成後にこのchatへ添付する。
4. Draftのまま@codex reviewと自己レビューを実施し、指摘を回収する。
5. P0/P1を修正する。P2/P3は根拠付きで判断し、要件・互換性・DoDへの影響があればユーザーへ相談する。
6. 各threadへ修正・判断をreplyしてresolveし、未対応指摘とP0/P1が0件であることを確認する。
7. Ready for reviewへ昇格し、同じheadのrequired checksが成功したことを確認する。

## Mergeと自動公開

- 承認済みのcommit/push/merge/release/cleanupは再確認せず継続する。
- 通常のmergeを使い、強制mergeやリモートbranch削除を指定しない。
- release/vX.Y.Zからmasterへのmerge後に起動するGitHub Actionsを公開の正とする。
- ローカルのtag作成、GitHub Release作成、crate直接公開へ迂回しない。
- CI/build/レビューや公開失敗は結果と原因を回収して修正し、同じフローを継続する。

```sh
rtk proxy just release-status
rtk proxy gh run list --repo HiroyukiFuruno/katana-ui-core --workflow Release --limit 5
```

## 公開後と整理

1. Release workflow成功、GitHub Releaseの版・commit、crates.ioの公開版を個別に確認する。
2. 公開registryから取得したKUCだけで、公開APIのcompileと対象入力回帰を実行する。
3. 各Issueの受入証跡を記録してCloseする。本人確認や外部採用をClose条件へ追加しない。
4. branch-hygieneに従い、masterを同期し、不要なローカルrelease branchを安全に整理する。
5. stash 0、master clean、作業branchのahead/behind、使用中worktreeを再確認し、終了用台帳を実際の証跡で更新する。

## 確認が必要な場合

DoDや重要な実装選択を特定できない場合、または未承認の不可逆操作・repo越境・設定変更が必要な場合に絞る。
実装・検査の失敗や既存差分の統合は、原因を直して継続する作業として扱う。

## 完了条件

- 対象Issue/OpenSpecの実装と受入回帰、依存更新、既存品質ゲートが完了している。
- Draftレビュー、指摘対応・reply/resolve、Ready、required checks、mergeの証跡がある。
- ActionsのGitHub Release/crates.io公開とregistry単独検証が成功している。
- Issue Close、branch-hygiene、終了用台帳の更新が完了している。
