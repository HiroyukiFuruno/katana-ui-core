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

### Requirement: Unicode、IME、hit test、AccessKit evidence を opaque に結合する
KUC は、各出力 frame と同じ root から観測した正確な `⭐️`（U+2B50 U+FE0F）、IME preedit/commit、hit test、AccessKit snapshot を receipt に結合し、複数 frame の検証済み観測から opaque summary と hash を生成して variable viewport artifact manifest に結合しなければならない（SHALL）。

#### Scenario: full evidence の結合
- **WHEN** 検証済み semantic observation の全 contributor root record hash が sequence 内の frame provenance と一致する
- **THEN** manifest は semantic evidence SHA-256、代表 root record hash、全 contributor root record hashes、`⭐️` scalar sequence、IME 観測結果、hit test 件数、AccessKit snapshot hash を記録する

#### Scenario: unrelated evidence の拒否
- **WHEN** semantic observation の contributor root record hash が対応 receipt または receipt sequence の root record hash と一致しない
- **THEN** writer は artifact を成功扱いせず additive な `VariableViewportMotionArtifactError` で fail closed し、既存 `MotionArtifactError` の variant集合を変更しない

### Requirement: platform profile release gate で回帰を検出する
KUC は macOS、Windows、Linux の release profile で full-motion plan の resize、IME、日本語、正確な `⭐️` と manifest 契約を自動検証しなければならない（SHALL）。

#### Scenario: release profile verification
- **WHEN** v0.3.4 release gate が各 supported OS profile で実行される
- **THEN** variable viewport artifact の contract test と既存 Unicode/color glyph/IME/hit test/AccessKit evidence test が成功する
