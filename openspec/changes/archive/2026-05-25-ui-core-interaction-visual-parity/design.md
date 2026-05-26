## Context

現在の KUC は、53 story の存在、最低 node 数、state id 衝突なし、panel theme 設定、visible snapshot までを検査している。
一方で、多くの UI は `open`、`selected_index`、`item_count`、`value` を持つ汎用 `UiInteractionState` に寄っており、UI ごとの意味が薄い。
可視描画も大半の UI を `node` hint として描いているため、利用者が細かい挙動を確認するには不足している。

## Goals / Non-Goals

**Goals:**

- UI ごとの専用 props / state / action を KUC core model として持つ。
- 状態（state）は component 内部で持ち、同じ UI を複数置いても `UiStateId` が衝突しない。
- Storybook は KUC panel 上で、UI 選択、light / dark theme 切替、操作、操作結果、callback log を確認できる。
- 可視描画は UI 種別ごとに最低限の形を持ち、`node` だけの表示に戻らない。
- `CodeDiff`、`ColorPicker`、`TreeView`、`CommandPalette`、`DynamicArrayEditor` は専用 props と詳細検査を持つ。
- KUC repo 内の script で検査し、`kal` 側には追記しない。

**Non-Goals:**

- consumer repo の移行は扱わない。
- Floem adapter を Storybook の主描画経路にしない。
- 互換 adapter の本実装は扱わない。

## Decisions

### 1. 汎用 interaction は互換層に残し、UI 詳細は typed model に持たせる

`UiInteractionState` は最低限の横断 summary として残す。
ただし UI 完了条件は、`ColorPickerState`、`CodeDiffModel`、`TreeViewState` のような専用 model を持つことに変える。

基盤 module 境界は次の通りにする。

- `interaction`: `UiAction`、`UiActionResult`、`UiCallbackLog` を置く。
- `component`: component 内部 state に action を適用する `ComponentAction` を置く。
- `atom` / `molecule`: 自身の `UiStateId` と内部 state を保持し、`UiAction` を受けてから neutral tree へ render する。
- `render_model`: `UiInteractionState` は互換 summary と snapshot 用に残す。

### 2. Facade は明示的に渡す core 設定窓口にする

`UiCoreFacade` は theme、font role、style sheet、global state を束ねる。
これは隠れたシングルトンではなく、`ComponentTree`、`RenderContext`、Storybook panel に明示的に渡す値である。

font は core で実ファイル path や OS 固有 family 名を持たない。
core は `FontFamily::Proportional` / `FontFamily::Monospace` と `font_role` だけを安定契約にし、Storybook や adapter が platform ごとに実フォントを解決する。

global state は component の内部 state を奪わない。
対象は active theme、focus target、active overlay、modal stack など、画面全体の横断状態に限定する。
Button の checked、Input の value、ColorPicker の色値のような UI ごとの状態は、引き続き component 内部で保持する。

### 3. Storybook panel は「見るだけ」から「操作確認面」にする

Storybook は左ナビと右プレビューを KUC `Panel` で描く。
右プレビューには story controls、操作結果、callback log を置く。
theme 切替は panel の明示設定を更新し、light / dark の両方で可視 snapshot を残す。

### 4. 可視検査は UI 種別ごとの描画責務を持つ

visual renderer は `UiNodeKind` ごとの renderer に分ける。
最低条件は、label、state、子要素数、theme、操作後差分が画像か report に出ること。
`node` だけの fallback は未知 UI の検出用に限定し、required UI では失敗扱いにする。

### 5. archive は復帰ではなく再構成する

archive 01〜24 の checkbox は引き継がない。
旧 props / operation / Storybook 条件を読み取り、KUC model の専用 task として作り直す。

## Work Batches

1. typed interaction model 基盤
2. `UiCoreFacade` / theme / font / global state 基盤
3. atoms / simple molecules の props と操作
4. overlay / selector / navigation 系 UI の props と操作
5. CodeDiff / ColorPicker / TreeView / CommandPalette / DynamicArrayEditor の詳細 model
6. Storybook panel 操作面
7. visual renderer coverage
8. KUC 専用 guard
9. full verification と証跡更新

## Risks / Mitigations

- [Risk] UI ごとの型が増えて core が肥大化する → [Mitigation] 型定義、状態、描画 model、操作を責務ごとに分ける。
- [Risk] Storybook が疑似 UI に戻る → [Mitigation] story controls は KUC component の state / action を通して更新する。
- [Risk] 検査が marker 依存に戻る → [Mitigation] screenshot、pixel、node coverage、operation report を組み合わせる。
- [Risk] lint を避けるために kal 側へ逃がす → [Mitigation] KUC repo 内 script だけを更新対象にする。
