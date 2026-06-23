# core / input / color Storybook variant audit（23.1〜23.10）

## 対象

- primitive: `Text`, `Icon`, `Spinner`, `LoadingDots`
- button: `SvgButton`, `TextButton`, `IconTextButton`
- selector: `Toggle`, `SegmentedToggle`, `SelectBox`, `ColorSwatch`, `ColorPickerRgba`
- input: `TextInput`, `SearchBox`
- composite: `ComboBox`, `SlideControl`

## 追加対応（2026-05-13）

### primitive/text

- `max_lines`: 画面上で複数行テキストを 1 行に省略する例を追加。
- `align`: start / center / end の配置差分を追加。
- widget 側の `Text::view` にも `max_lines` と `align` を反映。

### primitive/spinner

- `speed_rps`: slow / normal / fast を比較する表示を追加。

### primitive/loading_dots

- `dot_count`: 2 / 3 / 5 を比較。
- `dot_gap`: narrow / wide を比較。
- `animation_speed_ms`: fast / slow を比較。
- `color`: color override を比較。

### button/svg

- `size`: sm / md / lg を実 widget で比較。
- `loading`: loading 中は callback が実行されないことを callback log で確認。

### selector/select

- `SelectSize`: sm / md / lg を比較。

### combo_box

- `disabled`: disabled 表示を追加。
- `open`: 初期 open 表示を追加。
- `placement`: BottomStart / TopEnd / End を比較。

### input/text

- `leading_icon_mode`: Visible / Reserved / Hidden を比較。
- clear、loading、readonly、disabled、size は既存 Storybook で確認。

### input/search

- `icon_preset`: Search / Clear / Submit を比較。
- `icon_mode`: Visible / Reserved / Hidden を比較。
- `control_mode`: Visible / Reserved / Hidden を比較。
- regex / word / case、clear、submit、callback log は既存 Storybook で確認。

### slide_control

- `custom_format`: 0〜1 の値を 0〜255 表示に変換する例を追加。
- `disabled`: disabled 表示を追加。
- `readonly`: readonly 表示を追加。

## theme 観点

- light / dark は Storybook の global theme 切替で確認する。
- 各 widget は theme token 経由の色を使うため、同一画面に light / dark を重複表示しない。

## callback 観点

- 値変化系は Storybook 画面内の log、selected value、preview、count、status に反映する。
- button 系はクリック log を表示する。
- disabled / readonly / loading など、操作を抑制する状態は「log が変わらないこと」を確認対象にする。

## 未確認

なし。

## 検証

- `cargo fmt --all`: pass
- `just storybook-check`: pass
- `just ast-lint`: pass
- `just storybook-smoke`: pass

`storybook-smoke` は初期表示クラッシュ確認だけに使い、variant 完了根拠にはしない。
