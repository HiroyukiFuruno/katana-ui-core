---
name: impl-release
description: katana-ui-core のIssue/OpenSpec実装、品質確認、Draft PRのレビュー、GitHub Actionsによる公開、registry検証、事後整理を一貫して進める。
---

# impl-release

KUC自身の要件・公開API・自動検証・公開を完了条件とする。

実行前に[現行リリース手順](../../../.codex/workflows/impl-release.md)を読む。

1. 現在の差分・stash・worktreeを確認し、同じ`release/vX.Y.Z`ブランチで対象差分を統合する。
2. `rtk proxy just VERSION=vX.Y.Z release-target-check`で公開済みrelease lineと対象版を照合する。
3. Issue/OpenSpecの受入条件を実装・自動回帰へ対応付け、依存更新と既存の品質ゲートを完了する。
4. Draft PRを作成し、`@codex review`と自己レビュー、指摘修正・reply/resolve、P0/P1ゼロを経てReadyへ進める。
5. required checks成功後にmergeし、GitHub ActionsのGitHub Releaseとcrates.io公開、registry版だけの検証、Issue Close、branch-hygieneを確認する。
6. 承認済みのcommit/push/merge/release/cleanupは再確認せず継続する。失敗は原因と証跡を回収して修正し、同じ公開フローへ戻る。
7. 外部repositoryの採用・受入・Issue状態をKUCの完了条件へ追加しない。
