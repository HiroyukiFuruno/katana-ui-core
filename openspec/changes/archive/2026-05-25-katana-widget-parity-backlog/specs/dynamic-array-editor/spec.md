## ADDED Requirements

### Requirement: DynamicArrayEditor widget

DynamicArrayEditor は画像3枚目のように、配列 item の追加、削除、編集、並び替えを扱えることを MUST とする。

#### Scenario: item を追加する

- **WHEN** 利用者が追加ボタンを押す
- **THEN** 新しい item が配列に追加され、on_change callback が呼ばれる

#### Scenario: item を削除する

- **WHEN** 利用者が削除ボタンを押す
- **THEN** 対象 item が配列から削除され、on_change callback が呼ばれる

#### Scenario: item 表示を上位から渡す

- **WHEN** item renderer が渡される
- **THEN** item は文字列固定ではなく任意 node として表示される
