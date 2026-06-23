# Design — ToastStackManager molecule

## 目的

複数 toast の表示順序、重複排除、最大表示数、キューイング、ホバー一時停止を統一する manager molecule を提供する。

## 採用方針

### 1. position と stack 方向

| position | stack 方向 |
| --- | --- |
| TopStart, TopCenter, TopEnd | 上から下に積む（新しいものを下） |
| BottomStart, BottomCenter, BottomEnd | 下から上に積む（新しいものを上） |

`enter_direction` / `exit_direction` は position から導出されるデフォルトを上書き可能。

### 2. 重複排除（dedup）

- `None`: 重複も全部表示
- `ById`: 同 id の toast が visible / queued に存在すれば、後者で前者を置換（`ToastReplaced` event）
- `ByIdAndSeverity`: id + severity が一致した場合のみ置換

置換時、残り duration をリセットする / しないは option（`replace_resets_duration: bool`、default true）。

### 3. キューイング

- visible.len() < max_visible のとき、queued から visible へ promote
- visible が timeout / dismiss された時点で promote が起きる
- queued は 100 件で打ち切り（option で変更可能、超過分は drop + warning event）

### 4. pause on hover

- `pause_on_hover = true` のとき、いずれかの visible toast にホバーしている間、全 toast の duration timer を一時停止
- focus が toast 内 interactive element に入った場合も同様
- マウスが離れて focus が外れたら resume

### 5. accessibility

- container は `aria-live = polite` または `assertive`（severity 由来）
- 各 toast は role=alert（Warning/Danger）または role=status（Info/Success/Neutral）

### 6. action button

- toast に primary / secondary action（`Button` atom）を持てる
- action 押下で `ToastDismissed { id, reason: Action }` が発火

### 7. duration

- duration_ms = 0 のとき auto dismiss なし（manual close 必須）
- duration_ms > 0 のとき、tick で残り時間を減らす

### 8. 描画 model

- ToastStackManager は overlay layer 上の絶対座標で描画
- consumer は manager を一つマウントするだけで toast 全体を扱える

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| `NotificationToast` を multiple モードで使う | 単一 toast の契約と多 toast の編成（キュー / dedup / pause）が混在し、preset が散らかる。 |
| consumer 側でキュー管理 | dedup / pause / 最大数 / 位置の差が cross-app で積み上がる。 |

## Out of scope

- グローバル window 跨ぎ toast manager（multi-window 連携）：v2 以降
- アニメーション spec：`add-animation-primitives-18` で扱う
- ストレージ永続化（再起動後再表示）：consumer 責務

## 影響範囲

- `NotificationToast` の Storybook ページに「複数まとめは ToastStackManager」リンクを追加
- adapter は overlay layer をサポートする必要がある（既に modal で同様の機構があれば共有）
