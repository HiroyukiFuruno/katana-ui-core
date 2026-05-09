---
name: impl-release
description: katana-ui-widgetで指定バージョンのOpenSpec実装、品質確認、リリースPR、GitHub Release、crates.io publish、事後整理までを一気通貫で進めるときに使う。/impl-release vX.Y.Z と同等のリリース実装ワークフロー。
---

# impl-release

このスキルは、`/impl-release vX.Y.Z` として扱うリリース実装ワークフローの入口です。

## 実行ルール

1. ユーザー指定のバージョン（例: `v0.12.2`）を対象にする。
2. ユーザー指定のバージョンを無条件に正しいものとして扱わない。
3. 作業開始前に `just VERSION=vX.Y.Z release-target-check` を実行し、公開済み release line から見て自然な次版であることを確認する。
4. `v0.17.6` の次に `v0.18.7` を指定するような不自然な飛び番は停止し、`v0.17.7` か `v0.18.0` かをユーザーに確認する。
5. `KUW_RELEASE_ALLOW_VERSION_LINE_OVERRIDE=1` は、修正リリースなどの理由をユーザーが明示承認した場合だけ使う。
6. 詳細手順は `.codex/workflows/impl-release.md` を正として読み込む。
7. OpenSpec 実装、検証、リリース準備、PR 作成、自己レビュー、公開、事後整理まで進める。
8. 作業開始前に必ず `git status --short --branch` を確認し、既存差分と関心事を混ぜない。
9. `just VERSION=vX.Y.Z release` が失敗した場合、代替の tag / publish コマンドで迂回しない。状態を調査し、修正してから同じフローへ戻る。

## 注意

- `workflows/impl-release.md` は手順書であり、Codex のスキル呼び出し対象ではない。
- `/` から呼び出したい場合は、この `impl-release` スキルを使う。
