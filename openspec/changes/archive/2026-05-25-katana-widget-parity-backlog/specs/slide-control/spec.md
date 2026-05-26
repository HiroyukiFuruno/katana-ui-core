## ADDED Requirements

### Requirement: SlideControl widget

SlideControl は整数 / 小数、最小値、最大値、step、単位、表示 format、適用先連動を扱えることを MUST とする。

#### Scenario: slider 値を変更する

- **WHEN** 利用者が slider を動かす
- **THEN** 値が step に従って更新される
- **AND** on_change callback が呼ばれる

#### Scenario: 数値入力と同期する

- **WHEN** slider と数値入力が同じ値を共有する
- **THEN** 片方の変更がもう片方に反映される
