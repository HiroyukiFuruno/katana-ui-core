## ADDED Requirements

### Requirement: CommandPalette widget

CommandPalette は検索入力、結果 list、キーボード操作、provider 注入を持つ overlay widget として動作することを MUST とする。

#### Scenario: query を入力する

- **WHEN** 利用者が検索 query を入力する
- **THEN** `on_search` または provider callback が呼ばれる
- **AND** 結果 list が更新される

#### Scenario: keyboard で結果を移動する

- **WHEN** 利用者が ArrowUp または ArrowDown を押す
- **THEN** active result が移動する

#### Scenario: active result を実行する

- **WHEN** 利用者が Enter を押す
- **THEN** active result の payload を `on_select` callback に渡す

#### Scenario: palette を閉じる

- **WHEN** 利用者が Escape を押す
- **THEN** overlay が閉じる
- **AND** `on_dismiss` callback が呼ばれる
