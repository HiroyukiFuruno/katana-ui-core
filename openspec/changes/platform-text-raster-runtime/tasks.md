## 1. Public Runtime Boundary

- [x] 1.1 public `katana-ui-core` crate の `text-raster` optional feature と `katana_ui_core::text_raster` module を追加し、default core に renderer dependency を漏らさない public module boundary を確立する
- [x] 1.2 `PlatformTextRasterizer`、configuration、request、raster/layout、grapheme bounds、hit-test、typed error/report を KatanA/KLE/KDV 型なしで定義する
- [x] 1.3 `UiEmojiTextSegments` / `UiTextSpanStyle::emoji` から text run を構成し、platform proportional/monospace/emoji font resolution を generic configuration に閉じる
- [x] 1.4 既定のsystem generic family解決を変えず、callerが最初に有効化できたproportional/monospace candidate faceを明示選択できる公開raster/Storybook host APIと回帰テストを追加する（#41）

## 2. Raster, Layout, and Cache Contracts

- [x] 2.1 `cosmic-text` shaping から RGBA pixels、color glyph override、size、grapheme byte range/bounds を一度に生成する
- [x] 2.2 `⭐️`、ZWJ、Japanese text の grapheme hit-test/caret query が partial scalar を返さないことを unit tests で検証する
- [x] 2.3 color-capable platform emoji font が利用可能な環境で chromatic output と resolved font family を検証し、利用不可状態を explicit report/error として検証する
- [x] 2.4 identical request/configuration の cache reuse と font-system non-reinitialization を report/statistics で検証し、bounded cache/eviction policy を追加する
- [x] 2.5 variation selector、combining mark、ZWJ を分離しない previous/next grapheme byte-range API を public runtime に追加し、KLE/KDV が独自の scalar editing 境界を持たないようにする
- [x] 2.6 non-finite wrap/scale と過大 pixel buffer を typed error または safe fallback で処理し、runtime が overflow panic を起こさないことを検証する
- [x] 2.7 monospace request の ASCII code glyph と non-ASCII platform fallback を KUC runtime で切り替え、日本語 glyph が blank raster にならないことを検証する
- [ ] 2.8 macOS Apple Color Emoji、Windows Segoe UI Emoji、Linux pinned Noto Color Emoji の各 release profile で、isolated `⭐️`/`☆` pixel proof と retained root evidence を実行する。deterministic catalog policy/loader tests はこの 3OS runtime proof を完了扱いにしない

## 3. KUC Storybook Migration

- [x] 3.1 `katana-ui-core-storybook` の internal font resolver、raster cache、text renderer を public runtime adapter へ置換する
- [x] 3.2 existing text/emoji visual contracts を public runtime output に移し、fallback fixture や handcrafted emoji drawing を acceptance evidence から除外する
- [ ] 3.3 KUC Storybook live text-area が runtime の layout/hit-test を使用することを state/event/action and numeric tests で検証する

## 4. Downstream Adapter Migration

- [ ] 4.1 KDV の direct emoji segmentation、OS font-family selection、rich text raster cache を shared runtime adapter に置換する
- [ ] 4.2 KDV document/export rendering で `⭐️`、ZWJ、Japanese text の color pixel/measurement behavior を runtime migration test で検証する
- [ ] 4.3 KUC-owned shared TextSurface egui adapter の visible emoji glyph、caret/selection/hit-test を runtime layout に接続し、KLE は thin binding として neutral `katana-language-editor` crate に runtime 型を漏らさない
- [ ] 4.4 KLE Storybook motion artifact を shared KUC TextSurface adapter の interactive window と同一 frame record/render path から生成し、KUC runtime identity、color pixels、caret/glyph/hit-test bounds を数値検証する

## 5. Guardrails and Verification

- [ ] 5.1 KUC/KDV/KLE の dependency and AST/source guard で duplicate segmentation、font resolver、raster cache、fallback-only acceptance artifact を検出する
- [ ] 5.2 KLE parity matrix の `kuc-text-input` gap と Storybook release gate を shared runtime evidence が揃うまで開いたままにする
- [ ] 5.3 KUC formatter、clippy、workspace tests、OpenSpec strict validate を通し、KLE/KDV downstream targeted tests を記録する
- [ ] 5.4 KUC/KLE/KDV の差分を責務境界で自己レビューし、platform renderer が Katana 固有 namespace や host action を取り込んでいないことを確認する
