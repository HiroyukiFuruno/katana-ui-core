## ADDED Requirements

### Requirement: SelectionList widget

SelectionList は画像2枚目のような section label、色付き marker、選択 highlight、もっと表示を扱えることを MUST とする。

#### Scenario: section ごとに item を表示する

- **WHEN** section と item 配列が渡される
- **THEN** section label の下に item を表示する

#### Scenario: item を選択する

- **WHEN** 利用者が item を押す
- **THEN** selected 表示が更新され、on_select callback が呼ばれる

#### Scenario: もっと表示を押す

- **WHEN** hidden item があり、利用者が「もっと表示」を押す
- **THEN** hidden item が追加表示される
