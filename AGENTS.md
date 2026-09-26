<!-- rtk-instructions v2 -->
## RTK

- 全シェルコマンドは先頭に `rtk` を付ける。専用filterがないコマンドは `rtk proxy <command>` を使う。
- `&&` で連結する各コマンドにも `rtk` を付ける。
- raw出力が必要な場合も `rtk proxy <command>` を使う。
<!-- /rtk-instructions -->

---

# katana-ui-core repository rules

## 承認済みリリースの継続

- 目的と完了条件が明確なら、承認済みの commit / push / merge / release / cleanup は再確認せず実行する。進捗報告でターンを終了しない。
- 元タスクがarchive・中断された場合は、同じ作業ツリーの実行状況と差分を確認し、競合がなければ残作業を引き継ぐ。引継ぎ待ちを新たな承認条件にしない。
- coverage失敗は既存LCOVの未到達行を先に修正し、限定テスト後に最終release-checkを実行する。終了コード・ログ・session_idを保持し、稼働targetをcleanしない。
- 文書・ルールだけの変更を理由に製品full gateを追加しない。ルール整理を製品リリースの代わりにしない。
- 公開後のconsumer受入がDoDに含まれる場合、GitHub Release/crates.io公開だけで完了にしない。受入未達をIssueへ記録するだけで停止せず、修正・検証を継続する。

## Storybook の扱い

Storybook は、利用者や開発者が KUC の部品を実画面で触り、見た目、操作感、設定変更時の振る舞いへフィードバックするための画面である。
Storybook を「確認環境」「目視確認の場」「静的な部品一覧」として扱ってはならない。
部品の正しさ、01〜24 の完了、v0.1.0 の release readiness は、Storybook やユーザー操作ではなく、自動テスト、数値化された layout / rendering contract、入力回帰、state / event / action contract、guard で判定する。

## 01〜24 と v0.1.0 DoD

01〜24 は、見た目だけ、文字だけ、同じ骨格の preview、ログだけの変化では完了にしない。
各 UI は option、action、event、state、preset、preview、settings、対応する自動テストを持ち、要件行から検証コードへ追跡できる必要がある。
v0.1.0 の DoD は、`katana` と `katana-chat-ui` が `katana-ui-core` だけで app UI を構築できるだけの public API と contract が揃っていることとする。

## 画像証跡の扱い

画像、スクリーンショット、手動目視、Storybook 操作を完了根拠にしてはならない。
表示の品質は、可能な限り bounds、alignment、hit target、state transition、render command、theme token、font metrics などの数値化された契約で検査する。
画像を補助証跡として増やす前に、同じ要求を自動テストまたは guard へ落とし込む。

## Guard の置き場所

KUC 固有の制約はこの repository の OpenSpec、docs、`scripts/`、Rust tests に固定する。
KUC 固有の都合を `kal` 側へ追記してはならない。

## runner 停止条件

v0.1.0 release readiness が未達の間、runner は未完了の 01〜24 / Storybook / 自動テスト / guard を自律的に消化し続ける。
ローカル実装、テスト、静的検査（lint）、OpenSpec 更新、ローカル保存（commit）は、ユーザーが「続けて」と明示している作業では停止理由にしない。

停止して確認するのは、未承認の外部送信（push）、公開（release）、破壊的操作、または repository 外の実装へ踏み出す場合だけとする。
push confirmation required / release confirmation required / destructive operation confirmation required は未承認の場合だけ適用し、承認済みの工程では停止しない。
それ以外で作業が残っている場合は、次の未完了タスクを選び、実装と自動テストへ進む。

## 進捗報告

リリース作業の完了率は、事前に固定した最上位ステップのうち、DoD と検証証跡を満たして完了した数だけで算出する。
進行中ステップは、内部の修正件数や部分テストの通過数にかかわらず完了数へ加えない。内部進捗は完了率と分離して報告する。
例として 5 ステップ中 2 ステップだけが完了し、3 番目が coverage 99% でも、完了率は `2 / 5 = 40%` とする。

## repository hook

`.githooks/pre-commit` は `just kuc-guardrails` を実行し、停止条件の誤り、Storybook の完了根拠化、KUC 固有 guard の欠落を commit 前に検出する。
hook が失敗した場合はユーザー確認で止まらず、失敗内容を修正して続行する。
