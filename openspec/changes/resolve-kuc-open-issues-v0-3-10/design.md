# 設計

- legacy document typographyはv0.3.8までの整数座標の描画・layout契約を維持する。
- fractional baseline typographyを明示したhostだけがlogical line-box cursorを用いる。
- consumer artifactのUnicode evidence pinはKUCが解決する。consumerはfont path/hash/fallbackを渡さない。
- render、wrap/layout、action hit、node hitは同じdocument typography選択を使用する。
