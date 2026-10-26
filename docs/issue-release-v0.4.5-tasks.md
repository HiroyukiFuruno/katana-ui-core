> 最終公開版は **v0.5.0** とする。初期計画のv0.4.5は公開しない。
> 公開enumへのVisibleAtCaret/SelectionDialog追加はRustのexhaustive matchに影響するため、
> pre-1.0の次minorへ区切る。初期15件と追加#112を一つのPR・一つのreleaseへ統合し、既存の品質条件は維持する。
> 重複buildを避けるため最終検査前にbranch、manifest、lockfile、release noteをv0.5.0へ整合済み。

# v0.5.0 Issue 対応とリリース（初期計画ファイル）

対象: 初期Open Issue 15件と追加#112の追跡16 Issue。実対応は重複1件・既存解決1件を除いた当初13件と追加#112の計14件。初期原文は issue-release-v0.4.5-source.json、追加要件は issue-release-v0.5.0-issue112.md。
現行方針: 複数 Issue は全て一つの Draft PR と一つの v0.5.0 リリースで対応する。v0.4.5 は公開せず、分割公開もしない。ファイル名と原文 JSON 名は初期計画の履歴として維持する。
既存の動作契約を維持して補完する。公開enumのvariant追加は次minorのv0.5.0で扱い、外部 repository の実装・採用は対象外。
2026-10-25更新指示: Close は KUC 自身の実装・自動検証・公開根拠で判定し、外部 consumer の採用・受入・Issue 状態を KUC の release gate に加えない。実行可能な検査や既存品質ゲートは省略しない。

## 完了条件と最上位ステップ

- [x] 1. #88 #89 #91 #92 #93 #94 #95 #97 #99 #101 #102 #108 #109 #110 #111 #112 の受入条件を実装・回帰へ対応付ける（全16件実装・公開回帰対応済み、最終gateはstep3で検査）。
- [x] 既存実装も含む境界監査・是正: 他repository固有のコード/コメント/API説明を汎用化し、KUC単独動作を維持。名称置換で依存を隠さず構造も確認する。15件と同じPR・releaseへ含める（2026-10-25追加指示）。
- [x] 2. 直接・推移依存と lockfile を最新互換版へ更新・再検証する（最終監査でminifb0.29/resvg0.49を追加採用、全互換再解決0件・型検査/SVG11件/window262件成功。docs/release/v0.5.0-dependencies.md）。
- [ ] 3. 既存 check / release-check、100% coverage、AST、KUC 公開 API regression を成功させる（外部 consumer の受入は release gate に含めない）。
- [ ] 4. Draft PR → @codex review / 自己レビュー → 指摘修正と reply/resolve → P0/P1ゼロ → Ready → required checks → merge。
- [ ] 5. Actions の GitHub Release / crates.io 公開確認、Issue への根拠記録と Close、branch-hygiene。

## 初期状態（履歴）

- master / origin/master: 87063e1。観測時の最新公開 v0.4.4、当初計画の次 patch は v0.4.5（v0.4.5 は公開しない）。
- master clean、stash 0、worktree は現 checkout の1件。重複する KUC 作業チャットなし。
- 新規 worktree、stash、ゲートの緩和、手動 publish/tag は禁止。

## 現行公開境界

- 現行対象は release/v0.5.0。manifest、lockfile、release note も v0.5.0 に整合済みとして記録する。
- 全16件を一つの Draft PR・一つの v0.5.0 release に統合する。PR review、required checks、coverage、公開、Issue close、cleanup はそれぞれ独立した未完了 gate として扱う。
- KUC 単独の公開 API、contract、automated regression、guard のみを完了判定に用いる。外部 repository の採用・consumer 受入・外部 Issue 状態はこの計画の完了条件ではない。

## 分担

- text_fixes: #88 #89 #95 #108 #109。
- search_ax: #91 #92 #94 #110 #111。
- public_ui: #93 #97 #99 #101。
- main: #102、依存関係、統合・最終検証・レビュー・公開。

## 検証記録

検査が完了した後に終了コード・実行対象・ログを記録する。

- #102: 既存修正 8e91902。2026-10-25 再検証 `python3 scripts/test_kuc_guardrails.py`: 125 tests / OK / exit 0。
- #102: preflight run https://github.com/HiroyukiFuruno/katana-ui-core/actions/runs/39917955273 は headless / FFmpeg / manifest / release-check 全て success。
- 依存: serde_json 1.0.154、sha2 0.11.1へ要件更新。lockfileのcc、clipboard-win、roxmltree、smallvec、uuidも最新互換へ更新。egui/eframe 0.37 は public adapter の型互換を壊すため0.36系維持。minifb/resvgの初期patch計画での据置き判断は後のv0.5.0最終監査で撤回し、0.29/0.49を実検証して採用した。
- 初回 compile は作業途中のtab root item接続とtexture上限の型不整合で失敗。担当が修正中。未完了扱い。
- 最新 `cargo check -p katana-ui-core --all-features --all-targets --locked` は成功 (session 4502、exit 0)。その後の変更は再検証中。
- `egui_selection_dialog_contract`: 3 passed。公開rootでFile移動/選択/取消/failure/disabled/stale/one-shotとSearch日本語⭐️query/検索通知。
- `egui_issue_text_selection_contract`: 1 passed、`egui_status_bar_contract`: 13 passed。
- `egui_ui_selection_snapshot_contract`: 2 passed。`abcdef\nx\nabcdef`の希望列保存復元とsnapshot再利用、IME未確定拒否。
- `egui_text_edit_selection_issue_contract`: 3 passed。最終selection通知、controlled echo、正逆選択折畳み、readonly/disabled。
- #89/#97 共通根因: raster の grapheme geometry から明示改行が抜け、選択端が次行先頭へ飛んでいた。hard separator geometry と行末caret候補を修正。
- #109 共通根因: drag_started時点の移動後の点をanchorに使っていた。press originを保持して選択を開始。locatorは空行のviewport中央を避け実glyph行から入力点を発行。
- Storybookの未使用markdown model dev-dependencyを削除。viewer adapter残骸68filesはmodule graph未接続。savepoint後に整理する必要がある。

## 未完了の重点

- #89 AXのLF/CRLF/空行/末尾改行/softwrapと原文byte selection対応の追加回帰。
- #91 caret toolbarの実focus/readonly/dismiss/inline無効対照回帰。
- #93/#99 modalのAX/pointer/history/options/filter/result/highlight/focus復帰/狭幅回帰とcoverage。
- #94 writer/verifier/native observation型の再利用、receipt/root/frame/hash結合と改変拒否。
- #97 方向/affinity/readonly/unfocused/hidden/discard、多framecontrolled同期、独立期待の希望列追加回帰。
- #101 mixed順序の実AX bounds回帰、typed proposal全操作、rename requestとlease同期後の状態保持検証。
- #108 tile上限2048/4096/変更と末尾scroll/selection/hit testの外部回帰。
- #109 公開Factoryの同一lease/毎frame同期と負対照を使った完全な5step回帰へ改修。
- 既存境界違反の是正・未接続ファイル整理、AST/lint/全test/coverage100%、PR/review/公開/cleanupは未完。

## ユーザーフィードバック

- [/] 全15件を1PR・1リリースに統合する方針をAGENTS.mdと本台帳へ固定。分割公開しない。
- [/] KUC単独で成立する汎用境界を維持し、今回の実装・コメント・API文書に外部repo固有の知識や名称を持ち込まない。AGENTS.mdへ明文化済み、差分監査と独立回帰は継続中。

## 追加検証・修正記録（継続中）

