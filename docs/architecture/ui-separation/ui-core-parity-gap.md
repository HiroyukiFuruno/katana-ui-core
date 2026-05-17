# UI Core parity gap

## 結論

`ui-core-root-plan` の既存タスク完了だけでは、旧 Floem 実装と同等+αの UI 実装完了とは言えない。
旧 Storybook 対象名は core catalog に並んでいたが、多くは `label + children` だけの薄いモデルで、UIごとの最低構造を検査していなかった。

## 是正方針

- Storybook は `katana-ui-core` の中核モデルだけを検査する。
- Storybook は Floem / adapter crate を経由しない。
- 検査は story 数だけでなく、各 story の最低 node 構造、必須 page 欠落、state conflict を見る。
- UIごとの状態は component 内部で作り、重複 UI でも `UiStateId` が一意であることを確認する。

## 現在の最低検査

`storybook/src/requirements.rs` が旧 Storybook 対象を必須 page として持つ。
`storybook/src/catalog.rs` は以下を検査する。

- `state_conflicts=0`
- `structure_failures=0`
- `missing_required_pages=0`
- `stories == validated`
- Storybook 自体は `katana-ui-core::panel::Panel` で構成する。
- panel は `ThemeSnapshot` を受け取り、左ナビと右プレビューの両方で theme 設定済みであることを検査する。
- KUC core は純 Rust の部品（component）合成と、CSS 的な後付け見た目設定（style sheet）を分離して扱う。
- 旧 Floem Storybook や静的HTML export は完了根拠にしない。
- Modal の別窓表示は `ModalWindowPlacement::same_display(...)` で親表示領域（display bounds）内に配置し、成功時は `frontmost=true` を必須にする。
- Storybook は `--runtime-regression` と `--open-modal-window` で、状態反映、重ね表示（overlay）描画、別ネイティブ画面（native window）描画を検査する。

`UiInteractionState` は旧実装で状態を持っていた複合UIの最低状態を表す。
`CommandPalette`、`ComboBox`、`ColorPicker`、`MenuButton`、`ModalOverlay`、`TreeView` などは、外部 store ではなく component 内部状態から `open`、`selected_index`、`item_count`、`value` を neutral tree へ出す。

## UI別再判定状況

| 対象 | 現状 | 次の判定 |
|---|---|---|
| Theme / Panel theme | `ThemeSnapshot`、panel theme id、light / dark panel gate は実装済み。 | UI 別 theme props は各 UI の再判定で確認する。 |
| Text / Icon / Spinner | Storybook catalog の最低構造と panel theme 適用は確認済み。 | atom 別の accessibility props は後続 UI 詳細で確認する。 |
| SvgButton / TextButton / IconTextButton | Storybook catalog の最低構造と同一 label の state 一意性は確認済み。 | 実操作結果の反映は後続 UI 詳細で確認する。 |
| Toggle / SegmentedToggle / SelectBox / ColorSwatch / TextInput | 選択・入力 state は内部 state から中立モデルへ出る。 | 実際の UI 操作再生は後続 adapter / visual gate で確認する。 |
| SearchBox / Tooltip / Badge / KeyCap / Card | 入力、hover 相当の開閉、補助情報、配置構造は KUC model で確認済み。 | 実 hover 操作と視覚密度は後続 adapter / visual gate で確認する。 |
| Accordion / SplitPane / Modal / Popover | 開閉 state と分割値は KUC model に出る。Modal は同一 display 配置計画、前面表示、別 native window 起動を Storybook visual gate で確認済み。 | Popover などの実 hover 操作密度は後続 adapter / visual gate で確認する。 |
| ColorPicker / CodeDiff | 色選択 value、open state、差分 item count は KUC model で確認済み。 | 表示密度と行単位 props は後続 UI 詳細で確認する。 |
| 追加 UI 群 | KUC model、内部 state、Storybook panel、theme gate の最低線は確認済み。 | 実操作と見た目密度は UI 別の後続詳細で確認する。 |

## 残るリスク

この是正後も、各 UI の詳細な操作意味まではまだ完全ではない。
次の段階では `CodeDiff`、`ColorPicker`、`TreeView`、`CommandPalette`、`DynamicArrayEditor` などに、UI別の専用 props を増やす。
