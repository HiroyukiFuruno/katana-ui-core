## 1. 公開回帰

- [x] 1.1 #52: legacy document typographyの整数座標契約を回帰テストで固定し、fractional baselineを明示したhostだけへlogical cursorを限定する。
- [x] 1.2 #52: render、wrap/layout、action hit、node hitが同じtypography選択を通ることを検証し、KDV registry-onlyの95/95再検証条件を記録する。
- [x] 1.3 #54: KUC所有のUnicode evidence pin resolverをconsumer artifactの既定issuerへ接続し、consumerがplatform policyを注入せず実行できる回帰テストを追加する。

## 2. 公開

- [x] 2.1 v0.3.10へ更新し、format、check、release-check、diff checkを通す。
- [x] 2.2 Draft PR、review、thread disposition、Ready、CI、mergeを完了する。
- [ ] 2.3 GitHub Release、crates.io、KDV/KLE registry-only再検証、branch hygieneを完了する。
- [ ] 2.4 Unicode evidence unavailableの原因型をpublic consumer artifact errorに保持し、consumer側の判別を回帰検証する（[Issue #57](https://github.com/HiroyukiFuruno/katana-ui-core/issues/57)）。

## 最新指示と実行台帳

- 最新指示: v0.3.10を正式リリースするまで継続する。目的が明確な承認済み工程で停止・再承認要求をしない。
- [/] 承認済み操作・担当引継ぎで再承認を求めない規則をAGENTSへ反映した。
- 承認範囲: commit / push / review reply / merge / release / cleanup。閾値緩和、未公開path/git依存、稼働targetのcleanは禁止。
- 既存検証: `/private/tmp/kuc-v0310-check-final.status` = 0、`/private/tmp/kuc-v0310-release-check.status` = 1。各同名 `.log` に出力を保持。
- coverage不足: Docker volume `kuc-coverage-target` の `kuc-workspace-coverage.lcov`。unicode_evidence、renderer_cursor、renderer_accordion_cursor、renderer_row_cursorの4ファイル。
- 依存調査: `cargo outdated --workspace --depth 1` と `--depth 999` はともに終了0。lockfileではegui群0.36.2など外部20packageが互換更新済みで、追加更新対象なし。ホストと最終coverage snapshotのCargo.lock SHA-256は `036a92e061a4217e1f273a36c1b35eca29c276b4b80a66c2fe8b434f28d07d3d` で一致する。
- memory ad-hoc noteへの追記は外部編集hookが拒否したため未実施。製品リリースの進行は継続する。
- 追加回帰: all-features lib `unicode_evidence` 55件、`logical_cursor` 3件が終了0。fmt / diff check / guardは終了0。独立製品レビューはP0/P1/P2指摘なし。
- 最終ゲート: `rtk proxy just VERSION=v0.3.10 release-check`。ログ `/private/tmp/kuc-v0310-release-final.log`、終了状態 `/private/tmp/kuc-v0310-release-final.status` に保持する。
- 最終ゲートsession_id 26860は終了1。全製品分岐は網羅、cursor.rs:633のassert失敗時メッセージ用`node.kind()`だけが未到達。静的メッセージへ変更して余計な未到達式を除去し、数値assertは維持した。
- 再実行ログは `/private/tmp/kuc-v0310-release-final-2.log`、終了状態は同名 `.status`。runningの場合は既存実行を回収し、重複起動しない。
- 再実行session_idは43856。KDV既存task `01a04a99-ba8b-7f12-8b68-188c349307de` とKLE既存task `019f2593-b323-7480-910e-700dfeecd8b9` は公開後のregistry-only再検証を準備済み。公開通知までは依存変更・ゲート実行を保留する。
- session43856は終了0。strict LCOV coverage passed、package検証、publish dry-run、単一公開crate検査、未公開version検査を完了した。最終ログは `kuc-v0310-release-final-2.log`。
- PR #55はDraft中のCodex reviewと自己レビューで指摘なしを確認後、Ready化。最終HEAD `8ef1d4a70daea059b6e9edbeb7910e0c11bdb197` の全review thread取得で未解決0、3 OS CIとpreflightはすべてsuccess。2026-09-12T22:33:38Zにmergeし、merge SHAは `772fb07d02607278dc59cf9b2e9068f7362ab85e`。
- リモートpreflight run `34718941782` は127 test binaries成功、strict LCOV coverage passed、package、publish dry-run成功。watcher終了コード0を回収済み。
- Release run `34723087129` は全step success、strict LCOV coverage passed、watcher終了コード0。2026-09-13T00:41:48ZにGitHub Release、00:42:12Zにcrates.io公開を確認した。tagと配布crate VCS SHAはmerge SHAに一致し、配布checksumはregistry indexと `1180cf3ff66d8f030179f60633aec012885047434676c85db4eca38f27d69d54` で一致、yanked=false。Cargoからv0.3.10取得成功。
- KDV再検証は公開registryの `=0.3.10` を使用。cargo update/cargo treeは終了0、sample.mdは89/95、sample_diagrams.mdは91/95で両受入テスト終了101。受入未達を[Issue #52](https://github.com/HiroyukiFuruno/katana-ui-core/issues/52#issuecomment-5649766521)へ記録しopenを維持。既存KDV作業で旧版比較・typography選択・残差の所有境界を切り分ける。公開完了はこの受入未達の解消を意味しない。
- KDVの追加比較ではv0.3.9/v0.3.10のlegacyが同じ89/91。公開baseline APIにKatanA artifactのlogical値を与えた候補は68/93へ悪化したため不採用。KUC public typographyとcanonical visual contractの残差は[Issue #52](https://github.com/HiroyukiFuruno/katana-ui-core/issues/52#issuecomment-5649790103)へ継続課題として残す。
- KLEは公開registryの `=0.3.10` と同checksumで `just kuc-contract-check` が終了0。default `ConsumerArtifactPlanIssuer::new()` とconsumerによるplatform policy/font hash注入なしでartifact実行成功、KUC関連テスト5 passed（113.07秒）。[Issue #54](https://github.com/HiroyukiFuruno/katana-ui-core/issues/54)を完了した。
- KLE成功証跡はsession `25471` の直接回収による `exit_code: 0`。成功artifactは `target/acceptance/kle-storybook-consumer-artifact/run-3455-1789261055299844000`。非採用background runのログは成功証跡として使用しない。
- リリースブランチ `release/v0.3.10` はmerge時にlocal/remoteとも削除済み。既存worktreeは保全し、stashは0。対象外のremote branchは保持した。
- 2.3はGitHub Release/crates.io公開・KLE受入・release branch整理を完了したが、KDV受入が未達のため未完了として保持する。本changeはarchiveしない。
- PR #56レビューで、v0.3.10のpublic artifact errorは `UnicodeEvidence(String)` へ原因を変換することを確認した。main specは公開済み挙動を記録し、deltaのtyped platform-unavailable要求は未達として2.4とIssue #57で保持する。

## 受入未達の修正を再開

- 最新ユーザー指示「えっと止まってるよね？？？ｗ」を受け、公開だけを区切りに停止した判断を訂正し、未完了2.3/2.4を継続する。
- [ ] 公開を完了条件へすり替えず、KDVの両sample95以上とregistry-only再検証、未達のtyped error要求まで解消する。
- v0.3.10の公開成果物は不変。修正候補はfix/issue-52-public-typographyで実装・検証する。公開前にKDVの正確な入力と数値契約を使って候補を検証し、公開後にregistry-onlyで再検証する。
- 閾値緩和、consumer固有の補償geometry、未公開path/git依存を最終成果物へ残すことは禁止。
- 隔離したKDV consumer（`tmp/issue52-consumer-probe`、専用target、作業中KDVのtracked filesを複製）で公開版と同じ89/91を再現した。診断用path patchはこのコピーのみに限定する。
- #52調査: KUC compact wrap scale 1.10とKDV line counter 1.25が不一致。1256 logical pxの本文がKUCで1行、plannerで2行になる。1.25候補と厳密な2行回帰を検証中。
- 参照の由来: diagramsの固定cropはphysical `2374x4450+64+202`でfilename headerを含む。sampleには元screenshot/crop manifestが不足。KatanA担当が両fixtureの同一runner原本とcontent geometryを採取し、KDV担当がdocument-content ROIへの適合を検証する。得点を根拠に参照を変更しない。
- #57候補: 既存enum/execute_nextは維持し、非網羅の`ConsumerArtifactPlanExecutionError`と`execute_next_with_evidence_error`を追加。focused lib 40件とfmtが成功、独立レビュー中。task完了はfull gateと公開後の受入まで保留する。
- ディスク不足はactive cargoがないことを確認後、KUC packageの生成cacheのみ`cargo clean -p katana-ui-core --target-dir target`で解消した。以後は`CARGO_INCREMENTAL=0`を使用する。原本・検証ログ・coverage evidenceは保持した。
- #52原因を更新: 実consumerは`paragraph`、KUC compact wrap対象にこの本文roleが欠落していた。logical fontは14で、物理font28の判定問題ではない。paragraphを追加した候補でsample90/95、diagrams91/95。本文2行目のloss band101..114は解消し、見出し位置とcanonical ROIの残差を継続調査する。
- `cargo outdated --workspace --depth 1`、`--depth 999`を再実行し両方終了0、追加更新対象なし。
- #57候補は独立レビューP0/P1/P2なし、focused lib41件成功。`just check`はkalのtypes.rs301行制限で終了1となり、追加エラー型を専用moduleへ分離して再検証する。
- [/] #52独立レビューP1: span link hitが未改行の元spanを1列配置しており、描画の`span_lines`と不一致。折り返し結果から行別hit rectを作り、2行目の実リンク位置と旧位置無効、同一linkの複数lineでaction保持を回帰検証した。独立再レビューP0/P1/P2なし。
- #52独立レビューP2の採否: 既存body/HTML/export/list等は従来の1.10を保持し、KDVと同じgeneric paragraphのみ1.25とする。既存roleへ無関係なwrap変更を広げない。
- #57追加型の責務分離後、focused41件/fmt/diff/`just ast-lint`がすべて終了0。
- 隔離consumerのcanonical viewportと公開baseline API同時probeは45/93。旧参照への適用を採用せず、原本のsemantic ROIと実際の入力geometryを揃えた検証へ進む。ログはRTK `1789283804_cargo_test.log`、dumpは`tmp/issue52-probe-canonical-baseline`。
- v0.3.10は公開済みで不変とし、残受入の修正版をv0.3.11として準備する。作業branchを`release/v0.3.11`へ変更し、workspace versionとKUC/storybook lock entryを更新した。95/95とP1解消、full gateを満たす前には公開しない。
- 新規原本のSHAとgeometryを検証し、diagnostic document-content ROI比較は50/85。KDVでもregistry0.3.10で45/85を確認。両方未達。KatanAの同一frame/source/controls manifestを監査し、KDV paragraph softbreakの行数・明示高さと描画の不一致、heading高さ契約を担当タスクで切り分け中。
- `just check`は新規wrapped-link回帰追加に伴うtest file長321行で終了1。テストを責務ごとに分割し、閾値を維持して再検証する。
- wrapped-link testsを専用moduleへ分離しast-lint成功。全体checkの次段で新error型のouter public facade reexport欠落と旧hit幅計算のdead codeを検出した。公開経路を修正してexternal integration testを追加し、不要になった旧製品helpersを削除する。内部lib test/局所レビューだけでは公開API reachabilityを検証できていなかったため、外部crateのimportとpattern matchを必須証跡にする。
- outer facade/旧helpers修正後、外部API integration5件と全features lib Clippyが成功。`just check` session37373は終了0、4,484 tests/128 suites、全Lint・guard・77 story contract成功。`just VERSION=v0.3.11 release-check` session84123を継続中（ログ`tmp/v0311-release/release-check.log`、終了時`.status`）。E2E/smokeは通過済み。
- [/] scroll回帰の初期診断を訂正: 通常rootの期待y11は誤りだった。実際のP1は通常rootで描画に適用されるscrollがhitに欠落すること。共有visible_line_yで修正し、ScrollArea内部はcontent高さで収集して親clipへ渡す。通常rootの実描画glyphとhit、Document/Viewport、短い明示height/fractional lineboxを6件の回帰で検証した。独立再レビューP0/P1なし。
- source変更が必要になったため、旧候補のrelease-checkはcoverage開始時に当該container `2f3ede518c3b` だけを停止して終了137を回収した。既存helper containerは保持。これは品質ゲート成功ではなく、修正後sourceでrelease-checkを再実行する。既存check成功もscroll修正前の証跡として区別する。
- KDV担当はexport surfaceのsoftbreak結合/Markdown hardbreak区別を修正中。ただし初版2fileを隔離consumerへ反映しても50/85は不変。scoreの実入力node propsを採取し、heading fixed heightとpublic baseline APIの指定を照合する。

- 修正前full check session27099終了0、4,487 tests/128 suites、全lint/guard成功。scroll最終修正後のrelease-checkは別ログrelease-check-2へ実行し、旧成功証跡と区別する。
- Text Autoの診断probeはsample48/95、diagramsゲート成功。明示heightを自然text高さへ委ねる影響であり、実KDVへの採用は担当のgeneric契約修正と検証で確定する。controls-off参照のoverlay-v2 manifestと両PNG SHA/同一frame/操作stateを直接照合済み。

- [x] #52根本原因: FirstCandidateの非400候補がSystem fallbackへ置換される問題を修正する。候補実index/weight/styleを内部保持し、source/indexを測定・描画で固定する。通常/bold/italic/任意weight、proportional/monospace、System既存挙動を回帰検証する。
- 実測はHiraginoW3.ttc index0/weight300でadvance551.347900、aliasへ400要求時はSFNS455.464844。KatanA参照551.375と一致するface選択が原因。以前のparagraph1.25補正は誤ったfaceの幅を経験的に補う候補だったため撤去し、raw描画幅に収まるparagraphの1行契約を追加する。既存body回帰は元の契約を維持する。
- このsource変更後はrelease-check-2の結果を最終証拠に使わず、修正後full gateを改めて実行する。KDV自然高は旧公開0.3.10で他契約を満たさず実作業側で撤回、KUC/tmp隔離候補には診断用に保持し、新face修正後に受入を再評価する。
- FirstCandidateはcollection index/weight/style/stretchを実faceから保持する修正を完了。実non400候補の全glyph source/index一致、System/emoji維持を回帰検証し、独立レビューP0/P1なし。最終sourceの`release-check-4`内で全4,492 tests/128 suites、全Lint・guard・E2Eが成功し、strict coverageへ進行した。全release-checkの終了はまだ未回収。
- 同一KDV baseline sourceの新KUC比較は626 passed/12 failed/21 ignored。旧KUCの11失敗に加わった1件はasset load timeoutで、同じ候補の単独再実行は1 passed/24.25秒、終了0。canonical候補はsample53/diagrams98のため、2.3は未完了のまま自然row高さとplan anchorの整合をKDV担当と修正する。

- 再開後の最終レビューで fractional Text の node/non-link hit の末尾1px欠落を検出し修正中。描画cursorやlayout advanceは維持し、hit範囲を floor(start)..ceil(start+既存整数paint height) とする。active heading後のlegacy Textも実paintとの包含を検証する。
- Linuxでwrapped linkの行数がmacOSと異なるため、固定4行の回帰をunscrolled全hit列/実glyphとの比較へ変更した。閾値や実行範囲は維持。release-check-4は4,492 tests/全Lint・E2E成功後、Linux 1件失敗で終了1。
- release-check-5は同URL別link spanのlabel誤対応を追加レビューで発見したため終了143へ打ち切り。これは成功証跡ではない。source spanの同一性を保つ修正と回帰を追加し、独立レビュー後に全gateを再実行する。
- KDV canonical body row leading/gapとcontent paddingの診断修正でsample63/95、diagrams/export通過。受入未達を保持。HTMLfragment高さ42/間隔14の由来をKatanA実UIで計測中。以前のegui縦spacing8という説明は誤り（defaultは3）で撤回済み。

- [x] #52残差の最大原因としてH4/H5/H6がH3へ束ねられ、後続H3/listへ最大14px累積することを数値解析で特定。H4〜H6の明示typography overrideを互換追加し、consumerで各native font/linebox/baselineを指定して検証する。未設定role/既存H1〜H3は維持。追加後の最終full gateを必要とし、gate7は追加前ソースの証跡として区別する。

- [x] #52追加原因: logical RowのCenter指定が子yへ反映されず、Row28内Text21が上端配置されていた。nativeの+3.5pxを一般Row整列契約で実装し、描画/node/action/scroll回帰で固定する。Paragraph→HTML gap18の補償候補は不採用とし、native gap14 + Row中央配置へ戻す。gate8の4,510 tests/全Clippy成功はRow修正前証拠、新sourceで全gate再実行を要する。

- [x] #52追加原因: inline-code/monospace spanの幅計測と実描画が別faceになる属性欠落を解消する。同一galley内のfont face/配置を保持し、normal/code混在、リンク、既存legacy/public互換を実glyph/幅/hitで回帰検証する。gate9の4,518 tests/全Clippy成功は修正前証拠とし、修正後に全gateを再実行する。

- [x] #52追加原因: fractional buttonのnode/action hitが描画下端1pxを含まない問題を修正し、通常/2倍canvasで全hover差分pixelsの包含と次cursorの維持を回帰検証する。consumer pan-up実測はhit[306,334)に対しpaint307..334 inclusive。

- [x] #52残差: native mark背景RGBA(255,255,0,60)をconsumerテーマへ供給できるよう、optional `text-highlight-background` tokenを追加する。未指定色・current-highlight優先順位・geometryを保持し、light/dark実paint回帰とconsumerの実背景への合成値で検証する。

- [x] #52公開hover helper回帰: UiTree::with_hover_surface_for_node_id がTextのclip用heightをStackのlayout advanceへ変え、fractional heading後の全行を移動させる問題を修正する。通常/hover actual hit・後続y不変、legacy/scrollを回帰化する。gate10は全Clippy成功後に終了143、最終成功証拠にしない。

- [x] #52追加回帰: ScrollAreaの内部offset適用後にabsolute controlへ再減算される問題を修正する。内部/外部offset、完全/部分表示、実hover描画/hit位置の一致を回帰化し、consumerの残hover検証を完了する。

- [x] #52最終レビュー回帰: fractional scroll境界をlogical top/bottomで判定し、Auto単一Text HoverSurfaceと描画の高さを共有する。複数child HoverSurfaceを単一Text用partial経路から除外し、既存explicit高さの単一Textを維持する。修正前2件失敗を回収済み、修正後focused/全体/strict coverage/consumer fullを最終sourceで検証する。

## v0.3.11 最終候補の検証結果

- `just VERSION=v0.3.11 release-check` gate14は終了0。最終macOSは4,535 tests / 129 suites成功、Linuxは128 binaries / 4,545 passed / 0 failed / 0 ignored。全Lint・guard・E2E、strict LCOV coverage、crate package検証、publish dry-run、単一公開crate・未公開version検査が成功した。
- gate14開始時の3,317ファイルは終了後のSHA-256照合で変更0件。coverage profileは `48198854f1ebfeb6008470ea330106e6a3fa597dac74bd23eec09bce16299d62`、strict-stateはpassed。生ログとLCOVは `tmp/v0311-release/release-check-14.log` / `gate14-coverage.lcov` に保持する。
- 同じ候補の隔離consumerは全target Clippy終了0、core library 1,907 passed / 0 failed / 1 ignored、Storybook 645 passed / 0 failed / 21 ignored。canonical sample / diagramsは両方95以上、export・hoverを含めて成功した。画像ロードの既定8秒制限と品質閾値は維持し、46ファイルのhandoff hashを固定した。
- ここまでのconsumer結果は診断用path patchであり、公開registryの受入証跡とは区別する。2.3/2.4はv0.3.11の正式公開・registry-only受入が揃うまで未完了とする。