- #91: caret toolbar 7件成功。focus、legacy Visible、inline Disabled、readonly/disabled、focus loss、Escapeを含む。
- #89: hard break/CRLF/空行/softwrap AX run IDの4件成功。softwrapはphysical row別IDへ是正。
- #93/#99 + #97: 公開root modalとsnapshot計7件成功。その後にopaque target別操作ID、空検索状態、directory確定ボタンを補強して再検証中。
- #97: pass終了までsnapshotをProvisionalとし、後からdiscardされたpassのsnapshotを拒否する回帰を追加。4件成功。hidden UIの回帰を追加し検査中。
- #94: 実Factory/live rootの日本語・⭐️・☆・ZWJから別々の実cropとnative observation型へ変換する回帰1件成功。font hash/AX IDはcallerから受け取らず実観測を使用。native originは認定しない。
- #101: lease同期2件、typed rename文字列読取り1件、support4件、inline rename1件成功。mixed AX回帰は差分TreeUpdateを完全treeと誤認していたため修正中。
- 全lib検査: 1907成功/10失敗。イベント種別追加に伴う期待2件、狭幅locator1件、誤った改行geometry fixture1件、tab内部path関連6件を修正して再検査する。
- 全lint: egui有効の製品lint通過。egui無効時のselection補助モジュールunusedをfeature gateで修正、再実行中。
- 空segmentのraster requestは既存の拒否契約を維持。status clearはsegments/バーの削除として検証し、full-rootのlabel更新・bounds・非操作AX回帰が成功。
- 容量不足によりcoverage初回はexit125、未完了。仮想diskの解放で38GB回復、検査の閾値は維持。生成物再構築後、coverage-iterateを実行中。
- AST/guard、差分空白検査は成功済み。最新sourceでは再検証を続ける。


## 統合検証の更新

- modal終了時のegui layer退役frameに届く文字入力の欠落を修正。元focusへ戻った再描画で一度だけ転送する。対象回帰成功。
- 320px幅/40項目の実overflow一覧で、AX boundsと末尾へのキーボード移動・typed選択を検証、成功。
- 長文rasterの高さ制限4096pxで末尾glyphが生成されない原因を是正。最終raster寸法/pixel上限は維持。長文回帰3件、公開root tile/末尾操作4件成功。
- native Unicode観測は実IME→確定日本語→⭐️/☆/ZWJ入力の同一root/frame/receiptへ結合。preedit、異なるroot/frame/hashを拒否する3件成功。
- 全lib再検査は1916成功/3失敗。残りはcapabilityを未指定だった既存root正対照fixture。対応中。
- AST、guard125件、raster-host-contract、check-types成功。製品all-features lint成功、no-default integration fixtureのfeature gateを修正して全lint再実行中。
- Cargoのproducer/cleanupに共通lockを導入。実子processの排他・終了伝播3件成功、実just cleanが稼働producer下で75を返し生成物へ触れないことを確認。
- 旧probe専用の未使用targetだけをcargo cleanで整理し9GiB回収。稼働targetとcoverage volumeは保持。
- full coverageは未成功。前回profileは一時的なstatus空raster変更のUT失敗でfail-fastしたため、元の契約に戻した最新版を再検査する。

- #101: root正対照fixture3件をcapability明示へ修正し各限定test成功。retained全67件成功。group dragのhost order変更とlease再投影後AX順序、disabled/self-drop/Escape不変の外部回帰2件成功。
- #101: tab/groupのghostとdrop indicator描画を共通化。異なるlabelのdragでcacheが前のtextureを再利用する不具合もraster hash identityで是正。paint planと境界数値回帰2件成功。
- #93/#99: File/Search各40項目のkeyboard選択変更時だけscrollを要求。初期末尾offscreen→idle repaint後画面内bounds→正しいtyped targetの対照回帰が各成功。
- production編集を一度凍結し、fmt後の全checkを開始。PR、公開、coverage、未接続Storybook残骸整理は引き続き未完。

- all-targets/all-features strict Clippyはunused fixture bindingと廃止済みIME Enabledの入力を是正後に成功。検査の無効化は追加していない。全check再開。

- 共有disk再不足により全checkのlinkがerrno28、coverage20359はVM IOエラーexit125。製品test不一致とは区別し、未完了のまま再実行する。
- 指示済みprivate just clean-allは一時file不足で一度起動失敗、trim後再実行。exit1だが未使用KUC target19.1GiB等を回収。producer lock/active cache guardは有効に維持。
- ColimaのIOエラーを再起動で復旧。未使用・接続0・中身debug/tmp/rustc metadataのみのDocker build cache volumeも整理した。source/未コミット差分/検証記録は削除しない。
- ローカル全checkはCARGO_PROFILE_DEV_DEBUG=0 / CARGO_PROFILE_TEST_DEBUG=0 / CARGO_INCREMENTAL=0 / CARGO_BUILD_JOBS=2で再実行。debug symbolsとincremental生成量だけを減らし、テスト対象・最適化レベル・lint/coverage閾値・受入条件は維持する。coverageは既存の専用container契約を維持して後続実行する。

- 最終差分レビューでShiftありの非空selectionも端へ折り畳む退行を検出。keyboardでShift時の移動基点をcaretへ分離し、正逆の拡張/縮小と日本語⭐️境界をRawInput回帰へ追加。限定4件成功、production再freeze。
- 実rootのIME→Text input回帰へ、typed SelectionChangedの9..9/36..36と受信したvalue/selectionのcontrolled echo検証を追加。次の全checkで回収する。
- #101の全typed操作→実host state回帰は公開consumer側テストを追加中。通知件数だけでは受入条件を満たさないため未完として継続。
- #110のempty/zero/invalid query/別focusの負対照を追加中。KUCへ検索処理を持ち込まず投影の結果を利用する。
- e2e/smokeのCargoを起動する既存shell recipeも共通producer lockで保護した。親lockから子lockを二重取得しない構成をdry-runで確認、実gateで後続検証する。
- 現在の全checkはsession39902、省容量envで実行中。coverage20359はexit125で未完、Colima復旧後はnative結果を回収してから最新版を再実行する。

- 全check39902: core lib1921件成功、integration多数成功。追加#110負対照がhost fixtureの固定mod3とAX focus前提で失敗しexit101、判定を受入条件へ整合して修正中。全check完了扱いにはしない。
- #101 stale AX再現: egui_tab_strip_stale_target_contractがexit101。A削除後にBが同じoccurrenceへ移ると、旧AへのAccessKit ClickがBへ適用された。再現ログ1792905036_cargo_test.log。
- 原因はoccurrence pathだけのegui ID再利用。projectionのopaque tab/group identityと操作family/移動先をIDへ結合し、label/trailing/menu/swatch/overlayへ適用。rename focusの同target維持を保ち、別targetへの再割当を防ぐ。AST成功、stale/既存rootの再検証中。

- 最終branchはrelease/v0.5.0、manifest/lockfile3packagesとrelease noteを0.5.0へ整合。v0.4.5は公開しない。既存の作業記録のファイル名は初期計画を表す。全15件を一つのPR・releaseにする条件は維持。

- v0.5.0でtab host-effect 10件、stale target 1件、search focus 3件の計14件成功(session64214)。旧AX操作のtarget転用、cap失効、host correlation拒否→fresh lease回復も実入力で確認。
- tab/group host fixtureは操作logだけの偽陽性を是正し、実tab order、membership、group stateを更新する。dragはStartDragの実sourceを保持し、第2tabを第1tabの前へ移動する独立期待で検査。
- selectable=false labelにegui由来のClick actionが残る不備を発見し、二次ボタンmenuのsenseを保ちつつ非操作AX labelのactionsを消去。取消・通常操作を弱めずに対応。
- native全check＋release-readiness-checkをv0.5.0、debug0/incremental0/jobs2でsession7434実行中。終了コード回収前に完了へしない。

