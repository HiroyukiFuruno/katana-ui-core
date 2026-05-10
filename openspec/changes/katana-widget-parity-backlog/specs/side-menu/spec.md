## ADDED Requirements

### Requirement: SideMenu widget

SideMenu は左右配置、幅制御、SVG icon action、icon からの pop 表示を扱えること。

#### Scenario: SVG icon action を実行する

- **WHEN** 利用者が menu icon を押す
- **THEN** icon ごとの callback が呼ばれる

#### Scenario: hover で展開する

- **WHEN** hover expand が有効で、利用者が menu に hover する
- **THEN** 幅 0 または折り畳み状態から指定幅へ展開する

#### Scenario: icon から pop を開く

- **WHEN** icon に pop content が設定されている
- **THEN** 指定方式で pop content を表示する
