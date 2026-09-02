## 1. 公開契約と型

- [x] 1.1 可変 viewport 用 artifact、manifest、source viewport、semantic evidence summary の公開型を追加する
- [x] 1.2 `MotionArtifactWriter::write_opaque_variable_viewport` を additive API として公開し、既存 `write_opaque` 契約を維持する
- [x] 1.3 新しい型と API を public facade および consumer contract から利用できることを検証する

## 2. 正規化と artifact 生成

- [x] 2.1 receipt の stage、PNG、provenance、RGBA、非空 pixel を既存契約と同じ厳格さで検証する
- [x] 2.2 最大 source viewport の固定 canvas へ各 frame を左上等倍配置し、source 外側を不透明黒で pad する
- [x] 2.3 専用 staging PNG 列から GIF/MP4 を生成し、正規化 source/decode hash と count/dimensions の一致を検証する
- [x] 2.4 元の source viewport、PNG SHA-256、root record hash と artifact/canonical hash を新 schema manifest へ記録する

## 3. Semantic evidence

- [x] 3.1 各出力 frame と同じ root の receipt observation から正確な `⭐️`、IME、hit test、AccessKit の opaque summary を生成する
- [x] 3.2 semantic evidence の全 contributor root record hash が対応 receipt と sequence 内 provenance に含まれない場合を typed error で拒否する

## 4. 自動テストと回帰

- [x] 4.1 異寸法 sequence の canvas 選択、配置、padding、manifest 順序を unit test で検証する
- [x] 4.2 source/decode hash、count/dimensions、evidence mismatch の fail-closed 回帰テストを追加する
- [x] 4.3 既存 `write_opaque` が異寸法を `WrongDimensions` で拒否し続けることを回帰テストで固定する
- [x] 4.4 full-motion plan の resize、IME、日本語、正確な `⭐️`、hit test、AccessKit evidence との結合を contract test で検証する

## 5. リリース検証

- [x] 5.1 直接・推移依存と `Cargo.lock` を監査し、既存品質 gate を維持できる互換版を確認する
- [ ] 5.2 focused test、`just check`、`just VERSION=v0.3.4 release-check`、`git diff --check` を入力保護修正後の最新 HEAD で成功させる
- [x] 5.3 Issue #34 と OpenSpec の追跡可能性、changelog、version を v0.3.4 release 成果物へ反映する

## 6. User Review Phase

- [x] 6.1 PR工程を `Draft作成 → review → 指摘対応・reply/resolve → Ready化 → merge` の順序へ固定し、Draft省略を禁止する
- [x] 6.2 `release/v0.3.4` をpushしてDraft PRを作成し、`@codex review` と自己レビューを実行する
- [ ] 6.3 review指摘を優先度分類し、P0/P1を必ず修正して各threadへreply/resolve後、最新HEADで再検証してReady化する
- [ ] 6.4 required checks完了後にmergeし、Release workflow、tag、GitHub Release、crates.io `katana-ui-core@0.3.4` を個別に確認する
- [ ] 6.5 `branch-hygiene` に従ってbranch/worktreeを整理し、Issue #34へ完了証跡を反映してcloseする

## レビュー追跡

- [/] PR #36 P2 `PRRT_kwDOSYoxuc6efRJp`: 入力保護と回帰テストを実装し、4a0367f の `just check` 成功後に reply/resolve 済み。
- [/] PR #36 P1 `PRRT_kwDOSYoxuc6evgRs`: staging の原子的確保と回帰テストを実装し、f11b7b5 の `just check` 成功後に reply/resolve 済み。
- [/] PR #36 P1 `PRRT_kwDOSYoxuc6evsSM`: 最終出力の非置換公開を実装し、cb89099 の `just check` 3957 tests成功後にreply/resolve済み。
- [/] PR #36 P1 `PRRT_kwDOSYoxuc6ev9Wz`: output/stagingのdirectory handle固定、no-follow/create-new書き込み、OS一時領域へのFFmpeg分離と回帰検証を実装し、6203977でreply/resolve済み。
- [/] 自己レビューP1: `egui_variable_viewport_full_motion_contract`を追加し、全46frameをpublic APIからreceipt化・実FFmpegでGIF/MP4化する検証がローカルで成功（22.44秒）。3OS共通gateに組み込み、最新HEADのCI成功を公開条件とする。
- [/] PR #36 P1 `PRRT_kwDOSYoxuc6ewdTU`: 公開前・最終検証でoutput/stagingのidentity不一致を拒否する実装と回帰検証を追加し、f51ee58でreply/resolve済み。
- [/] PR #36 P2 `PRRT_kwDOSYoxuc6ewdTV`: 非UTF-8出力先を副作用前に拒否する実装と回帰検証を追加し、f51ee58でreply/resolve済み。
- [/] 自己レビューP1: GIF/MP4/manifestの作成済みfile handleを保持し、各entryの最終検証で観測した置換を拒否する実装と回帰検証を追加済み。
- [/] PR #36 P1 `PRRT_kwDOSYoxuc6ewwY6`: 全正規化frameの作成済みHandleも保持・検証する実装と回帰検証を追加し、c3fd253でreply/resolve済み。
- [/] PR #36 P1 `PRRT_kwDOSYoxuc6ew9J9`: 原Issueにないnamespaceの絶対不変保証をレビュー中に追加していた点を訂正。観測時点のidentity検証とcallerの外部変更防止責務をAPI/仕様に明記し、14bee91で根拠をreply/resolve済み。
- [/] PR #36 P2 `PRRT_kwDOSYoxuc6ew9J-` / `PRRT_kwDOSYoxuc6ew9J_`: 非UTF-8の一時領域・検出したFFmpeg実行ファイルをencode/manifest記録前に拒否し、14bee91の回帰検証後にreply/resolve済み。
- [/] PR #36 P1 `PRRT_kwDOSYoxuc6exNsv`: PNG64 MiB/provenance1 MiBのbounded read、成長・容量overflow・読み込み失敗の回帰を実装し、eb2066cでreply/resolve済み。3OS CI成功。
- [ ] CI容量不足: ephemeral GitHub-hosted Linuxだけで検証済みhost buildと成功済みcoverage buildを順に解放し、全テスト・100%coverage・package/dry-runの完全gateを通す。ローカルcache/summary/他worktreeは保護する。
- [/] Ready後P1 `PRRT_kwDOSYoxuc6exjX1`: 同一commit frameの実Node getter値・scalar sequence・有効boundsを検証し、最終TreeUpdateとhash一致を全46frame実FFmpeg試験で確認。82811051でreply/resolve、just check3973 tests成功。
- [/] P2 `PRRT_kwDOSYoxuc6eyMCY`: 上限内でもmetadata初期長とEOF長の一致を要求する提案は、既存仕様が上限超過拒否と読後SHA/provenance検証であるため不採用。独立監査後、根拠をreply/resolve済み。仕様・検証範囲は変更しない。
- [ ] strict coverage: 2030c01のclean CIで残った3行を関数別に特定し、同じunit instanceの非UTF-8出力先拒否、実Fileのmetadata取得後の成長拒否、文字列パスのI/Oエラーを回帰テストで補強した。Linuxの単一object・新規profrawによる限定78testsは成功し、variable_viewport1506/1506、frames231/231、output190/190、functions111/111を確認。最新HEADの完全な100%gate成功を引き続き必須とする。
- [/] ユーザー承認済み。進捗報告だけで終了せず、Draft review → Ready → required checks → merge → GitHub Release/crates.io 公開確認 → cleanup → Issue #34 close まで継続する。
