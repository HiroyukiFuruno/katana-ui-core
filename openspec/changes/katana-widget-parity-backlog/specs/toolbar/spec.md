## ADDED Requirements

### Requirement: Toolbar widget

Toolbar は leading slot と trailing slot を左右に分け、任意 node を配置できることを MUST とする。

#### Scenario: leading と trailing を表示する

- **WHEN** leading slot と trailing slot が渡される
- **THEN** leading を左側、trailing を右側に表示する

#### Scenario: alignment を変更する

- **WHEN** alignment が Center / Top / Bottom のいずれか
- **THEN** 子要素の垂直位置が指定に従う

#### Scenario: gap を変更する

- **WHEN** gap が指定される
- **THEN** 子要素間の spacing が指定値に従う
