## Implementation

- [x] Preserve Issue #85 changes and implement independent code raster builder on the same release branch.
- [x] Add public regression tests for body isolation, code drawing / wrapping / hit equivalence and scroll cache invalidation.
- [x] PR #90 P1: invalidate plain-line wrapping cache when code raster config/face changes; cache keyへrenderer設定identityを含めるunit regression、およびplain code-roleのdraw/wrap contractが通過。
- [x] PR #90 P2: RTL本文とLTR inline codeが混在する独立 raster 行で、renderer境界とUnicode bidi visual run境界の両方をUTF-8 char boundaryで分割し、visual order と論理コピー順を回帰で固定する。
- [x] PR #90 P2: 視覚順へ再配置した bidi fragment の glyph、背景、underline/strikethrough、link hit に同一の座標を使い、RTL overrideを含む code link の回帰で固定する。
- [x] PR #90 P2 follow-up: span 境界をまたぐ明示 bidi embedding / override / isolate を raster request のみに保持し、source selection/copy/link payload へ制御文字を露出しない回帰を追加する。
- [x] PR #90 P2 follow-up: bidi visual fragment に分割された link の内部空白を hit と decoration から除外せず、元 source span の両端だけを trim する回帰を追加する。
- [/] PR #90 P2 follow-up: RTL raster の grapheme bounds を単調な論理幅へ畳まず、視覚 x 座標と幅を選択 model へ保持する。部分選択の highlight geometry と、全選択時の論理順 copy payload を回帰で固定する。明示 embedding 内で強い方向文字を持たない fragment には resolved level の isolate も重ね、Draft review と完全 release-check を再実行する。
- [x] Verify focused tests, format and strict Clippy: public code config 5 tests + Card 8 tests passed; just fmt-check and raster-host lib/integration strict Clippy passed.
- [ ] Parent release workflow: review, version, full release checks, Draft PR, review/fixes, Ready, CI, merge and public release.

## Release-preflight diagnostic (2026-10-21)

- [x] PR #90のrelease-preflight 39614760727は子test 135件とJSON/LCOV report生成の成功後にstrict LCOVで失敗した。検査対象やcoverage基準を下げず、report phase・終了コード・stderr・空き容量・profile/report状態をfailure artifactへ残す回帰を追加し、hosted runnerで再実行した。
- [/] #98 strict LCOVで未到達の49行を、baseline付きbidi raster描画、FSI/PDI context、raster failure、UTF-8境界の回帰テストで埋め、functions/lines 100%のrelease-preflightを通す。PR #90のbidi P2修正後、隔離したDocker共有スナップショットで全テスト4,670件/136 suite、native IME、Storybook契約、functions `14641/14641`、strict LCOV、`cargo package`、`cargo publish --dry-run`、未公開版確認まで再実行して通過した。今回のRTL選択 geometry P2で実装が変わったため、最終 strict LCOV とrelease-preflightの再実行待ち。
