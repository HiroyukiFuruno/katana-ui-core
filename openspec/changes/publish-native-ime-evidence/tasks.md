# Issue #81 / v0.4.1 作業台帳

対象: https://github.com/HiroyukiFuruno/katana-ui-core/issues/81

## 今回の公開DoD（最新のユーザー指示）

- [x] v0.4.1 full release-check（100% coverage / compatibility / package）を通す。
- [ ] Draft PR、cloud / self review、指摘 reply / resolve、Ready、current HEAD CIを通す。
- [ ] masterへmergeしRelease workflow、三OS配布binary、GitHub Releaseを確認する。
- [ ] crates.io 0.4.1とregistry-only verifierの動作を確認する。
- [ ] branch-hygieneでlocal release branchを整理しmaster clean / stash 0 / syncを確認する。

Windows/Linuxは仮説に基づく公開をユーザーが明示指示。native証跡未取得はIssue #81に残し、公開を待機にしない。

## Issue #81 の実証完了条件（未取得分を維持）

1. [x] KUC の generic native evidence producer / verifier を実装する。合成 RawInput、fixture、過去 artifact を実入力証跡として受理しない。
2. [ ] 同一 revision の macOS・Windows・Linux 実IME preedit / commit run を取得する。⭐️ の scalar、色付き RGBA crop、☆ との非同一性、measurement、caret、hit test、AccessKit、frame/root/receipt、font SHA-256、input method と run provenance を検証する。
3. [x] 依存関係を調査し、既存の完全な品質 gate と V1/V2 public compatibility を維持して検証する。
4. [ ] Draft PR → @codex review / 自己レビュー → 指摘修正・reply/resolve → P0/P1再確認 → Ready → required checks → merge を行う。
5. [ ] 三OS evidence artifact、GitHub Release、crates.io registry-only verifier を確認し、Issue を close、不要 local release branch を整理する。

## 制約

- KUC 内だけを編集する。consumer の採用や Issue 状態を KUC の DoD に含めない。
- 新規 worktree を作成しない。stash を作成しない。master を直接編集しない。
- local publish / tag による release workflow 迂回をしない。
- OS 入力環境が不足しても synthetic 入力へ受入条件を変更しない。

## 開始時の確認

- master `48d9552f9157f3f7a5af77ff17658cb203823c97`、origin と同期、clean。
- stash 0、worktree は本 checkout のみ、open PR 0。
- 最新公開版 v0.4.0。次期候補 v0.4.1。
- open Issue は #81 のみ。旧 #79 の未公開という memory は現状に一致しない。
- repository self-hosted runner は 0。三OS hosted CI はあるが native IME evidence job はない。
- 既存 capture は `egui::Event::Ime` を生成し `RawInput` に渡す契約試験。native acceptance には利用できない。

## 進行中

- [x] native 入力境界・既存 root evidence との結合箇所を調査。`design.md` に信頼境界と RawInput の制約を記録。
- [ ] 三OS native IME の実行経路を実装し、利用できる環境で測定する。Windows / Linux は実環境なしのため仮説と自動検証で進める。
- [x] `cargo outdated --workspace --depth 2` が exit 0、直接・推移的依存は最新互換版。dependency requirement / lockfile は変更なし。
- [x] producer / verifier を実装。外部の trusted digest と run binding を検証し、自己申告だけでは受理しない。product gate は検証中。

完了率は上記 5 ステップの完了数だけで算出する。

## 2026-10-02 再確認

- open Issue は #81 の1件。`release/v0.4.1` は origin/master に対し ahead/behind 0/0。stash 0、worktree は本 checkout のみ。
- 同一 cwd の別 active session は一覧に存在しない。
- GitHub API の repository self-hosted runners は `total_count: 0`。
- 既存 capture は Preedit / Commit を生成する regression 用。Storybook の minifb 経路は key polling であり native IME transcript を取得しない。既存 macOS probe も三OS producer の代用にはならない。
- [x] `openspec/changes/publish-native-ime-evidence/specs/native-ime-evidence/spec.md` に native 境界、fresh provenance、別run拒否、registry-only verifier、三OS release acceptance のシナリオを固定。`scripts/openspec validate publish-native-ime-evidence --strict` が exit 0。
- Windows / Linux GUI の照会は最新指示で不要。実環境なしを前提に仮説で実装を進める。hosted runners の三OS提供は確認できるが、実IME event 取得が可能であることは未検証。
- 残作業: 三OS実入力、全品質 gate、PR、公開、Issue close。最上位完了数は実測条件を含めて再判定する。

