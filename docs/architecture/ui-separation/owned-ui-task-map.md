# KUC owned UI task map

## 結論

archive 済みの 01〜24 は、そのまま完了扱いで復帰しない。
旧実装は Floem Storybook 前提を含むため、KUC では `katana-ui-core` の中立 model、内部 state、panel Storybook、theme 設定を必須条件に組み替える。

## 復帰元

| 旧 change | KUC 独自UI task |
|---|---|
| 01-theme-tokens | Theme / Panel theme |
| 02-text-primitive | Text |
| 03-icon-primitive | Icon |
| 04-spinner-primitive | Spinner |
| 05-svg-button | SvgButton |
| 06-text-button | TextButton |
| 07-icon-text-button | IconTextButton |
| 08-toggle | Toggle |
| 09-segmented-toggle | SegmentedToggle |
| 10-select-box | SelectBox |
| 11-color-swatch | ColorSwatch |
| 12-text-input | TextInput |
| 13-search-box | SearchBox |
| 14-tooltip | Tooltip |
| 15-badge | Badge |
| 16-key-cap | KeyCap |
| 17-card | Card |
| 18-accordion | Accordion |
| 19-split-pane | SplitPane |
| 20-modal-overlay | Modal / ModalOverlay |
| 21-popover | Popover |
| 22-rgba-color-picker | ColorPicker |
| 23-color-picker-complete-parity | ColorPicker parity |
| 24-code-diff | CodeDiff |

## 新しい完了条件

各UIは以下を満たすまで完了にしない。

- `katana-ui-core` の中立 model として表現する。
- UIごとの状態を component 内部で持ち、重複UIでも `UiStateId` が一意である。
- Storybook は `katana-ui-core::panel::Panel` で左ナビと右プレビューを構成する。
- Panel は `ThemeSnapshot` を受け取り、theme 設定済みである。
- Storybook gate は story 数だけでなく、必須UI、最低構造、状態衝突、panel theme を検査する。
- Floem adapter は検証対象にしてよいが、Storybook の描画経路にはしない。

## 現在の扱い

- root / navigation / preview の表示枠（panel）は KUC core model の対象にする。
- 表示枠（panel）の見た目テーマ（theme）は必須要件であり、未設定なら gate 失敗にする。
- 旧 01〜24 の完了 checkbox は KUC 完了証跡として使わない。
- 各 UI は `openspec/changes/katana-widget-parity-backlog/tasks.md` の `28. KUC 独自 UI parity reset` で再判定する。