- fullcheck9322は公開receiptのclass分類数旧期待で停止。既存dispatcherの7 handlerとreceiptの8分類を区別し、新dialog count=0を含め13件成功(session42370)。95519の誤ったhandler数期待も修正済み。残り全test/guard/readinessはsession23940、tmp/v0.5.0-native-continuation.logへ記録中。
- 新規公開API文書4件を境界guardへ追加、自己test3件成功。Cargo guardはsignal開始窓・Windows lock待機・cleanup errnoと漏れたshell経路を是正し、実IPC4件成功。
- v0.5.0 coverage-iterateはsession89379で実行中。source snapshotは上記receipt期待修正前のため、当該fixture結果を回収後に最新版へ再実行する。最終100% gate未達は維持。

- session23940: workspace全150suite/4830件成功、native tools10件、native artifact22件、native Unicode6件成功。残guardは旧coverageコマンド文字列期待でexit1。排他wrapper+CARGO_TOOLを要求する新契約へself-test fixtureごと更新し、self-test/実readiness成功。session31552でguard残部/consumer/E2E/smoke再開。
- 現行inventory表を用途/state/action/event契約へ汎用化。旧35/37/40関連changeは既存の版/score/検査結果を保持した過去追跡記録と明示し、現在のrelease gateへ転用しない。3 Issueが既にclosedであることも公開APIで確認した。

- coverage89379は旧receipt fixtureを含むsnapshotと判明済みのため、最新sourceへ一本化するため所有containerを停止。ビルド6m13s成功、Storybook UTはexit0/525s、core libはexit0/595s、残suiteは未完。成功profile/100%とは扱わず、checkpoint・未使用68files整理後に最新sourceの完全release-checkを実行する。

- session31552: kuc-guardrails/overlay/menu/consumer-app/E2E/headless smoke/interaction smoke/release-readiness全てexit0。ログtmp/v0.5.0-readiness.log。strict lint各構成、全150suite4830件、native contract10+22+6件と合わせcheckpoint前の必須検証が完了。最終100% coverage/release-checkは未完。
- 公開文書guardも一般の他katana repo名へ拡張。自repo旧称・自repo change識別子は保持し、共有lint toolへの実装先指示はrepo-local guard責務へ修正。違反comment/現行docs拒否・修正後成功の自己test3件成功。

- 署名付きcheckpoint b46c4c94c5f03223de258e349a7b77fd2625a2a3 (GPG=G)を作成。全検証成功後の保存として68files整理前条件を満たした。
- 未使用viewer4paths/計68filesをtmp/trash/2026-10-25-151420-unused-viewerへ退避し、追跡差分68files/9253行削除、残存mod/include参照0、git diff --check成功を確認。最終PR前にcheckpointを含む差分全体を単一署名commitへまとめる。
- 実装対応付け・既存境界是正・依存更新再検証を完了し、残りは最終release-check(100%含む)、PR/review/CI/merge、公開/registry/Close/cleanup。

## 最終レビューで検出した残検証

- [x] sanitized_document_root.rs::public_command_projection_stays_generic_and_output_free: sourceを小文字化した後で大文字KatanAを探す最後のassertが常に成功する。外部repo固有の分岐ではなくsource guardの既存偽陽性。現coverageの完全profileを回収後、caseを整合して限定検査→最終gateへ含める。production API/品質閾値は変更しない。

- 最終coverage48635: native_window_contractは78.95sでexit101、winit XOpenDisplayFailed。旧profileは失敗として回収し成功扱いしない。失敗stdout/stderrはtmp/v0.5.0-release-check.logへrunnerが保存（container終了後の個別copyは失敗し、専用directoryは作成していない）。
- 診断: 同じwindow binaryの連続/paired各40回は両設定成功し、単純なwindow再現とは区別。libX11 XOpenDisplay＋generation markerを300回対照実行したtmp/v0.5.0-x11-reset-probe.jsonは、既定Xvfb=12接続失敗/288generation、-noreset=接続失敗0/1generation。winit sourceの同エラーがXOpenDisplayFailedに対応することも確認。Xvfb最終client終了時resetと新規接続の競合をharnessで是正し実回帰へ固定する。
- 上記ループに合わせ、sanitized sourceguardの小文字化後assertを小文字のkatanaへ修正。限定検証と最終再検証は残作業。

- coverage48635の全体終了コード1を回収。元のfail-fastは維持しnative_window失敗を通過扱いにしない。Xvfb -noreset、100接続の実libX11 generation回帰（Linux coverage起動時必須）、host static回帰とfingerprint入力へ追加。通常hostのstatic＋readiness self-testは成功。新coverage-iterateで実probe/native-window/完全profile/LCOVを回収する。

- sourceguard修正の限定native test54749はexit0/1passed。再発防止の実Xvfb testはproduction run-strictのCLI引数そのものを読み、-noreset指定と実接続100回でgeneration不変を検査する。起動失敗時stderr読み込みが待ち続けないようにfinally cleanupも確認。新coverage-iterate session35609を実行中。

- coverage35609でLinux実Xvfb100接続＋staticの2件ともexit0。server stderr未closeのResourceWarningを検出し、finallyの明示closeで修正。Rustの実装/閾値は不変、現在のcoverage profileは未到達診断として回収し、最終release-checkは最新scriptを含めて再実行する。

- 最新Xvfb回帰script（stderr明示close済み）を同じLinux imageで独立実行。PYTHONWARNINGS=error::ResourceWarning、実接続100回＋static2件ともexit0、ログtmp/v0.5.0-xvfb-regression-final.log。ホストnative target/coverage profileへ書き込まず検証した。

- 完全coverage35609は全149binary exit0、1795秒でstrict100%不足としてexit1。summary: functions15057/15087、lines160749/161000。実LCOV/JSONはtmp/v0.5.0-coverage-retry、分類はtmp/v0.5.0-coverage-misses.json。3workerへdialog/Unicode-snapshot/tabsのtest-only回帰を分担し、親はtext input/paint/context/error messageを担当。既存の閾値・ignore・scopeは維持。
- 固定型JSONなど3箇所の同一error文字列変換を、既存trace helperへの直接map_errへ統合した。重複するcause変換を一か所へ寄せ、成功hash/serialized bytes/公開エラー内容は変更しない。実SerdeErrorの原因文字列保持回帰を追加し、到達不能な匿名closureを作らずtyped errorをそのまま伝える。

- 追加回帰native91100: core lib1938件成功。facadeは追加した改行観測拒否の期待のみ不成立（16成功/1失敗）。hard separatorは既存layout/hit-testの実geometryを持つため、拒否を新たな仕様にしない。当該誤った追加assertを除去し、UTF-8/grapheme/receipt拒否とprivate metadata破損回帰を維持。残integrationを再実行する。AST98368 exit0。

- LCOVのDA行だけが全到達でも、LF/LHとJSON summaryに未到達が残り得る既存判定漏れを確認。既存strict JSONの100%判定も最終判定へ接続し、失敗時はregion詳細JSONを保存する。品質閾値を維持し、summary不足を通過扱いにしない。JSON自己検査とbash構文検査はexit0。
- native84168はfacade17件成功、dialog11/13で失敗。原因は同名LabelをButtonと誤認した追加テストのAX locator。Button限定へ修正後dialog13件全成功。残integration61278 exit0。保持同期86222の非公開field参照をclear_tab_stripの実動作検査へ改修し85550で再実行中。coverage16402は当該修正前snapshotのため所有containerだけ停止、exit143で成功扱いしない。