## User Review Phase（2026-10-02 最新指示）

- [/] Windows / Linux の実環境は用意できないため、仮説で進める。環境接続情報を実装開始の前提にしない。検索語: `native IME hypothesis`, `no environment wait`。
- [ ] producer / verifier を実装し、macOSで可能な実測、三OS compile、入力順序・provenance・replay拒否の自動回帰を実施する。
- 仮説: Windows は winit の IMM/keyboard layout、Linux は winit の X11/Wayland IME と input-method service に接続する。実測がない項目は仮説として記録し、synthetic contract 成功を native artifact に昇格させない。
- アンチパターン: 実環境なしを理由に仕様整理だけで止める。正: OS API と自動検証に基づいて実装し、未実測の範囲を区別する。
- 当初の三OS実入力完了条件は未達として残す。ユーザーの「仮説で進める」は実装継続の指示であり、未取得 evidence の捏造・実測済み扱いの許可とは解釈しない。

## 実装・検証記録（2026-10-02）

- native producer、GUI不要の verifier CLI、root observation capture、三OS CI compile job を追加。
- verifier 8件と root session 3件の契約試験は成功。三OS producer コンパイルは成功（AccessKit adapter追加前）。
- macOS 実入力試行は native-ready / Google Japanese input method を確認したが commit 前にtimeout、artifact未取得。OS AccessKit adapter を接続して macOS compile を再確認した。
- 全体unit-testは最初ディスク不足で失敗。停止中のKUC package cache 75.1GiBをcargo cleanで再生成対象へ戻し、incremental無効・test debug情報削減で同じテスト範囲を再実行中。
- Windows / Linux 実IME は未実測。仮説ベースの実装を進め、実環境待ちを開始条件にしない。

- 最終全体unit-test: exit 0、4,601 passed / 132 suites。追加契約は verifier 8件 / root 3件 / producer拒否3件成功。
- Windows / Linux producer は AccessKit 接続込みの最終cross-checkもexit 0。strict Clippy は修正後exit 0。
- macOS native起動でAccessKit adapterの表示前初期化制約を再現し、hidden window生成→adapter初期化→表示へ修正。修正版の起動は確認中。

- 修正版macOS producerはnative-ready、Google Japanese input source、640×240 frameを確認。CUAのfull app path接続はtimeoutし、native preedit/commit操作は未実施。実測artifactは保存していない。
- 最終strict Clippy / AST guard / fmt / OpenSpecはexit 0。Windows / Linux実測を待たず実装・compile・契約試験を完了。Issue #81の三OS実入力・公開条件は未達として保持する。

## 最新リリース指示（2026-10-02）

- [/] ユーザーの「リリースまでやれ」により commit / push / Draft PR / review / Ready / merge / GitHub Release / crates.io / local cleanup を明示承認。進捗報告で停止しない。
- 今回のv0.4.1公開は仮説に基づく実装を公開する。Windows/Linuxの実環境を公開前提にせず、実IME証跡取得は未検証としてIssue #81に残す。未取得artifactを実証済み扱いしない。
- [ ] release-check、レビュー修正、公開、registry-only CLI、local cleanupを完了する。

- release-check実行session: `22264`。lint / readinessは通過、全体unit / full coverage / package検証進行。依存関係は追加producerを含め `cargo outdated --workspace --depth 2` exit 0、最新互換版。
- 配布: Release workflowの三OS `native-tools` job、binary SHA-256 manifest、`native_ime_measured: false` を追加。archive照合のunit contractも成功。

- 最新release-check session: `65623`。前回 `22264` は全体4,612test / native20contract成功後、共有Linux installer使用guardで停止。installerを共通化し120 guard tests成功。追加private testsをtests.rsへ分離してASTの250行type-separationを維持。

