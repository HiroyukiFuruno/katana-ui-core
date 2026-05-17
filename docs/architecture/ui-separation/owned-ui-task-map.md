# KUC owned UI task map

## 結論

01〜24 は `establish-kuc-atoms-molecules-catalog` へ移管する。
旧個別 change の完了チェックは履歴であり、現在の KUC 実装完了の根拠にしない。

現在の目標は、利用側が最小部品（atoms）と組み合わせ部品（molecules）を組み合わせて UI を構築できることに置く。
画面（pages）、画面ひな形（templates）、公開用の大きな画面単位（organisms）は今回の実装正本に含めない。
ただし Storybook 自身の shell / navigation / inspector などは内部構成部品として作ってよい。

## 正本

| 種別 | 正本 |
| --- | --- |
| root architecture | `openspec/changes/ui-core-root-plan/` |
| atoms / molecules / Storybook catalog | `openspec/changes/establish-kuc-atoms-molecules-catalog/` |
| 旧changeの入力元 | `openspec/changes/archive/`、active `18` / `23` / `24` |

## 01〜24 再分類

各 UI は `option`、`action`、`event`、`state`、`preset`、`preview`、`settings`、`automated-test`、`visual-regression`、`storybook-page` を満たすまで完了にしない。

| # | 旧 change | KUC 分類 | 現在の対象 |
| --- | --- | --- | --- |
| 01 | theme-tokens | core foundation | Theme / Panel theme |
| 02 | text-primitive | atom | Text |
| 03 | icon-primitive | atom | Icon |
| 04 | spinner-primitive | atom | Spinner / LoadingDots |
| 05 | svg-button | atom / molecule | SvgButton |
| 06 | text-button | atom / molecule | TextButton |
| 07 | icon-text-button | atom / molecule | IconTextButton |
| 08 | toggle | atom / molecule | Toggle |
| 09 | segmented-toggle | molecule | SegmentedToggle |
| 10 | select-box | molecule | SelectBox |
| 11 | color-swatch | atom / molecule | ColorSwatch |
| 12 | text-input | atom | Input / TextInput |
| 13 | search-box | molecule | SearchBox |
| 14 | tooltip | molecule | Tooltip |
| 15 | badge | atom | Badge |
| 16 | key-cap | atom | KeyCap |
| 17 | card | molecule | Card |
| 18 | accordion | molecule | Accordion |
| 19 | split-pane | molecule / layout | SplitPane |
| 20 | modal-overlay | molecule | ModalOverlay / Modal |
| 21 | popover | molecule | Popover |
| 22 | rgba-color-picker | molecule | ColorPicker |
| 23 | color-picker-complete-parity | molecule | ColorPicker parity |
| 24 | code-diff | molecule | CodeDiff |

## 追加 UI の扱い

`katana-widget-parity-backlog` で採用した追加 UI は、同じ基準で `establish-kuc-atoms-molecules-catalog` へ移す。

| UI | KUC 分類 | 備考 |
| --- | --- | --- |
| ProgressBar | atom | 進捗表示 |
| Tabs | molecule | Storybook preset 切替にも使う |
| Breadcrumb | molecule | 階層パス表示 |
| SideMenu | molecule | shell に近いが MVP では molecules 側 |
| SelectionList | molecule | section / marker / selected row |
| SlideControl | molecule | 数値調整 |
| DynamicArrayEditor | molecule | 配列編集 |
| AlignCenterWrapper | layout / molecule | 中央配置 |
| TreeView | molecule | Storybook 左ペインにも使う |
| ComboBox | molecule | input + option list |
| MenuButton | molecule | trigger + menu |
| CommandPalette | molecule | provider を domain 外へ出す |
| StatusBar | molecule | severity message |
| Toolbar | molecule | action rail |
| NotificationToast | molecule | transient message |

## Storybook internal

Storybook 自身を構成するため、次は内部構成部品として許可する。
公開 widget API としての organisms / templates ではない。

| 内部構成 | 役割 |
| --- | --- |
| catalog shell | 画面全体の分割 |
| navigation tree | TreeView による部品一覧 |
| preview workspace | 選択部品の表示領域 |
| settings inspector | option 変更、state、event、action 履歴 |

## 完了判定

- Storybook は部品カタログであり、正しさの主根拠ではない。
- 部品の正しさは自動テスト、layout regression、visual regression、input regression、guard で判定する。
- `kal` 側へ KUC 固有ルールを追記しない。
- `katana-widget-parity-backlog` と `ui-core-interaction-visual-parity` は、この文書と新 change へ要件を移した後は superseded として扱う。
