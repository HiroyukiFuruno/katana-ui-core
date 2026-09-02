## ADDED Requirements

### Requirement: 可変 viewport の opaque sequence を単一 artifact へ出力できる
KUC は、異なる viewport 寸法を含む `OpaqueMotionReceiptSequence` を consumer に raster や paint plan を公開せず、単一の GIF と MP4 へ出力する additive な公開 API を提供しなければならない（SHALL）。

#### Scenario: resize を含む sequence の出力
- **WHEN** KUC-issued full-motion plan から生成された receipt sequence が複数の viewport 寸法を含む
- **THEN** writer は全 frame を一つの固定 export canvas へ正規化し、source frame count と同数の frame を持つ GIF/MP4 を生成する

#### Scenario: 既存固定寸法契約の維持
- **WHEN** consumer が既存 `write_opaque` に異寸法 sequence を渡す
- **THEN** writer は従来どおり `WrongDimensions` で拒否し、新 API の追加によって既存挙動を変更しない

### Requirement: 正規化は source 座標と pixels を維持する
KUC は export canvas を全 source viewport の最大幅・最大高として選択し、各 frame を左上原点へ等倍配置し、scale と crop を行ってはならない（MUST NOT）。

#### Scenario: 小さい frame の正規化
- **WHEN** source viewport が export canvas より小さい
- **THEN** source pixels は同じ座標へ保持され、source bounds 外だけが決定論的な不透明黒で pad される

#### Scenario: 最大 frame の正規化
- **WHEN** source viewport が export canvas と同じ寸法である
- **THEN** writer は source pixels を移動、拡大縮小、crop せず正規化 frame へ配置する

### Requirement: manifest は正規化前後と root provenance を結合する
KUC は新しい versioned manifest schema に、source frame count、decoded frame count、固定 export 寸法、各 source viewport 寸法、各 source PNG SHA-256、正規化 source/decode frame hash、各 root record hash、artifact path/hash、canonical SHA-256 を記録しなければならない（SHALL）。

#### Scenario: source と decode の一致
- **WHEN** MP4 encode/decode が完了する
- **THEN** decoded frame count と寸法は source count と export canvas に一致し、正規化 source frame hashes と decoded frame hashes は完全一致する

#### Scenario: source provenance の維持
- **WHEN** 異寸法 frame が正規化される
- **THEN** manifest は元の viewport 寸法、元 PNG SHA-256、root record hash を同じ frame 順序で保持する

#### Scenario: 巨大または読み込み中に成長するreceiptファイル
- **WHEN** encode済みPNGが64 MiBまたはprovenance JSONが1 MiBを超える
- **THEN** writerはmetadataとbounded readで上限を検証し、全量の無制限確保を行わず`InvalidSettings`で拒否する
- **AND** decoded working setの1 GiB上限、source hash、provenance、全frameの検証は維持する

#### Scenario: 入力 artifact と既存出力先の保護
- **WHEN** staging directory、GIF、MP4、manifest の出力先が既存の file/directory または hardlink/symlink として存在する
- **THEN** writer は書き込み開始前に `OccupiedOutputTarget` で拒否し、元 receipt PNG と provenance の bytes・hash・寸法を変更しない

#### Scenario: 同時出力の排他
- **WHEN** 複数の writer が同じ未使用出力先の事前検査を同時に通過する
- **THEN** staging directory を原子的に確保した一つの writer だけが書き込みへ進み、他は `OccupiedOutputTarget` で拒否される

#### Scenario: 検査後に追加された最終出力先の保護
- **WHEN** 事前検査後、GIF/MP4/manifest の最終出力先へ別の file または symlink が追加される
- **THEN** staging からの公開は既存ターゲットを置換せず拒否し、そのリンク先や入力PNG/provenanceを変更しない