- 上記JSON strict接続案は調査後に不採用。LLVMのFunctionCoverageSummary::get(InstantiationGroup)とLineCoverageInfo::mergeは、型別instantiationの到達行をunionせずcovered/countの最大値で統合する。LCOV DAは同じsourceの実到達行を統合するため、差47を未到達の物理行と扱えない。既存のLCOV全物理行＋論理function100%ゲートを維持し、未承認のbest-instantiation全行という別条件は追加しない。JSON summaryは別指標として実測値を併記し、summaryを100%と偽らない。失敗時の詳細JSON保存は維持し、実profileで差を追跡する。一次実装: https://raw.githubusercontent.com/llvm/llvm-project/main/llvm/tools/llvm-cov/CoverageSummaryInfo.cpp と CoverageSummaryInfo.h。

- 保持同期85550: 3passed/exit0。strict Clippy21466: 全5構成exit0（tmp/v0.5.0-regression-lint-complete.log）。余分なborrow、Copy clone、未使用test bindingをルール通り除去し、disable/ignoreは追加していない。公開facadeのfixture文面・AX label・test名からも不要な旧consumer名を汎用化。registry-only fixtureは現行facadeからsource-only4tests/120行のみ除外した差分を機械確認した。

- Open Issue再確認で新規#112を検出し、ユーザーの全件同一PR/release指示に従い計16件へ拡張。追加要件/sourceをissue-release-v0.5.0-issue112*.md/jsonへ固定。non-wire enabled resolverとtyped error、attachments内部整理を実装し公開root回帰を追加中。native21389は編集途中の112型接続でexit101、成功扱いにしない。
- #112のlib限定checkでFactoryError Display不足を検出して補完し、初回adapter初期化時のresolver二重評価をget_or_insert_withで是正。all-targets4408は追加integration fixtureの寿命/move2件でexit101、担当が実APIに沿って修正/限定test中。marker→surface→root→facadeのtyped変換UT22578は1passed/exit0。新利用ガイドを境界guard対象へ追加、自己test3件exit0。

- coverage28078: 全149binary exit0、strict不足で全体exit1。論理functions15078/15084、DA未到達50行、LLVM summary lines160916/160996。tmp/v0.5.0-coverage-regression-fixedへ完全LCOV/summary/detail JSONを保持し、当時sourceは同名-sourceへ保存。残6関数/50行をdialogs/tabsのprivate拒否・lifecycle回帰へ分担。閾値と対象範囲は維持。
- DialogPainterの個別catalog/config再照合をPlatformTextRasterResources::rasterizerへ統一（同resourceから生成したpair不変条件はresources.rsで保証済み）。private constructorをinfallibleへ揃えた。入力保持はcallerの既存owned keyをentry/or_insert_with_keyへmoveし、重複lookupと成立不能なget_mut失敗を除去。追加allocationや検査無効化で回避しない。描画時の実rasterize失敗はそのままErr伝播し、追加UTで検証する。
- 追加UT接続後43701/62058はmodule path・egui0.36 run_ui・私有helperのtest位置等で失敗。helperのvisibilityを製品側で広げず、focus自身のchild testへ移し、実APIへ整合。AST15590の352行testをgroup/tab責務で分離。最終統合check実行中。

- #112入りbaseline11720: workspace151suite/4861passed（1939 filteredは子process限定testの併記）、native tools10/typed22も成功。その後に接続した新private UTのcompileエラーでnative contract末尾がexit101。source更新を挟んだ診断であり最終gate完了とは扱わない。
- 残UTの初回44402は1955passed/1failed（RestoreClosed routeのcapability不足）。restore_availableを実仕様へ揃えた限定1件がexit0。新edges4件のegui0.36 run_ui接続を親で修正し、最終core lib再検査中。
- 通常consumer構成で露出していたprivate artifact helperのdead_codeを用途のfeatureへ限定。allfeatures用method2件のcfg(test)誤分類と専用import漏れは限定Clippyで検出し、実callerのstorybook-artifacts条件へ修正。通常egui/native構成のstrict ClippyをJustfile lintへ常設し、既存全機能検査を維持。
- root_event/batch.rsがcfg追加で302行となったため、transportの観測専用2methodをbatch_observation.rsへ責務分離。AST7997とfmt47985はexit0。

- 最終統合lib49376: 1960件＋分離子process1件でexit0（RTK集計1961passed/1959filtered、tmp/v0.5.0-complete-regression-unit-fixed.log）。strict lint71583全6構成exit0。後続のfallback空ラベル両経路と子ContextMenu resolver失敗は最終gateで回収する。
- source freeze後、最終release-check11438を開始。env debug0/incremental0/jobs2、VERSION=v0.5.0、tmp/v0.5.0-release-final.log、coverage保存先tmp/v0.5.0-release-final。品質閾値・全scope維持。fmt4381/AST7997を成功回収済み、PR/publicationは未実施。

- 最終11438はlint中のguardへSIGINTを送りexit130で回収。追加レビューでresolver失敗中のEscape閉鎖不能を実回帰で再現（81634、1failed、typed error）。KUCの閉鎖操作を先に処理するP1修正を行い、新sourceで最終gateを再実行する。
- dismiss修正初回60251はis_outside_click import不足とhelper visibilityでcompile失敗。親差分レビューで併せてclosed target解放と新context gesture条件も指摘し、修正後に再検査する。
- 読み取り専用差分レビュー（search_ax）: 公開API/receipt/Unicode/status/neutral guard/Cargo guardを確認し、追加P0/P1なし。dismiss P1は親が修正・検証中。
- dismiss P1修正11758: enabled resolver公開integration全9件exit0（tmp/v0.5.0-issue112-dismiss-fixed2.log）。失敗中のEscape/outside閉鎖、閉鎖frameと以降の照会0を確認。fmt90466/AST42423/差分checkはexit0。最新112fixtureをregistry-only試験へ同期。
- 最終61427は静的Lint全6構成、UI readinessと操作smokeを通過。native全機能151suite/4883passed（子process用1959filtered併記）を回収。native契約→全coverage→package/dry-runを継続中。
- coverage旧search82/98の回帰不足をsource照合で確認。通常includeの成功と100万文字includeの実raster失敗伝播・projection未消費をtest-onlyで追加、限定2件成功（text_fixes実行）。native全機能151suite成功後の追加なので、後続native Clippyと完全coverageで最新版を回収する。製品コード・閾値は不変。
- 最終61427はnative tool10/証跡22/Unicode6、依存・neutral・overlay/menu guardを通過しDocker完全coverageへ進行。snapshot後は編集を凍結する。追加filter testは261行版を含む。
- 最終61427のcoverage開始snapshot後、追加filter回帰がMac実測7891068pxを固定している非移植的assertを親レビューで検出。所有container18c92fe7a150（source/log mount一致）だけstopして終了を回収。固定寸法を要求せず実RasterTooLargeのprefix/pixel-limit causeとquery/include2入力mountを判定する回帰へ是正。閾値/対象/製品コードは不変、fresh最終gateを取り直す。
- fresh最終release-check54876開始。tmp/v0.5.0-release-final3.log、coverage-final3、最新source-final3-source。fmt77341/差分checkはexit0、全agent編集凍結済み。前61427は全体exit143を回収。
- 最新54876: native151suite/4885passed（子process用1961filtered併記）、Lint6構成/全check成功。Linux完全profileは実Xvfb100接続2test成功、native_window_contract379.40s/最初のStorybook353.02sでexit0。本体profileを継続中。
- 完全Linux54876で2番目のStorybookがSIGKILL(-9)。cgroup memory.eventsのoom_kill=1、memory.peak=7707049984 bytes/VM MemTotal8114272kBを実回収し、メモリー不足と確定。証跡tmp/v0.5.0-release-final3/oom-diagnostic.json。追加filter回帰の100万文字を16384文字へ抑え、同じ8192寸法/16777216pixel上限の実Errを維持する。入力確保負荷の軽減は仮説対処として再測定し、品質閾値・全scopeは維持する。
- 54876全体exit1を回収。core binaryは1969件/686.72sでexit0だがStorybook OOMのためprofile未完。100万文字filter単体の実Linux計測はexit0/7.06s/maxRSS411068KiB（filter-large-rss.json）。単体だけをOOMの唯一原因と断定せず、parallel時のピーク対策としてStorybook全binaryを排他phase→残りは既存parallelへ変更。thread数/scope/閾値/全targetsは不変、元indexも維持する。selftestは複数Storybook排他・light parallel/refill・5targets全件・index保持を実子process時系列で検査。
- 16384文字版filter限定9905:2passed/0.26s/exit0。runner90747 selftest4/4成功、複数Storybookの排他・remaining並行/refill・全5targets・元indexの保持を実子processの時系列とstatusで検査。py_compile/差分check成功。fresh最終gate-final4開始、保存先tmp/v0.5.0-release-final4.log/同名coverage/source-final4-source。全agent編集凍結、scope/threads/閾値維持。
- 最終66762は新しい回帰コメントの形式をASTが拒否してexit1（coverage未開始）。内容を既存WHY block形式へ整合し、ASTを再確認してからfresh gateを再実行する。Lint抑止は追加しない。
- AST43059/差分check exit0を回収。fresh最終release-check72388を再開始。tmp/v0.5.0-release-final5.log/同名coverage、検証sourceは最新コメントへ同期したtmp/v0.5.0-release-final4-source。scope/threads/閾値は不変、全agent編集凍結。

