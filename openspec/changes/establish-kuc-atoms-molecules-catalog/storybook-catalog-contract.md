# Storybook contract

作成日: 2026-05-18
対象: `kuc-storybook-catalog`

## 結論

Storybook は KUC が提供する UI 群を、利用者や開発者が実画面で触ってフィードバックするための場である。
静的見本帳のように部品を並べるだけの画面は Storybook として不十分である。
部品の正しさは自動テストと品質ゲートで判定し、Storybook やユーザー操作に完了判定を委ねない。
各 UI page は option、action、event、state、preset、preview、settings を同じ画面で扱える必要がある。
settings や log だけが変わり、preview 本体が変わらない画面は未完了として扱う。

## 画面構成

画面上では次の構成にする。

| 領域 | 表示するもの | 使用する KUC UI |
| --- | --- | --- |
| 左ペイン | 部品一覧。atomic design とカテゴリでネストする。 | `TreeView` |
| 中央上 | preset 切替。部品ごとに複数状態を切り替える。 | `Tabs` |
| 中央 | 選択中 UI の layout、rendering、contract、status を扱う。全件カード一覧は置かない。 | component preview |
| 右または下 | option 設定、state、event 履歴、action 履歴、quality を読む。 | settings inspector |
| 全体 | theme control、font role control、Storybook status。 | Storybook internal shell |

Navigation / Preview / Details は、それぞれ独立した panel scroll state を持つ。
画面全体を一つの縦スクロールとして扱い、どの panel を操作したか分からなくなる構成は採用しない。
各 panel は scrollbar の表示方式、thumb 位置、drag 操作を model として持ち、自動テストで差分を検証できるようにする。

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
  Storybook shell
  Navigation tree
  Preview workspace
  Settings inspector
```

TreeView の selection event は、preview page、settings panel、state / event / action 履歴を同時に切り替える。
TreeView 自体も Storybook の対象 component として、自分自身の page を持つ。

## 6.2 Component page

各 component page は次を持つ。

| 項目 | 契約 |
| --- | --- |
| preview | 現在の option と preset を反映した実 UI。layout と rendering 差分を実画面で触れる。 |
| settings | option 値を画面上で変更する controls。変更は typed action として履歴に残す。 |
| state summary | `UiStateId`、主要 state、child state id。 |
| event history | 発火した event、target、payload summary。 |
| action history | 実行した action、before / after state summary。 |
| requirement status | option / action / event / state / preset / test / rendering contract の現在状態。 |

placeholder の `node` 表示や文字列だけの代替表示は不可とする。
中央本文に全 component を毎回並べるカード一覧も不可とする。
カテゴリー探索は左 TreeView の責務であり、中央本文は選択中 UI の詳細を扱うために使う。

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

## 6.3.1 Interactive preview

preview は画像のような表示だけではなく、操作対象を持つ。
操作できる UI は次を同じ page 内で扱える必要がある。

| 観点 | 契約 |
| --- | --- |
| layout | hit target、余白、align、scroll bounds を自動テストで検証できる。 |
| option | settings 操作で typed option が変わり、preview に反映される。 |
| action | click、type、open、close、select、drag などが action として記録される。 |
| event | action に対応する event、target、payload summary が表示される。 |
| state | 操作前後の component state と `UiStateId` が表示される。 |
| rendering | 操作や preset によって preview 本体の pixel / marker が変わる。 |

chip、枠線、ログ文言だけが変わり、preview 本体が変わらない場合は Storybook として不十分である。

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
| storybook shell | 左ペイン、preview、settings の分割 |
| navigation tree | KUC TreeView を使った部品一覧 |
| preview workspace | 選択部品の実描画面 |
| settings inspector | option controls と履歴表示 |
| theme toolbar | theme / font role の切替 |
| panel scroll state | Navigation / Preview / Details の独立 scroll 管理 |

これらを `widget::organisms` として公開する場合は、別 change を作る。

## 6.6 Automated verification boundary

Storybook はフィードバック用の実画面であり、完了判定の根拠にしない。
CI/CD の完了判定は、単体テスト、入力回帰、配置回帰、数値化された rendering contract、guard の自動検証を根拠にする。
