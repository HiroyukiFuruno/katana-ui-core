## ADDED Requirements

### Requirement: ComboBox widget

ComboBox は TextInput と Popover を組み合わせ、入力による選択肢フィルタリングと選択を扱えることを MUST とする。

#### Scenario: 入力で選択肢を絞り込む

- **WHEN** 利用者が input に文字を入力する
- **THEN** 選択肢 list が入力値で絞り込まれる
- **AND** `on_input_change` callback が呼ばれる

#### Scenario: 選択肢を選択する

- **WHEN** 利用者が選択肢 item を押す
- **THEN** selected value が更新される
- **AND** `on_select` callback が呼ばれる

#### Scenario: strict mode で自由入力を拒否する

- **WHEN** `strict_mode` が true
- **THEN** item に存在しない値は確定値として扱わない
