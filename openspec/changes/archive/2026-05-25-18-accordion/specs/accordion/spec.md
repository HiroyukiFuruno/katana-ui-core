## ADDED Requirements

### Requirement: Accordion widget

Accordion は header と body を持ち、展開、折り畳み、disabled、controlled / uncontrolled、trigger area、tree mode、reduced motion を扱えることを MUST とする。

#### Scenario: header を押して開閉する

- **WHEN** 利用者が有効な header を押す
- **THEN** body の表示状態が切り替わる
- **AND** callback log が更新される

#### Scenario: disabled では開閉しない

- **WHEN** Accordion が disabled
- **THEN** header を押しても expanded state は変わらない

#### Scenario: trigger area を指定する

- **WHEN** trigger area が icon only / label only / icon + label / full row のいずれか
- **THEN** 指定された領域だけが開閉操作を発火する

#### Scenario: group の同時展開を制御する

- **WHEN** group が single mode
- **THEN** 新しい item を開くと既存の開いている item は閉じる
- **AND** group が multiple mode の場合は複数 item を同時に開ける

#### Scenario: tree mode を表示する

- **WHEN** depth、selected、show_lines が渡される
- **THEN** Accordion は tree 表現として indent、選択背景、階層線を表示する

#### Scenario: reduced motion を尊重する

- **WHEN** reduced motion が有効
- **THEN** 展開アニメーションを抑制し、状態だけを切り替える
