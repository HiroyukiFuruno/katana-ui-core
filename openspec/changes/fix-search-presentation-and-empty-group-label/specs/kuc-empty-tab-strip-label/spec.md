## ADDED Requirements

### Requirement: 空のTabStrip labelを代替文字なしで描画する
KUCのTabStripはgroup labelが空文字でもroot frameを成功させ、表示値とAccessKit labelを空文字のまま保持しなければならない（MUST）。空値を空白・仮名・別glyphへ置き換えてはならない（MUST NOT）。下位PlatformTextRasterizerの空text検査を無効化してはならない（MUST NOT）。

#### Scenario: 初期投影の空group labelを描画する
- **WHEN** public full-editor rootが空group labelを含むprojectionを受け取る
- **THEN** root frameは成功し、操作可能なheader boundsは正の幅を持ち、AccessKit labelは空文字になる

#### Scenario: 非空から空を経て日本語/emojiへ戻る
- **WHEN** group nameを日本語、正確なVS16付き`⭐️`、空文字の順に入力し、hostが各変更を次のleaseへ反映する
- **THEN** 各root frameが成功し、空文字時もheader hit targetが保たれ、同一rootで後続の日本語/emoji入力が継続できる

#### Scenario: 下位rasterizerのempty拒否を保つ
- **WHEN** consumerが汎用PlatformTextRasterizerへ直接empty textを要求する
- **THEN** 既存EmptyText validation errorが返り、TabStrip componentだけが空表示を処理する

### Requirement: empty labelのpaintを安全に省略する
KUCは空labelのtexture raster/paint operationを作成せず、背景/marker/route/hit-test/AccessKitの非テキスト契約を維持しなければならない（MUST）。

#### Scenario: 空labelでheader操作を維持する
- **WHEN** group labelが空のTabStripをpointer/keyboard/AccessKit対象として描画する
- **THEN** texture operationなしで背景/markerと正のboundsが維持され、対応header actionが正しいgroupへ一度だけ配送される
