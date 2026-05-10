---
name: kml-harness-engineering
description: katana-markdown-linter の検証ハーネス（harness）、静的検査（lint）、回帰テスト、ドッグフード（dogfood）、上流 markdownlint 比較、リリース前確認を強化するときに使う。失敗の再発防止、手順の永続化、自動化、エージェント向け評価導線の追加を求められた場合も使う。
---

# katana-markdown-linter ハーネスエンジニアリング（Harness Engineering）

このスキルは、このリポジトリで AI エージェントが同じ失敗を繰り返さないために、注意書きではなく実行できる検証導線へ落とすための入口です。

## 基本方針

1. 作業前に `git status --short` と `just --list --unsorted` を確認する。
2. 自己流コマンドより `Justfile` のターゲットを優先する。
3. 失敗や曖昧さを見つけたら、テスト、静的検査（lint）、比較ハーネス、スキル、再利用スクリプトのどれかへ昇格する。
4. このリポジトリは汎用 Markdown リンターであり、KatanA の画面、Problems パネル、`MarkdownDiagnostic` などの消費側概念を契約へ混ぜない。
5. 変更しただけで完了扱いにしない。該当する評価を実行してから報告する。

## 追加先の選び方

- Markdown ルールの意味論: `tests/fixtures/rule-fixture-matrix.json`、`tests/fixtures/rule-fixture-matrix.md`、`tests/rule_fixture_harness.rs`、または対象ルールの `tests/*_regressions.rs` に追加する。
- コマンドライン（CLI）契約: `tests/cli_*_contract.rs` に追加し、狭い確認から始める。
- 公開 Markdown と文書品質: `README.md` と `docs/` は英語だけにし、`just ast-lint` と `just dogfood` を評価に入れる。
- 上流 markdownlint 互換: `just upstream-golden`、必要なら `just upstream-golden-live`、既定ブランチ差分なら `just upstream-drift` を使う。
- 修正処理の収束性: `tests/cli_convergence_contract.rs`、`just public-confidence`、必要なら `just dogfood-fix` 後の差分確認を使う。
- 性能や大きなロジック変更: `just perf-check`、厳格確認が必要なら `just perf-check-strict` を使う。
- リリース前品質: `just VERSION=vX.Y.Z release-check` を基準にする。

## スクリプト化の基準

- 使い捨てスクリプトをリポジトリ直下へ置かない。
- CI 用は `scripts/ci/`、上流比較は `scripts/upstream/`、ベンチマークは `scripts/bench/`、リリース用は `scripts/release/` に置く。
- OpenSpec を呼ぶ手順やスキルでは、裸の `openspec` ではなく `scripts/openspec` を使う。
- 既存の検証ターゲットへ接続できる場合は、単独スクリプトで終わらせず `Justfile` へ接続する。

## 評価手順

1. まず変更点に最も近い狭いテストを実行する。
2. その後、関係する `just` recipeを実行する。
3. スキルやエージェント手順を更新した場合は、スキル検証スクリプトで `SKILL.md` を確認し、`just ast-lint` で agent instruction まわりの禁止表現も確認する。
4. ベースライン更新は、意図した診断変化や性能変化を説明できる場合だけ行う。

## 報告

報告では、次だけを簡潔に書く。

- 何を機械的に防げるようにしたか。
- 実行した評価コマンドと結果。
- まだ人間判断が必要な残り。
