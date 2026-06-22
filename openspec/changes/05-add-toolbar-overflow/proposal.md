## Why

`katana` editor の `toolbar.rs` / `toolbar_popup.rs`、`katana-chat-ui` の adapter bar / composer 上部 action 列、`katana` workspace_toolbar の各画面で「ツールバー（toolbar）アクションが幅に収まらない時に隠れた action を overflow popup に集約する」挙動が求められている。

KUC `Toolbar` molecule は actions / groups / disabled / overflow を契約に持つが、現状の実装は overflow popup の挙動（しきい値計算、可視 / 不可視の確定、popup の anchor / placement、キーボード ナビゲーション、accelerator 表示）が薄く、consumer 側で再実装されている。さらに「primary action + secondary action / dropdown action（split button）」「icon-only / icon+label の切替え」「action group の divider」など、現実の toolbar 表現に必要な option が不足している。

## What Changes

- `Toolbar` molecule の option を拡張する:
  - `overflow_strategy`: `Hide` / `Menu` / `Custom`
  - `display_mode`: `IconOnly` / `IconLeading` / `IconTrailing` / `LabelOnly`
  - `density`: `Compact` / `Default` / `Spacious`
  - `actions`: 個別 action に `priority`（はみ出し時に隠す順序）と `accelerator`（KeyCap 表示）を typed option で持つ
  - `groups`: action 集合に `divider` / `label` を typed option で持つ
  - `split_action`: 1 つの action に primary + secondary（dropdown）の構造
- overflow popup の placement は共通 placement engine（`04-add-rich-popover-and-hover-card`）を使う。
- キーボードナビゲーション（Tab / Arrow / Home / End / Enter / Space）と accelerator（`Cmd/Ctrl + X` で対応 action 起動）を契約に含める。
- icon-only 表示時の Tooltip / accessibility label を必須化する。
- `01-add-context-menu` と組み合わせ、toolbar の空領域 / action 右クリックで関連操作メニューを開ける hook を契約に含める。

## Capabilities

### Modified Capabilities

- `kuc-widget-layer`: `Toolbar` molecule の拡張 option（overflow / display_mode / density / split_action / accelerator）を明記する。

### New Capabilities

- `kuc-toolbar-overflow`: overflow しきい値判定、priority による hide 順序、overflow popup 起動、accelerator 起動、icon-only での accessibility ラベル必須化を定義する。

## Impact

- `crates/katana-ui-core/src/molecule/basic.rs`（`Toolbar`）または専用 `molecule/toolbar/` を再配置する。
- 既存 `Toolbar` の Storybook ページに「overflow」「split action」「density」「accelerator」preset を追加する。
- consumer (`katana` workspace_toolbar、editor toolbar、`katana-chat-ui` adapter bar) は overflow / display_mode 切替えを KUC option で表現できるようになる。
- 共通 placement engine（`04-add-rich-popover-and-hover-card`）に依存。
