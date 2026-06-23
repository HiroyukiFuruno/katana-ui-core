## ADDED Requirements

### Requirement: MenuButton widget

MenuButton は trigger を押すと任意 content を Popover として開けることを MUST とする。

#### Scenario: menu を開く

- **WHEN** 利用者が trigger を押す
- **THEN** menu content が表示される
- **AND** `on_open` callback が呼ばれる

#### Scenario: menu を閉じる

- **WHEN** 利用者が外側 click、Esc、または menu item 選択で閉じる
- **THEN** menu content が非表示になる
- **AND** `on_close` callback が呼ばれる

#### Scenario: unframed variant を表示する

- **WHEN** variant が `Unframed`
- **THEN** ボタン枠なしの text / icon trigger として表示される
