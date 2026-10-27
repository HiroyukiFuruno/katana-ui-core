## ADDED Requirements

### Requirement: UTF-8論理一致とgrapheme表示範囲の分離
KUC SHALL expose a public projection from valid UTF-8 byte match ranges to complete grapheme display ranges while preserving the original match ranges unchanged. A nonempty match SHALL highlight all intersected graphemes. A valid zero-width match SHALL remain in logical ranges without adding a painted highlight. Multiple ranges SHALL preserve original logical ordering and multiplicity.

#### Scenario: VS16または結合文字だけの合法一致
- **WHEN** a search engine returns bytes 3..6 for the exact text ⭐️, or a subrange of Japanese combining text or a ZWJ emoji
- **THEN** the logical match remains unchanged and the full grapheme is rendered, measured, and hit-tested without replacing or dropping color glyphs

### Requirement: 不正範囲と既存validatorの維持
KUC SHALL return typed errors for reversed ranges, out-of-bounds positions, and invalid UTF-8 boundaries. Existing SearchResultRow highlight grapheme-boundary validation SHALL remain strict.

#### Scenario: 合法一致と不正一致を区別する
- **WHEN** a valid UTF-8 match splits a grapheme
- **THEN** display projection produces a valid whole-grapheme range accepted by the existing validator

#### Scenario: 不正byte位置
- **WHEN** a range is reversed, outside text bounds, or splits a UTF-8 code point
- **THEN** projection fails with a typed error instead of silently discarding the highlight
