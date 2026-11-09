## Why

#124では検索欄のヒント、include/excludeの検証状態、履歴見出しを、検索やregexの実装をhostに残したままKUC公開APIで表せません。#126ではgroup名を空文字へ確定して再投影すると、KUCのTabStrip描画が空テキストを汎用rasterizerへ渡してroot全体を失敗させます。

## What Changes

- mode target別の検索presentationとしてquery/include/exclude hint、include/exclude validity、任意の履歴見出しを公開する。
- hintを操作名/AccessKit名から分離し、履歴見出しを非操作labelとして描画する。
- hostが指定する検証状態をKUCの既存themeで描画する。検証ロジック・検索処理は引き続きhost所有とする。
- TabStripの空表示文字列をKUC component rasterizerで空のrasterとして扱い、テキスト描画を省略して他の描画・操作領域・AccessKit labelを保つ。下位PlatformTextRasterizerの空入力拒否は維持する。
- 公開APIと実root入力をつないだ統合回帰、利用例、依存監査、既存品質gateを追加・更新する。

## Capabilities

### New Capabilities
- `kuc-search-mode-presentation`: 検索mode別のhint、filter validity、履歴見出しの公開契約。
- `kuc-empty-tab-strip-label`: TabStrip componentにおける空表示文字列の描画・操作契約。

### Modified Capabilities

なし。既存specに該当する要件はなく、新規契約として定義する。

## Impact

`SelectionDialogProjectionLease`と関連presentation型、selection dialog retained input/paint、`TabStripTextRasterizer`およびTabStrip retained paint、公開root integration tests、利用者向けAPI docs。外部公開struct literalとの互換性を保ち、existing SearchResultModeにフィールドを追加しない。
