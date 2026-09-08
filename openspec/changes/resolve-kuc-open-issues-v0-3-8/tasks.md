## 1. 統合基準と公開 raster host

- [ ] 1.1 v0.3.7 の KDV registry-only 91/95 を同一 crop convention で KUC-owned numerical contract として再現し、差分を raster host の render / wrap / layout / hit-test metrics に限定する。
- [ ] 1.2 document role typography と physical / logical metrics を一度だけ解決して全 raster host 経路へ渡し、KDV/KatanA固有補正なしで修正する。
- [ ] 1.3 四辺別 grid border API、legacy uniform border、clip、merged anchor の互換性を KUC regression で維持する。
- [ ] 1.4 `raster-host` feature の GUI-runtime非依存、private Storybook parity、document raster/hit metrics を統合 regression で検証する。

## 2. HiDPI text raster contract（KUC #43）

- [x] 2.1 physical raster extent を logical UI extent に変換する共通 helper を導入し、TextSurface gutter / placeholder と Diagnostics の予約・paint・clip・hit boundsへ適用する。
- [x] 2.2 Diagnostics の `max_width_px` 二重 scale を除去し、text raster request の logical-width contract を固定する。
- [x] 2.3 StatusBar と CommandChrome の physical raster consumer を監査し、共通 helper適用または等価性 regression を追加する。
- [x] 2.4 scale 1 / 1.5 / 2、Japanese、`⭐️` を使い、gutter、Diagnostics reservation、wrap、clip、hit bounds の数値 regression を追加する。

## 3. synchronized scenario session（KUC #44）

- [x] 3.1 `ScenarioSessionState` と update projection に search visibility、options、replace mode、result position を追加する。
- [x] 3.2 close を `search: None` として次 lease へ投影し、KUC-owned explicit reopen APIだけが保存済み検索状態を可視化するよう実装する。
- [x] 3.3 retain一回・毎frame synchronizeの trace で query、option、navigation、close、closed verify、reopen、receipt、locator、AccessKit、frame output を連続 regression にする。
- [x] 3.4 navigation / replace の外部効果を session が推測して保存しない regression を追加する。

## 4. consumer-defined full-editor artifact plan（KUC #40）

- [x] 4.1 versioned opaque plan、KUC-defined generic class、opaque token、stage binding、typed validation error の公開型を追加し、domain payload / RawInput / renderer callback を排除する。
- [x] 4.2 retained rootを共有する KUC-owned issuer / ordered one-shot stage lifecycle を実装し、重複、欠落、順序違反、stale revision、token-root mismatch を fail-closed にする。
- [x] 4.3 KUC writer が stage ごとの numbered PNG、decode/pixel/root-record hash、AccessKit、Unicode evidence、one-shot forwarding receipt を manifest へ結合し、media overwrite / receipt reuse / cross-bind を拒否する。
- [x] 4.4 foreign-consumer compile、fixed scenario compatibility、adversarial validation、IME/Japanese/`⭐️`/selection/hit-test/accessibility/resize/search/context/floating の full-root evidence regression を追加する。

## 5. 統合検証と公開受入

- [x] 5.1 直接・推移依存と `Cargo.lock` を監査し、不要な downgrade や path / git release dependency を残さない。
- [x] 5.2 v0.3.8 の version、release note、OpenSpec task evidence を更新する。
- [x] 5.3 統合 HEAD で format、AST lint、type check、lint、workspace test、consumer contract、raster host contract、Storybook regression、strict coverage、`just VERSION=v0.3.8 release-check`、`git diff --check` を実行する。
- [ ] 5.4 Draft PRを作成し、review、指摘対応・reply/resolve、P0/P1=0、Ready化、required 3 OS CI を完了する。
- [ ] 5.5 merge後に tag、GitHub Release、crates.io `katana-ui-core@0.3.8` を個別に確認する。
- [ ] 5.6 KDV Issue #48 の既存 consumer で exact registry v0.3.8 を採用し、boundary check、per-side border、canonical crop 95/95 を確認する。KLE consumerで #40 の3 OS artifact evidence を確認する。
- [ ] 5.7 #35、#37、#40、#43、#44 の Issue に公開・consumer evidence を記録して close し、`branch-hygiene` に従って不要な release branch / worktree を整理する。
