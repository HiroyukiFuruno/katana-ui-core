# 三OS native IME evidence の公開

## Why

Issue #81 は実 OS IME による preedit / commit を要求する。v0.4.0 の `KucUnicodeColorGlyphEvidenceCapture::capture` は `egui::Event::Ime` を生成して `RawInput` に渡すため、既存の contract test や公開版だけではこの条件を証明できない。

## What Changes

- KUC 所有の native producer を提供し、OS IME event の入力経路を記録する。
- 同一 run の Unicode、RGBA crop、measurement、caret、hit test、AccessKit、frame / root / receipt と、revision、platform、font catalog / file SHA-256、input method、fresh run identity を結合する。
- registry-only consumer が使用できる verifier を KUC に提供する。
- 三OS native evidence の生成・検証・artifact 公開を release の必要条件にする。
- 既存 V1 / V2 consumer artifact と synthetic contract test は互換性を維持する。

## Scope

KUC 内に限定する。KLE / KatanA の payload、state、geometry を受け取らない。consumer 側の採用を完了条件に加えない。

## Capabilities

### New Capabilities

- `native-ime-evidence`: OS 入力境界を所有する producer と、外部の期待 provenance に照合する registry-only verifier、三OS公開 artifact。

### Modified Capabilities

なし。既存 synthetic Unicode evidence と ConsumerArtifactPlan V1 / V2 の契約を維持する。

## Unresolved Input

三OSの native IME 実行経路を確定する必要がある。repository self-hosted runner は未登録。既存 hosted CI は三OSで Rust test を実行するが、IME の設定・OS keyboard input・native event provenance を扱わない。hosted runner を使用できないとはまだ判断していない。
