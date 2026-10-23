## Why
公開0.4.3で、独立code rasterを混ぜたRTL fragmentへ4096pxの内部幅が漏れ、glyphとhitが一致しない（#105）。またsoft wrap選択copyが原文にないLFを挿入する（#104）。

## What Changes
- 幅未指定のrasterをintrinsic layoutとして扱い、RTL glyph/boundsを同じローカル座標へ置く。
- visual wrapとsource paragraph boundaryを分離し、source separatorだけをcopyする。
- 公開APIの型互換性、既存品質gateとsource paragraph separator契約を維持する。

## Impact
KUCのgeneric text raster/selection契約を修正し、patch release v0.4.4として公開する。consumer repoの採用をKUC自身のDoDへ追加しない。
