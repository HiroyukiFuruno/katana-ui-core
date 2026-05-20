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
| atoms / molecules / Storybook | `openspec/changes/establish-kuc-atoms-molecules-catalog/` |
| 旧changeの入力元 | `openspec/changes/archive/`、active `18` / `23` / `24` |

## 01〜24 再分類

各 UI は `option`、`action`、`event`、`state`、`preset`、`preview`、`settings`、`automated-test`、`numeric-rendering-contract`、`storybook-page` を満たすまで完了にしない。

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
| ScrollArea | layout / molecule | axis / offset / extent / scrollbar / event を持つ scroll container |
| SplitPane | layout / molecule | 2 pane / ratio / resize event / reset を持つ split layout |
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
| StatusBar | molecule | footer に固定する SingleMessage / MultiSegment status bar。segment popover、progress meter、density を持ち、画面上部や form 内に残る告知は Banner を使う |
| Toolbar | molecule | action rail。overflow、split action、display mode、density、accelerator を持つ |
| NotificationToast | molecule | 単一の transient message。複数 toast の queue / dedup / position は ToastStackManager を使い、消えずに残す inline 告知は Banner を使う |
| SearchControlStrip | molecule | query option / navigation / replace request |
| ContextMenu | molecule | pointer 起点 / submenu / keyboard navigation |
| DragAndDropPrimitive | atom / molecule / interaction | DragHandle / DropIndicator / DragPreview / DragData / DropTarget / keyboard drag。OS payload は adapter contract で `os/*` tag に変換する |
| CloseableTabStrip | molecule | closeable / dirty / draggable tab。workspace / document domain は持たない |
| TextArea | atom | 複数行フォーム入力。KLE 本文 editor ではない |
| Chip | atom | filter / tag 表示。variant / tone / size / selected / dismissible / keyboard dismiss を持つ |
| AttachmentChip | molecule | file / image / URL / paste / resource 添付。status / progress / retry action を持つ |
| ChipGroup | molecule | wrap / overflow menu / horizontal scroll / reorder opt-in を持つ chip container |
| DiagnosticsList | molecule | severity / location / quickfix / fix preview / bulk fix / keyboard navigation の汎用問題一覧。lint domain は持たない |
| EmptyState | molecule | 空状態の heading / body / icon or illustration / primary and secondary action / live region |
| Banner | molecule | 画面内に残る inline alert。toast / status bar と責務を分ける |
| ToastStackManager | molecule | 複数 toast の queue / dedup / position / pause-on-hover / action dismiss |
| HoverCard | molecule | hover / focus で開く rich content。delay、pointer follow、slot、共有 placement を持つ |
| ProgressMeter | atom | linear / ring / pie の進捗表示 |
| ShortcutCombo | atom | 複数キーの組み合わせ表示。platform_display / separator / size / tone を持つ |
| ShortcutCheatsheet | molecule | ショートカット一覧。group_layout / query / select event を持つ |
| SettingsList | molecule | セクション付き設定フォーム。density / dirty_visualization / query / collapse / reset / focus state を持つ |
| CollapsiblePanel | molecule | 折りたたみ / hover / resize panel。AppShell は持たない |
| CommandLauncherResults | molecule | query + result row + shortcut + keyboard selection |
| Virtualization | interaction / molecule contract | List / SelectionList / TreeView / CommandPalette / DiagnosticsList が共有する visible range / overscan / row height / aria-setsize 契約 |
| Skeleton / SkeletonCluster | atom / molecule | loading placeholder。shape / size / animation / reduced-motion / live region / preset layout を数値化された contract で持つ |

## Storybook internal

Storybook 自身を構成するため、次は内部構成部品として許可する。
公開 widget API としての organisms / templates ではない。

| 内部構成 | 役割 |
| --- | --- |
| storybook shell | 画面全体の分割 |
| navigation tree | TreeView による部品一覧 |
| preview workspace | 選択部品の layout、rendering、contract、status 表示領域 |
| settings inspector | option 変更、state、event、action、quality 履歴 |
| panel scroll state | Navigation / Preview / Details の独立縦スクロール |

## 完了判定

- Storybook は静的見本帳ではなく、選択中 UI の layout / option / action / event / state / rendering / panel 独立 scroll を実画面で扱うフィードバック用の画面である。
- 左 TreeView は探索と選択のために使い、中央本文に全件 component card を毎回出す構成は採用しない。
- Storybook は正しさの主根拠ではない。
- 部品の正しさは自動テスト、数値化された layout / rendering contract、input regression、state / event / action contract、guard で判定する。
- `kal` 側へ KUC 固有ルールを追記しない。
- `katana-widget-parity-backlog` と `ui-core-interaction-visual-parity` は、この文書と新 change へ要件を移した後は superseded として扱う。
