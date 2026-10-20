## Decisions

`UiTreeCanvasRenderer::with_code_text_raster_config(self, PlatformTextRasterConfig, PlatformTextFaceSelection) -> Self` を公開する。code_text が描画・span measurement・折返し・hit で共有される既存設計を利用する。body / document / export renderer と document typography は触らない。code FontToken は保持する。

renderer を使用後に builder を呼べるため、font metrics に依存する scroll_height_cache をクリアする。新規依存・model schema 変更・既存 constructor の変更は不要。

本文と同じ候補 face を利用する場合、config.monospace_candidates に config.proportional_candidates を設定し、CandidateChain で code にのみ適用する。

独立設定を使用した場合、rich line を plain/code の連続 run に分け、計測と同じ renderer で描画する。独立設定なしでは従来の行全体描画を維持し、既存 constructor の画像互換を保つ。

先頭 run の face に装飾位置を依存させないため、line の raster baseline は body renderer から求める。各 run の glyph は同じ line-box target baseline に整列する。code-first / plain-after の本文 glyph と underline の Y/pixel 同一性を公開 API 回帰で固定する。
