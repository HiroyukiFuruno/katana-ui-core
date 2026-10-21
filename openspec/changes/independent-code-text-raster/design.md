## Decisions

`UiTreeCanvasRenderer::with_code_text_raster_config(self, PlatformTextRasterConfig, PlatformTextFaceSelection) -> Self` を公開する。code_text が描画・span measurement・折返し・hit で共有される既存設計を利用する。body / document / export renderer と document typography は触らない。code FontToken は保持する。

renderer を使用後に builder を呼べるため、font metrics に依存する scroll_height_cache をクリアする。model schema と既存 constructor は変更しない。独立描画で rich line を分割する際は、Unicode bidi の行レベル visual run 境界と renderer 境界の両方で UTF-8 char boundary を保って分割し、各 fragment の X 座標を visual order で決める。glyph、span background、underline/strikethrough、link hit は同じ fragment 座標を共有する。これにより RTL 本文と LTR inline code の混在行、および一つの plain/code run が複数の visual run をまたぐ行でも visual order と操作領域を維持する。

明示的 bidi embedding / override / isolate が span 境界をまたぐ場合、fragment の raster request だけに開いている制御文字を prefix と対応する閉じ文字を suffix として合成する。source text、selection/copy の grapheme offset、link payload には合成文字を含めない。link span が bidi visual fragment に分割される場合、先頭・末尾の空白を除く既存挙動は元の source span の両端だけに適用し、fragment 間の空白は glyph、装飾、hit の対象に残す。

本文と同じ候補 face を利用する場合、config.monospace_candidates に config.proportional_candidates を設定し、CandidateChain で code にのみ適用する。

独立設定を使用した場合、rich line を plain/code の連続 run に分け、計測と同じ renderer で描画する。独立設定なしでは従来の行全体描画を維持し、既存 constructor の画像互換を保つ。

先頭 run の face に装飾位置を依存させないため、line の raster baseline は body renderer から求める。各 run の glyph は同じ line-box target baseline に整列する。code-first / plain-after の本文 glyph と underline の Y/pixel 同一性を公開 API 回帰で固定する。