- [/] 追加指摘: 既存repo-local運用スキルに外部repo固定cd/gh、旧Floem-onlyと別製品用harnessが残存。b46c4c9と一致する未変更4guideをtmp/trash/2026-10-25-193855-legacy-skillsへ退避、参照なし確認済み。PR運用2skill/impl入口2skillを現行KUCのmaster/Actionsへ整合し、active SKILL/workflowの境界guardとmissing-doc fail-closed/大文字略称の実回帰を追加。文書を理由に製品full gateは追加せず、現在72388を継続。
- 追加境界是正の限定検証: neutral82751/2489は4件exit0、KUC guard125件とassertはexit0、skill metadata4entry有効、local link7件実在、差分check成功。製品source/検査範囲は不変で72388を継続。
- 全AI向けinstructionをtracked45fileから照合し、追加4legacyコピーもcheckpoint一致・既知内容一致・参照なしを確認して同trashへ退避（計8）。impl入口4/PR2skillを検証、運用7rootの境界実回帰4group成功。ignore設定はgit check-ignore実測とstatusでsnapshot混入0を確認。製品runtime/harnessは不変のまま72388継続、policyだけ限定再検証。
- 新guardはKUC名に日本語が隣接する正当な記述をASCII識別子境界で許可し、類似した外部名と相対monorepo pathを拒否する実回帰も成功（79382、4group）。Linux72388は前回OOMのStorybookも472.14sでexit0、全Storybook排他phase完了・oom_kill0。本体/公開API回帰を既存parallelで継続。

- 最新の件数指示: 進捗は実対応単位で数える。当初15 Issue−同一対応1（#92/#111）−既存実装1（#102）=13、追加#112込みで14。14/14の実装・個別回帰済み、全体gate/PR/公開は別に追跡し、Issue総数16と混同しない。

- 最終72388は全150 Linux binary exit0/OOM0を回収し、strict coverage8行・論理1関数不足で全体exit1。tmp/v0.5.0-release-final5-misses.jsonへ分類保存。tab/group ghost Err伝播、group middle候補、fallback opaque path、typed error Fromのtest-only回帰を追加。限定実検証とsupplementで不足解消を診断後、fresh最終full gateへ進む。
- 残回帰のnative検証: group6件・tab ghost1件・typed From unit1件はexit0。最終fmt19524はdrop_and_drag既存assertを行数制限のため圧縮した形式が不一致でexit1、supplement69004も前処理exit1（coverage未開始）。test leafを責務分離してfmt/ASTを維持し、品質条件を弱めず再実行する。
- test leaf分離後のfmt11474もexit1。直接rustfmtによるCargo設定との差を正規just fmtで是正中。Just内蔵guardへ外側guardを重ねた二重待機は所有processを終了回収し、Justを直接実行する。coverage未開始のためprofileは変更していない。
- 正規formatterでtest leaf分離後、fmt/AST成功。supplement52337はtab関連全回帰が成功し、LCOV未到達を8→3行（error Fromのみ）へ削減、論理1関数残でexit1を回収。最後のtyped From unit supplement52180を実行中。これは診断でありfresh full release-checkの代用にしない。
- typed error supplement52180: unit1件とstrict gate全体がexit0。8未到達物理行・論理1関数を全て解消。最終診断の全scope profileでLCOV物理行/論理function100%を確認（LLVM別summaryはinstantiation統合のため別値）。全source凍結、fresh release-check-final6を実施する。text_fixesの正規fmt/AST/全lintもexit0。
- fresh最終release-check93362開始。tmp/v0.5.0-release-final6.log/同名coverage、source manifest保存。debug0/incremental0/jobs2、coverage8threads（既存2binary/4threads）・scope/閾値維持。PR/publicationは未実施。
- Rust外の現行コメントを追加監査し、Justfile173の旧外部workflow名1件のみを汎用WHYへ是正。scripts/CI/Cargo/configの他候補0。検証snapshotとの非コメント行を機械比較して完全一致、製品Rust差分0。文書だけの是正なので製品full gateは再追加せず、現93362を継続。
- fresh93362: 全150binary exit0/OOM0、strict gateはroot/root_types.rs76-78のmarker→RootError直接From1関数3行不足でexit1。tmp/v0.5.0-release-final6へLCOV/summary/detailを保存。前supplementには旧unitの到達が残り今回の単純化による欠落を隠したため、診断成功を完全runの代用にしなかったことが有効。既存のsurface/root/facade経路を削らず直接root変換も明示回帰へ追加、全型付きFromを照合後fresh final7へ進む。
- 不足は全LCOV再分類でroot_types76-78のみと確定（tmp/v0.5.0-release-final6-misses.json）。既存marker→surface→root→facadeとsurface→facadeを保持し、marker→root直接Fromのtyped/Display回帰を追加。fresh release-check94847(final7)開始、tmp/v0.5.0-release-final7.log/同名coverage/source-manifest。品質条件と製品実装は不変。公開見込みは再実行分を含め10月26日2:30〜3:30 JSTへ更新。
- marker→root直接From unitは1passed/exit0（tmp/v0.5.0-context-menu-enabled-unit-final.log）。RootError Displayは既存の型付きprefixを含むため、marker原因文字列保持を検査する。full native/coverage開始前に最終assertへ凍結しsource manifestを同期。正規fmt/ASTもexit0。
- final7の94847は全native4889/guard成功後、Docker compile中にI/O errorでexit125。ホスト空き116MiB/100%を原因として実確認。Justの一時fileも作成不能だったため、guarded consumer-app cache cleanで26.9MiB確保→private既定just clean-all80677実行。3.9+9.2GiB回収、別cacheのCACHEDIR.TAG安全拒否で全体exit1（迂回しない）。ホスト空き13GiBを確認。Dockerは当タスク1containerのみ、Colima再起動でI/O復旧中。詳細tmp/v0.5.0-storage-recovery.json。
- 環境復旧: Colima stop/start exit0、unused builder cache1.617GB整理→guest trim18.1GiBでホスト67GiB空き、coverage volumeのwrite/fsync/read実検査exit0。既存image/volumeは削除せず保持。fresh final8 release-check66396とimage再準備25213開始。tmp/v0.5.0-release-final8.log/同名coverage。製品source/品質条件は不変。
- 最終release-check66396(final8) exit0。native全151suite4889passed、strict lint全6構成、native tools10/証跡22/Unicode6、guard全成功。Linux全150binary exit0、strict LCOV物理行/論理関数100%成功、package/publish dry-run成功、v0.5.0未公開を確認。tmp/v0.5.0-release-final8.log/同名artifacts、source2498Rust files不一致0の証跡を保持。これ以降のPR/review/CI/merge/公開/registry/Close/cleanupは未実施であり、このjournalはPR作成前の実績を記録する。
- [/] 公開後整理の既存scriptがremoteを無条件削除することを自己レビューで検出。ユーザーのremote明示時のみ削除ルールに従い、--remote opt-in/既定localのみへ修正し、実Git ref回帰を追加する。製品Rust/受入条件は不変、CIは最終headへ更新して再検証する。
- [/] PR113 P1: TabMenuを開いたままlease再同期すると旧pathが別targetを指す。opaque targetで再配置/消失時閉鎖し実root入力/host effect回帰を追加（thread PRRT_kwDOSYoxuc6wB4m0）。
- [/] PR113 P2: search dialogのEnterをquery focus以外でも奪う。history/delete/cancel等の本来activationへ渡す入力回帰を追加（thread PRRT_kwDOSYoxuc6wB4m5）。既存keyboard要件を満たす是正でDoD/互換性の変更ではないため本PRで修正。

