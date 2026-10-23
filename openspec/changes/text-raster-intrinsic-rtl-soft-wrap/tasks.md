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
- [/] review P1（再review）: aligned/scaled blitのrun再構築でもvisual-line metadataを保持し、転送後のprose/rich copyを回帰検証する（discussion_r4356036444）。
- [/] review P1（3回目）: oversized wordの文字分割後にも原文の語間空白を保持する（discussion_r4356221362）。実Canvasでplain/rich・別span空白・明示区切りを回帰検証した。更新HEADへのpush・thread返信/resolve・再reviewは公開DoDで継続する。
- [/] review P1（4回目）: 非oversized語の直後に空白だけのspanがoverflowした場合も、すべての非明示visual境界へ後続spanの原文空白を蓄積する（discussion_r4356701474）。push後のthread返信/resolveと更新HEADの再reviewを公開DoDで継続する。
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

- exact102c18eへの再cloud reviewでblit後の語間空白欠落P1を取得。最初のP1はpush後reply/resolve済み（discussion_r4356015992）。再指摘の修正・検証・push・該当thread返信/resolveと更新後HEADの完全CIが終わるまでDraftを維持する。

- 再reviewのblit P1は実Canvas aligned/scaledでred後、metadata転送によりfocused6件とstrict Clippyが成功（`tmp/verification/v044-blit-soft-wrap-{red,green}.log`）。root全体check初回は追加helperの数値定数、次はその整形差分で停止したため、どちらも成功証拠としない。
- 102c18eのUbuntu CIはrich beta-only試験でglyph下端41が次行top40へ入る座標を検出しFAIL（`tmp/v044-review-ci-tests-failed.log`）。選択点をglyph中央の同じ高さへ修正し、期待値betaは維持する。Linux実fontでfocused再検証する。
- #107の30秒Acquire設定だけではpreflight39899020226もexit124を3回発生。3回目の取得64.7MB/4分57秒でdpkg完了時間が残らない。`tmp/v044-preflight-acquire-timeout-failed.log`。outer300秒/3回は維持し、低速Azure mirrorから既存公式HTTPS mirrorへ限定切替を実装・検証する。

- 最終working treeの完全 `just check`: session74136 exit0、`tmp/v044-blit-mirror-full-check.log`。4746 tests/136 suites、fmt/AST/全strict Clippy、native IME10/native contracts22、guard125、Storybook77が成功。Linux/arm64の既存coverage image内でもprose copy6件が成功（`tmp/verification/v044-prose-linux-center.log`、ゼロ件だった初回呼出しは成功証拠から除外）。公開前の同一HEAD CI完全release-checkは引き続き必須。
- Azure mirror限定fallbackは4件のunit、ShellCheck、bash構文、Python compileと上記guard125で成功。署名・suite・component・package/font hash・outer300秒/3回を維持。実hosted CIの成功は未回収。

- exact438fed0の3回目cloud reviewでoversized word直後の語間空白欠落を取得。blit P1はpush後discussion_r4356195659へ返信してresolve済み。新P1修正後のHEADで再reviewと完全CIを必要とする。438fed0のpreflightはAPT取得を通過しRelease check実行中だが、新修正の最終証拠とはしない。

- 3回目P1の追加red/green: `tmp/verification/pr106-crossspan-red.log`で、oversized word後の空白だけのspanと次spanの明示改行の組合せを再現した。空のvisual lineだけで原文空白を区切りへ移し、非空lineのprefixを重複させない。CR/LF/CRLF/U+2028/U+2029とspan間CRLF、既存の非隣接CR・空白・LF正規化を維持した。Linuxのcopy contract16件とwrap-state8件、fmt・AST・focused Clippy・diff-checkが成功（`pr106-crossspan-green-final.log`、`pr106-copy-contracts-final2.log`、`pr106-wrap-state-final2.log`、`pr106-final-{fmt,ast,clippy}.log`）。
- 先行fullcheck88628は追加cross-span欠落の修正前にrootが停止しexit143となったため成功証拠から除外する。修正後の完全 `just check` はsession30752、`tmp/v044-crossspan-final-full-check.log`で実行中。完了結果は回収後に記録する。

- session30752はmacOS arm64の生成済みrlib内の内部LLVMシンボル欠落でlink失敗（exit101）。対象KUCのdev cacheだけを `cargo clean -p katana-ui-core --profile dev` で再生成した後、同じ完全 `just check` を `CARGO_INCREMENTAL=0` で再実行し、session47204がexit0となった（`tmp/v044-crossspan-clean-full-check.log`）。全target/feature4754件・136suites、native IME10、native contracts22、Unicode6、guard125、Storybook77、fmt/AST/types/全strict Clippyとconsumer契約が成功。ソースや品質gateは緩和していない。修正後HEADの完全CI release-check（strict coverage/packageを含む）は引き続き必須。

- b013b6dを通常commit/pushし、3回目P1へdiscussion_r4356671177で返信・thread resolveした。4回目のexact-head review（request issuecomment-6358691806）は非LFの自動折り返し境界の追加P1を指摘したためDraftを維持する。b013b6dのCI/preflight/NativeIMEは39909924742/39909924672/39909924779で実行中だが、追加修正後HEADの証拠には流用しない。

- 4回目P1の実Canvas redは、120pxに収まるboldの25個の`i`と別spanの空白・LFで原文空白が欠落する例（`tmp/verification/pr106-normal-boundary-final5.log`）。すべての非明示境界をautomaticとして扱う1行修正で、richの1/2/3空白span、通常語/oversized語、後続語/全5区切りと分割CRLF、cold/warmのmatrixが成功した。plainは既存の正規化契約を維持する1空白と全5区切りを検査し、codeはEOF末尾空白も幅1/8で厳密保持を検査した。plainの複数空白正規化を誤ってP1と扱った初回splitspaceログはred証拠に含めない。focused17件成功（`pr106-normal-boundary-copy-contracts-final2.log`）。
- 最終ソースの完全 `just check` はexit0、4755 tests/136 suites、native IME10/native contracts22/Unicode6、guard125、Storybook77とfmt/AST/全strict Clippy・consumer契約が成功（`tmp/v044-all-automatic-boundaries-full-check.log`）。同じ更新HEADの完全CI release-check・再review・公開はまだ未完了。
