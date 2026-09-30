## 1. Implementation

- [x] 1.1 `ConsumerArtifactPlanV2` と schema v2 issuer path を追加する。
- [x] 1.2 v1 固定 sequence validation を維持し、v2 に generic validation を適用する。

## 2. Verification

- [x] 2.1 11 stage の v2 issuance と unique evidence/receipt を回帰検証する。
- [x] 2.2 v1 compatibility と v2 invalid binding rejection を検証する。
- [ ] 2.3 format、focused tests、release gate を実行する。

## 3. Release

- [ ] 3.1 v0.4.0 Draft PR、review、thread resolution、Ready、required CI、merge を行う。
- [ ] 3.2 GitHub Release/crates.io publish を確認する。

## User Review Phase

- [/] KUC の完了・release 条件に KLE を含む外部 repository の受入を混在させない。KUC 自身の要件、public API、contract、検証だけで判定する。
