## ADDED Requirements

### Requirement: Breadcrumb widget

Breadcrumb は階層 path を crumb 配列として表示し、各 crumb の click を扱えることを MUST とする。

#### Scenario: crumb を選択する

- **WHEN** 利用者が click 可能な crumb を押す
- **THEN** その crumb の on_click callback が呼ばれる

#### Scenario: 長い path を省略する

- **WHEN** crumb が表示幅を超える
- **THEN** 指定された省略方式で path を短く表示する
