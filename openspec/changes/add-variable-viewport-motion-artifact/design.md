## Context

`MotionArtifactWriter::write_opaque` は sequence の先頭 receipt 寸法を固定 canvas として採用し、後続 frame の寸法が異なる場合は `WrongDimensions` で fail closed する。この契約は固定 viewport の artifact 生成に必要であり変更しない。一方、`FullTextCommandSurfaceMotionPlan` の resize stage は KUC-owned `RawInput::screen_rect` を変更するため、consumer が input を書き換えたり opaque receipt 内部の raster を読み取ったりせず、一つの動画へ正規化する別契約が必要である。

KUC には既に root record hash、PNG provenance、source/decode frame hash と、root frame から `⭐️`、IME、hit test、AccessKit を観測できる内部契約がある。新 API は各観測を同じ frame の receipt に保存し、可変 viewport sequence と semantic evidence を一つの新しい manifest schema へ暗号学的に結合する。

## Goals / Non-Goals

**Goals:**

- 可変 viewport の opaque receipt sequence を単一の GIF/MP4 へ決定論的に正規化する。
- source frame を scale/crop せず、KUC が固定 export canvas と配置を決める。
- source viewport、source PNG hash、正規化後 source/decode hash、root record hash、Unicode/IME/hit test/AccessKit evidence の結合を manifest で検証可能にする。
- 既存の固定寸法 API と manifest schema を source/behavior compatible に維持する。

**Non-Goals:**

- consumer 固有の timeline 編集、transition、字幕、音声を追加しない。
- KLE 固有の型や manifest を KUC に導入しない。
- source frame の拡大縮小、crop、再レイアウト、KUC-issued input の書き換えを行わない。
- 既存 `write_opaque` の異寸法拒否を緩和しない。

## Decisions

### 1. additive な専用 API と manifest 型を追加する

`MotionArtifactWriter::write_opaque_variable_viewport` を追加し、専用の `VariableViewportMotionArtifact` / `VariableViewportMotionArtifactManifest` を返す。既存の public `MotionArtifactManifest` に field を追加すると外部の struct literal を壊し得るため採用しない。既存 API の挙動変更案も fixed-dimension 契約を曖昧にするため採用しない。

### 2. export canvas は source viewport の最大幅・最大高とする

全 receipt を検証した後、`max(width)` × `max(height)` を固定 export canvas とする。各 source frame は左上 `(0, 0)` に pixel 等倍で配置し、source 外側だけを不透明黒で pad する。左上固定は egui の座標原点と hit test 座標を維持し、中央寄せによる座標変換を避ける。scale/crop は provenance と hit test の対応を壊すため行わない。

### 3. 正規化 frame は専用 staging directory に生成する

元 receipt の PNG と provenance は変更せず、OS の一時領域に所有者限定の scratch directory を作り、連番 PNG、GIF、MP4 を生成する。FFmpeg へ渡す pathname は caller の output 配下に置かない。GIF と MP4 は同じ正規化 PNG 列から生成し、source/decode framemd5 の一致を必須とする。元 PNG の SHA-256 と source viewport 列も別に記録し、正規化前後の境界を明示する。

書き込み開始前に staging directory、GIF、MP4、manifest の全出力先が未使用であることを検証する。既存の directory/file、hardlink、dangling を含む symlink は `OccupiedOutputTarget` で拒否する。既存ファイルの再利用や上書きはせず、再出力時は新しい出力先を指定する。これにより、入力 receipt と出力先の重複や別名経由の参照でも元 PNG/provenance を破壊しない。

出力rootを一度開いたディレクトリハンドルで固定し、相対 `create_dir` で public staging を原子的に確保して `open_dir_nofollow` で固定する。同時 export や検査後に追加された symlink が先に同名を占有した場合は拒否する。確保後に出力rootやstagingの名前がrename/symlinkで差し替えられても、その後の公開書き込みは固定済みハンドルに対してだけ行う。確保した directory は出力に残すため、同じ出力先を使用できる writer は一つだけとなる。

正規化PNGと完成したGIF/MP4は、固定済みディレクトリ内の `create_new` と no-follow で取得したファイルハンドルへcopyする。既存のfile/hardlink/symlinkは上書きせず、`OccupiedOutputTarget` として拒否する。manifestも同じ方法でGIF/MP4の公開後、最後に書く。途中失敗時は作成済みの新規成果物を保持し、再試行には新しい出力先を使用する。scratchはRAIIで整理する。OS一時領域はcaller出力先とは別の信頼境界とし、Unixではscratchを0700とする。

公開前と最終検証では、固定したoutput/stagingハンドルとpathnameのdirectory identityを比較し、rename/replacementによる不一致を観測した場合にエラーにする。manifestのpath文字列をlossy変換しないため、非UTF-8出力pathは出力directory作成前に `InvalidSettings` で拒否する。OS一時領域はscratch作成前、検出したFFmpeg実行ファイルはsource evidence/encode呼び出し前にUTF-8表現性を検証する。

