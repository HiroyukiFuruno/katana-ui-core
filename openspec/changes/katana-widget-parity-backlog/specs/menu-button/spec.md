# MenuButton Widget Spec

## 概要

ボタンクリックでドロップダウンメニューを開く widget。framed / unframed variant を持つ。

## 出典

- `../katana/crates/katana-ui/src/widgets/menu_button/`
- `../katana/crates/katana-ui/src/views/app_frame/breadcrumbs.rs` (unframed 利用)

## 階層配置

`composite/button/menu`

## 依存

- SVG Button (05)
- Popover (21)

## API 概要（TBD）

- `MenuButton`: trigger (node), content (node), variant (Framed | Unframed), on_open, on_close