- PR是正の親検証: public suites26件/exit0、private helper1件/exit0、fmt/AST/strict lint全6構成55357 exit0。P1同一公開root回帰は元HEAD lease_stateでRED101（誤target Pin）、修正sourceでGREEN0、finally復元SHA一致を実確認（76641、tmp/v0.5.0-p1-public-repro.json）。P2はSearchRequested誤発行RED101→HistorySelectedのみGREEN、mode切替stale focusも含め15件成功。cleanup実bare remote保持/明示削除9件53589 exit0、guardへ追加。
- 製品是正後の最終full gateと最終head CIは未完。既存Draft PR113を署名付き1commitのまま更新し、full gate/CIを並行する。各threadはpush後にreply/resolveし、再reviewでP0/P1ゼロを確認してからReady化する。
- 署名amend70285は新規guardのcleanup9suite内でexit1。Git hook由来GIT_INDEX_FILE=.git/indexをforeign fixtureが継承し、二次worktreeの.gitがfileであるためNot a directoryになると再現15754で原因確定。main HEAD0cdd180は不変。Gitが列挙するrepo-local環境をfixtureの子processから分離し、通常/hook環境両方で同じ安全回帰を通す（検査や保護を除外しない）。
- cleanup fixtureのGit local envを子processから分離し、通常/hook GIT_INDEX_FILE環境とも10tests exit0。新規worktree生成は廃止しprimary checkoutの実dirty/in-use状態を本体collect→evaluateへ通して拒否を検証、CLIのdirty拒否も維持。fixture切替はswitchへ統一。Rust sourceは固定済み、署名amendを再実行する。

- [/] PR113追加P2: file dialogで選択行を保持したままTabで取消へ移ると、global Enterが取消を奪いFileSelected/Navigatedを発行する。既存keyboard要件の是正として、実focusのactivationへ統一し、file/folder双方の公開root実入力回帰で修正前RED・修正後GREENを検証する（thread PRRT_kwDOSYoxuc6wCNKs）。full9 38502は修正統合のためexit130で回収し、最新sourceのfresh full10/CIを必要とする。

- file Enter P2是正: global Enterを廃止し、矢印選択行へ実focusを設定。eguiの残存方向キー移動をNoneへ戻し、Tab遷移は通常処理を保持。公開16件31260/private selection-dialog13件44859が成功、親fmt-check/AST46927 exit0。旧実装では取消focus後EnterがNavigated/FileSelectedを発行するREDを確認。最新headの完全検証は別途実行する。

- [/] PR113追加P2: lease同期時にtab drag sourceが削除/非draggable化してもself.dragが残る。sourceの存在・draggableを再照合し、無効drag/release待ちを解除する。公開root実dragと関連private回帰で検証（thread PRRT_kwDOSYoxuc6wCbOB）。full10 5206は所有guard SIGINT/exit130回収、修正後fresh full11を実行する。

- 最終検証の実行場所: host空き14GiBと他の使用中buildを確認。private既定clean-allは安全保護を維持して16.8GiBを回収（一部cacheのCACHEDIR.TAG拒否で全体exit1）、builder未使用cache整理/online trimも実施。追加修正の限定検査はlocalで回収し、最新版の完全release-checkは公式release-preflight CI、全native検証は既存3OS CIで回収する。検査scope/coverage/品質閾値を変更せず、full11というlocal重複実行は追加しない。公開前に同一最終headの全required checksと実release-checkのexit/artifactを必須とする。

- tab drag P2親統合: source削除/非draggable/collapsed祖先を同期時に無効化しdrag/release待ちを解除。tab/groupの実Primary releaseをresponse消失時にも処理。公開回帰は削除/disabled双方で次dragのhost.tab_order変更を保証。親38308は同一最終testを旧3sourceでRED101（次操作2件≠期待3件）、fixed3sourceへSHA復元確認後GREEN0。AX初期化不足で失敗した1792942247はRED根拠に含めない。private87件82724/public12件28755、fmt-check/AST83026は全exit0。registry fixtureへ同期し最新head CI完全検証へ進む。

- [/] Ready化に伴う追加reviewでP2二件: group rename draft保持中のrenamable/placeholder剥奪によるMissingRoute（PRRT_kwDOSYoxuc6wCrt-）とdialog rasterのbackend texture上限超過（PRRT_kwDOSYoxuc6wCruC）。Ready時点のmanual reviewは418532abで指摘0だったが、追加結果を受けDraftへ戻して是正。最新版へ全検証を適用する。

- 追加P2共通原因是正を統合中。rename draftは新leaseのrenamable/placeholderを再検証。backend textureは借用RGBAを遅延tile生成する共通primitiveへ統一し、dialog/tab/drag ghost/context menu/command chrome/status/popover/source address/diagnostics全直接描画経路を保護。全画像複製や新publicAPI・品質閾値変更は行わず、実textures_deltaの各寸法と非空、既存配置・状態契約、画素復元/比例boundsの無隙間・非重複で検証する。全worker source凍結、親fmt/AST59664と統合検証を実行する。
- 容量復旧: private既定clean-all31928はCACHEDIR.TAG/active cache保護を保持してexit1、未使用のrepo内probe Cargo target（正規CACHEDIR.TAG）をguarded cargo clean83626で1.3GiB回収。未使用の当task Docker registry cache362MiB/再生成可能coverage image832MBを所有確認後整理、online trim68400でhost空き5.8GiBを確認。既存source/現在の検証証跡・他task使用中cacheは保持。最終full検証は公式CIの同一release-checkを使用する。

