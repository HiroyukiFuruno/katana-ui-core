# Storybook catalog contract

作成日: 2026-05-18
対象: `kuc-storybook-catalog`

## 結論

Storybook は KUC が提供する UI 群を実画面で操作して確認する部品カタログである。
部品の正しさは自動テストと品質ゲートで判定し、Storybook は操作確認と目視確認の場に限定する。

## 画面構成

画面上では次の構成にする。

| 領域 | 表示するもの | 使用する KUC UI |
| --- | --- | --- |
| 左ペイン | 部品一覧。atomic design とカテゴリでネストする。 | `TreeView` |
| 中央上 | preset 切替。部品ごとに複数状態を切り替える。 | `Tabs` |
| 中央 | 選択中 preset の実 preview。 | component preview |
| 右または下 | option 設定、state、event 履歴、action 履歴。 | settings inspector |
| 全体 | theme control、font role control、catalog status。 | Storybook internal shell |

## 6.1 TreeView navigation

左ペインは KUC `TreeView` で構築する。
分類は次を必須とする。

```text
Atoms
  Text
  Icon
  Buttons
  Input
  Selection
  Feedback
  Data display
Molecules
  Selection controls
  Form and editing
  Overlay and transient
  Surface and navigation
  Structured navigation
  Color and code
Storybook internal
  Catalog shell
  Navigation tree
  Preview workspace
  Settings inspector
```

TreeView の selection event は、preview page と settings panel を同時に切り替える。
TreeView 自体も Storybook の対象 component として、自分自身の page を持つ。

## 6.2 Component page

各 component page は次を持つ。

| 項目 | 契約 |
| --- | --- |
| preview | 現在の option と preset を反映した実 UI。 |
| settings | option 値を画面上で変更する controls。 |
| state summary | `UiStateId`、主要 state、child state id。 |
| event history | 発火した event、target、payload summary。 |
| action history | 実行した action、before / after state summary。 |
| requirement status | option / action / event / state / preset / test / visual regression の現在状態。 |

placeholder の `node` 表示や文字列だけの代替表示は不可とする。

## 6.3 Settings interaction

settings は option を変更できる画面操作として提供する。
変更は preview、state summary、action history に即時反映する。

必須操作:

| 操作 | 例 |
| --- | --- |
| boolean | disabled、readonly、open |
| enum | variant、tone、size、placement |
| text | label、placeholder、value |
| number | progress percent、slider value、split ratio |
| color | RGB / RGBA、theme token |
| list | options、tabs、tree nodes |

settings の変更は UI action として記録する。
内部で state だけを書き換えて履歴に残らない経路は不可とする。

## 6.4 Preset tabs

各 component page は、意味のある複数 preset を KUC `Tabs` で切り替える。
preset は最低限、default、disabled / readonly、interactive、theme variation、edge case を持つ。

preset 切替時は次を更新する。

- preview
- settings initial values
- state summary
- action / event history の current preset 表示

## 6.5 Storybook internal organisms

Storybook には画面を成立させる内部構成部品を置いてよい。
ただし公開 API にはしない。

| internal component | 役割 |
| --- | --- |
| catalog shell | 左ペイン、preview、settings の分割 |
| navigation tree | KUC TreeView を使った部品一覧 |
| preview workspace | 選択部品の実描画面 |
| settings inspector | option controls と履歴表示 |
| theme toolbar | theme / font role の切替 |

これらを `widget::organisms` として公開する場合は、別 change を作る。

## 6.6 Screenshot handling

Storybook screenshot は次の確認だけに使う。

- 画面が開く
- 主要 UI が見える
- theme が反映される
- 操作後の見た目差分が人間にも確認できる

Storybook screenshot は、単体テスト、入力回帰、配置回帰、画像回帰、guard の代替にしない。
CI/CD の完了判定は品質ゲート側の自動検証を主根拠にする。