#### Scenario: 出力directoryの差し替えからの保護
- **WHEN** 出力rootまたはstagingを確保した後、そのpathnameがrenameやsymlinkで差し替えられる
- **THEN** 公開書き込みは固定済みdirectory handleに対してno-follow/create-newで行い、差し替え先の入力PNG/provenanceを上書きしない
- **AND** FFmpegのpathname出力はcallerの出力treeから分離した所有者限定scratchに閉じ込める

#### Scenario: 返却pathと成果物の整合性
- **WHEN** 公開前または最終検証でoutput/stagingのpathnameと固定済みdirectoryの不一致を観測する
- **THEN** writerは成功を返さず、観測した不一致をエラーとして扱う

#### Scenario: 親directoryを変更しない成果物entry置換
- **WHEN** 正規化PNG/GIF/MP4/manifestの各entryの最終検証で、作成時のfile identityとの不一致を観測する
- **THEN** writerは保持したfile handleとのidentity比較で拒否する
- **AND** この検証はnamespaceのロックではなく、最後の観測後から関数返却までを含む外部変更の不可能性を保証しない。安定したpathが必要な間の外部変更防止はcallerの責務とする

#### Scenario: manifestで表現できない出力path
- **WHEN** 出力pathが非UTF-8のfilesystem bytesを含む
- **THEN** writerは出力directoryを作成する前に `InvalidSettings` で拒否し、lossy変換したpathをmanifestへ記録しない

#### Scenario: FFmpeg引数またはmanifestで表現できないhost path
- **WHEN** OS一時領域または検出したFFmpeg実行ファイルのpathが非UTF-8である
- **THEN** writerは一時領域をFFmpeg引数へ渡す前、検出した実行ファイルをencodeやmanifest記録に使用する前に `InvalidSettings` で拒否し、空文字引数やlossyな実行ファイルpathをmanifestへ記録しない

### Requirement: Unicode、IME、hit test、AccessKit evidence を opaque に結合する
KUC は、各出力 frame と同じ root から観測した正確な `⭐️`（U+2B50 U+FE0F）、IME preedit/commit、hit test、AccessKit snapshot を receipt に結合し、複数 frame の検証済み観測から opaque summary と hash を生成して variable viewport artifact manifest に結合しなければならない（SHALL）。

#### Scenario: full evidence の結合
- **WHEN** 検証済み semantic observation の全 contributor root record hash が sequence 内の frame provenance と一致する
- **THEN** manifest は semantic evidence SHA-256、代表 root record hash、全 contributor root record hashes、`⭐️` scalar sequence、IME 観測結果、hit test 件数、AccessKit snapshot hash を記録する

#### Scenario: commit frame の AccessKit 内容の検証
- **WHEN** commit frameの実AccessKit text-input projectionが空、committed valueを欠く、scalar sequenceが値と一致しない、またはboundsが非有限・非正である
- **THEN** writerはhashの存在だけで成功扱いせず`InvalidSemanticEvidence`で拒否する
- **AND** 正常時は同じframeの実nodeから観測したrole/value/scalars/boundsをcanonical hashへ結合し、全46frameの公開APIテストで最終TreeUpdateと照合する

#### Scenario: unrelated evidence の拒否
- **WHEN** semantic observation の contributor root record hash が対応 receipt または receipt sequence の root record hash と一致しない
- **THEN** writer は artifact を成功扱いせず additive な `VariableViewportMotionArtifactError` で fail closed し、既存 `MotionArtifactError` の variant集合を変更しない

### Requirement: platform profile release gate で回帰を検出する
KUC は macOS、Windows、Linux の release profile で full-motion plan の resize、IME、日本語、正確な `⭐️` と manifest 契約を自動検証しなければならない（SHALL）。

#### Scenario: release profile verification
- **WHEN** v0.3.4 release gate が各 supported OS profile で実行される
- **THEN** variable viewport artifact の contract test と既存 Unicode/color glyph/IME/hit test/AccessKit evidence test が成功する
