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
- [x] 5.2 focused test、`just check`、`just VERSION=v0.3.4 release-check`、`git diff --check` を最新 HEAD で成功させる
- [x] 5.3 Issue #34 と OpenSpec の追跡可能性、changelog、version を v0.3.4 release 成果物へ反映する

## 6. User Review Phase

- [x] 6.1 PR工程を `Draft作成 → review → 指摘対応・reply/resolve → Ready化 → merge` の順序へ固定し、Draft省略を禁止する
- [x] 6.2 `release/v0.3.4` をpushしてDraft PRを作成し、`@codex review` と自己レビューを実行する
- [ ] 6.3 review指摘を優先度分類し、P0/P1を必ず修正して各threadへreply/resolve後、最新HEADで再検証してReady化する
- [ ] 6.4 required checks完了後にmergeし、Release workflow、tag、GitHub Release、crates.io `katana-ui-core@0.3.4` を個別に確認する
- [ ] 6.5 `branch-hygiene` に従ってbranch/worktreeを整理し、Issue #34へ完了証跡を反映してcloseする
