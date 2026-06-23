## ADDED Requirements

### Requirement: AlignCenterWrapper widget

AlignCenterWrapper は katana の AlignCenter のように、子要素を中央揃えで表示できることを MUST とする。

#### Scenario: 子要素を中央揃えする

- **WHEN** child node が渡される
- **THEN** 指定された幅と高さの中で縦横中央に配置する
