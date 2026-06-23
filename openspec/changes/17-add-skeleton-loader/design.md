# Design — Skeleton atom + SkeletonCluster molecule

## 目的

ロード中の形状プレースホルダを標準化する。layout shift を抑え、UX 品質を上げる。

## 採用方針

### 1. shape

```text
SkeletonShape =
  | Rect
  | Circle
  | Line { thickness: f32 }
  | Text { lines: usize, last_line_ratio: f32 }
```

- `Text { lines, last_line_ratio }` は段落 placeholder。最終行は短くする（`last_line_ratio = 0.6` 等）。

### 2. size

`SkeletonSize = Fixed(f32) | Fill | Auto`

- `Fixed`: 絶対値
- `Fill`: 親に追従
- `Auto`: aspect_ratio とテキスト長から導出

### 3. animation

- `None`: 静止
- `Pulse`: 不透明度を 0.4 ↔ 1.0 で揺らす
- `Shimmer`: 横スライドのグラデーション
- `Wave`: 縦に動く wave gradient

reduced-motion 環境では `None` に降格（adapter 設定 / accessibility 設定に従う）。

### 4. SkeletonCluster

preset：
- `Card`: 上に Rect、下に 2 line Text
- `ListRow`: 左 circle、右 2 line Text
- `Message`: avatar + body Text + meta
- `Paragraph`: 3-5 lines Text
- `ImageCard`: image rect + title + meta
- `CodeBlock`: rect lines（rectangle width 不揃い）

cluster は children の skeleton を内部 `Row` / `Column` で配置。

### 5. accessibility

- `accessibility_label` を指定すれば screen reader に announce
- announce は live region polite
- 大量の skeleton が並ぶ場合は cluster 単位で 1 件のみ announce（重複防止）

### 6. layout shift 防止

- consumer は real content と同じ枠サイズの skeleton を表示することを推奨
- molecule embed 時、loading=true で skeleton、false で real content を出す pattern を contract test で検証

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| `Spinner` を使う | layout shift が起きる。形状が伝わらない。 |
| Card / Text の透明版を作る | 動的アニメーションが付けにくい。reduced-motion 対応の共通化が散らかる。 |

## Out of scope

- 自動 measure（real content と完全に合致する skeleton 生成）：v2 以降
- アニメーションのカスタム curve：`18-add-animation-primitives` で扱う

## 影響範囲

- DiagnosticsList / SelectionList / TreeView 等の loading 表示で embed
- reduced-motion 設定の参照
