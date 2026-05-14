## ADDED Requirements

### Requirement: LoadingDots widget

LoadingDots は dot の点滅アニメーションで非同期処理中を表示できることを MUST とする。

#### Scenario: dots を表示する

- **WHEN** dot_count が指定される
- **THEN** 指定数の dot を横並びで表示する

#### Scenario: active animation を表示する

- **WHEN** active が true
- **THEN** dot が animation_speed_ms に従って点滅または拡縮する

#### Scenario: label 付きで表示する

- **WHEN** label が渡される
- **THEN** label と dots を同じ行に表示する
