## ADDED Requirements

### Requirement: StatusBar widget

StatusBar は severity icon、message、任意 trailing content、action callback を持つ水平バーとして表示できることを MUST とする。

#### Scenario: severity を表示する

- **WHEN** severity が Error / Warning / Success / Info のいずれか
- **THEN** severity に対応する icon と色を theme token から解決して表示する

#### Scenario: trailing content を表示する

- **WHEN** trailing slot が渡される
- **THEN** message の右側に任意 node を表示する

#### Scenario: action を実行する

- **WHEN** 利用者が action 領域を押す
- **THEN** `on_action` callback が呼ばれる
