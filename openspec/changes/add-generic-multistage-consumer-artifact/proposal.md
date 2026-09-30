## Why

公開済み `katana-ui-core` v0.3.17 の consumer artifact plan は固定 10 stage の full-editor sequence を一度だけ要求するため、10 を超える source-derived leaf を持つ consumer が KUC-issued stage、media、receipt を一対一で取得できない。KLE v0.1.0 は registry 公開版のこの generic contract を必要としており、現在 fail-closed である。

## What Changes

- `ConsumerArtifactPlanV2` を追加し、ひとつの generic full-editor root に対して任意個の検証済み opaque stage binding を発行できるようにする。
- stage ごとの KUC-issued ID、media、AccessKit・Unicode evidence、one-shot receipt の独占性を維持し、欠落・重複・stale・再使用を typed error で拒否する。
- 既存 `ConsumerArtifactPlanV1` の固定 full-editor sequence と挙動を変更しない。

## Capabilities

### New Capabilities

- `generic-multistage-consumer-artifact-plan`: generic consumer artifact root から可変長 stage evidence を発行する公開契約。

### Modified Capabilities

- `consumer-defined-full-editor-artifact-plan`: fixed v1 contract と additive な可変長 v2 contract の互換性を定義する。

## Impact

- `crates/katana-ui-core` の `egui` public API、consumer artifact issuer、artifact evidence tests。
- v0.4.0 release、KLE の registry-only downstream acceptance。
