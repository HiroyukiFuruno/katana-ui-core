## Overview

ColorPicker は、閉じた状態では小さな色ボタンとして表示され、押すとポップパネルを開く。
ポップパネルは画像1枚目のように、透明チェッカー、色 preview、数値欄、合成方式、色面、色相 slider、alpha slider を持つ。

## API Direction

- `InlineColorPicker`: 小さい色ボタンだけを配置する用途。
- `LabeledColorPicker`: katana の設定行のように label と色ボタンを横並びにする用途。
- `ColorPickerRgba`: RGBA mode を既定にした互換入口。

利用側は以下を指定できる。

- 初期色
- RGB / RGBA mode
- disabled / readonly
- label 幅
- popover placement
- blending mode
- `on_change(ColorPickerValue)` callback

## Visual Requirements

- 透明を示す checker background を表示する。
- 色 preview は alpha を反映する。
- `U8` 表示を置く。
- スポイト icon を置く。初期実装で OS の色取得ができない場合も、slot と callback は必ず用意する。
- R/G/B/A の数値欄を表示する。
- Blending は Normal / Additive を選択できる。
- 色面は hue と saturation / value の変化を視覚的に示す。
- 色相 slider と alpha slider を持つ。
- 現在位置を示すドラッグハンドルを表示する。
- dark / light で border、text、icon が theme token に追従する。

## Acceptance Strategy

画像1枚目の UI は要素ごとに受け入れ判定する。
見た目だけを似せた静的表示は不可とし、各操作が `on_change` と preview に反映されることを Storybook で確認する。

## Out of Scope

- OS 全体の色を拾うスポイト機能の完全実装は、初期 scope では callback slot までを必須とする。
- 高度なカラーマネジメントは扱わない。
