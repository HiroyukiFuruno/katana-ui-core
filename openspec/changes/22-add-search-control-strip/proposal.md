## Why

`katana` の検索 modal、KLE の find / replace、KDV の viewer 検索、`katana-chat-ui` の履歴検索は、どれも「検索入力 + option toggle + 前後移動 + 件数表示」を必要とする。
画面上では、検索欄の横に match case、whole word、regex、前へ / 次へ、clear、件数が並ぶ。

KUC には `SearchBox` molecule があるが、現状は query input に近く、次を option だけでは補えない。

- match case / whole word / regex の typed option
- previous / next navigation
- result count / active index の表示
- optional replace input / replace actions
- query / option / navigation / replace event
- command launcher result list との組み合わせ境界

このままだと、KLE と `katana` が検索 control row を別々に作り、shortcut、状態表示、入力回帰が分裂する。

## What Changes

- `SearchControlStrip` molecule を追加する。
- `SearchBox` は単純な query input として残す。
- `SearchControlStrip` は次を持つ。
  - `query`
  - `match_case`
  - `whole_word`
  - `use_regex`
  - `result_count`
  - `active_index`
  - `replace_mode`
  - `replace_value`
  - `actions`
- action を定義する。
  - `SetSearchQuery`
  - `ToggleSearchOption`
  - `FindPrevious`
  - `FindNext`
  - `ClearSearch`
  - `SetReplaceValue`
  - `ReplaceCurrent`
  - `ReplaceAll`
- event を定義する。
  - `SearchQueryChanged`
  - `SearchOptionChanged`
  - `SearchNavigationRequested`
  - `SearchCleared`
  - `ReplaceRequested`

## Capabilities

### New Capabilities

- `kuc-search-control-strip`: search control row の option、action、event、state、replace mode、Storybook、DoD を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `SearchBox` は simple input、`SearchControlStrip` は検索操作 row、`CommandPalette` は result launcher として責務を分ける。

## Impact

- `crates/katana-ui-core/src/molecule/structured` または `selection` に `SearchControlStrip` を追加する。
- `SearchBox` の既存 API は破壊しない。
- `21-add-command-launcher-results` と組み合わせることで、検索 modal / slash launcher / history search を domain-free に構築できる。
- KLE の editor search 実行、KDV の viewer search 実行、`katana` の workspace search 実行は consumer が持つ。