- 最初の統合再現39641は全体exit1: renameはroute unavailableのRED101→GREEN0だが、dialogはRawInput max64がegui TextureAtlas最小1024を下回り両方Tiny texture atlasで失敗。dialog側を不具合再現に数えず、全新公開texture回帰を1024へ是正し、dialog/本文等は高さ>1024かつ8192/16MP内へ調整。失敗source/logをtmp/v0.5.0-review6-invalid-atlasへ保持。static11444/fmt19868 exit0、同一公開RED/GREENを82281で取り直し中。

- 統合runner39903はexit101: dialog最終GREEN1件とtile private3件はexit0、tab privateは86成功/2失敗。既存rename path保持2fixtureがrenamable=false/popup無しのままeditorを手動設定しており、新contract revoke検査が正しく閉じたための不一致。元のopaque path/renameSome/target削除Closed assertionは保持し、fixtureだけ有効なrename許可+placeholderへ整合して取り直す。公開9suiteは未実行。失敗ログ/JSONはtmp/v0.5.0-review6-before-rename-fixture-fixへ保持。

- local統合55430はdialog GREEN1/tile private3/tab private88が全成功した後、公開9suiteのtest helper mut漏れでcompile101。mutを是正しprivate testをidentity/capabilityのcohesive leafへ分割、正規fmt/AST66141 exit0。再実行26123はdialog GREEN0の直後にJSON保存でENOSPC/exit1、残検査は未実行。主targetは683MiBにすぎず、稼働対象を保護しguarded core cleanで266MiB→private既定clean-all90939で3.9GiB回収（安全拒否を保持して全体exit1）・online trimを実施、host空き4.5GiBへ復旧。再度localで同じ全検査を重ねず、公式最終head CIの公開全suite/strict6 lint/完全release-checkを必須回収する。検査scope/coverage/閾値は変更しない。

- [/] PR113第7レビュー: modal初回frameでeguiの事前計算済みpointer responseが背面tabへ届く（PRRT_kwDOSYoxuc6wDHww）。背景rootを同一disabled scopeで描画し、本文anchorとtab/group interactionを停止。進行中tab dragは旧port/correlationへCancelを一度通知し、通知失敗時は旧状態を保持する。invalid lease同期も同じterminal処理へ統一。公開click/drag×file/search、dialog実入力、close後再操作/再dragの同一回帰を旧rootでRED101→修正rootでGREEN0、73737 exit0/finally SHA復元true（tmp/v0.5.0-modal-public-repro.json）。関連する独立Areaの親enabled伝播とdisabled本文focusも共通原因として是正し、追加回帰へ固定する。
- f0fa82 CI3workflowは全failureを回収。strict lintのneedless_borrow3箇所と固定長bufferのuseless_vec1箇所を機械修正。品質抑止やscope/閾値変更は行わず、新headで全ゲートを再実行する。

- modal関連の親統合61005: egui private899件中898成功、rename初回mountへText/Enterを投入する1fixtureが失敗。Area sizing passはinvisible=disabledとなる実egui0.36.2 sourceを確認し、サイズ計測中はmodel focus/selection/composition/anchorとpending focus requestを保持して入力・ackだけを実行しない。明示disabled時の解除契約は維持する。公開59510はcommand chrome8件成功、context10成功/新1失敗（Openedを含むrouter総数の0期待誤り、blocked activation無しは成功）。down後baseline不変＋reenable後activationでfixtureを整合し、再実行する。検証失敗は成功へ含めない。

- 第7レビュー統合の回収: 背景無効化前のrestore focus保存順序を是正。サイズ計測中のmodel/pending focus保持後のegui内部900件とpointer suspend1件は19205で成功。focus capture是正後のdialog内部13件は51725で成功。最終公開9suite/110件は5274 exit0（tmp/v0.5.0-modal-final3-public9.log）で、click/drag×file/search、閉後同focus復帰/本文入力/新dragの実host order、disabled Areaのactivation遮断/reenableを全て検証。host fixtureはBefore/After/EndOfStripを実配置し、Cancel通知によるStart1→Cancel2→次Start3/Finish4を保持。text-raster専用root fixtureはport不要として未使用helper警告を除去。fixture compile時の不足import2件は是正し失敗logを保持。fmt56352/AST89195はexit0、strict6 lint82408は実行中。registry専用fixtureは25 Rust files（24完全copy＋facade source guard4件のみ除外）へ同期、公開後に実registryから取得して検証する。

- 最終strict lint全6構成63736 exit0を回収。先行82408は新modal host fixtureのcollapsible_ifでexit101、抑止せず等価なlet-chainへ修正。fmt89054/差分check成功、製品sourceは以後固定。公開110件/9suite、dialog内部13件、共通入力内部900件と固有RED/GREENの実証を保持し、署名1commitの更新・同一head公式CI/reviewへ進む。

- [/] 第8review PRRT_kwDOSYoxuc6wDlhb: 無効tab leaseのCancel拒否時に外側rootだけ新本文/revision/routerへ進み、同revision retryがDuplicateLeaseとなる。唯一fallibleなtab同期を外側状態更新より先行し、残りをinfallible commitへ分離。通常/状態保持付きの両経路を是正し、private sync責務をcohesive leafへ分離。親47631は同じ現行public/private回帰を旧processでRED101（通常はrevision2/期待1、状態保持はrouter new/期待old）、修正版でGREEN0（内部12＋公開1）、finally復元SHA一致を実確認。公開回帰はAX本文・実root router・旧port correlationとdrag状態を保持し、同revision retry後に新本文/router/target選択を確認する。typecheck54704/AST20676/fmt98090も成功。fixtureの戻り値・viewport定数・identity責務分離を是正し、重複した空identity回帰1件は元assertをidentity leafに一度だけ保持する。関連全suite/strict6lint/改訂head CIは別途必須。

- 第8是正の最終限定検証: 関連host-root内部34件＋公開facade17件/tab host14件の計公開31件が44460 exit0。strict lint全6構成45889 exit0、canonicalfmt74319/AST37765/差分check成功。レジストリfixtureの公開host回帰も同期済み。host空き3.6GiBを検出しprivate既定clean-all81374はactive KUC/CACHEDIR保護を保持して2.9GiB回収（全体exit1の部分成功）、6GiBへ復旧。共有buildの消費で現在2.8GiBのため、安全な既定整理を継続する。改訂headで全CI/完全release-checkを必須回収し、既存の品質閾値は変えない。

- [/] 第9/10review: valid drag中の新leaseで通知先だけ更新され旧Start hostへ終端が届かない（PRRT_kwDOSYoxuc6wD2-h）ことと、inline rename requestのfalse→true再要求が消費済み扱いで開かない（PRRT_kwDOSYoxuc6wD2-l）ことを是正。drag originは最初のlease更新時に旧port/correlationを一度だけ所有し、後続通常操作は最新port、Finish/Cancelだけ開始portへ配送。取消拒否時はorigin/dragを保持し、新portへのfallbackは行わない。inline consumedは要求falseで解除し、手動popup renameの有効性はフラグから独立させる。親28921は同一公開17件を旧lease_stateでRED101（新3回帰のみ失敗/既存14成功）、修正後GREEN0/17成功、finally復元SHA一致。独立old/middle/latest hostと異corr、実drop/Escape、旧hostのorder/activeと最新hostの選択、確定/取消後の再要求・true据置きの非反復を検証。全tests typecheck5910/AST63386/canonicalfmt97540/差分checkも成功。関連private/公開modalとstrict6lintは実行中。

