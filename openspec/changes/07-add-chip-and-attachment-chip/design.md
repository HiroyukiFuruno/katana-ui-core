# Design — Chip / AttachmentChip / ChipGroup

## 目的

`Badge`（表示専用）と `Button`（汎用アクション）の中間にある「アイコン+ラベル+dismiss」のチップ型 UI を atom として確立し、attachment やフィルタタグ等の用途に応える。

## 採用方針

### 1. Chip atom

- 既存 `Badge` を破壊しない（`Badge` はパッシブな状態表示のまま）
- Chip は interactive=true で press / dismiss を持つ。interactive=false で静的表示も可能
- `variant`: Solid / Soft / Outline / Ghost
- `tone`: Neutral / Accent / Success / Warning / Danger / Muted
- `size`: Compact / Default / Large
- `dismissible=true` の時、trailing 領域に dismiss icon を表示。dismiss はキーボード Backspace / Delete でも発火
- `selected=true` で選択状態（フィルタタグ用）

### 2. AttachmentChip molecule

- `kind`: File / Image / URL / Paste / Resource
- File: name + size + mime label
- Image: thumbnail + name
- URL: domain + title
- Paste: snippet label + length
- Resource: type icon + id
- `progress` を持つ場合（uploading）は progress overlay
- `status`:
  - `Pending` → アイコンに灰色 dot
  - `Uploading` → progress overlay
  - `Ready` → 通常
  - `Error` → danger tone + retry action

### 3. ChipGroup molecule

- container: 横並び chip の集合
- option: `wrap`（折り返す / しない）, `gap`, `overflow`（None / Menu / ScrollHorizontal）
- overflow=Menu のとき、表示しきれない chip を `Menu` molecule に集約
- overflow=ScrollHorizontal のとき、横スクロール

### 4. 状態と event

- `Chip`:
  - state: selected, focused, disabled, callback_log
  - event: `ChipPressed { id }`, `ChipDismissed { id }`, `Focus`, `Blur`
- `AttachmentChip`:
  - state: status, progress, callback_log
  - event: `AttachmentChipOpened`, `AttachmentChipDismissed`, `AttachmentChipRetry`, `AttachmentChipStatusChanged`
- `ChipGroup`:
  - state: visible_ids, hidden_ids, overflow_open
  - event: `OverflowOpened`, `ChipReordered { from, to }` (drag が enable の場合)

### 5. drag & drop（optional）

- `ChipGroup` は `02-add-drag-drop-primitive` を opt-in で使い、chip の reorder ができる
- attachment では reorder 不要（add 順）なので default では off

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| `Badge` atom に dismiss を追加 | Badge は表示専用契約。dismiss を入れると interactive vs passive の混在で contract と test が散らかる。 |
| `Button` を rounded variant で使う | icon + label + dismiss + thumbnail + progress + status の組み合わせを 1 つの `Button` 内に詰めると option が肥大化し、ChipGroup overflow の判定材料も曖昧になる。 |
| `Chip` を molecule に置く | dismiss / select / press は単一 atom として完結する。複雑な内容を持つ `AttachmentChip` は molecule に分離する方が責務が明確。 |

## Out of scope

- 多選択（multi-select chip group）：v2 以降
- chip 内の sub-action（dropdown）：用途が薄い
- 動的アニメーション（spring / scale）：`add-animation-primitives-18`

## 影響範囲

- `Badge` Storybook に「interactive / dismiss は Chip」リンク追加
- `02-add-drag-drop-primitive` の opt-in 使用
- consumer の attachment 表現を KUC で統一する migration ガイドが必要
