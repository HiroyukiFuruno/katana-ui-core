# v0.5.0 公開境界監査

## 判定方針

KUCの要件・public API・自動検証・公開で完了を判断する。hostの本文store、検索処理、
ファイル解釈、永続化、操作採否をKUCに移さず、opaque targetとtyped operationで接続する。
他repositoryの採用・受入・Issue状態はKUCのrelease条件に含めない。

## 是正内容

- production APIコメントに残っていた特定host名を汎用契約へ是正した。
- 現行のUI分離計画、root計画、AST改善計画、fallible root設計、作業対応表をhost非依存へ改めた。
- AGENTSの旧DoDと外部受入を要求する記述を、KUC自身のpublic API契約とregistry検証へ整合させた。
- Storybookの表示用説明・設定名から特定hostを除いた。
- Storybookの未使用domain model dev-dependencyを削除した。
- 過去に公開した版のrelease/ADR記録、Issue要求原文は履歴証拠として保持する。

## 未接続コード

Storybookのdocument viewer adapter残骸はmodule graph未接続で、独自domain型を持つ68ファイル。
lib/main、includeによる参照がないことと、汎用部品へ救済すべき独立テスト・WHYがないことを監査済み。
署名付きcheckpoint b46c4c94c5f03223de258e349a7b77fd2625a2a3 後、68ファイル・9253行を削除した。
残存module/include参照なし、削除差分の対象と空白検査を確認済み。最終release-checkは作業台帳で追跡する。

## 再発防止

- `assert-core-public-api-neutral.sh`: crates/examplesのRustコメントをテストも含めて走査。
  line/doc/block/nested commentとRust literal/lifetimeを区別し、外部host固有名を拒否する。
  現行公開文書も対象に含める。
- `test_core_public_api_neutral.py`: 一時fixtureで違反commentの拒否、汎用修正後の成功、
  lifetime後の違反、literal内の非検出を実際のguardで検証する。
- `assert-core-dependency-boundary.sh`: coreの依存treeとStorybook manifestからdomain依存を拒否する。
- 公開rootのfile/search、tab/group、Unicode観測、UI selectionの回帰は、独自host widgetや
  座標補正なしで入力・state・typed effect・AX boundsを検証する。

名称監査だけを独立動作の証明にはせず、release-checkと公開registry smokeを別途実行する。

## 運用ガイダンスの既存違反

別製品用harnessと旧Floem-only guideの8コピーを、参照なし・checkpoint内容一致を確認して退避した。
PR手順2skill、impl入口4skill、canonical release workflowをKUCのmaster/Draft/Actions手順へ整合した。
active SKILL/workflowも同じneutral guardへ含め、外部固定コマンド・大文字略称・必須文書欠落のfail-closedを4件の実回帰で検証した。
