# v0.4.4 text raster/selection 修正台帳

## DoR・範囲
- [x] #104/#105をOpen Issueとして記録し、外部編集許可へURLを登録する。
- [x] 既存KUC作業sessionとの重複がないこと、checkout clean、stash 0、既存worktree1件を確認する。
- [x] release/v0.4.4を公開0.4.3を含むorigin/masterから作成する。

## 実装・検証
- [x] 幅未指定RTL rasterの失敗するpublic contract回帰を先に置く。
- [x] RTL intrinsic glyph/bounds/run/decoration/hitの座標を修正する。
- [x] soft wrap copyの失敗するpublic contract回帰を先に置く。
- [x] visual wrapとsource newlineを分離し、CR/LF/CRLF/U+2028/U+2029を維持する。
- [/] review P2: inline codeの空白保持をspan単位に限定し、混在する通常本文の空白契約を回帰検証する。
- [x] 公開v0.4.4へversion/内部依存/lockを同期し、全直接・推移依存の最新互換版を評価する。
- [x] CI Actionの公式release/tagを照合し、SHA固定とOIDC/artifact契約を保って更新する（YAML構文、ガード121件、梱包5件が成功）。
- [/] review P1: plain/rich proseのsoft wrapで除かれる語間空白をselection metadataへ保持し、コピーで単語を結合しない（実Canvas cold/warm・全体/逆選択/betaのみ・複数style span、raster-host core1064成功）。
- [x] #107: APT archive取得停止へのrequest timeout/retryを明示し、outer300秒/3attempts・package/font hashを維持する（local guard122成功、実CIは未検証）。
- [ ] focused回帰、fmt、strict lint、AST、完全release-checkを通す。品質gateを緩和しない。

## 公開DoD
- [ ] 自己review、Draft PR、cloud review、全指摘reply/resolve、P0/P1 zero、Ready、required checks、mergeの順を守る。
- [ ] tag/GitHub Release/crates.io取得とIssue解決を確認する。
- [ ] branch-hygieneで不要なローカルbranchとworktreeを確認する。remote branchを明示なく削除しない。

## 検証証跡
- RTL red: `tmp/rtl-intrinsic-red.log`、幅未指定Arabic raster幅4096で1件FAIL。
- RTL green: `tmp/rtl-intrinsic-rounded-green.log`、intrinsic/raw rasterとmixed code/linkの2件PASS。float raw x=-0.000067はpixelの丸め後座標で検査する。
- soft wrap red/green: `tmp/kuc-soft-wrap-copy-fix-results.log`。helper統合・空白の既存HTML契約修正後に `tmp/v044-core-lib-tests-final.log` で全1059件PASS。
- 依存: `tmp/v044-update-safe.log`、`tmp/v044-outdated.log`。
- review P2: `tmp/v044-mixed-inline-code-red.log`で全体空白modeの漏れを再現後、`tmp/v044-mixed-inline-code-green.log`で40/64pxの混在copyと本文空白契約が成功。inline_code 10件、code_font_without_code_role 1件、focused Clippyも成功。
- 完全gate初回: `tmp/v044-release-check.log`、strict lintと全target/featureの4738テストは成功。追加review P2の修正を反映するためcoverage前に停止（exit143）し、完走とは扱わない。修正後に完全gateを再実行する。既存coverage/AST/lint基準を維持。

- Draft PR #106: cloud reviewでprose soft wrap copyの空白欠落P1を取得（discussion_r4355707725）。修正とred/greenを継続する。f9d1baaの完全gateは4739 testsまで成功し、coverageを実行中。修正後HEADの全品質gateは別途再確認し、古いsnapshotの成功を新HEADへ流用しない。

- #107 CI/preflight: runs39892796413/39892796439はAPT archive取得でexit124を3回発生。`tmp/v044-{ci,preflight}-failed.log`。修正は30秒HTTP/HTTPS timeout +Acquire::Retries2、outer300/3維持。`tmp/verification/apt-acquire-green.log`でguard122/122、ShellCheck、bash構文、Python compile成功。実CI再実行後に回収する。

- cloud P1のred/green: `tmp/verification/spanwrap-source-whitespace-red.log`、`spanwrap-core-green.log`（raster-host core1064成功）、`spanwrap-clippy.log`。visual line IDでsource whitespaceを一度だけemitし、plain cacheもmetadataを保持する。公開APIのprivate field以外は変更しない。
- f9d1baa先行snapshotの完全gate42875はexit0。coverage functions14844/14844、lines157930/157942、既存strict LCOV判定成功、package/publish dry-runと未公開確認成功（`tmp/v044-release-check-final.log`）。その後のreview修正は含まれないため、修正後HEADの最終証拠へ流用しない。修正後の`just check`をローカル実行し、同一HEADのCI `release-check`で全coverage/package/publication guardを回収する。検査対象・基準は維持する。

- review修正後の完全 `just check`: session74126 exit0、`tmp/v044-review-fix-check-complete.log`。fmt/AST/types/全strict lint、全target/feature4743（136suites）、native IME10、native contracts22、Unicode6、guard122とconsumer/Storybook contractが成功。修正後HEADのCI完全release-check（coverage/packageを含む）は未回収。
