## ADDED Requirements

### Requirement: ProgressBar widget

ProgressBar は確定進捗と未確定進捗を表示できることを MUST とする。

#### Scenario: 確定進捗を表示する

- **WHEN** value と max が渡される
- **THEN** 現在値に応じた bar 幅を表示する
- **AND** label / percent 表示を任意で表示できる

#### Scenario: 未確定進捗を表示する

- **WHEN** indeterminate が true
- **THEN** 固定値ではなく進行中であることを示す表示になる