- 最新gate session `82649`。native_session private testsを既存方針のtests.rsへ分割、数値を定数化してAST guardを再確認済み。新規共有installerは既存timeout/retryの境界を維持してGUI headerを追加。

- session `82649` は132 coverage test binariesすべて成功。strict coverageでnative_session / observations / verifierの未到達行・関数を検出してexit 1。実LCOVに基づくエラー経路回帰と到達不能な固定要素分岐の整理を進め、100%基準は維持する。
- 最新差分の追加自己レビューは新規P0/P1なし。registry-only positive/negative検証用standalone projectをtmp内に準備、公開前のregistry解決は未実行。
- LCOV対応: 不正な読み込み可能フォント、context共有、数値上限、二重receipt消費、寸法/画素数不一致、必須scalarを消すIME commitの拒否を追加検証。固定2cropの到達不能分岐とエラー変換closureの重複を整理し、既存エラー種別を維持。
- 最終clean release-check再実行session: `66633`。元の100% line/function coverageと全品質gateを再実行する。
- session `66633` は全体4,619件、clean coverage全テスト成功・関数coverage100%。crop範囲外のエラー伝播1行（observations.rs:137）が未到達でexit 1。実LCOVをtmpへ保存し、範囲外crop拒否の回帰を追加して再検証する。
- 範囲外texture offsetによる `composite crop pixel missing` 拒否テスト成功。production codeは変更なし。最終gate session `61217`、ログ `tmp/v0.4.1-final-release-check.log`、100%基準を維持して再実行。
- session `61217` はexit 0。全体4,620 test、clean full coverageのline/function 100%、package検証、publish dry-run、公開scope検証、未公開版確認が成功。依存関係は最新互換版、V1/V2 contractと厳格lintを維持。

## PR #82 review対応

- [ ] P1: 実描画textのcommit byte rangeとIME commitを結合し、消失/不一致/UTF-8境界を拒否する。
- [ ] P2: hit queryをmeasurement/target boundsへ結合し、対象scalarの実rangeを検証する。
- [ ] P1: 三OS archiveの成功・revision/version/digest検証を公開の前提にし、全asset添付後にGitHub Releaseを公開する。
- [ ] Rust1.99 strict Clippyに対応し、修正HEADのfull release-check・cloud review・CIを再検証する。

## 再レビュー対応（85117ea）

- [ ] P1: native sessionの初回IME前にretained text面へfocusを要求し、クリックなしpreedit/commitの契約を追加する。
- [ ] P2: ⭐️/☆それぞれのcrop寸法を対応する実hit target boundsへ結合し、不一致を拒否する。
- [ ] 旧snapshotのgate31834はunit4624成功後に中断。追加修正後にfullgateとcloud/CIを再実行する。

## 最終レビュー P2

- [ ] caret右端・下端をchecked additionで計算し、measurement外/overflowを拒否する。x/y範囲外とoverflowのresealed回帰を追加。
- [ ] 最終修正HEADのfull release-check / cloud / CI / public release / registry検証を完了する。

## 追加 P2 対応

- [ ] AccessKit boundsのchecked終端overflowを拒否する（raster measurementとの直接包含は別座標系のため要求しない）。
- [ ] Linux ibus/fcitx照会のchild/pipe待機をremaining timeoutへ制限し、startupも同一monotonic timeoutへ算入する。
- [ ] 修正後full release-check、最新cloud/CI、公開、registry/local cleanupを完了する。

## 最新統合修正

- [/] AccessKit overflowを拒否する回帰22件、Linux照会の実process timeout/終了status/output回帰6件とLinux cross-target strict Clippy成功。
- [/] Windows hosted契約で末尾caretがglyph画像幅192を越える事例を確認。measurementを実viewportへ結合し、viewport寸法一致とcaret包含の回帰を追加（root契約6件成功）。
- [ ] 最新差分のfull release-check / cloud review / 三OS CI / 公開 / registry / cleanupを完了する。旧HEADのfull gate成功は最新HEADへ流用しない。