公開する全正規化PNGと返却するGIF/MP4/manifestについても作成したハンドルを保持し、各entryの最終検証でpathnameが同じfile identityを指すことを必須とする。親directoryを変えないentry置換も、検証時に観測した不一致として拒否する。OSのhandle上限へ達した場合はI/O errorを伝播し、frameを省略しない。

identity比較は個別の観測でありnamespaceをロックしない。別actorが書き込み可能なpathを返すAPIでは、最後の観測後から関数返却までのrenameも排除できない。atomic renameや再検査でも同じ境界が残る。書き込み権限を持つ別actorによる同一inode内の改変も対象外であり、安定した成果物pathが必要な間の外部変更防止はcallerの責務とする。Issue #34の元要件である全frame・provenance・decode一致を維持し、レビュー中に追加した絶対的な「返却まで不変」という過剰な保証だけを訂正する。入力へのwriter自身の上書き防止と既存の回帰検証は維持する。

このOS共通のハンドル相対I/Oに `cap-std` / `cap-fs-ext`、identity比較に `same-file`、subprocess用scratchに `tempfile` を `storybook-artifacts` feature限定で使用する。通常のcore依存と既存固定寸法writerの契約は変更しない。

### 4. semantic evidence は opaque summary として結合する

`OpaqueMotionReceiptWriter` は、出力対象と同じ root frame から正確な `⭐️` scalar sequence、IME preedit/commit、hit test、AccessKit snapshot を KUC 内部で検証・抽出し、非公開の frame semantic observation として receipt に結合する。新 API は複数 frame の観測を集約し、各 contributor root record hash が sequence 内 provenance と一致しない場合 fail closed する。raster bytes、font bytes、paint plan、child geometry は公開しない。

AccessKitはhashの非空判定だけでは検証済みとしない。KUCの有効なnode builder内でsetter後の実node getterからrole/value/scalar sequence/boundsを観測し、rootごとにresetするprivate ledgerから同じframeのreceiptへ保持する。commit frameの期待値と一致するtext-input nodeを選び、値・scalar sequence・finiteで正のboundsを検証する。manifestの`accesskit_snapshot_hash`は選択した1 nodeのcanonical projectionをhash化し、同じroot recordとsemantic artifact hashへ結合する。既存root ledger hashのschemaと公開APIの呼出順序は変えない。全46frameの公開APIテストは、最終FullOutputのTreeUpdateから同じprojection hashを独立再計算して一致を検証する。

この非公開 observation は既存 receipt の補助cacheであり、公開 `PartialEq` 契約へ含めない。`OpaqueRootArtifactReceipt` と sequence の等価性は従来どおり公開artifactだけで決定する。

### 5. 新 schema で canonical hash を分離する

新 manifest は `kuc.variable-viewport-motion.v1` とし、既存 schema から独立させる。canonical hash は自身の hash field を空にした JSON から計算し、公開ファイルに格納する。

### 6. 新 writer の error は専用型として追加する

既存の公開 `MotionArtifactError` は exhaustive enum であり、patch release で variant を追加すると downstream の網羅 match を壊す。新 API は `VariableViewportMotionArtifactError` を返し、既存 error は `Motion` variantで包み、semantic evidence と出力先占有の失敗を専用variantにする。既存 enum の variant集合は変更しない。

## Risks / Trade-offs

- [最大 canvas が極端に大きいと memory 使用量が増える] → non-zero dimension、算術 overflow、source bytes と正規化 working set の 1 GiB 上限を decode 前に検証し、`InvalidSettings` で fail closed する。
- [receiptのファイルが巨大な入力に置換される] → decoded working setとは別に、encode済みPNGは1枚64 MiB、provenance JSONは1件1 MiBを上限とする。metadataだけでなく上限+1byteのbounded readで読込中の成長も検出し、上限超過・確保失敗を`InvalidSettings`で拒否する。PNGは1枚ずつ処理して保持を積み重ねず、既存固定寸法APIの読込契約は変更しない。
- [黒 padding が source 外領域の見た目へ現れる] → padding policy を manifest schema とテストへ固定し、source pixels 自体は変更しない。
- [semantic evidence と receipt が不一致になる] →各観測を同じ frame の receipt に結合し、全 contributor root record hash が sequence の root record hashes に含まれることを必須化する。
- [platform ごとの font/emoji 差] →既存 platform profile evidence gate を維持し、可変 viewport manifest の結合契約を OS 共通テストで検証する。

## Migration Plan

1. v0.3.4 で新 API と型を additive に公開する。
2. 既存 consumer は変更不要で、固定 viewport では従来の `write_opaque` を継続利用する。
3. KLE は registry 公開後に可変 viewport full plan だけ新 API へ移行する。
4. 問題があれば consumer は従来の segment 出力へ戻せる。既存 API/schema の rollback は不要である。

## Open Questions

なし。
