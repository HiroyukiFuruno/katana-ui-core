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
