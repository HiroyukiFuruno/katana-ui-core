## ADDED Requirements

### Requirement: Tabs widget

Tabs は content を内部に持つ用途と、外部 UI を callback で切り替える用途の両方を扱えることを MUST とする。

#### Scenario: content あり tab を表示する

- **WHEN** tab item に content node が渡される
- **THEN** 選択中 tab に対応する content を表示する

#### Scenario: content なし tab を選択する

- **WHEN** tab item に content がない
- **THEN** on_select callback を呼び、外部 UI が連動できる
