## Why

本文の System face 選択を保ちながら code span の物理 face を独立設定する必要がある。意味情報 monospace を落とす回避は採用しない。Issue: https://github.com/HiroyukiFuruno/katana-ui-core/issues/86

## What Changes

- UiTreeCanvasRenderer に with_code_text_raster_config builder を追加する。
- code renderer の font token と本文設定を維持して raster config / face selection のみ置き換える。
- 既存 scroll measurement cache を無効化する。

## Impact

公開 raster-host API、内部 TextRenderer、公開 API 契約テスト。既存 constructor と既定動作は不変。
