## Why

Issues #114/#115/#116/#117/#120/#121/#122 は、KUCの公開full-rootにおける汎用の入力policy、group presentation、drop geometry、proposal provenanceを要求する。v0.5.1の公開契約だけでは全てをconsumer側設定と自動回帰で表現できるか不明なため、現APIの境界を保ったまま追加契約の必要性を検証し、一つのrelease lineで解決する。

## What Changes

- group rename確定タイミングとinline rename開始成立を公開typed contractで選択・観測する。
- group表示RGBAをpopup swatch候補とは独立に投影する。
- 既存drop方針を既定維持し、最近接挿入境界とpointer-to-ghost anchorを選択可能にする。
- tab-strip proposalへ明示的な操作起点を保持し、起点なし既存constructorは推測しない。
- SearchResultModeごとにquery矢印キーの結果行移動/履歴移動policyを選び、履歴移動由来のtyped eventを通知する。
- 実full-root外部consumer regressionで入力・host効果・描画数値・stale/duplicate/IME契約を結ぶ。

- host定義global actionをretained rootのmenu/shortcutと認証済みone-shot配送へ接続する。
- 元のUTF-8一致範囲を維持し、結合文字・VS16・ZWJを分割しないsnippet表示範囲を投影する。

## Non-goals

検索algorithm、filesystem、文書store、アプリ固有操作順位、consumer独自geometry/入力widget、KUC外repository実装。

## Capabilities

### New Capabilities
- `kuc-tab-strip-policies`: group編集・表示色・drop・操作起点の追加契約。
- `kuc-search-history-policy`: query履歴のキー操作とretained lifecycle。

- `kuc-global-actions`: 文書状態から独立したmenu/shortcutとroot認証配送。
- `kuc-search-highlight-projection`: UTF-8一致範囲とgrapheme表示範囲の分離。

### Modified Capabilities
なし。

## Compatibility

既存constructor、既定のEnter rename、既存drop policy、既存結果行navigationを維持し、追加policyは明示opt-inとする。