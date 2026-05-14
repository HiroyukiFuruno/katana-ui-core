## ADDED Requirements

### Requirement: Katana-style RGBA ColorPicker

ColorPicker MUST 画像1枚目のような GUI を持ち、色ボタンからポップパネルを開いて色を編集できる。
利用者 MUST red、green、blue、alpha、blending を編集でき、選択色は preview と `on_change` に即時反映される。

#### Scenario: 色ボタンからパネルを開く

- **WHEN** 利用者が色ボタンを押す
- **THEN** ColorPicker の編集パネルが開く
- **AND** パネルには透明チェッカー、preview、`U8` 表示、スポイト、R/G/B/A 値、Blending、色面、色相 slider、alpha slider が表示される

#### Scenario: 色面を操作する

- **WHEN** 利用者が色面を操作する
- **THEN** 選択色が更新される
- **AND** preview と `on_change` に反映される

#### Scenario: RGBA channel を変更する

- **WHEN** 利用者が R/G/B/A のいずれかを変更する
- **THEN** 選択色が更新される
- **AND** alpha は透明チェッカー上の preview に反映される
- **AND** `on_change` が更新後の値で呼ばれる

#### Scenario: blending を切り替える

- **WHEN** 利用者が Normal または Additive を選択する
- **THEN** blending mode が更新される
- **AND** `on_change` に反映される

#### Scenario: readonly または disabled で操作する

- **WHEN** ColorPicker が readonly または disabled
- **THEN** 利用者の操作で選択色は変更されない
- **AND** `on_change` は呼ばれない

#### Scenario: dark mode で表示する

- **WHEN** theme が dark mode
- **THEN** text、border、icon、panel background が theme token に追従する
