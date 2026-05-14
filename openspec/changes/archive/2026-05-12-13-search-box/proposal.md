## Why

検索 UI は「leading に search icon」「trailing に clear（値があるとき）」「Esc で clear」「on_submit で確定」などの慣習があり、`TextInput` を直接組ませると毎回同じ合成コードが書かれる。`SearchBox` を 1 件提供して定型を吸収する。`../katana/crates/katana-ui/src/widgets/search_bar/` の役割を Floem に移植。

## What Changes

- `composite/input/search/` に `SearchBox` widget を提供。
- props: `value: String`、`on_change: Fn(String)`、`on_submit: Option<Fn(String)>`、`placeholder`、`size`、`disabled`、`a11y_label`。
- 内部で `TextInput` を `leading=search icon` / `trailing=ClearButton`（value が非空のとき自動表示）で合成。
- キーバインド: Esc → clear、Enter → on_submit。

## Capabilities

### New Capabilities

- `widget-search-box`: 検索専用入力 widget。慣用の icon / clear / Esc / Enter 動作を吸収。

## Impact

- フィルタバー / コマンドパレット骨格 / リスト絞り込みで利用。
