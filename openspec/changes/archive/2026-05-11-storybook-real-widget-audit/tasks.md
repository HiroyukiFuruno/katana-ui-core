# Tasks — storybook-real-widget-audit

## 1. 方針

- [x] 1.1 Storybook は resolved 値の説明表示ではなく、利用者が実際に触る runtime widget を表示する
- [x] 1.2 readonly 表示は「操作できない実 widget」として表示し、ラベルや四角形による疑似描画で代用しない
- [x] 1.3 動的部品は、状態変化が画面上の別領域へ反映される live feedback を持つ
- [x] 1.4 破棄済み signal の `get()` / `get_untracked()` による panic を許容しない

## 2. 対応済み

- [x] 2.1 Toggle の疑似表示を除去し、サイズ・disabled・操作を実 widget に反映する
- [x] 2.2 SegmentedToggle の疑似表示を除去し、実 widget で表示する
- [x] 2.3 SelectBox の疑似 trigger/options 表示を除去し、実 widget で表示する
- [x] 2.4 ColorSwatch の疑似 swatch 表示を除去し、選択結果 preview / RGBA 表示を追加する
- [x] 2.5 TextInput の leading / trailing を文字ではなく実 icon / spinner / clear action へ置換する

## 3. 対応中

- [x] 3.1 SearchBox を katana 準拠にし、input 内に leading / clear / submit の 3 SVG icon slot を持たせる
- [x] 3.2 SearchBox の各 icon slot は default hidden とし、visible / reserved / hidden を利用側で制御できるようにする
- [x] 3.3 SearchBox の icon は preset と任意 SVG override を両方サポートする
- [x] 3.4 SearchBox の Storybook に default hidden / all visible / reserved / custom SVG の実例を置く

## 4. 未対応

- [x] 4.1 Tooltip の Storybook から文字で配置を説明するだけの疑似表示を除去し、実 hover / focus tooltip を中心にする
- [x] 4.2 Tooltip の interactive sample は実 `Tooltip::view` と実 overlay の挙動確認に寄せる
- [x] 4.3 Popover の Storybook から overlay 座標説明中心の疑似表示を除去し、実 overlay 表示に寄せる
- [x] 4.4 01〜21 の Storybook 全ページを横断し、疑似表示が残っていないか確認する

## 5. 完了確認

- [x] 5.1 `just fmt-check`
- [x] 5.2 `just storybook-check`
- [x] 5.3 `RUSTFLAGS="-D warnings" cargo test -p katana-ui-widget`
- [x] 5.4 `just ast-lint`

## 6. Archive 判定

- [x] 6.1 この change の内容は `katana-widget-parity-backlog` と各個別 widget change に吸収済み。
- [x] 6.2 追加実装対象としては 0 と判定し、重複管理を避けるため archive する。