- 第9/10是正の関連検証18885はprivate tab-strip94件/public modal18件でexit0。strict lint全6構成40646もexit0。原点port欠落時に最新portへfallbackしないこと、原点Cancel拒否時の保持と再試行、複数同期での開始port維持を固有private回帰で検査。`docs/tab-strip-host-proposals.md`へ継続操作の配線寿命とrename再要求を明記。製品sourceは固定済み、署名1commit更新・同一改訂headの全3OS CI/完全release-checkへ進む。

## 追加レビュー11・12（2026-10-26）

- [x] 末尾改行でscroll content extentだけが拡張した場合も、本文textureの描画寸法をraster寸法へ保持する。公開root/adapter数値回帰で文字伸張なし・空行scroll/caret維持を検証する（PRRT_kwDOSYoxuc6wEMvK）。
- [x] RTL行のhard breakを論理行末のvisual edgeへ配置し、改行前caretと右端hit-testの誤挿入を防ぐ。実rasterと公開入力経路で旧実装RED/修正GREENを回収する（PRRT_kwDOSYoxuc6wEMvO）。
- 現head90a55のNative compile contract3OSは成功。CI3OSと完全release-preflightは進行中。修正後headの全7checkを改めて回収し、未対応指摘を残したReady/mergeは行わない。

- 依存最終監査: minifb0.29/resvg0.49の公式変更と実使用箇所を照合。初期patch理由の据置き記述だけでは最終v0.5.0の採否根拠として不足するためstep2を再開。egui/eframe0.37は公開AsId削除・Ui ID契約変更を含む。minifb/resvgは実更新・数値/入力回帰で判断する。詳細tmp/v0.5.0-dependency-final-audit.md。

- review11/12: 末尾空行textureは旧assignmentで24px/48pxのassert失敗（tmp/v0.5.0-review11-red.log）、修正後の公開render14件を含む公開5suite32件成功（52707/exit0）。RTLは同一公開回帰で旧実装17.679688対0.0のRED101→GREEN0（87169、tmp/v0.5.0-review12-reproduction.json、sourceSHA復元true）。最終実装はcosmic glyph自身のresolved bidi levelを利用し、空行・混在行と内部raster88件/surface38件が成功。
- AST300行超過でadapterのraster/gutter処理を責務単位のrasterization子moduleへ分離し、閾値を維持。canonicalfmt33598/AST39786/fmt-check88239はexit0。全6lint21926はexit0で成功。最終版のCI/完全release-checkとレビューを回収する。

- 最終435859 CI40228344111: WindowsのStorybook bin27件中1件が旧minifbのpayload無しDisplay期待で失敗（main.rs:387）、Mac/Linuxはfail-fastでcancel。0.29の製品挙動は原因文字列を保持する正しいもの。期待値をpayload有りへ強化し、bin全27件を検査して同じ1PRへamendする。完全preflight40228344137は進行中で成功扱いにしない。PRは修正中Draftへ戻した。

- 上記binの2期待を原因文字列を含む形へ強化し、bin全27件GREEN/exit0、canonicalfmt/diffcheck成功。tmp/v0.5.0-ci-display-green.log / ci-display-reproduction.json。製品の挙動と品質閾値は維持し、最新headで全CIを再回収する。

## 追加レビュー13・14（Ready後）

- [x] softwrapで同じlogical_rowを持つvisual rowを維持しつつ、自動gutterとAX targetを論理行ごとに一意化する（PRRT_kwDOSYoxuc6wEqqs、4372269157）。
- [x] 検索履歴でqueryが変わった際に旧selected resultを無効化し、新lease未到着時のEnterが古いResultSelectedを配送しないようにする（PRRT_kwDOSYoxuc6wEqqu、4372269160）。
- ad9cdffの追加差分レビューは0件だったが、Ready自動レビューで上記2件を回収。PRをDraftへ戻し、同じ1PR/1Releaseで修正・実回帰を継続する。完了条件・品質閾値は維持。

- review13/14最終検証: mainがgutterのconfig/rows解決をcohesive child gutter/rows.rsへ分離し、constructor/range helperを親へ保持。BTreeSetで自動gutterとviewport絞込み後の可視論理行を一意化、navigation用visual rowsは保持。履歴/include/exclude/option変更の全経路で旧selectedを解除した。公開35件（初回63961exit0）を同じまま旧gutter/search3fileで実行すると3件がRED101（gutter25対1、履歴/含めるがResultSelected対SearchRequested）、SHA完全復元後GREEN0（40136、tmp/v0.5.0-review14-reproduction.json）。
- 最終sourceでcore全1991件/exit0（28144）、strict lint全6構成/exit0（42559）、AST70029/fmt-check25903もexit0。独立read-onlyレビューは追加P0/P1/P2なし。ローカル失敗14260はmodule分割後のimport1件でexit101、修正後の上記GREENとは区別する。

## 最終CIの関数カバレッジ補完

- a7826c9のCI/Nativeは全3環境で成功。preflight40232956516はLinux全150binaryがexit0、LCOV全行到達だが、lease_state.rsの関数40/41で失敗した。取得済みLCOV/detail JSONをregion単位で集約し、root_itemsが空の旧形式projectionでdirect root tabを逆引きする104行目のclosureだけが未到達と特定した。
- 既存のmixed/nested/fallbackテストへ、root tabのopaque targetからroot-tab-0への逆引きassertを追加する。製品実装と行・関数100%基準は変更せず、限定検査後に同じ1PRを更新し、改訂headの完全検査を再回収する。
- 追加assertを含む既存回帰は52189で1passed/exit0、fmt-check75293/AST46938/strict lint全6構成80718もexit0。実績はtmp/v0.5.0-coverage41-*.logへ保存した。新headの完全検査と公開は未完であり、先行headの失敗を成功扱いにしない。

## 追加レビュー15：検索入力のIME確定

- [/] Ready自動レビューで、query/filterのIME変換中EnterがSearchRequested/ResultSelectedを発行するP2を回収した（PRRT_kwDOSYoxuc6wGOeQ、4372931085）。現在modeの実focus中入力にcompositionがある間と、同frameにIMEイベントがある場合は検索ショートカットを送らない。確定後の通常Enterと他modeの入力は抑止しない。
- 公開rootからquery/include/exclude・結果選択の有無・Commit/Enterの順序を組み合わせ、変換開始同frame・継続中・確定同frameの誤配送なし、実確定文字のAX value、通常Enter復帰、未確定の別modeからの切替後の検索を回帰検査する。旧sourceで同じ回帰のRED、修正sourceのGREENとSHA復元を回収し、同じ1PR/1Releaseのまま最終検査を再実行する。
- 親21541で同一公開IME回帰を旧search_actions.rsへ戻すとRED101（mode1・選択なし・Enter先行でhost誤配送）、修正sourceへSHA復元後は公開21件GREEN0。tmp/v0.5.0-review15-reproduction.jsonへ実終了と復元一致を記録した。core全1991件53769/canonicalfmt41595/AST最終28329もexit0。最初のAST57663はWHYコメントの書式でexit1となり、既存規約のblock形式へ是正した。
- strict lint全6構成20094/fmt-check48865もexit0。独立差分レビューの追加P0/P1/P2は0件。registry-only公開回帰へ新IMEテストを同期し、25完全copyの一致を確認した。旧head3b8aの長時間preflight40244441783は修正統合に伴いcancelledを実確認し、最終sourceの成功とは扱わない。改訂headの全検査・レビュー・公開を必須回収する。
