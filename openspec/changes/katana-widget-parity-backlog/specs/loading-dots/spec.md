# LoadingDots Widget Spec

## 概要

アニメーション付きドットインジケーター。Spinner (04) とは異なる「テキスト横の点滅ドット」パターン。

## 出典

- `../katana-chat-ui/crates/katana-chat-ui-floem/src/widget/thinking_indicator.rs`

## 階層配置

`primitive/loading_dots`

## 依存

- Theme tokens のみ（最小依存）

## API 概要（TBD）

- `LoadingDots`: dot_count, dot_size (idle / active), animation_speed_ms, label (Option), tone
