# Design — DiagnosticsList molecule

## 目的

severity 付き問題リストを汎用 molecule として提供し、problems panel / inline diagnostics / lint result / tool result の差を統一する。

## 採用方針

### 1. データモデル

```text
DiagnosticsItem {
  id, severity (Error | Warning | Info | Hint),
  message,
  location: DiagnosticLocation,        // file_label + line/col、または scope
  source: String,                       // rule id / linter name
  quickfix: Option<DiagnosticAction>,
  fix_preview: Option<DiagnosticFixPreview>,
}

DiagnosticAction {
  id, label,
}

DiagnosticsGroup {
  id, label, severity_summary, count, collapsed,
  items: Vec<DiagnosticsItem>,
}
```

### 2. グループ化方針

- `group_by`: `Severity` / `Source` / `Location` / `None`
- group ごとに count badge、severity summary（混在の場合は最大 severity）を表示
- group / sort / filter は `DiagnosticsListPlanner` の純関数で snapshot 化する

### 3. fix preview

- `fix_preview: CodeDiffSnapshot`（typed）を子に embed
- expand 時に `CodeDiff` molecule が描画される
- 子 `UiStateId` は親と分離
- apply fix で `DiagnosticFixApplied { id }` 発火、親側で fix_preview を消化

### 4. bulk action

- bulk fix の preview は consumer が渡す `bulk_preview` slot を別 modal で見せる（KUC `ModalOverlay` を使う）
- bulk apply の結果は `BulkFixApplied { applied_ids, skipped_ids }` event

### 5. severity filter

- chip row 風の filter（`07-add-chip-and-attachment-chip` の `Chip` を利用）
- 選択 severity を `DiagnosticsListOptions.severity_filter` に保持
- 表示 item は filter で評価

### 6. empty / loading slot

- `empty_slot: Option<UiTree>` を受け取り、結果 0 件時に表示する
- `loading_slot: Option<UiTree>` を受け取り、loading=true のときに表示する
- `EmptyState` と `Skeleton` は推奨 child だが、DiagnosticsList の必須依存にしない
- slot の子 `UiStateId` は親と分離する

### 7. キーボード

- ↑↓ で item 移動
- ← / → で選択 item の fix preview を collapse / expand
- Enter で `NavigateRequested`、Space で `ApplyFix`（quickfix がある場合）
- F8 / Shift+F8: 次 / 前の error にジャンプ（accelerator）

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| `SelectionList` に severity / fix_preview / actions を追加 | SelectionList の汎用契約が肥大化し、preset が散らかる。 |
| consumer 側で `List` + `Card` を組み合わせる | severity sort / group / filter / bulk / fix preview の挙動差が cross-app で積み上がる。 |
| LSP の `Diagnostic` model をそのまま持ち込む | KUC は LSP に依存しない。`DiagnosticLocation` は file_label + line/col のような単純な model に抽象化する。 |

## Out of scope

- LSP プロトコル統合：consumer 責務
- 自動修復のアルゴリズム：consumer 責務
- ファイル横断検索：consumer 責務

## 影響範囲

- `CodeDiff` molecule に「snapshot として埋め込み利用」される旨を明記
- 別 change `09-add-empty-state` / `17-add-skeleton-loader` / `07-add-chip-and-attachment-chip` と組合せ
- consumer の problems panel / editor diagnostics / lint result 描画を KUC molecule で置換
