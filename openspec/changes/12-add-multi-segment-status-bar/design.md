# Design — Multi-segment StatusBar

## 目的

エディタ / chat / linter で共通の「複数 segment status bar」を1つの molecule で扱えるよう、既存 `StatusBar` を mode 拡張する。

## 採用方針

### 1. mode

```text
StatusBarMode = SingleMessage | MultiSegment
```

- `SingleMessage`: 既存契約（severity + message + dismiss + actions）
- `MultiSegment`: leading / center / trailing 3 列に segment 配列を並べる

mode はデフォルト `SingleMessage` で後方互換。MultiSegment へは consumer が明示的に切替える。

### 2. segment 構造

```text
StatusBarSegment {
  id, label, icon, tone, alignment, tooltip,
  interactive: bool,
  popover: Option<PopoverSpec>,
  progress: Option<f32>,
  accessibility_label: Option<String>,
}
```

- `progress` を持つ segment は薄い progress bar を背景に描画
- `interactive=true` は role=button、Enter / Space で `SegmentPressed`
- `popover` を持つ場合、segment クリックで popover open（共通 placement engine）

### 3. layout

- container は 3 列 grid（leading / center / trailing）
- 各列内では segment 順に並べ、列内 gap は theme token
- center が overflow するとき、center 内で内部スクロールではなく省略表記
- density は padding / font_size を変える

### 4. ホバー / フォーカス

- segment にホバーで tooltip
- focus で focus ring
- popover が ある segment は trailing カラットアイコンを表示

### 5. 後方互換

- mode = SingleMessage の挙動は既存 API そのまま
- MultiSegment と SingleMessage の同時利用は禁止（validation で reject）

### 6. accessibility

- container role=status
- 各 segment は読み上げ順=leading → center → trailing
- live region update（label が変わったら polite announce）

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| 別 molecule `StatusBarMultiSegment` を追加 | API 重複と consumer の選択コスト増。同 molecule の mode 切替で十分。 |
| `Toolbar` molecule を status bar 用に流用 | toolbar は action 中心、status bar は表示中心。density / segment role が異なる。 |

## Out of scope

- 複数 status bar（ウィンドウ上下両方）：consumer 責務
- グローバルアニメーション：`18-add-animation-primitives`

## 影響範囲

- `StatusBar` API の拡張（mode option を追加）
- 共通 placement engine（`add-rich-popover-and-hover-card-04`）依存
- consumer 側の status bar 実装を KUC で統一
