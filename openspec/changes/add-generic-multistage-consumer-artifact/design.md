## Context

Issue #79 は v1 の固定 `FULL_EDITOR_SEQUENCE` が KLE の source leaf 数を表現できないことを示している。KUC は KLE の source schema や renderer を受け取らず、opaque leaf と KUC-defined interaction/effect のみを扱う必要がある。

## Goals / Non-Goals

**Goals:** 一つの KUC root 上で任意個の unique opaque leaf を順序どおり発行し、各 stage を unique ID、media、AccessKit、Unicode evidence、one-shot receipt に結合する。v1 callers は不変にする。

**Non-Goals:** KLE/KatanA type、content、path、host payload、RawInput、renderer callback を public API へ追加しない。OS 実機の IME proof は CI と公開後の KLE acceptance で確認する。

## Decisions

`ConsumerArtifactPlanV2` は schema version 2 と `Vec<ConsumerArtifactStageBinding>` を持つ。issuer は v1 の固定 sequence validation を残し、v2 には empty/duplicate/effect/revision/root identity validation を適用する。issued plan は既存の KUC root、sequential execution、receipt registry、manifest writer を再利用する。

## Risks / Trade-offs

closed interaction/effect enum と opaque tokens を維持して host semantics の流入を防ぐ。v1 fixed-sequence rejection test と v2 11-stage acceptance test を並置して互換性を検証する。
